#![allow(non_camel_case_types)]

use crate::processor::elements::cheri::SafeTaggedCap;
use std::ops::Range;
use std::marker::PhantomData;
use crate::processor::isa_mods::*;
use crate::processor::exceptions::IllegalInstructionException::*;
use super::csrs::CSRProvider;
use std::cmp::min;
use anyhow::{Context, Result};

use crate::processor::decode::{Opcode,InstructionBits};

mod types;
use types::*;

mod conns;
use conns::*;

mod decode;
use decode::*;

mod registers;
pub use registers::*;

/// The Vector Unit for the processor.
/// Stores all vector state, including registers.
/// Call [Rv32v::exec_inst()] on it when you encounter a vector instruction.
/// This requires a [VecMemInterface<uXLEN>] to access other resources.
pub struct Rvv<uXLEN: PossibleXlen, TElem> {
    // TODO use a RegisterFile for this?
    vreg: Box<dyn VectorRegisterFile<TElem>>,

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
pub type Rv32v = Rvv<u32, u128>;
pub type Rv64v = Rvv<u64, u128>;
pub type Rv64Cheriv = Rvv<u64, SafeTaggedCap>;

impl<uXLEN: PossibleXlen, TElem> Rvv<uXLEN, TElem> {
    /// Returns an initialized vector unit.
    pub fn new(vreg: Box<dyn VectorRegisterFile<TElem>>) -> Self {
        Rvv {
            vreg,

            vtype: VType::illegal(),
            vl: 0,
            vstart: 0,

            _phantom_xlen: PhantomData,
        }
    }

    /// Reset the vector unit's state
    pub fn reset(&mut self) {
        self.vreg.reset();
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
    fn exec_config(&mut self, inst_kind: ConfigKind, inst: InstructionBits, sreg: &mut dyn VecRegInterface<uXLEN>) -> Result<()> {
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
                        sreg.sreg_read_xlen(rs1)?.into()
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
                    sreg.sreg_read_xlen(rs2)?.into()
                },
            };
            // Try to parse vtype bits
            let req_vtype = VType::decode(vtype_bits as u32)?;

            // Calculate the maximum number of elements per register group
            // (under some configurations, e.g. Sew=8,Lmul=1/4,Vlen=32, this could be < 1 which is illegal)
            let elems_per_group = req_vtype.elems_per_group();

            let vtype_supported = elems_per_group > 0;

