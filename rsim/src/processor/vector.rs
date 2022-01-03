use std::mem::size_of;
use std::cmp::min;
use anyhow::{Result};

use crate::memory::Memory;

use super::decode::{Opcode,InstructionBits};

use crate::processor::uXLEN;



/// Maximum element length in bits
pub const ELEN: usize = 32;

/// Unsigned type of length [ELEN]
/// 
/// ```
/// use rsim::processor::vector::{uELEN, ELEN};
/// use std::mem::size_of;
/// 
/// assert_eq!(size_of::<uELEN>() * 8, ELEN);
/// ```
#[allow(non_camel_case_types)]
pub type uELEN = u32;
const_assert!(size_of::<uELEN>() * 8 == ELEN);



/// Vector register length in bits
pub const VLEN: usize = 128; // ELEN * 4

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
#[allow(non_camel_case_types)]
pub type uVLEN = u128;
const_assert!(size_of::<uVLEN>() * 8 == VLEN);

/// The Vector Unit for the processor.
/// Stores all vector state, including registers.
/// Call [VectorUnit::exec_inst()] on it when you encounter a vector instruction.
/// This requires a [VectorUnitConnection] to access other resources.
pub struct VectorUnit {
    vreg: [uVLEN; 32],

    sew: Sew,
    lmul: Lmul,
    vl: u32,
    vtype_reg: u32,
    // VMA, VTA not required.
    // agnostic = undisturbed || overwrite with ones, so just assume undisturbed
    // vma: bool,
    // vta: bool,
}

/// References to all scalar resources touched by the vector unit.
pub struct VectorUnitConnection<'a> {
    pub sreg: &'a mut [uXLEN; 32],
    pub memory: &'a mut Memory,
}

impl VectorUnit {
    /// Returns an initialized VectorUnit.
    pub fn new() -> VectorUnit {
        VectorUnit {
            vreg: [0; 32],

            sew: Sew::e64,
            lmul: Lmul::e1,
            vl: 0,
            vtype_reg: 0,
        }
    }

    /// Reset the vector unit's state
    pub fn reset(&mut self) {
        self.vreg = [0; 32];

        self.sew = Sew::e64;
        self.lmul = Lmul::e1;
        self.vl = 0;
        self.vtype_reg = 0;
    }

    /// (Internal) Execute a configuration instruction, e.g. vsetvli family
    /// Requires a [VectorUnitConnection].
    /// 
    /// # Arguments
    /// 
    /// * `inst_kind` - Which kind of configuration instruction to execute
    /// * `inst` - Decoded instruction bits
    /// * `conn` - Connection to external resources
    fn exec_config(&mut self, inst_kind: ConfigKind, inst: InstructionBits, conn: VectorUnitConnection) -> Result<()> {
        if let InstructionBits::VType{rd, funct3, rs1, rs2, vm, funct6, zimm11, zimm10} = inst {
            let avl = match inst_kind {
                ConfigKind::vsetvli | ConfigKind::vsetvl => { // vsetvli, vsetvl
                    if rs1 != 0 {
                        conn.sreg[rs1 as usize]
                    } else {
                        if rd != 0 {
                            bail!("vsetvl{{i}} called with rd != 0, rs1 == 0, which requires VMAX. Haven't thought about that yet.");
                            u32::MAX
                        } else {
                            self.vl
                        }
                    }
                } ,
                ConfigKind::vsetivli => { // vsetivli
                    rs1 as u32 // Use rs1 as a 5-bit immediate
                }
            };
            let vtype = match inst_kind {
                ConfigKind::vsetvli => {
                    // vsetvli
                    zimm11 as u32
                },
                ConfigKind::vsetivli => {
                    // vsetivli
                    zimm10 as u32
                },
                ConfigKind::vsetvl => {
                    conn.sreg[rs2 as usize] 
                },
            };

            let req_sew = match bits!(vtype, 3:5) {
                0b000 => Sew::e8,
                0b001 => Sew::e16,
                0b010 => Sew::e32,
                0b011 => Sew::e64,

                invalid => bail!("Bad vtype - invalid SEW selected {:b}", invalid)
            };
            let req_lmul = match bits!(vtype, 0:2) {
                0b000 => Lmul::e1,
                0b001 => Lmul::e2,
                0b010 => Lmul::e4,
                0b011 => Lmul::e8,

                0b101 => Lmul::eEighth,
                0b110 => Lmul::eQuarter,
                0b111 => Lmul::eHalf,

                invalid => bail!("Bad vtype - invalid Lmul selected {:b}", invalid),
            };

            let vector_elements = elements_in(req_sew, req_lmul);
            let vtype_valid = vector_elements > 0 && 
                req_sew != Sew::e64 && match req_lmul {
                Lmul::eHalf | Lmul::eQuarter | Lmul::eEighth => false,
                _ => true
            };
            if vtype_valid {
                self.vtype_reg = vtype;
                self.sew = req_sew;
                self.lmul = req_lmul;
                self.vl = min(vector_elements, avl);

                conn.sreg[rd as usize] = self.vl;
            } else {
                self.vtype_reg = 0x8000_0000;
                bail!("Valid but unsupported vtype: {:b} -> {:?} {:?} elems {:}", vtype, req_sew, req_lmul, vector_elements);
            }

            Ok(())
        } else {
            bail!("vector::exec_config instruction MUST be InstructionBits::VType, got {:?} instead", inst);
        }
    }

