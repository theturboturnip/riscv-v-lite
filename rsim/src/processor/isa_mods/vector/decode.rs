use super::types::*;
use super::conns::VecMemInterface;
use crate::processor::decode::{Opcode,InstructionBits};

use anyhow::Result;

/// Memory OPeration enum
/// 
/// Vector Load/Store operations have four variants:
/// 
/// - Unit-Stride, e.g. access contiguous memory, which has special-case versions (see [UnitStrideLoadOp, UnitStrideStoreOp])
/// - Variable Stride
/// - Indexed, which can be Ordered or Unordered
#[derive(Debug,PartialEq,Eq,Clone,Copy)]
enum RvvMopType {
    UnitStride,
    Strided(u64),
    Indexed{ordered: bool},
}

/// Special variants of vector loads with unit-stride
#[derive(Debug,PartialEq,Eq,Clone,Copy)]
pub enum UnitStrideLoadOp {
    Load,
    WholeRegister,
    ByteMaskLoad,
    FaultOnlyFirst
}

/// Special variants of vector stores with unit-stride
#[derive(Debug,PartialEq,Eq,Clone,Copy)]
pub enum UnitStrideStoreOp {
    Store,
    WholeRegister,
    ByteMaskStore
}

/// The "direction" of a memory operation.
/// Used by [OverallMemOp].
#[derive(Debug,PartialEq,Eq,Clone,Copy)]
pub enum MemOpDir { 
    /// Load = taking values from memory and putting them in vector registers
    Load,
    /// Store = taking values from vector registers and putting them in memory
    Store
}

