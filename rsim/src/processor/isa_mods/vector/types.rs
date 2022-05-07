#![allow(non_camel_case_types)]

use std::mem::size_of;
use anyhow::Result;

/// Unsigned type of length [VLEN]
/// 
/// Used for storing vector registers
pub type uVLEN = u128;

/// Vector register length in bits
pub const VLEN: usize = size_of::<uVLEN>() * 8;

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

            // Not explicitly specified by the spec, but it's what Clang defaults to
            // and likely what the spec would use anyway
            Sew::e128 => 0b100,
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
            // This is a reserved encoding, but it's what Clang uses and it's likely what would be used for this anyway
            0b100 => Sew::e128,

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
    e64,
    /// On CHERI, used to load/store capabilities.
    e128,
}
impl Sew {
    pub fn width_in_bytes(&self) -> u64 {
        match self {
            Sew::e8 => 1,
            Sew::e16 => 2,
            Sew::e32 => 4,
            Sew::e64 => 8,
            Sew::e128 => 16,
        }
    }
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
impl Lmul {
    /// Returns the number of vector registers a group actually consumes with this Lmul.
    pub fn num_registers_consumed(&self) -> u8 {
        use Lmul::*;
        match self {
            eEighth | eQuarter | eHalf | e1 => 1,
            e2 => 2,
            e4 => 4,
            e8 => 8
        }
    }
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
    
    bits_per_group / (s.width_in_bytes() as u32 * 8)
}

/// A struct describing an element of a vector register group
#[derive(Debug)]
pub struct VectorElem {
    /// The first register of the group.
    /// Should be a multiple of emul.num_registers_consumed()
    pub base_reg: u8,
    /// The element width,
    pub eew: Sew,
    /// The index of the element within the group
    pub elem_within_group: u32
}
impl VectorElem {
    pub fn check_with_lmul(base_reg: u8, eew: Sew, emul: Lmul, elem_within_group: u32) -> VectorElem {
        return Self::check_with_num_regs(base_reg, eew, emul.num_registers_consumed(), elem_within_group);
    }
    pub fn check_with_num_regs(base_reg: u8, eew: Sew, num_regs: u8, elem_within_group: u32) -> VectorElem {
        // Sanity check - make sure all accesses are contained within the group
        let referenced_reg_in_group = elem_within_group * (eew.width_in_bytes() as u32) / (VLEN as u32/8);
        assert!(referenced_reg_in_group < num_regs as u32);
        return VectorElem {
            base_reg, eew, elem_within_group
        };
    }
}