    /// Execute a vector-specific instruction, e.g. vector arithmetic, loads, configuration
    /// Requires a [VectorUnitConnection].
    /// 
    /// # Arguments
    /// 
    /// * `opcode` - The major opcode of the instruction
    /// * `inst` - Decoded instruction bits
    /// * `inst_bits` - Raw instruction bits (TODO - we shouldn't need this)
    /// * `conn` - Connection to external resources
    pub fn exec_inst(&mut self, opcode: Opcode, inst: InstructionBits, inst_bits: u32, conn: VectorUnitConnection) -> Result<()> {
        use Opcode::*;
        match (opcode, inst) {
            (Vector, InstructionBits::VType{funct3, ..}) => {
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

                    _ => bail!("Vector arithmetic currently not supported")
                }
            }

            (LoadFP, InstructionBits::FLdStType{rd, width, rs1, rs2, funct7, vm, mew, mop, nf}) => {
                if mew { bail!("LoadFP with mew = 1 is reserved") }

                let eew = match width {
                    0b0001 | 0b0010 | 0b0011 | 0b0100 => bail!("LoadFP uses width for actual floats, not vectors"),
                    0b1000..=0b1111 => bail!("LoadFP using reserved width {}", width),

                    0b0000 => 8,
                    0b0101 => 16,
                    0b0110 => 32,
                    0b0111 => 64,

                    _ => bail!("LoadFP has impossible width {}", width)
                };

                let emul_times_8 = val_times_lmul_over_sew(eew * 8, self.sew, self.lmul);

                if emul_times_8 > 64 || emul_times_8 <= 1 {
                    bail!("emul * 8 too big or too small: {}", emul_times_8);
                }

                let nf = nf + 1;

                if (emul_times_8 * nf as u32) > 64 {
                    bail!("emul * nf too big: {}", emul_times_8 * (nf as u32) / 8);
                }

                let mop = match mop {
                    0b00 => Mop::UnitStride,
                    0b10 => Mop::Strided(conn.sreg[rs2 as usize]),
                    0b01 => Mop::Indexed{ordered: false},
                    0b11 => Mop::Indexed{ordered: true},

                    _ => panic!("impossible mop bits {:2b}", mop)
                };

                let base_addr = conn.sreg[rs1 as usize];

                match mop {
                    Mop::UnitStride => {
                        let lumop = match rs2 {
                            0b00000 => UnitStrideLoadOp::Load,
                            0b01000 => UnitStrideLoadOp::WholeRegister,
                            0b01011 => UnitStrideLoadOp::ByteMaskLoad,
                            0b10000 => UnitStrideLoadOp::FaultOnlyFirst,
    
                            _ => bail!("invalid unit stride type {:05b}", rs2)
                        };
                        bail!("Vector Load not fully implemented yet")
                    }
                    Mop::Strided(stride) => {
                        bail!("Vector Load not fully implemented yet")
                    }
                    Mop::Indexed{ordered} => {
                        bail!("Vector Load not fully implemented yet")
                    }
                }
            }

            _ => bail!("Unexpected opcode/InstructionBits pair at vector unit ({:?}, {:?})", opcode, inst)
        }

        Ok(())
    }

    /// Dump vector unit state to standard output.
    pub fn dump(&self) {
        for i in 0..32 {
            println!("v{} = 0x{:032x}", i, self.vreg[i]);
        }
        println!("sew: {:?}\nlmul: {:?}\nvl: {}\nvtype_reg: {:08x}", self.sew, self.lmul, self.vl, self.vtype_reg);
    }
}

/// Config instruction kind enum
/// 
/// RISC-V V 1.0 has three vector config instructions, this differentiates between them.
/// 
/// - `vsetvl` = Take application-vector-length and vector-type from registers
/// - `vsetvli` = Take application-vector-length from register, vector-type from immediate
/// - `vsetivli` = Take application-vector-length and vector-type from immediates
enum ConfigKind {
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
enum Sew {
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
enum Lmul {
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
/// `x` - base value to multiply/divide
/// `s` - Selected element width enum
/// `l` - Length multiplier enum
fn val_times_lmul_over_sew(x: u32, s: Sew, l: Lmul) -> u32 {
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
/// Shorthand for [val_times_lmul_over_sew] with x = VLEN
/// 
/// Used for calculating the number of vector elements a vector register can hold in a given configuration.
fn elements_in(s: Sew, l: Lmul) -> u32 {
    val_times_lmul_over_sew(VLEN as u32, s, l)
}

/// Memory OPeration enum
/// 
/// Vector Load/Store operations have four variants:
/// 
/// - Unit-Stride, e.g. access contiguous memory, which has special-case versions (see [UnitStrideLoadOp, UnitStrideStoreOp])
/// - Variable Stride
/// - Indexed, which can be Ordered or Unordered
#[derive(Debug,PartialEq,Eq,Clone,Copy)]
enum Mop {
    UnitStride,
    Strided(u32),
    Indexed{ordered: bool},
}

/// Special variants of vector loads with unit-stride
#[derive(Debug,PartialEq,Eq,Clone,Copy)]
enum UnitStrideLoadOp {
    Load,
    WholeRegister,
    ByteMaskLoad,
    FaultOnlyFirst
}

/// Special variants of vector stores with unit-stride
#[derive(Debug,PartialEq,Eq,Clone,Copy)]
enum UnitStrideStoreOp {
    Store,
    WholeRegister,
    ByteMaskStore
}
