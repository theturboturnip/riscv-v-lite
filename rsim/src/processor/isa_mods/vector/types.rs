#![allow(non_camel_case_types)]

use std::mem::size_of;
use crate::processor::decode::{Opcode,InstructionBits};
use anyhow::Result;

use super::conns::VecMemInterface;

/// Unsigned type of length [ELEN]
/// 
/// ```
/// use rsim::processor::vector::{uELEN, ELEN};
/// use std::mem::size_of;
/// 
/// assert_eq!(size_of::<uELEN>() * 8, ELEN);
/// ```
pub type uELEN = u32;


/// Unsigned type of length [VLEN]
/// 
/// Used for storing vector registers
/// 
/// ```
/// use rsim::processor::vector::{uVLEN, VLEN};
/// use std::mem::size_of;
/// 
/// assert_eq!(size_of::<uVLEN>() * 8, VLEN);
/// ```
pub type uVLEN = u128;

/// Vector register length in bits
pub const VLEN: usize = size_of::<uVLEN>() * 8; // ELEN * 4
const_assert!(size_of::<uVLEN>() % size_of::<uELEN>() == 0);


/// Trait for possible XLEN values
/// We need to be able to turn it into u64s (used for talking to memory subsystem), and getting it from a u32 (the type used for Vlen)
pub trait PossibleXlen: Into<u64> + From<u32> {}
impl PossibleXlen for u64 {}
impl PossibleXlen for u32 {}

/// Vector type information
/// 
/// Records the current vector state the program has requested, including element width.
/// Convertible to/from u32, e.g. a register value.
/// 
/// ```
/// use rsim::processor::vector::{VType,Sew,Lmul};
/// 
/// let encoded_vtype = 0b00001010011;
/// let decoded_vtype = VType::decode(encoded_vtype).unwrap();
/// assert_eq!(decoded_vtype,
///     VType {
///         vill: false,
///         vma: false,
///         vta: true,
///         vsew: Sew::e32,
///         vlmul: Lmul::e8
///     }
/// );
/// 
/// let reencoded_vtype = decoded_vtype.encode();
/// assert_eq!(encoded_vtype, reencoded_vtype);
/// ```
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct VType {
    /// Illegal value.
    /// 
    /// If set, then the program has requested an unsupported configuration.
    pub vill: bool,
    /// Vector mask agnostic.
    /// 
    /// If set, the processor is alowed to overwrite masked-off elements with all 1s.
    pub vma: bool,
    /// Vector tail agnostic.
    /// 
    /// If set, the processor is allowed to overwrite tail elements with all 1s.
    pub vta: bool,
    /// Selected element width. See [Sew]
    pub vsew: Sew,
    /// Length multiplier. See [Lmul]
    pub vlmul: Lmul,
}
impl VType {
    /// Generate a VType with the illegal bit `vill` set, and all other bits zeroed.
    /// This should be used when an unsupported vtype is requested by the program.
    pub fn illegal() -> Self {
        VType::decode(1 << (32 - 1)).unwrap()
    }

    /// Shorthand for [VType::val_times_lmul_over_sew] with x = VLEN
    /// 
    /// Used for calculating the number of vector elements a vector register can hold in a given configuration.
    pub fn elems_per_group(self) -> u32 {
        self.val_times_lmul_over_sew(VLEN as u32)
    }

    /// Encode the VType structure into a u32
    /// This is necessary when a program queries the vector type CSR.
    pub fn encode(&self) -> u32 {
        let mut val: u32 = 0;

        if self.vill {
            // Set top bit
            val |= 1 << (32 - 1);
        }

        if self.vma {
            val |= 1 << 7;
        }

        if self.vta {
            val |= 1 << 6;
        }

        let sew_bits = match self.vsew {
            Sew::e8 =>  0b000,
            Sew::e16 => 0b001,
            Sew::e32 => 0b010,
            Sew::e64 => 0b011,
        };
        val |= sew_bits << 3;

        let lmul_bits = match self.vlmul {
            Lmul::eEighth => 0b101,
            Lmul::eQuarter => 0b110,
            Lmul::eHalf => 0b111,
            Lmul::e1 => 0b000,
            Lmul::e2 => 0b001,
            Lmul::e4 => 0b010,
            Lmul::e8 => 0b011
        };
        val |= lmul_bits << 0;

        val
    }

    /// Attempt to decode a u32 vtype value (e.g. one encoded in a register value)
    /// into an actual VType.
    pub fn decode(vtype_bits: u32) -> Result<VType> {
        let vsew = match bits!(vtype_bits, 3:5) {
            0b000 => Sew::e8,
            0b001 => Sew::e16,
            0b010 => Sew::e32,
            0b011 => Sew::e64,

            // can't have >2bits
            invalid => unreachable!("Bad vtype - invalid SEW selected {:b}", invalid)
        };
        let vlmul = match bits!(vtype_bits, 0:2) {
            0b000 => Lmul::e1,
            0b001 => Lmul::e2,
            0b010 => Lmul::e4,
            0b011 => Lmul::e8,

            0b101 => Lmul::eEighth,
            0b110 => Lmul::eQuarter,
            0b111 => Lmul::eHalf,

            0b100 => bail!("Reserved Lmul selected 0b100"), 
            // can't have >3bits
            invalid => unreachable!("Bad vtype - invalid Lmul selected {:b}", invalid),
        };

        match bits!(vtype_bits, 8:(32-2)) {
            0 => {
                // As expected, all middle bits should be zero
            },
            // TODO - how to handle this? the vector spec says these encodings are "reserved",
            // do we throw a parseable error on that?
            invalid => bail!("Bad vtype - reserved middle bits nonzero: {:b}", invalid)
        }

        Ok(VType {
            vill: bits!(vtype_bits, (32-1):(32-1)) == 1,
            vma:  bits!(vtype_bits, 7:7) == 1,
            vta:  bits!(vtype_bits, 6:6) == 1,
            vsew,
            vlmul
        })
    }

