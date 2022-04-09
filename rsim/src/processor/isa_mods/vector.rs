#![allow(non_camel_case_types)]

use std::marker::PhantomData;
use crate::processor::isa_mods::*;
use crate::processor::exceptions::IllegalInstructionException::*;
use super::csrs::CSRProvider;
use std::cmp::min;
use anyhow::{Context, Result};
use std::convert::{TryInto};

use crate::processor::decode::{Opcode,InstructionBits};

mod types;
use types::*;
mod conns;
use conns::*;
pub use conns::{Rv32vConn,Rv64vCheriConn};


/// The Vector Unit for the processor.
/// Stores all vector state, including registers.
/// Call [Rv32v::exec_inst()] on it when you encounter a vector instruction.
/// This requires a [VecMemInterface<uXLEN>] to access other resources.
pub struct Rvv<uXLEN: PossibleXlen> {
    // TODO use a RegisterFile for this?
    vreg: [uVLEN; 32],

    vtype: VType,
    vl: u32,

    /// This is used by the hardware to support resuming vector instructions after traps.
    /// e.g. if a vector load hits a page fault at element #N, set vstart to N before taking the trap,
    /// and the load will resume from vstart when you get back.
    /// Reset to zero after every vector load instruction.
    /// 
    /// This potentially impacts fast paths, 
    /// e.g. if a fast-path load pulls full lines from memory into a vector register, vstart must be 0.
    vstart: u32,

    _phantom_xlen: PhantomData<uXLEN>,
}
pub type Rv32v = Rvv<u32>;
pub type Rv64v = Rvv<u64>;


impl<uXLEN: PossibleXlen> Rvv<uXLEN> {
    /// Returns an initialized vector unit.
    pub fn new() -> Self {
        Rvv {
            vreg: [0; 32],

            vtype: VType::illegal(),
            vl: 0,
            vstart: 0,

            _phantom_xlen: PhantomData,
        }
    }

    /// Reset the vector unit's state
    pub fn reset(&mut self) {
        self.vreg = [0; 32];
        self.vtype = VType::illegal();
        self.vl = 0;
        self.vstart = 0;
    }

    /// (Internal) Execute a configuration instruction, e.g. vsetvli family
    /// Requires a [Rv32vConn].
    /// 
    /// # Arguments
    /// 
    /// * `inst_kind` - Which kind of configuration instruction to execute
    /// * `inst` - Decoded instruction bits
    /// * `conn` - Connection to external resources
    fn exec_config(&mut self, inst_kind: ConfigKind, inst: InstructionBits, conn: &mut dyn VecMemInterface<uXLEN>) -> Result<()> {
        if let InstructionBits::VType{rd, funct3, rs1, rs2, zimm11, zimm10, ..} = inst {
            assert_eq!(funct3, 0b111);

            // avl = application vector length
            // e.g. the total number of elements to process
            // Either read it from a register, or from an immediate.
            // See Section 6.2 of the spec.
            let avl = match inst_kind {
                ConfigKind::vsetvli | ConfigKind::vsetvl => { // vsetvli, vsetvl
                    // Read AVL from a register
                    if rs1 != 0 {
                        // default case, just read it out
                        conn.sreg_read_xlen(rs1)?.into()
                    } else {
                        if rd != 0 {
                            // rs1 == 0, rd != 0
                            // => set the AVL to the maximum possible value,
                            // use that to calculate the maximum number of elements in this configuration,
                            // which will get written out to rd.
                            u64::MAX
                        } else {
                            // Request the same vector length, even if the vtype is changing.
                            self.vl as u64
                        }
                    }
                } ,
                ConfigKind::vsetivli => { // vsetivli
                    // Read AVL from an immediate
                    // Use rs1 as a 5-bit immediate
                    rs1 as u64
                }
            };

            // Depending on the instruction, the vtype selection is different
            // See RISC-V V spec, section 6
            let vtype_bits = match inst_kind {
                ConfigKind::vsetvli => {
                    zimm11 as u64
                },
                ConfigKind::vsetivli => {
                    zimm10 as u64
                },
                ConfigKind::vsetvl => {
                    conn.sreg_read_xlen(rs2)?.into()
                },
            };
            // Try to parse vtype bits
            let req_vtype = VType::decode(vtype_bits as u32)?;

            // Calculate the maximum number of elements per register group
            // (under some configurations, e.g. Sew=8,Lmul=1/4,Vlen=32, this could be < 1 which is illegal)
            let elems_per_group = req_vtype.elems_per_group();

            let vtype_supported = elems_per_group > 0 && 
                req_vtype.vsew != Sew::e64 &&  // ELEN = 32, we don't support larger elements
                match req_vtype.vlmul {
                    Lmul::eEighth => false, // As per the spec (section 3.4.2) we aren't required to support Lmul = 1/8
                    _ => true
                };

            if vtype_supported {
                self.vtype = req_vtype;
                // dbg!(avl, elems_per_group);
                // TODO - section 6.3 shows more constraints on setting VL
                self.vl = min(elems_per_group, avl as u32);

                conn.sreg_write_xlen(rd, self.vl.into())?;
            } else {
                self.vtype = VType::illegal();
                // TODO - move this bail to the next vector instruction that executes
                // Setting vtype to an illegal type is fine, but trying to do anything (other than reconfigure) with invalid vtype isn't
                bail!("Valid but unsupported vtype: {:b} -> {:?}, elems_per_group {}", vtype_bits, req_vtype, elems_per_group);
            }

            Ok(())
        } else {
            unreachable!("vector::exec_config instruction MUST be InstructionBits::VType, got {:?} instead", inst);
        }
    }