/// The different kinds of RISC-V V vector loads/stores.
/// One top-level enum which encapsulates Strided access (also used for basic unit-stride access),
/// Indexed access, and the special cases of unit-stride access (e.g. whole-register, bytemasked, fault-only-first).
#[derive(Debug,PartialEq,Eq,Clone,Copy)]
pub enum DecodedMemOp {
    /// Moves elements of [nf] vector register groups to/from contiguous segments of memory,
    /// where each segment is separated by a stride.
    /// 
    /// - The start of each segment is separated by [stride] bytes.
    /// - Each segment is `nf * eew` bits long, i.e. [nf] elements long.
    /// - Each element in the i-th segment maps to the i-th element of a vector register group.
    /// - This instruction doesn't do anything if the stored `vstart >= vl`.
    /// 
    /// In the simplest case, `nf = 1`.
    /// For example: `stride = 8`, `eew = 32 bits = 4 bytes`
    /// ```text
    /// base addr + (i * 8) <=> v0[i]
    /// ```
    /// 
    /// Increasing [nf] makes it more complicated.
    /// For example if `nf = 3`, `stride = 8`, `eew = 32 bits = 4 bytes`:
    /// ```text
    /// base addr + (i * 8) + (0 * 4) <=> v0[i]
    /// base addr + (i * 8) + (1 * 4) <=> v1[i]
    /// base addr + (i * 8) + (2 * 4) <=> v2[i]
    /// ```
    /// 
    /// In the most complicated case, [EMUL] may also be > 1.
    /// If `EMUL = 2`, `nf = 3`, `stride = 8`, `eew = 32 bits = 4 bytes`:
    /// ```text
    /// base addr + (i * 8) + (0 * 4) <=> (v0..v1)[i]
    /// base addr + (i * 8) + (1 * 4) <=> (v2..v3)[i]
    /// base addr + (i * 8) + (2 * 4) <=> (v4..v5)[i]
    /// ```
    /// Element 2 of the segment maps to vector register *group* 2,
    /// i.e. v4 and v5, rather than v2.
    Strided{
        /// The stride.
        /// AFAIK this should be specified in bytes, but right now it's used in terms of element width.
        /// TODO make this bytes everywhere
        stride: u64, 
        
        /// The direction, i.e. load or store
        dir: MemOpDir,
        /// The effective element width - this is encoded in the instruction instead of copying from vtype
        eew: Sew,
        /// The effective LMUL of the operation, e.g. the size of the vector register group.
        /// Computed as (EEW/vtype.SEW)*vtype.LMUL
        /// 
        /// AFAIK this is to keep the Effective Vector Length (EVL) the same based on the element width.
        /// For example, if you set `vtype = (SEW = 32, LMUL = 1) and vl = 4` to prepare for 32-bit arithmetic,
        /// and then load 4x 64-bit elements (EEW = 64), the effective LMUL of the load will double to make room.
        emul: Lmul,
        /// The effective vector length - always equal to the current vl
        evl: u32,
        /// Number of Fields for segmented access
        nf: u8,
    },
    /// Moves elements of [nf] vector register groups to/from contiguous segments of memory,
    /// where each segment is offset by an index taken from another vector.
    /// 
    /// - The start of each segment is defined by `base address + index_vector[i]`.
    /// - Each segment is `nf * eew` bits long, i.e. [nf] elements long.
    /// - Each element in the i-th segment maps to the i-th element of a vector register group.
    /// - Accesses within each segment are not ordered relative to each other.
    /// - If the ordered variant of this instruction is used, then the segments must be accessed in the order specified by the index vector.
    /// - This instruction doesn't do anything if the stored `vstart >= vl`.
    /// 
    /// The EEW and EMUL for the elements themselves are equal to the SEW, LMUL stored in `vtype`.
    /// The EEW and EMUL for the indices are defined in the instruction.
    /// 
    /// In the simplest case, `nf = 1`.
    /// For example:
    /// ```text
    /// base addr + index_vector[i] <=> v0[i]
    /// ```
    /// 
    /// Increasing [nf] makes it more complicated.
    /// For example if `nf = 3`, `element width = 32 bits = 4 bytes`:
    /// ```text
    /// base addr + index_vector[i] + (0 * 4) <=> v0[i]
    /// base addr + index_vector[i] + (1 * 4) <=> v1[i]
    /// base addr + index_vector[i] + (2 * 4) <=> v2[i]
    /// ```
    /// 
    /// In the most complicated case, [EMUL] may also be > 1.
    /// If `EMUL = 2`, `nf = 3`, `element width = 32 bits = 4 bytes`:
    /// ```text
    /// base addr + index_vector[i] + (0 * 4) <=> (v0..v1)[i]
    /// base addr + index_vector[i] + (1 * 4) <=> (v2..v3)[i]
    /// base addr + index_vector[i] + (2 * 4) <=> (v4..v5)[i]
    /// ```
    /// Element 2 of the segment maps to vector register *group* 2,
    /// i.e. v4 and v5, rather than v2.
    Indexed{
        /// Whether elements must be accessed in the order specified by the index vector.
        ordered: bool,
        /// The width of the indices. Indices should byte offsets, but right now the emulator treats them in terms of element width.
        /// TODO make indices byte offsets everywhere
        index_ew: Sew,
        
        /// The direction, i.e. load or store
        dir: MemOpDir,
        /// The width of the elements being accessed from memory
        eew: Sew,
        /// The effective LMUL of the operation. See [DecodedMemOp::Strided::emul].
        emul: Lmul,
        /// The effective vector length - always equal to the current vl
        evl: u32,
        /// Number of Fields for segmented access
        nf: u8,
    },
    /// Moves the contents of [nf] vector registers to/from a contiguous range in memory.
    WholeRegister{
        /// The direction, i.e. load or store
        dir: MemOpDir,
        /// The number of registers to load or store.
        /// Must be power-of-2
        nf: u8,
        /// TODO REMOVE
        emul: Lmul,
    },
    /// Moves the contents of a mask register to/from a contiguous range of memory.
    /// 
    /// This instruction transfers at least `vl` bits into the mask register,
    /// one bit for each element that could be used in subsequent vector instructions.
    /// 
    /// It is therefore equivalent to a unit-stride load where
    /// - EVL = `ceil(vl/8)`
    /// - EEW = 8-bits
    /// - EMUL = 1 (The maximum LMUL is 8, thus `vl/8` bytes must be able to fit into a single vector register)
    /// - the tail-agnostic setting is always on
    ByteMask{
        /// The direction, i.e. load or store
        dir: MemOpDir,
        /// The number of bytes to load, i.e. `ceil(vl/8)`
        evl: u32
    },
    /// Loads elements from contiguous segments in memory into [nf] vector register groups.
    /// If an exception is encountered while loading elements from segment 0, it is trapped as usual.
    /// However, an exception encountered after that point is ignored, and `vl` is set to the current segment instead.
    /// 
    /// - The start of the range is defined by `base address`.
    /// - Each segment is `nf * eew` bits long, i.e. [nf] elements long.
    /// - Each element in the i-th segment maps to the i-th element of a vector register group.
    /// - Accesses within each segment are not ordered relative to each other.
    /// - This instruction doesn't do anything if the stored `vstart >= vl`.
    /// 
    /// The mappings of address to element are the same as for [DecodedMemOp::Strided], where the stride = the element width.
    /// 
    /// ```text
    /// These accesses can trap an exception
    /// base addr + (0 * 8) + (0 * 4) <=> (v0..v1)[0]
    /// base addr + (0 * 8) + (1 * 4) <=> (v2..v3)[0]
    /// base addr + (0 * 8) + (2 * 4) <=> (v4..v5)[0]
    /// 
    /// These accesses set vl = i on an exception, where i != 0
    /// base addr + (i * 8) + (0 * 4) <=> (v0..v1)[i]
    /// base addr + (i * 8) + (1 * 4) <=> (v2..v3)[i]
    /// base addr + (i * 8) + (2 * 4) <=> (v4..v5)[i]
    /// ```
    FaultOnlyFirst{
        /// The width of the elements being accessed from memory
        eew: Sew,
        /// The effective LMUL of the operation. See [DecodedMemOp::Strided::emul].
        emul: Lmul,
        /// The effective vector length - always equal to the current vl
        evl: u32,
        /// Number of Fields for segmented access
        nf: u8,
    },
}
impl DecodedMemOp {
    pub fn dir(&self) -> MemOpDir {
        use DecodedMemOp::*;
        match *self {
            Strided{dir, ..} => dir,
            Indexed{dir, ..} => dir,
            WholeRegister{dir, ..} => dir,
            ByteMask{dir, ..} => dir,
            FaultOnlyFirst{..} => MemOpDir::Load,
        }
    }
    pub fn access_params(&self) -> (Lmul, Sew) {
        use DecodedMemOp::*;
        match *self {
            Strided{emul, eew, ..} => (emul, eew),
            Indexed{emul, eew, ..} => (emul, eew),
            // For WholeRegister, just use any old eew
            WholeRegister{emul, ..} => (emul, Sew::e8),
            ByteMask{..} => (Lmul::e1, Sew::e8),
            FaultOnlyFirst{emul, eew, ..} => (emul, eew),
        }
    }
    pub fn try_get_evl(&self) -> Option<u32> {
        use DecodedMemOp::*;
        match *self {
            Strided{evl, ..} => Some(evl),
            Indexed{evl, ..} => Some(evl),
            WholeRegister{..} => None,
            ByteMask{evl, ..} => Some(evl),
            FaultOnlyFirst{evl, ..} => Some(evl),
        }
    }

