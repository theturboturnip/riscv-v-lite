#![allow(non_camel_case_types)]

use std::ops::Range;
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
pub use conns::{Rv32vConn,Rv64vConn,Rv64vCheriConn};

mod decode;
use decode::*;

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

    /// Try doing fast-path capability checks for accesses for a vector load/store.
    /// Fast-paths exist for all accesses, although in hardware some may not be as fast as others.
    /// Return values:
    /// - Ok(true) if the fast-path check raised no capability exceptions
    ///   - Therefore the full access should not raise any capability exceptions
    /// - Ok(false) if the fast-path check failed in a tolerable manner 
    ///   - Therefore the full access *may* raise a capability exception
    ///   - A tolerable fast-path failure = fault-only-first, which might absorb the exception,
    ///     or masked operations that might mask out the offending element.
    /// - Err() if the fast-path check failed in a not-tolerable manner
    fn fast_check_load_store(&mut self, addr_provenance: (u64, Provenance), rs2: u8, vm: bool, op: DecodedMemOp, conn: &mut dyn VecMemInterface<uXLEN>) -> (Result<bool>, Range<u64>) {
        let (base_addr, provenance) = addr_provenance;

        use DecodedMemOp::*;
        let mut is_fault_only_first = false;
        // Try to calculate an address range that totally encompasses the access.
        let addr_range = match op {
            Strided{stride, eew, evl, nf, ..} => {
                let offset_range = Range::<u64> {
                    start: self.vstart as u64 * stride,
                    // The index of the final segment = (evl - 1)
                    // The start of the final segment = (evl - 1) * stride
                    // The end of the final segment = (evl - 1) * stride + (nf * eew)
                    end: (evl as u64) * stride + (nf as u64) * eew.width_in_bytes()
                };
                Range::<u64> {
                    start: base_addr + offset_range.start,
                    end: base_addr + offset_range.end,
                }
            },
            FaultOnlyFirst{evl, nf, eew, ..} => {
                is_fault_only_first = true;
                let index_range = Range::<u64> {
                    start: self.vstart as u64 * nf as u64 * eew.width_in_bytes(),
                    // The index of the final segment = (evl - 1)
                    // The start of the final segment = (evl - 1) * stride
                    // The end of the final segment = (evl - 1) * stride + (nf * eew)
                    // stride = eew * nf
                    // => The end of the final segment = (evl - 1) * eew * nf + eew * nf
                    // = evl * eew * nf
                    end: (evl as u64) * (nf as u64) * eew.width_in_bytes()
                };
                Range::<u64> {
                    start: base_addr + index_range.start ,
                    end: base_addr + index_range.end * eew.width_in_bytes(),
                }
            },
            Indexed{evl, nf, eew, index_ew, ..} => {
                let mut offsets = vec![];
                for i_segment in self.vstart..evl {
                    offsets.push(self.load_vreg_elem(index_ew, rs2, i_segment).unwrap());
                }

                let offset_range = Range::<u64> {
                    start: *offsets.iter().min().unwrap() as u64,
                    end: *offsets.iter().max().unwrap() as u64 + (nf as u64 * eew.width_in_bytes()),
                };
                Range::<u64> {
                    start: base_addr + offset_range.start,
                    end: base_addr + offset_range.end,
                }
            }
            WholeRegister{eew, ..} => {
                // op.evl() accounts for the number of registers
                let index_range = Range::<u64> {
                    start: 0,
                    end: (op.evl() as u64)
                };
                Range::<u64> {
                    start: base_addr + index_range.start * eew.width_in_bytes(),
                    end: base_addr + index_range.end * eew.width_in_bytes(),
                }
            }
            ByteMask{evl, ..} => {
                // bytemask does not have segment support
                let index_range = Range::<u64> {
                    start: self.vstart as u64,
                    end: (evl as u64)
                };
                Range::<u64> {
                    start: base_addr + index_range.start,
                    end: base_addr + index_range.end,
                }
            }
        };

        let check_result = conn.check_addr_range_against_provenance(addr_range.clone(), provenance, op.dir());
        match check_result {
            Ok(()) => {
                // if that range check succeeded, we can return true
                return (Ok(true), addr_range);
            }
            Err(e) => {
                // the full range encountered a capability exception
                // if this is a fault-only-first operation, that's ok - it will handle that
                // if this is a masked operation (i.e. vm == false), it's also ok 
                //     - the element that generates that exception might be masked out
                if is_fault_only_first || (vm == false) {
                    return (Ok(false), addr_range);
                }
                // we aren't in a state that can tolerate errors.
                // this instruction will not succeed.
                // raise the exception.
                return (Err(e), addr_range);
            }
        }
    }

    /// Converts a decoded memory operation to the list of accesses it performs.
    fn get_load_store_accesses(&mut self, rd: u8, addr_p: (u64, Provenance), rs2: u8, vm: bool, op: DecodedMemOp) -> Result<Vec<(VectorElem, u64)>> {
        let mut map = vec![];

        let (base_addr, _) = addr_p;

        use DecodedMemOp::*;
        match op {
            Strided{stride, evl, nf, eew, emul, ..} => {
                // For each segment
                for i_segment in self.vstart..evl {
                    let seg_addr = base_addr + (i_segment as u64 * stride);

                    // If we aren't masked out...
                    if !self.seg_masked_out(vm, i_segment as usize) {
                        // For each field
                        let mut field_addr = seg_addr;
                        for i_field in 0..nf {
                            // ... perform the access
                            let vec_elem = VectorElem::check_with_lmul(
                                rd + (i_field * emul.num_registers_consumed()),
                                eew, emul,
                                i_segment
                            );
                            map.push((vec_elem, field_addr));
                            // and increment the address
                            field_addr += eew.width_in_bytes();
                        }
                    }
                }
            }
            FaultOnlyFirst{evl, nf, eew, emul} => {
                // We don't handle the exceptions here
                // This just lists the accesses that will be attempted
                // This is exactly the same code as for Strided, but it calculates the stride
                let stride = eew.width_in_bytes() * (nf as u64);

                // For each segment
                for i_segment in self.vstart..evl {
                    let seg_addr = base_addr + (i_segment as u64 * stride);

                    // If we aren't masked out...
                    if !self.seg_masked_out(vm, i_segment as usize) {
                        // For each field
                        let mut field_addr = seg_addr;
                        for i_field in 0..nf {
                            // ... perform the access
                            let vec_elem = VectorElem::check_with_lmul(
                                rd + (i_field * emul.num_registers_consumed()),
                                eew, emul,
                                i_segment
                            );
                            map.push((vec_elem, field_addr));
                            // and increment the address
                            field_addr += eew.width_in_bytes();
                        }
                    }
                }
            }
            Indexed{index_ew, evl, nf, eew, emul, ..} => {
                // i = element index in logical vector (which includes groups)
                for i_segment in self.vstart..evl {
                    // Get our index
                    let seg_idx = self.load_vreg_elem(index_ew, rs2, i_segment)?;
                    let seg_addr = base_addr + (seg_idx as u64);

                    // If we aren't masked out...
                    if !self.seg_masked_out(vm, seg_idx as usize) {
                        // For each field
                        let mut field_addr = seg_addr;
                        for i_field in 0..nf {
                            // ... perform the access
                            let vec_elem = VectorElem::check_with_lmul(
                                rd + (i_field * emul.num_registers_consumed()),
                                eew, emul,
                                i_segment
                            );
                            map.push((vec_elem, field_addr));
                            // and increment the address
                            field_addr += eew.width_in_bytes();
                        }
                    }
                }
            }
            WholeRegister{num_regs, eew, ..} => {
                if vm == false {
                    // There are no masked variants of this instruction
                    bail!("WholeRegister operations cannot be masked")
                }

                let mut addr = base_addr;
                let vl = op.evl();
                for i in 0..vl {
                    let vec_elem = VectorElem::check_with_num_regs(rd, eew, num_regs, i as u32);
                    map.push((vec_elem, addr));
                    addr += eew.width_in_bytes();
                }
            }
            ByteMask{evl, ..} => {
                if vm == false {
                    // vlm, vsm cannot be masked out
                    bail!("ByteMask operations cannot be masked")
                }

                let mut addr = base_addr;
                for i in self.vstart..evl {
                    let vec_elem = VectorElem::check_with_lmul(
                        rd,
                        Sew::e8, Lmul::e1,
                        i
                    );
                    map.push((vec_elem, addr));
                    addr += 1;
                }
            }
        };

        Ok(map)
    }

    /// Execute a decoded memory access, assuming all access checks have already been performed.
    fn exec_load_store(&mut self, expected_addr_range: Range<u64>, rd: u8, rs1: u8, rs2: u8, vm: bool, op: DecodedMemOp, conn: &mut dyn VecMemInterface<uXLEN>) -> Result<()> {
        // Determine which accesses we need to do
        let addr_p = conn.get_addr_provenance(rs1)?;
        let accesses = self.get_load_store_accesses(rd, addr_p, rs2, vm, op)?;
        let (_, provenance) = addr_p;

        // Check the fast-path range contains all of the addresses we're planning to access
        for (_, addr) in &accesses {
            if !expected_addr_range.contains(&addr) {
                bail!("Computed fast-path address range 0x{:x}-{:x} doesn't contain access address 0x{:x}",
                    expected_addr_range.start, expected_addr_range.end, addr);
            }
        }

        use DecodedMemOp::*;
        match op {
            Strided{dir, ..} | Indexed{dir, ..} | WholeRegister{dir, ..} | ByteMask{dir, ..} => {
                // For each access...
                for (VectorElem{ base_reg, eew, elem_within_group, ..}, addr) in accesses {
                    let addr_p = (addr, provenance);
                    // Perform the access!
                    match dir {
                        MemOpDir::Load => self.load_to_vreg(conn, eew, addr_p, base_reg, elem_within_group)?,
                        MemOpDir::Store => self.store_to_mem(conn, eew, addr_p, base_reg, elem_within_group)?
                    }
                }
            }
            FaultOnlyFirst{..} => {
                // For each access...
                for (VectorElem{ base_reg, eew, elem_within_group, ..}, addr) in accesses {
                    let addr_p = (addr, provenance);
                    // Perform the access
                    let load_fault: Result<()> = 
                        self.load_to_vreg(conn, eew, addr_p, base_reg, elem_within_group);
                    
                    // Check for faults
                    if elem_within_group == 0 {
                        // Any potentially faulted load should fault as normal if i == 0
                        load_fault?;
                    } else if load_fault.is_err() {
                        use crate::processor::exceptions::{MemoryException, CapabilityException};
                        // There was *some* error from the load, check if it was a memory fault
                        let load_err = load_fault.unwrap_err();
                        // Only shrink the vlen if it's a MemoryException related to an invalid address...
                        let mut error_reduces_vlen = match load_err.downcast_ref::<MemoryException>() {
                            Some(MemoryException::AddressUnmapped{..}) => true,
                            _ => false
                        };
                        // .. or a CapabilityException
                        match load_err.downcast_ref::<CapabilityException>() {
                            Some(_) => { error_reduces_vlen = true; },
                            _ => {}
                        };
                        if error_reduces_vlen {
                            // "vector length vl is reduced to the index of the 
                            // element that would have raised an exception"
                            self.vl = elem_within_group;
                            // exception received, finish instruction
                            break;
                        } else {
                            // Re-raise exception
                            return Err(load_err)
                        }
                    }
                }
            }
        };
        Ok(())
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

    /// Returns true if the mask is enabled and element `i` has been masked *out*, e.g. that it should not be touched.
    fn seg_masked_out(&self, vm: bool, i: usize) -> bool {
        // vm == 1 for mask disabled, 0 for mask enabled
        (!vm) && (bits!(self.vreg[0], i:i) == 0)
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

                    0b000 => {
                        // Vector-Vector int
                        let vs1 = rs1;
                        let vd = rd;

                        match funct6 {
                            0b010111 => {
                                // vmv.v.v
                                if !vm {
                                    bail!("vector-vector move can't be masked");
                                }

                                for i in self.vstart..self.vl {
                                    let val = self.load_vreg_elem(self.vtype.vsew, vs1, i)?;
                                    self.store_vreg_elem(self.vtype.vsew, vd, i, val)?;
                                }
                            }
                            _ => bail!("Unsupported OPIVV funct6 {:b}", funct6)
                        }
                    }

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
                                    let val = if self.seg_masked_out(vm, i as usize) {
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

                // Pre-check that the mem-op doesn't do anything dumb
                if op.dir() == MemOpDir::Load && (!vm) && rd == 0 {
                    // If we're masked, we can't load over v0 as that's the mask register
                    bail!("Masked instruction cannot load into v0");
                }
                // Check for no-op
                if op.evl() <= self.vstart {
                    println!("EVL {} <= vstart {} => vector {:?} is no-op", op.evl(), self.vstart, op.dir());
                    return Ok(None)
                }

                let addr_provenance = conn.get_addr_provenance(rs1)?;

                // TODO - set vstart in exec_load_store

                // Pre-check capability access
                let (fast_check_result, addr_range) = self.fast_check_load_store(addr_provenance, rs2, vm, op, conn);
                match fast_check_result {
                    // There was a fast path that didn't raise an exception
                    Ok(true) => {
                        self.exec_load_store(addr_range, rd, rs1, rs2, vm, op, conn)
                            .context("Executing pre-checked vector access - shouldn't throw CapabilityExceptions under any circumstances")?;
                    },
                    // There was a fast path that raised an exception, re-raise it
                    Err(e) => {
                        // This assumes imprecise error handling
                        todo!("set vstart");
                        return Err(e);
                    }
                    // There was no fast path, or it was uncertain if a CapabilityException would actually be raised
                    Ok(false) => {
                        self.exec_load_store(addr_range, rd, rs1, rs2, vm, op, conn)
                            .context("Executing not-pre-checked vector access - may throw CapabilityException")?;
                    },
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

impl<uXLEN: PossibleXlen> CSRProvider<uXLEN> for Rvv<uXLEN> {
    fn has_csr(&self, csr: u32) -> bool {
        match csr {
            // Should be implemented, aren't yet
            0x008 | 0x009 | 0x00A | 0x00F => todo!(),

            0xC20 | 0xC21 | 0xC22 => true,

            _ => false
        }
    }

    fn csr_atomic_read_write(&mut self, csr: u32, _need_read: bool, _write_val: uXLEN) -> Result<Option<uXLEN>> {
        match csr {
            0xC20 | 0xC21 | 0xC22 => bail!("CSR 0x{:04x} is read-only, cannot atomic read/write", csr),
            _ => todo!()
        }
    }

    fn csr_atomic_read_set(&mut self, csr: u32, set_bits: Option<uXLEN>) -> Result<uXLEN> {
        if set_bits != None {
            match csr {
                0xC20 | 0xC21 | 0xC22 => bail!("CSR 0x{:04x} is read-only, cannot atomic set", csr),
                _ => todo!()
            }
        } else {
            match csr {
                0xC20 => Ok(self.vl.into()),
                0xC21 => Ok(self.vtype.encode().into()),
                0xC22 => Ok(((VLEN/8) as u32).into()),

                _ => todo!()
            }
        }
    }
    fn csr_atomic_read_clear(&mut self, _csr: u32, _clear_bits: Option<uXLEN>) -> Result<uXLEN> {
        todo!()
    }
}