    /// Load a value of width `eew` from a given address `addr` 
    /// into a specific element `idx_from_base` of a vector register group starting at `vd_base`
    fn load_to_vreg(&mut self, conn: &mut dyn VecMemInterface<uXLEN>, eew: Sew, addr_provenance: (u64, Provenance), vd_base: u8, idx_from_base: u32) -> Result<()> {
        let val = conn.load_from_memory(eew, addr_provenance)?;
        self.store_vreg_elem(eew, vd_base, idx_from_base, val as uELEN)?;
        Ok(())
    }
    /// Stores a value of width `eew` from a specific element `idx_from_base` of a 
    /// vector register group starting at `vd_base` into a given address `addr` 
    fn store_to_mem(&mut self, conn: &mut dyn VecMemInterface<uXLEN>, eew: Sew, addr_provenance: (u64, Provenance), vd_base: u8, idx_from_base: u32) -> Result<()> {
        let val = self.load_vreg_elem(eew, vd_base, idx_from_base)?;
        conn.store_to_memory(eew, val, addr_provenance)?;
        Ok(())
    }

    /// Store a value in an element in a vertex register group, with specified EEW.
    /// Requires the type of the value to store matches the EEW.
    /// 
    /// Example: if EEW=32bits, VLEN=128bits (4 32-bit elements per register), `vd_base` = 3, `idx_from_base` = 5,
    /// the actual `vd` = 3 + (idx_from_base / 4) = 4, and
    /// the actual `idx` = idx_from_base % 4 = 1.
    /// This would store `val` into v4\[64:32\] (element 1 of v4)
    fn store_vreg_elem(&mut self, eew: Sew, vd_base: u8, idx_from_base: u32, val: uELEN) -> Result<()> {
        let (elem_width_mask, elem_width) : (uELEN, u32) = match eew {
            Sew::e8  => (0xFF, 8),
            Sew::e16 => (0xFFFF, 16),
            Sew::e32 => (0xFFFF_FFFF, 32),
            Sew::e64 => bail!("64-bit vreg elem unsupported")
        };
        // Assert the value doesn't have more data
        assert_eq!(val & (!elem_width_mask), 0);

        // TODO refactor to use shifting
        let elems_per_v: u32 = (VLEN as u32)/elem_width;
        let vd: u8 = (vd_base as u32 + (idx_from_base / elems_per_v)).try_into()
            .context(format!("calculating destination register for vd_base={},idx_from_base={},eew={:?}", vd_base, idx_from_base, eew))?;
        let idx = idx_from_base % elems_per_v;

        // Get the previous value for the vector
        let old_value = self.vreg[vd as usize];
        // Mask off the element we want to write
        let mask = (elem_width_mask as uVLEN) << (elem_width*idx);
        let old_value_with_element_removed = old_value & (!mask);
        // Create a uVLEN value with just the new element, shifted into the right place
        let new_element_shifted = (val as uVLEN) << (elem_width*idx);
        // Combine (old value sans element) with (new element)
        let new_value = old_value_with_element_removed | new_element_shifted;

        self.vreg[vd as usize] = new_value;

        Ok(())
    }

