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
                let op = self.decode_load_store(opcode, inst, conn)?;
                use MemOpDir::*;

                if op.dir == Load && (!vm) && rd == 0 {
                    // If we're masked, we can't load over v0 as that's the mask register
                    bail!("Masked instruction cannot load into v0");
                }

                if op.evl <= self.vstart {
                    println!("EVL {} <= vstart {} => vector {:?} is no-op", op.evl, self.vstart, op.dir);
                    return Ok(None)
                }

                let (base_addr, provenance) = conn.get_addr_provenance(rs1)?;

                let addr_base_step = match op.eew {
                    Sew::e8 => 1,
                    Sew::e16 => 2,
                    Sew::e32 => 4,
                    Sew::e64 => bail!("unsupported {:?} in vector load/store", op.eew),
                };

                let elems_per_group = val_times_lmul_over_sew(VLEN as u32, op.eew, op.emul);

                use OverallMemOpKind::*;
                match (op.dir, op.kind) {
                    (Load, Strided(stride)) => {
                        if stride > 1 && op.nf > 1 {
                            println!("Non-unit stride Load with NFIELDS != 1 ({}) not checked", op.nf);
                        }

                        // i = element index in logical vector (which includes groups)
                        let mut addr = base_addr;
                        // For each segment
                        for i in self.vstart..op.evl {
                            // For each field
                            for i_field in 0..op.nf {
                                // If we aren't masked out...
                                if !self.idx_masked_out(vm, i as usize) {
                                    // ... load from memory into register
                                    let addr_p = (addr, provenance);
                                    self.load_to_vreg(conn, op.eew, addr_p, rd, i + (i_field as u32 * elems_per_group))?;
                                }
                                // Either way increment the address
                                addr += addr_base_step * stride;
                            }
                        }
                    }
                    (Store, Strided(stride)) => {
                        if stride > 1 && op.nf > 1 {
                            println!("Non-unit stride Store with NFIELDS != 1 ({}) not checked", op.nf);
                        }

                        // i = element index in logical vector (which includes groups)
                        let mut addr = base_addr;
                        // For each segment
                        for i in self.vstart..op.evl {
                            // For each field
                            for i_field in 0..op.nf {
                                // If we aren't masked out...
                                if !self.idx_masked_out(vm, i as usize) {
                                    // ... store from register into memory
                                    let addr_p = (addr, provenance);
                                    self.store_to_mem(conn, op.eew, addr_p, rd, i + (i_field as u32 * elems_per_group))?;
                                }
                                // Either way increment the address
                                addr += addr_base_step * stride;
                            }
                        }
                    }
                    (Load, Indexed{ordered: _ordered, index_ew}) => {
                        if index_ew != Sew::e32 {
                            bail!("Indexed Load with index width != 32 not supported")
                        }
                        if op.nf > 1 {
                            bail!("Indexed Load with NFIELDS != 1 ({}) not supported", op.nf);
                        }

                        // i = element index in logical vector (which includes groups)
                        for i in self.vstart..op.evl {
                            // Get our index
                            let idx = self.load_vreg_elem(Sew::e32, rs2, i)?;
                            let addr = base_addr + addr_base_step * (idx as u64);

                            // If we aren't masked out...
                            if !self.idx_masked_out(vm, i as usize) {
                                // ... load from memory(idx) into register(i)
                                let addr_p = (addr, provenance);
                                self.load_to_vreg(conn, op.eew, addr_p, rd, i)?;
                            }
                        }
                    }
                    (Store, Indexed{ordered: _ordered, index_ew}) => {
                        if index_ew != Sew::e32 {
                            bail!("Indexed Store with index width != 32 not supported")
                        }
                        if op.nf > 1 {
                            bail!("Indexed Store with NFIELDS != 1 ({}) not supported", op.nf);
                        }

                        // i = element index in logical vector (which includes groups)
                        for i in self.vstart..op.evl {
                            // Get our index
                            let idx = self.load_vreg_elem(Sew::e32, rs2, i)?;
                            let addr = base_addr + addr_base_step * (idx as u64);

                            // If we aren't masked out...
                            if !self.idx_masked_out(vm, i as usize) {
                                // ... store from register(i) to memory(idx)
                                let addr_p = (addr, provenance);
                                self.store_to_mem(conn, op.eew, addr_p, rd, i)?;
                            }
                        }
                    }
                    (Load, FaultOnlyFirst) => {
                        // FaultOnlyFirst loads can be strided 
                        // (https://github.com/riscv/riscv-opcodes/blob/master/opcodes-rvv, non-zero NF is allowed)

                        let stride = 1;
                        // i = element index in logical vector (which includes groups)
                        let mut addr = base_addr;
                        // For each segment
                        'top_loop: for i in self.vstart..op.evl {
                            // For each field
                            for i_field in 0..op.nf {
                                // If we aren't masked out...
                                if !self.idx_masked_out(vm, i as usize) {
                                    // ... load from memory into register
                                    let addr_p = (addr, provenance);
                                    let load_fault: Result<()> = 
                                        self.load_to_vreg(conn, op.eew, addr_p, rd, i + (i_field as u32 * elems_per_group));
                                    
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
                    (Load, WholeRegister) => {
                        let mut addr = base_addr;

                        // TODO refactor decoding to make this unnecessary
                        let eew = Sew::e8;
                        let vl = (op.nf as u32) * ((VLEN/8) as u32);
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
                    (Store, WholeRegister) => {
                        let mut addr = base_addr;

                        // TODO refactor decoding to make this unnecessary
                        let eew = Sew::e8;
                        let vl = (op.nf as u32) * ((VLEN/8) as u32);
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
                    (Load, ByteMask) => {
                        if op.eew != Sew::e8 {
                            bail!("ByteMask {:?} must have EEW = byte", op.eew);
                        }
                        if op.nf != 1 {
                            // See https://github.com/riscv/riscv-opcodes/blob/master/opcodes-rvv
                            // vlm.v, vsm.v both specify the top bits (i.e. nf) = 0
                            // this gets incremented by the decode so it must be 1
                            bail!("NF for ByteMask operations must == 1");
                        }
                        if vm == false {
                            // As above, vlm, vsm cannot be masked out
                            bail!("ByteMask operations cannot be masked")
                        }

                        let mut addr = base_addr;
                        for i in self.vstart..op.evl {
                            let addr_p = (addr, provenance);
                            self.load_to_vreg(conn, op.eew, addr_p, rd, i)?;

                            // Increment the address
                            addr += addr_base_step;
                        }
                    }
                    (Store, ByteMask) => {
                        if op.eew != Sew::e8 {
                            bail!("ByteMask {:?} must have EEW = byte", op.eew);
                        }
                        if op.nf != 1 {
                            // See https://github.com/riscv/riscv-opcodes/blob/master/opcodes-rvv
                            // vlm.v, vsm.v both specify the top bits (i.e. nf) = 0
                            // this gets incremented by the decode so it must be 1
                            bail!("NF for ByteMask operations must == 1");
                        }
                        if vm == false {
                            // As above, vlm, vsm cannot be masked out
                            bail!("ByteMask operations cannot be masked")
                        }

                        let mut addr = base_addr;
                        for i in self.vstart..op.evl {
                            let addr_p = (addr, provenance);
                            self.store_to_mem(conn, op.eew, addr_p, rd, i)?;

                            // Increment the address
                            addr += addr_base_step;
                        }
                    }

                    _ => bail!("vector memory op {:?} {:?} not yet supported", op.dir, op.kind)
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

    /// Decode a Load/Store opcode into an OverallMemOp structure.
    /// Performs all checks to ensure the instruction is a valid RISC-V V vector load/store.
    fn decode_load_store(&self, opcode: Opcode, inst: InstructionBits, conn: &mut dyn VecMemInterface<uXLEN>) -> Result<OverallMemOp> {
        if let InstructionBits::FLdStType{width, rs2, mew, mop, nf, ..} = inst {
            // MEW = Memory Expanded Width(?)
            // Expected to be used for larger widths, because it's right next to the width field,
            // but for now it has to be 0
            if mew { bail!("LoadFP with mew = 1 is reserved") }
    
            // Get the element width we want to use (which is NOT the same as the one encoded in vtype)
            // EEW = Effective Element Width
            let eew_num = match width {
                0b0001 | 0b0010 | 0b0011 | 0b0100 => bail!("LoadFP uses width for normal floats, not vectors"),
                0b1000..=0b1111 => bail!("LoadFP using reserved width {}", width),
    
                0b0000 => 8,
                0b0101 => 16,
                0b0110 => 32,
                0b0111 => 64,
    
                _ => bail!("LoadFP has impossible width {}", width)
            };
    
            if eew_num == 64 {
                // We are allowed to reject values of EEW that aren't supported for SEW in vtype
                // (see section 7.3 of RISC-V V spec)
                bail!("effective element width of 64 is not supported");
            }
    
            // Check the effective element width is valid, given the current SEW and LMUL
    
            // EMUL = Effective LMUL
            // because LMULs can be as small as 1/8th, evaluate it as an integer * 8 (effectively 29.3 fixed point)
            let emul_times_8 = self.vtype.val_times_lmul_over_sew(eew_num * 8);
    
            // Limit EMUL to the same values as LMUL
            if emul_times_8 > 64 || emul_times_8 <= 1 {
                bail!("emul * 8 too big or too small: {}", emul_times_8);
            }
    
            // NF = Number of Fields
            // If NF > 1, it's a *segmented* load/store
            // where "packed contiguous segments" are moved into "multiple destination vector register groups"
            // For example
            // a0 => rgbrgbrgbrgbrgb (24-bit pixels, 8-bits-per-component)
            // vlseg3e8 v8, (a0) ; NF = 3, EEW = 8
            //  ->  v8  = rrrr
            //      v9  = gggg
            //      v10 = bbbb
            let nf = nf + 1;
    
            // EMUL * NF = number of underlying registers in use
            // => EMUL * NF should be <= 8
            if (emul_times_8 * (nf as u32)) > 64 {
                bail!("emul * nf too big: {}", emul_times_8 * (nf as u32) / 8);
            }
    
            // Convert EEW, EMUL to enums
            let eew = match eew_num {
                8  => Sew::e8,
                16 => Sew::e16,
                32 => Sew::e32,
                64 => Sew::e64,
                _ => bail!("Impossible EEW {}", eew_num)
            };
            let emul = match emul_times_8 {
                1 => Lmul::eEighth,
                2 => Lmul::eQuarter,
                4 => Lmul::eHalf,
                8 => Lmul::e1,
                16 => Lmul::e2,
                32 => Lmul::e4,
                64 => Lmul::e8,
                _ => bail!("Impossible EMUL-times-8 {}", emul_times_8)
            };
    
            // MOP = Memory OPeration
            // Determines indexing mode
            let mop = match mop {
                0b00 => Mop::UnitStride,
                0b10 => Mop::Strided(conn.sreg_read_xlen(rs2)?.into()),
                0b01 => Mop::Indexed{ordered: false},
                0b11 => Mop::Indexed{ordered: true},
    
                _ => panic!("impossible mop bits {:2b}", mop)
            };
    
            let kind = match mop {
                Mop::UnitStride => {
                    match opcode {
                        Opcode::LoadFP => {
                            use UnitStrideLoadOp::*;
                            let lumop = match rs2 {
                                0b00000 => Load,
                                0b01000 => WholeRegister,
                                0b01011 => ByteMaskLoad,
                                0b10000 => FaultOnlyFirst,
        
                                _ => bail!("invalid unit stride type {:05b}", rs2)
                            };
    
                            match lumop {
                                Load => OverallMemOpKind::Strided(1),
                                WholeRegister => OverallMemOpKind::WholeRegister,
                                ByteMaskLoad => OverallMemOpKind::ByteMask,
                                FaultOnlyFirst => OverallMemOpKind::FaultOnlyFirst,
                            }
                        },
                        Opcode::StoreFP => {
                            use UnitStrideStoreOp::*;
                            let sumop = match rs2 {
                                0b00000 => Store,
                                0b01000 => WholeRegister,
                                0b01011 => ByteMaskStore,
            
                                _ => bail!("invalid unit stride type {:05b}", rs2)
                            };
            
                            match sumop {
                                Store => OverallMemOpKind::Strided(1),
                                WholeRegister => OverallMemOpKind::WholeRegister,
                                ByteMaskStore => OverallMemOpKind::ByteMask,
                            }
                        },
                        _ => bail!("Incorrect opcode passed to decode_load_store: {:?}", opcode)
                    }
                    
                }
                Mop::Strided(stride) => OverallMemOpKind::Strided(stride),
                Mop::Indexed{ordered} => OverallMemOpKind::Indexed{ordered, index_ew: eew}
            };

            if kind == OverallMemOpKind::ByteMask && eew != Sew::e8 {
                bail!("Trying to do a byte-masked operation with EEW != 8 is impossible");
            }

            let nf_pow2 = match nf {
                1 | 2 | 4 | 8 => true,
                _ => false
            };
            if kind == OverallMemOpKind::WholeRegister && !nf_pow2 {
                bail!("WholeRegister operation with non-power2 nf {} impossible", nf);
            }
    
            Ok(OverallMemOp {
                dir: match opcode {
                    Opcode::LoadFP => MemOpDir::Load,
                    Opcode::StoreFP => MemOpDir::Store,
                    _ => bail!("Incorrect opcode passed to decode_load_store: {:?}", opcode)
                },
                eew: match kind {
                    // Indexed accesses use SEW as the unit for accessing elements from memory,
                    // and EEW for the size of the indices
                    OverallMemOpKind::Indexed{index_ew: _index_ew, ..} => self.vtype.vsew,
                    _ => eew
                },
                emul,
                evl: match kind {
                    OverallMemOpKind::ByteMask => {
                        // As per section 7.4, evl = ceil(vl/8)
                        // We don't have div_ceil in Rust yet, so do (vl + 7) / 8 which is equivalent
                        (self.vl + 7) / 8
                    }
                    // For WholeRegister, this is ignored
                    OverallMemOpKind::WholeRegister => self.vl,
                    _ => self.vl
                },
                kind,
                nf: nf
            })
        } else {
            bail!("decode_load_store MUST be passed an instruction of FLdStType, got {:?}", inst)
        }
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