            if vtype_supported {
                self.vtype = req_vtype;
                // TODO - section 6.3 shows more constraints on setting VL
                self.vl = min(elems_per_group, avl as u32);

                sreg.sreg_write_xlen(rd, self.vl.into())?;
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

    /// Find the last segment index that isn't masked out.
    /// Used to find a tight range of the segments/elements that will actually be processed.
    fn get_active_segment_range(&mut self, vm: bool, evl: u32) -> Option<Range<u32>> {
        // Find the smallest not-masked-out segments >= vreg
        // Find the largest not-masked-out segments < evl

        // Take range vstart-evl
        // remove masked-out segments
        // take minimum
        // In theory, this could be replaced with a lowest-bit detection (with a shift to remove segments < vstart)
        let start = (self.vstart..evl)
            .filter_map(|i| match self.vreg.seg_masked_out(vm, i) {
                true => None,
                false => Some(i as u32)
            })
            .min();

        // Take range vstart-evl
        // remove masked-out segments
        // take maximum
        // In theory, this could be replaced with a highest-bit detection (with some kind of mask/shift to remove segments >= than evl?)
        let final_accessed = (self.vstart..evl)
            .filter_map(|i| match self.vreg.seg_masked_out(vm, i) {
                true => None,
                false => Some(i as u32)
            })
            .max();

        // If the ranges weren't empty, i.e. at least one element in vstart..evl is active, return a range containing that segment.
        // Otherwise no segments will be accessed.
        match (start, final_accessed) {
            (Some(start), Some(final_accessed)) => Some(Range::<u32> {
                start,
                end: final_accessed + 1 // Exclusive range, needs to contain final_accessed
            }),
            _ => None
        }
    }

    /// Try doing fast-path capability checks for accesses for a vector load/store.
    /// Fast-paths exist for all accesses, although in hardware some may not be as fast as others.
    /// Return values:
    /// - Ok(true) if the fast-path check raised no capability exceptions
    ///   - Therefore the full access should not raise any capability exceptions
    /// - Ok(false) if the fast-path check failed in a tolerable manner 
    ///   - Therefore the full access *may* raise a capability exception
    ///   - A tolerable fast-path failure = fault-only-first, which might absorb the exception.
    /// - Err() if the fast-path check failed in a not-tolerable manner
    /// panics if all elements are masked out
    fn fast_check_load_store(&mut self, addr_provenance: (u64, Provenance), rs2: u8, vm: bool, op: DecodedMemOp, sreg: &mut dyn VecRegInterface<uXLEN>) -> (Result<bool>, Range<u64>) {
        let (base_addr, provenance) = addr_provenance;

        use DecodedMemOp::*;
        let mut is_fault_only_first = false;
        // Calculate an address range that tightly encompasses the access.
        let addr_range = match op {
            Strided{stride, eew, evl, nf, ..} => {
                // Calculate the range of not-masked-out segments
                // active_vstart = the smallest segment >= vstart that isn't masked out
                // active_evl = (the largest segment < evl that isn't masked out) + 1
                let Range{ start: active_vstart, end: active_evl } = self.get_active_segment_range(vm, evl).unwrap();

                // todo!("negative range");
                let offset_range = Range::<u64> {
                    start: active_vstart as u64 * stride,
                    // The index of the final segment = (evl - 1)
                    // The start of the final segment = (evl - 1) * stride
                    // The end of the final segment = (evl - 1) * stride + (nf * eew)
                    end: (active_evl as u64 - 1) * stride + (nf as u64) * eew.width_in_bytes()
                };
                Range::<u64> {
                    start: base_addr + offset_range.start,
                    end: base_addr + offset_range.end,
                }
            },
            FaultOnlyFirst{evl, nf, eew, ..} => {
                is_fault_only_first = true;

                // Calculate the range of not-masked-out segments
                let Range{ start: active_vstart, end: active_evl } = self.get_active_segment_range(vm, evl).unwrap();

                let offset_range = Range::<u64> {
                    start: (active_vstart as u64) * (nf as u64) * eew.width_in_bytes(),
                    // The index of the final segment = (evl - 1)
                    // The start of the final segment = (evl - 1) * stride
                    // The end of the final segment = (evl - 1) * stride + (nf * eew)
                    // stride = eew * nf
                    // => The end of the final segment = (evl - 1) * eew * nf + eew * nf
                    // = evl * eew * nf
                    end:   (active_evl as u64)    * (nf as u64) * eew.width_in_bytes()
                };
                Range::<u64> {
                    start: base_addr + offset_range.start,
                    end: base_addr + offset_range.end,
                }
            },
            Indexed{evl, nf, eew, index_ew, ..} => {
                // Calculate the range of not-masked-out segments
                let Range{ start: active_vstart, end: active_evl } = self.get_active_segment_range(vm, evl).unwrap();

                let mut offsets = vec![];
                for i_segment in active_vstart..active_evl {
                    offsets.push(self.vreg.load_vreg_elem_int(index_ew, rs2, i_segment).unwrap());
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
                // Can't be masked out
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
                // Can't be masked out
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

        let check_result = sreg.check_addr_range_against_provenance(addr_range.clone(), provenance, op.dir());
        match check_result {
            Ok(()) => {
                // if that range check succeeded, we can return true
                return (Ok(true), addr_range);
            }
            Err(e) => {
                // the full range encountered a capability exception
                // if this is a fault-only-first operation, that's ok - it will handle that
                if is_fault_only_first {
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
                    if !self.vreg.seg_masked_out(vm, i_segment) {
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
                    if !self.vreg.seg_masked_out(vm, i_segment) {
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
                    let seg_offset = self.vreg.load_vreg_elem_int(index_ew, rs2, i_segment)?;
                    let seg_addr = base_addr + seg_offset as u64;

                    // If we aren't masked out...
                    if !self.vreg.seg_masked_out(vm, i_segment) {
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
    fn exec_load_store(&mut self, expected_addr_range: Range<u64>, rd: u8, rs1: u8, rs2: u8, vm: bool, op: DecodedMemOp, sreg: &mut dyn VecRegInterface<uXLEN>, mem: &mut dyn VecMemInterface<uXLEN, TElem>) -> Result<()> {
        // Determine which accesses we need to do
        let addr_p = sreg.get_addr_provenance(rs1)?;
        let accesses = self.get_load_store_accesses(rd, addr_p, rs2, vm, op)?;
        let (_, provenance) = addr_p;

        // Check the fast-path range is a tight range, equal to the min/max accessed addresses
        // Get minimum and maximum element access addresses
        let min_addr = accesses.iter().map(|(_, addr)| *addr).min().unwrap();
        // For the maximum, take the maximum of (address + width of element) to get the top of the exclusive range of accessed bytes
        let max_addr = accesses.iter().map(|(elem, addr)| addr + elem.eew.width_in_bytes()).max().unwrap();
        if expected_addr_range.start != min_addr || expected_addr_range.end != max_addr {
            bail!("Computed fast-path address range 0x{:x}-{:x} doesn't match the min/max accessed addresses 0x{:x}-{:x}",
                expected_addr_range.start, expected_addr_range.end,
                min_addr, max_addr
            );
        }

        use DecodedMemOp::*;
        match op {
            Strided{dir, ..} | Indexed{dir, ..} | WholeRegister{dir, ..} | ByteMask{dir, ..} => {
                // For each access...
                for (VectorElem{ base_reg, eew, elem_within_group, ..}, addr) in accesses {
                    let addr_p = (addr, provenance);
                    // Perform the access!
                    match dir {
                        MemOpDir::Load => self.load_to_vreg(mem, eew, addr_p, base_reg, elem_within_group)
                            .with_context(|| format!("Failure on element {}", elem_within_group))?,
                        MemOpDir::Store => self.store_to_mem(mem, eew, addr_p, base_reg, elem_within_group)
                            .with_context(|| format!("Failure on element {}", elem_within_group))?
                    }
                }
            }
            FaultOnlyFirst{..} => {
                // For each access...
                for (VectorElem{ base_reg, eew, elem_within_group, ..}, addr) in accesses {
                    let addr_p = (addr, provenance);
                    // Perform the access
                    let load_fault: Result<()> = 
                        self.load_to_vreg(mem, eew, addr_p, base_reg, elem_within_group);
                    
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
    fn load_to_vreg(&mut self, mem: &mut dyn VecMemInterface<uXLEN, TElem>, eew: Sew, addr_provenance: (u64, Provenance), vd_base: u8, idx_from_base: u32) -> Result<()> {
        let val = mem.load_from_memory(eew, addr_provenance)?;
        self.vreg.store_vreg_elem(eew, vd_base, idx_from_base, val)?;
        Ok(())
    }
    /// Stores a value of width `eew` from a specific element `idx_from_base` of a 
    /// vector register group starting at `vd_base` into a given address `addr` 
    fn store_to_mem(&mut self, mem: &mut dyn VecMemInterface<uXLEN, TElem>, eew: Sew, addr_provenance: (u64, Provenance), vd_base: u8, idx_from_base: u32) -> Result<()> {
        let val = self.vreg.load_vreg_elem(eew, vd_base, idx_from_base)?;
        mem.store_to_memory(eew, val, addr_provenance)?;
        Ok(())
    }

    /// Dump vector unit state to standard output.
    pub fn dump(&self) {
        self.vreg.dump();
        println!("vl: {}\nvtype: {:?}", self.vl, self.vtype);
    }
}

pub type VecInterface<'a, uXLEN, TElem> = (
    &'a mut dyn VecRegInterface<uXLEN>,
    &'a mut dyn VecMemInterface<uXLEN, TElem>
);

impl<uXLEN: PossibleXlen, TElem> IsaMod<VecInterface<'_, uXLEN, TElem>> for Rvv<uXLEN, TElem> {
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
    fn execute(&mut self, opcode: Opcode, inst: InstructionBits, inst_bits: u32, conn: VecInterface<'_, uXLEN, TElem>) -> ProcessorResult<Option<()>> {
        let (sreg, mem) = conn;
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
                        self.exec_config(inst_kind, inst, sreg)?
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
                                    let val = self.vreg.load_vreg_elem(self.vtype.vsew, vs1, i)?;
                                    self.vreg.store_vreg_elem(self.vtype.vsew, vd, i, val)?;
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
                        let imm = rs1 as u128;

                        match funct6 {
                            0b011000 => {
                                // VMSEQ
                                // Mask Set-if-EQual
                                // This cannot itself be masked
                                let mut val: uVLEN = 0;
                                for i in self.vstart..self.vl {
                                    let reg_val = self.vreg.load_vreg_elem_int(self.vtype.vsew, rs2, i)?;
                                    if reg_val == imm {
                                        val |= (1 as uVLEN) << i;
                                    }
                                }
                                self.vreg.store_vreg_int(rd, val)?;
                            }
                            0b011001 => {
                                // VMSNE
                                // Mask Set-if-Not-Equal
                                // This cannot itself be masked
                                let mut val: uVLEN = 0;
                                for i in self.vstart..self.vl {
                                    if self.vreg.load_vreg_elem_int(self.vtype.vsew, rs2, i)? != imm {
                                        val |= (1 as uVLEN) << i;
                                    }
                                }
                                self.vreg.store_vreg_int(rd, val)?;
                            }

                            0b010111 => {
                                if (!vm) && rd == 0 {
                                    bail!(UnsupportedParam("Can't handle vmerge on the mask register, because it uses the mask register :)".to_string()));
                                }

                                // vmerge or vmv
                                // if masked, vmerge, else vmv
                                for i in self.vstart..self.vl {
                                    let val = if self.vreg.seg_masked_out(vm, i) {
                                        // if masked out, this must be vmerge, write new value in
                                        self.vreg.load_vreg_elem_int(self.vtype.vsew, rs2, i)?
                                    } else {
                                        // either vmerge + not masked, or vmv
                                        // either way, write immediate
                                        imm
                                    };
                                    self.vreg.store_vreg_elem_int(self.vtype.vsew, rd, i, val)?;
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
                                    let nr = rs1 + 1;
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
                                        let val = self.vreg.load_vreg(rs2 + vx)?;
                                        self.vreg.store_vreg(rd + vx, val)?;
                                    }
                                } else {
                                    bail!(UnimplementedInstruction("vsmul"));
                                }
                            }

                            0b000000 => {
                                // vadd
                                if (!vm) && rd == 0 {
                                    bail!(UnsupportedParam("Can't handle vadd on the mask register, because it uses the mask register :)".to_string()));
                                }

                                for i in self.vstart..self.vl {
                                    if !self.vreg.seg_masked_out(vm, i) {
                                        let val = self.vreg.load_vreg_elem_int(self.vtype.vsew, rs2, i)?;
                                        // Cast the value down to the element type, do the wrapping addition, then cast it back up
                                        let val = match self.vtype.vsew {
                                            Sew::e8 => {
                                                (val as u8).wrapping_add(imm as u8) as u128
                                            }
                                            Sew::e16 => {
                                                (val as u16).wrapping_add(imm as u16) as u128
                                            }
                                            Sew::e32 => {
                                                (val as u32).wrapping_add(imm as u32) as u128
                                            }
                                            Sew::e64 => {
                                                (val as u64).wrapping_add(imm as u64) as u128
                                            }
                                            Sew::e128 => {
                                                (val as u128).wrapping_add(imm as u128) as u128
                                            }
                                        };
                                        self.vreg.store_vreg_elem_int(self.vtype.vsew, rd, i, val)?;
                                    }
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
                let op = DecodedMemOp::decode_load_store(opcode, inst, self.vtype, self.vl, sreg)?;

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

                let addr_provenance = sreg.get_addr_provenance(rs1)?;

                // Any exception at this point does not set the vstart CSR
                // In the fast-path Success or Indeterminate cases the access is still executed,
                // and the element index is reported to the user through the context string.

                // Pre-check capability access
                let (fast_check_result, addr_range) = self.fast_check_load_store(addr_provenance, rs2, vm, op, sreg);
                match fast_check_result {
                    // There was a fast path that didn't raise an exception
                    Ok(true) => {
                        self.exec_load_store(addr_range, rd, rs1, rs2, vm, op, sreg, mem)
                            .context("Executing pre-checked vector access - shouldn't throw CapabilityExceptions under any circumstances")
                    },
                    // There was a fast path that raised an exception, re-raise it
                    Err(e) => {
                        // This assumes imprecise error handling
                        Err(e)
                    }
                    // There was no fast path, or it was uncertain if a CapabilityException would actually be raised
                    Ok(false) => {
                        self.exec_load_store(addr_range, rd, rs1, rs2, vm, op, sreg, mem)
                            .context("Executing not-pre-checked vector access - may throw CapabilityException")
                    },
                }.context(format!("Executing vector access {:?}", op))?;
            }

            _ => bail!("Unexpected opcode/InstructionBits pair at vector unit")
        }

        // If we get this far, the vector instruction has completed
        // As per RVVspec 3.7, we "reset the vstart CSR to zero at the end of execution"
        self.vstart = 0;

        Ok(None)
    }
}

impl<uXLEN: PossibleXlen, TElem> CSRProvider<uXLEN> for Rvv<uXLEN, TElem> {
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