    fn _get_encoded_emul_eew_nf(inst: InstructionBits, current_vtype: VType) -> Result<(Lmul, Sew, u8)> {
        if let InstructionBits::FLdStType{width, nf, ..} = inst {
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
            let emul_times_8 = current_vtype.val_times_lmul_over_sew(eew_num * 8);
    
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

            return Ok((emul, eew, nf));
        } else {
            unreachable!("get_encoded_emul_eew_nf MUST be passed an instruction of FLdStType, got {:?}", inst)
        }
    }
    /// Decode a Load/Store opcode into an DecodedMemOp structure.
    /// Performs all checks to ensure the instruction is a valid RISC-V V vector load/store.
    pub fn decode_load_store<uXLEN: PossibleXlen>(opcode: Opcode, inst: InstructionBits, current_vtype: VType, current_vl: u32, conn: &mut dyn VecMemInterface<uXLEN>) -> Result<DecodedMemOp> {
        if let InstructionBits::FLdStType{rs2, mop, ..} = inst {
            let dir = match opcode {
                Opcode::LoadFP => MemOpDir::Load,
                Opcode::StoreFP => MemOpDir::Store,
                _ => bail!("Incorrect opcode passed to decode_load_store: {:?}", opcode)
            };

            let (emul, eew, nf) = DecodedMemOp::_get_encoded_emul_eew_nf(inst, current_vtype)?;

            // As per section 7.4, evl for ByteMask operations = ceil(vl/8)
            // We don't have div_ceil in Rust yet, so do (vl + 7) / 8 which is equivalent
            let bytemask_vl = (current_vl + 7) / 8;
            // WholeRegister operations only support pow2 nfs
            let nf_pow2 = match nf {
                1 | 2 | 4 | 8 => true,
                _ => false
            };

            // MOP = Memory OPeration
            // Determines indexing mode
            let mop = match mop {
                0b00 => RvvMopType::UnitStride,
                0b10 => RvvMopType::Strided(conn.sreg_read_xlen(rs2)?.into()),
                0b01 => RvvMopType::Indexed{ordered: false},
                0b11 => RvvMopType::Indexed{ordered: true},

                _ => panic!("impossible mop bits {:2b}", mop)
            };

            let decoded_mop = match mop {
                RvvMopType::UnitStride => match dir {
                    MemOpDir::Load => {
                        use UnitStrideLoadOp::*;
                        let lumop = match rs2 {
                            0b00000 => Load,
                            0b01000 => WholeRegister,
                            0b01011 => ByteMaskLoad,
                            0b10000 => FaultOnlyFirst,
    
                            _ => bail!("invalid unit stride type {:05b}", rs2)
                        };

                        match lumop {
                            Load => DecodedMemOp::Strided{
                                stride: 1,
                                dir, eew, emul, nf, evl: current_vl,
                            },
                            WholeRegister => if nf_pow2 {
                                DecodedMemOp::WholeRegister{
                                    dir, emul, nf,
                                }
                            } else {
                                bail!("WholeRegister operation with non-power2 nf {} impossible", nf);
                            },
                            ByteMaskLoad => if eew == Sew::e8 {
                                DecodedMemOp::ByteMask {
                                    dir, evl: bytemask_vl,
                                }
                            } else {
                                bail!("Can't have ByteMaskLoad with non-byte EEW")
                            },
                            FaultOnlyFirst => DecodedMemOp::FaultOnlyFirst{
                                eew, emul, evl: current_vl, nf,
                            },
                        }
                    },
                    MemOpDir::Store => {
                        use UnitStrideStoreOp::*;
                        let sumop = match rs2 {
                            0b00000 => Store,
                            0b01000 => WholeRegister,
                            0b01011 => ByteMaskStore,
        
                            _ => bail!("invalid unit stride type {:05b}", rs2)
                        };
        
                        match sumop {
                            Store => DecodedMemOp::Strided{
                                stride: 1,
                                dir, eew, emul, nf, evl: current_vl,
                            },
                            WholeRegister => if nf_pow2 {
                                DecodedMemOp::WholeRegister{
                                    dir, emul, nf,
                                }
                            } else {
                                bail!("WholeRegister operation with non-power2 nf {} impossible", nf);
                            },
                            ByteMaskStore => if eew == Sew::e8 {
                                DecodedMemOp::ByteMask {
                                    dir, evl: bytemask_vl,
                                }
                            } else {
                                bail!("Can't have ByteMaskStore with non-byte EEW")
                            },
                        }
                    },
                }
                RvvMopType::Strided(stride) => DecodedMemOp::Strided{
                    stride, dir, eew, emul, nf, evl: current_vl,
                },
                RvvMopType::Indexed{ordered} => DecodedMemOp::Indexed{
                    ordered, index_ew: eew, eew: current_vtype.vsew,
                    dir, emul, evl: current_vl, nf,
                }
            };
            return Ok(decoded_mop);
        } else {
            bail!("decode_load_store MUST be passed an instruction of FLdStType, got {:?}", inst)
        }
    }
}