    /// Load a value from an element in a vertex register group, with specified EEW
    /// Requires the type of the value to store matches the EEW.
    /// 
    /// Example: if EEW=32bits, VLEN=128bits (4 32-bit elements per register), `vd` = 3, `idx` = 5,
    /// the actual `vd` = 3 + (idx_from_base / 4) = 4, and
    /// the actual `idx` = idx_from_base % 4 = 1.
    /// this would return v4\[64:32\] (element 1 of v4)
    fn load_vreg_elem(&self, eew: Sew, vd_base: u8, idx_from_base: u32) -> Result<uELEN> {
        let (elem_width_mask, elem_width) : (uELEN, u32) = match eew {
            Sew::e8  => (0xFF, 8),
            Sew::e16 => (0xFFFF, 16),
            Sew::e32 => (0xFFFF_FFFF, 32),
            Sew::e64 => bail!("64-bit vreg elem unsupported")
        };

        // TODO refactor to use shifting
        let elems_per_v: u32 = (VLEN as u32)/elem_width;
        let vd: u8 = (vd_base as u32 + (idx_from_base / elems_per_v)).try_into()
            .context(format!("calculating destination register for vd_base={},idx_from_base={},eew={:?}", vd_base, idx_from_base, eew))?;
        let idx = idx_from_base % elems_per_v;

        let full_reg = self.vreg[vd as usize];
        // Shift the register down so the new element is at the bottom,
        // and mask off the other elements
        let individual_elem = (full_reg >> (elem_width*idx)) & (elem_width_mask as uVLEN);

        // Convert the element to the expected type and return
        Ok(individual_elem as uELEN)
    }

    /// Dump vector unit state to standard output.
    pub fn dump(&self) {
        for i in 0..32 {
            println!("v{} = 0x{:032x}", i, self.vreg[i]);
        }
        println!("vl: {}\nvtype: {:?}", self.vl, self.vtype);
    }
}

impl<uXLEN: PossibleXlen> IsaMod<&mut dyn VecMemInterface<uXLEN>> for Rvv<uXLEN> {
    type Pc = ();
    fn will_handle(&self, opcode: Opcode, inst: InstructionBits) -> bool {
        use crate::processor::decode::Opcode::*;
        match (opcode, inst) {
            // Delegate all instructions under the Vector opcode to the vector unit
            (Vector, _) => true,

            (LoadFP | StoreFP, InstructionBits::FLdStType{width, ..}) => {
                // Check the access width
                match width {
                    0b0001 | 0b0010 | 0b0011 | 0b0100 => false,
                    0b1000..=0b1111 => false,

                    // This width corresponds to a vector, delegate this instruction to the vector unit
                    _ => true
                }
            },

            _ => false
        }
    }
    