    /// Function that evaluates (X * LMUL) / SEW from their enum values
    /// 
    /// # Arguments
    /// 
    /// * `x` - value to multiply/divide
    pub fn val_times_lmul_over_sew(self, x: u32) -> u32 {
        val_times_lmul_over_sew(x, self.vsew, self.vlmul)
    }
}

/// Config instruction kind enum
/// 
/// RISC-V V 1.0 has three vector config instructions, this differentiates between them.
/// 
/// - `vsetvl` = Take application-vector-length and vector-type from registers
/// - `vsetvli` = Take application-vector-length from register, vector-type from immediate
/// - `vsetivli` = Take application-vector-length and vector-type from immediates
pub enum ConfigKind {
    vsetvl,
    vsetvli,
    vsetivli
}

/// Selected-Element-Width enum
/// 
/// The set of possible element-widths that a program can request for e.g. arithmetic.
/// For example, a program could ask the processor to treat vector registers as vectors of 8-bit elements.
/// 
/// Depending on ELEN, the maximum element length, some of these values may not be usable in practice.
#[derive(Debug,PartialEq,Eq,Copy,Clone)]
pub enum Sew {
    e8,
    e16,
    e32,
    e64
}

/// Length-Mul enum
/// 
/// RISC-V V allows programs to *group* vector registers together for greater theoretical parallelism.
/// For example, configuring LMUL=8 means that subsequent vector instructions will operate on 8 vector registers worth of elements.
/// 
/// This requires care if you are resizing elements.
/// An example program in v1.0 of the specification (section 6.4, p28)
/// sets LMUL=4, vtype=16-bit for initial operations.
/// This will operate on LMUL * VLEN / SEW = 4 * VLEN / 16 = VLEN/4 elements.
/// 
/// It then widens the elements to 32-bit using a widening vector multiply.
/// To ensure the following instructions operate on the same number of elements, they reconfigure with doubled LMUL.
/// LMUL = 8, vtype = 32-bit => LMUL * VLEN / SEW = 8 * VLEN / 32 = VLEN/4 elements, same as before.
#[derive(Debug,PartialEq,Eq,Copy,Clone)]
pub enum Lmul {
    eEighth,
    eQuarter,
    eHalf,
    e1,
    e2,
    e4,
    e8
}

/// Function that evaluates (X * LMUL) / SEW from their enum values
/// 
/// # Arguments
/// 
/// * `x` - base value to multiply/divide
/// * `s` - Selected element width enum
/// * `l` - Length multiplier enum
pub fn val_times_lmul_over_sew(x: u32, s: Sew, l: Lmul) -> u32 {
    let mut bits_per_group: u32 = x;
    match l {
        Lmul::eEighth => {
            bits_per_group /= 8;
        },
        Lmul::eQuarter => {
            bits_per_group /= 4;
        },
        Lmul::eHalf => {
            bits_per_group /= 2;
        },
        Lmul::e1 => {},
        Lmul::e2 => {
            bits_per_group *= 2;
        },
        Lmul::e4 => {
            bits_per_group *= 4;
        },
        Lmul::e8 => {
            bits_per_group *= 8;
        },
    };
    
    bits_per_group / match s {
        Sew::e8 => 8,
        Sew::e16 => 16,
        Sew::e32 => 32,
        Sew::e64 => 64,
    }
}


/// Memory OPeration enum
/// 
/// Vector Load/Store operations have four variants:
/// 
/// - Unit-Stride, e.g. access contiguous memory, which has special-case versions (see [UnitStrideLoadOp, UnitStrideStoreOp])
/// - Variable Stride
/// - Indexed, which can be Ordered or Unordered
#[derive(Debug,PartialEq,Eq,Clone,Copy)]
pub enum Mop {
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
    Strided{
        stride: u64, 
        
        dir: MemOpDir,
        emul: Lmul,
        eew: Sew,
        nf: u8,
        evl: u32
    },
    Indexed{
        ordered: bool,
        index_ew: Sew,
        
        dir: MemOpDir,
        emul: Lmul,
        eew: Sew,
        nf: u8,
        evl: u32
    },
    WholeRegister{
        dir: MemOpDir,
        emul: Lmul,
        nf: u8,
    },
    ByteMask{
        dir: MemOpDir,
        emul: Lmul,
        evl: u32
    },
    FaultOnlyFirst{
        emul: Lmul,
        eew: Sew,
        nf: u8,
        evl: u32
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
            ByteMask{emul, ..} => (emul, Sew::e8),
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
                0b00 => Mop::UnitStride,
                0b10 => Mop::Strided(conn.sreg_read_xlen(rs2)?.into()),
                0b01 => Mop::Indexed{ordered: false},
                0b11 => Mop::Indexed{ordered: true},

                _ => panic!("impossible mop bits {:2b}", mop)
            };

            let decoded_mop = match mop {
                Mop::UnitStride => match dir {
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
                                    dir, emul, evl: bytemask_vl,
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
                                    dir, emul, evl: bytemask_vl,
                                }
                            } else {
                                bail!("Can't have ByteMaskStore with non-byte EEW")
                            },
                        }
                    },
                }
                Mop::Strided(stride) => DecodedMemOp::Strided{
                    stride, dir, eew, emul, nf, evl: current_vl,
                },
                Mop::Indexed{ordered} => DecodedMemOp::Indexed{
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