    /// Execute a vector-specific instruction, e.g. vector arithmetic, loads, configuration
    /// Requires a [Rv32vConn].
    /// 
    /// # Arguments
    /// 
    /// * `opcode` - The major opcode of the instruction
    /// * `inst` - Decoded instruction bits
    /// * `inst_bits` - Raw instruction bits (TODO - we shouldn't need this)
    /// * `conn` - Connection to external resources
    fn execute(&mut self, opcode: Opcode, inst: InstructionBits, inst_bits: u32, conn: &mut dyn VecMemInterface<uXLEN>) -> ProcessorResult<Option<()>> {
        use Opcode::*;
        match (opcode, inst) {
            (Vector, InstructionBits::VType{funct3, funct6, rs1, rs2, rd, vm, ..}) => {
                match funct3 {
                    0b111 => {
                        // Configuration family - vsetvli etc.
                        let inst_kind = match bits!(inst_bits, 30:31) {
                            0b00 | 0b01 => ConfigKind::vsetvli,
                            0b11 => ConfigKind::vsetivli,
                            0b10 => ConfigKind::vsetvl,

                            invalid => panic!("impossible top 2 bits {:2b}", invalid)
                        };
                        self.exec_config(inst_kind, inst, conn)?
                    }

                    // 0b000 => {
                    //     // Vector-Vector int
                    // }

                    // 0b010 => {
                    //     // Vector-Vector Move
                    // }

                    0b011 => {
                        // Vector-immediate
                        // TODO - this assumes no sign extending?
                        let imm = rs1 as u32;

                        if self.vtype.vsew != Sew::e32 {
                            bail!(UnsupportedParam(format!("Sew {:?} != 32 for arithmetic not supported", self.vtype.vsew)));
                        }

                        match funct6 {
                            0b011000 => {
                                // VMSEQ
                                // Mask Set-if-EQual
                                // This cannot itself be masked
                                let mut val: uVLEN = 0;
                                for i in self.vstart..self.vl {
                                    let reg_val = self.load_vreg_elem(Sew::e32, rs2, i)?;
                                    if reg_val == imm {
                                        val |= (1 as uVLEN) << i;
                                    }
                                }
                                self.vreg[rd as usize] = val;
                            }
                            0b011001 => {
                                // VMSNE
                                // Mask Set-if-Not-Equal
                                // This cannot itself be masked
                                let mut val: uVLEN = 0;
                                for i in self.vstart..self.vl {
                                    if self.load_vreg_elem(Sew::e32, rs2, i)? != imm {
                                        val |= (1 as uVLEN) << i;
                                    }
                                }
                                self.vreg[rd as usize] = val;
                            }

                            0b010111 => {
                                if (!vm) && rd == 0 {
                                    bail!(UnsupportedParam("Can't handle vmerge on the mask register, because it uses the mask register :)".to_string()));
                                }

                                // vmerge or vmv
                                // if masked, vmerge, else vmv
                                for i in self.vstart..self.vl {
                                    let val = if self.idx_masked_out(vm, i as usize) {
                                        // if masked out, this must be vmerge, write new value in
                                        self.load_vreg_elem(Sew::e32, rs2, i)?
                                    } else {
                                        // either vmerge + not masked, or vmv
                                        // either way, write immediate
                                        imm
                                    };
                                    self.store_vreg_elem(Sew::e32, rd, i, val)?;
                                }
                            }

                            0b100111 => {
                                if vm == true {
                                    // vmv<nr>r.v (section 16.6)
                                    // copy whole registers/register groups

                                    // By the spec, nr = simm5?
                                    // No such field, but section 11.8  mentions it.
                                    // I imagine it's a leftover from a previous draft.
                                    // rs1 looks right for this case, but need to double check.
                                    let nr = rs1 as usize + 1;
                                    let emul = match nr {
                                        1 => Lmul::e1,
                                        2 => Lmul::e2,
                                        4 => Lmul::e4,
                                        8 => Lmul::e8,

                                        _ => bail!(UnsupportedParam(format!("Invalid nr encoding in vmv<nr>r.v: nr = {}", nr)))
                                    };

                                    let eew = self.vtype.vsew;

                                    let evl = val_times_lmul_over_sew(VLEN as u32, eew, emul);
                                    if self.vstart >= evl {
                                        bail!(UnsupportedParam(format!("evl {} <= vstart {} therefore vector move is no op", evl, self.vstart)))
                                    }
                                    if rd == rs2 {
                                        // architetural no-op
                                        return Ok(None)
                                    }

                                    for vx in 0..nr {
                                        self.vreg[rd as usize + vx] = self.vreg[rs2 as usize + vx];
                                    }
                                } else {
                                    bail!(UnimplementedInstruction("vsmul"));
                                }
                            }

                            _ => bail!(MiscDecodeException(format!(
                                    "Vector arithmetic funct3 {:03b} funct6 {:06b} not yet handled", funct3, funct6)
                            ))
                        }
                    }

                    _ => bail!(UnsupportedParam(format!("Vector arithmetic funct3 {:03b} currently not supported", funct3)))
                }
            }

            (LoadFP | StoreFP, InstructionBits::FLdStType{rd, rs1, rs2, vm, ..}) => {
                let op = DecodedMemOp::decode_load_store(opcode, inst, self.vtype, self.vl, conn)?;
                use MemOpDir::*;

                let (op_emul, op_eew) = op.access_params();

                if op.dir() == Load && (!vm) && rd == 0 {
                    // If we're masked, we can't load over v0 as that's the mask register
                    bail!("Masked instruction cannot load into v0");
                }

                if let Some(evl) = op.try_get_evl() {
                    if evl <= self.vstart {
                        println!("EVL {} <= vstart {} => vector {:?} is no-op", evl, self.vstart, op.dir());
                        return Ok(None)
                    }
                }

                let (base_addr, provenance) = conn.get_addr_provenance(rs1)?;

                let addr_base_step = match op_eew {
                    Sew::e8 => 1,
                    Sew::e16 => 2,
                    Sew::e32 => 4,
                    Sew::e64 => bail!("unsupported {:?} in vector load/store", op_eew),
                };

                let elems_per_group = val_times_lmul_over_sew(VLEN as u32, op_eew, op_emul);

                use DecodedMemOp::*;
                match op {
                    Strided{dir: Load, stride, evl, nf, eew, ..} => {
                        // i = element index in logical vector (which includes groups)
                        let mut addr = base_addr;
                        // For each segment
                        for i in self.vstart..evl {
                            // For each field
                            for i_field in 0..nf {
                                // If we aren't masked out...
                                if !self.idx_masked_out(vm, i as usize) {
                                    // ... load from memory into register
                                    let addr_p = (addr, provenance);
                                    self.load_to_vreg(conn, eew, addr_p, rd, i + (i_field as u32 * elems_per_group))?;
                                }
                                // Either way increment the address
                                addr += addr_base_step * stride;
                            }
                        }
                    }
                    Strided{dir: Store, stride, evl, nf, eew, ..} => {
                        // i = element index in logical vector (which includes groups)
                        let mut addr = base_addr;
                        // For each segment
                        for i in self.vstart..evl {
                            // For each field
                            for i_field in 0..nf {
                                // If we aren't masked out...
                                if !self.idx_masked_out(vm, i as usize) {
                                    // ... store from register into memory
                                    let addr_p = (addr, provenance);
                                    self.store_to_mem(conn, eew, addr_p, rd, i + (i_field as u32 * elems_per_group))?;
                                }
                                // Either way increment the address
                                addr += addr_base_step * stride;
                            }
                        }
                    }
                    Indexed{dir: Load, ordered: _, index_ew, evl, nf, eew, ..} => {
                        if index_ew != Sew::e32 {
                            bail!("Indexed Load with index width != 32 not supported")
                        }
                        if nf > 1 {
                            bail!("Indexed Load with NFIELDS != 1 ({}) not supported", nf);
                        }

                        // i = element index in logical vector (which includes groups)
                        for i in self.vstart..evl {
                            // Get our index
                            let idx = self.load_vreg_elem(Sew::e32, rs2, i)?;
                            let addr = base_addr + addr_base_step * (idx as u64);

                            // If we aren't masked out...
                            if !self.idx_masked_out(vm, i as usize) {
                                // ... load from memory(idx) into register(i)
                                let addr_p = (addr, provenance);
                                self.load_to_vreg(conn, eew, addr_p, rd, i)?;
                            }
                        }
                    }
                    Indexed{dir: Store, ordered: _, index_ew, evl, nf, eew, ..} => {
                        if index_ew != Sew::e32 {
                            bail!("Indexed Store with index width != 32 not supported")
                        }
                        if nf > 1 {
                            bail!("Indexed Store with NFIELDS != 1 ({}) not supported", nf);
                        }

                        // i = element index in logical vector (which includes groups)
                        for i in self.vstart..evl {
                            // Get our index
                            let idx = self.load_vreg_elem(Sew::e32, rs2, i)?;
                            let addr = base_addr + addr_base_step * (idx as u64);

                            // If we aren't masked out...
                            if !self.idx_masked_out(vm, i as usize) {
                                // ... store from register(i) to memory(idx)
                                let addr_p = (addr, provenance);
                                self.store_to_mem(conn, eew, addr_p, rd, i)?;
                            }
                        }
                    }
                    FaultOnlyFirst{evl, nf, eew, ..} => {
                        // FaultOnlyFirst loads can be strided 
                        // (https://github.com/riscv/riscv-opcodes/blob/master/opcodes-rvv, non-zero NF is allowed)

                        let stride = 1;
                        // i = element index in logical vector (which includes groups)
                        let mut addr = base_addr;
                        // For each segment
                        'top_loop: for i in self.vstart..evl {
                            // For each field
                            for i_field in 0..nf {
                                // If we aren't masked out...
                                if !self.idx_masked_out(vm, i as usize) {
                                    // ... load from memory into register
                                    let addr_p = (addr, provenance);
                                    let load_fault: Result<()> = 
                                        self.load_to_vreg(conn, eew, addr_p, rd, i + (i_field as u32 * elems_per_group));
                                    
                                    if i == 0 {
                                        // Any potentially faulted load should fault as normal if i == 0
                                        load_fault?;
                                    } else if load_fault.is_err() {
                                        use crate::processor::exceptions::MemoryException;
                                        // There was *some* error from the load, check if it was a memory fault
                                        let load_err = load_fault.unwrap_err();
                                        // Only shrink the vlen if it's a MemError related to an invalid address
                                        let error_reduces_vlen = match load_err.downcast_ref::<MemoryException>() {
                                            Some(MemoryException::AddressUnmapped{..}) => true,
                                            _ => false
                                        };
                                        if error_reduces_vlen {
                                            // "vector length vl is reduced to the index of the 
                                            // element that would have raised an exception"
                                            self.vl = i;
                                            // error received, finish instruction
                                            break 'top_loop;
                                        } else {
                                            // Re-raise error
                                            return Err(load_err)
                                        }
                                    }
                                }
                                // Either way increment the address
                                addr += addr_base_step * stride;
                            }
                        }
                    }
                    WholeRegister{dir: Load, nf, emul: _} => {
                        let mut addr = base_addr;

                        let eew = Sew::e8;
                        let vl = (nf as u32) * ((VLEN/8) as u32);
                        let addr_base_step = 1;
                        // vstart is ignored, except for the vstart >= evl case above
                        // NFIELDS doesn't behave as normal here - it's integrated into EVL at decode stage
                        for i in 0..vl {
                            // We can't be masked out.
                            // Load from memory into register
                            // dbg!("ld", i, addr);
                            let addr_p = (addr, provenance);
                            self.load_to_vreg(conn, eew, addr_p, rd, i)?;

                            addr += addr_base_step;
                        }
                    }
                    WholeRegister{dir: Store, nf, emul: _} => {
                        let mut addr = base_addr;

                        let eew = Sew::e8;
                        let vl = (nf as u32) * ((VLEN/8) as u32);
                        let addr_base_step = 1;
                        // vstart is ignored, except for the vstart >= evl case above
                        // NFIELDS doesn't behave as normal here - it's integrated into EVL at decode stage
                        for i in 0..vl {
                            // We can't be masked out.
                            // Store from register into memory
                            // dbg!("st", i, addr);
                            let addr_p = (addr, provenance);
                            self.store_to_mem(conn, eew, addr_p, rd, i)?;

                            addr += addr_base_step;
                        }
                    }
                    ByteMask{dir: Load, evl, emul: _} => {
                        if vm == false {
                            // vlm, vsm cannot be masked out
                            bail!("ByteMask operations cannot be masked")
                        }

                        let mut addr = base_addr;
                        for i in self.vstart..evl {
                            let addr_p = (addr, provenance);
                            self.load_to_vreg(conn, Sew::e8, addr_p, rd, i)?;

                            // Increment the address
                            addr += addr_base_step;
                        }
                    }
                    ByteMask{dir: Store, evl, emul: _} => {
                        if vm == false {
                            // As above, vlm, vsm cannot be masked out
                            bail!("ByteMask operations cannot be masked")
                        }

                        let mut addr = base_addr;
                        for i in self.vstart..evl {
                            let addr_p = (addr, provenance);
                            self.store_to_mem(conn, Sew::e8, addr_p, rd, i)?;

                            // Increment the address
                            addr += addr_base_step;
                        }
                    }
                }
            }

            _ => bail!("Unexpected opcode/InstructionBits pair at vector unit")
        }

        // If we get this far, the vector instruction has completed
        // As per RVVspec 3.7, we "reset the vstart CSR to zero at the end of execution"
        self.vstart = 0;

        Ok(None)
    }
}

impl<uXLEN: PossibleXlen> Rvv<uXLEN> {
    /// Returns true if the mask is enabled and element `i` has been masked *out*, e.g. that it should not be touched.
    fn idx_masked_out(&self, vm: bool, i: usize) -> bool {
        // vm == 1 for mask disabled, 0 for mask enabled
        (!vm) && (bits!(self.vreg[0], i:i) == 0)
    }
}

impl<uXLEN: PossibleXlen> CSRProvider<u32> for Rvv<uXLEN> {
    fn has_csr(&self, csr: u32) -> bool {
        match csr {
            // Should be implemented, aren't yet
            0x008 | 0x009 | 0x00A | 0x00F => todo!(),

            0xC20 | 0xC21 | 0xC22 => true,

            _ => false
        }
    }

    fn csr_atomic_read_write(&mut self, csr: u32, _need_read: bool, _write_val: u32) -> Result<Option<u32>> {
        match csr {
            0xC20 | 0xC21 | 0xC22 => bail!("CSR 0x{:04x} is read-only, cannot atomic read/write", csr),
            _ => todo!()
        }
    }

    fn csr_atomic_read_set(&mut self, csr: u32, set_bits: Option<u32>) -> Result<u32> {
        if set_bits != None {
            match csr {
                0xC20 | 0xC21 | 0xC22 => bail!("CSR 0x{:04x} is read-only, cannot atomic set", csr),
                _ => todo!()
            }
        } else {
            match csr {
                0xC20 => Ok(self.vl),
                0xC21 => Ok(self.vtype.encode()),
                0xC22 => Ok((VLEN/8) as u32),

                _ => todo!()
            }
        }
    }
    fn csr_atomic_read_clear(&mut self, _csr: u32, _clear_bits: Option<u32>) -> Result<u32> {
        todo!()
    }
}
