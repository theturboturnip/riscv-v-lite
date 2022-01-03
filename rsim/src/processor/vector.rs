use std::mem::size_of;
use std::cmp::min;
use anyhow::{Context, Result};

use crate::memory::Memory;

use super::decode::{Opcode,InstructionBits};

use crate::processor::{uXLEN,XLEN};



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

    vtype: VType,
    vl: u32,
    vstart: u32, // Always 0????
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

            vtype: VType::illegal(),
            vl: 0,
            vstart: 0,
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
    /// Requires a [VectorUnitConnection].
    /// 
    /// # Arguments
    /// 
    /// * `inst_kind` - Which kind of configuration instruction to execute
    /// * `inst` - Decoded instruction bits
    /// * `conn` - Connection to external resources
    fn exec_config(&mut self, inst_kind: ConfigKind, inst: InstructionBits, conn: VectorUnitConnection) -> Result<()> {
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
                        conn.sreg[rs1 as usize]
                    } else {
                        if rd != 0 {
                            // rs1 == 0, rd != 0
                            // => set the AVL to the maximum possible value,
                            // use that to calculate the maximum number of elements in this configuration,
                            // which will get written out to rd.
                            u32::MAX
                        } else {
                            // Request the same vector length, even if the vtype is changing.
                            self.vl
                        }
                    }
                } ,
                ConfigKind::vsetivli => { // vsetivli
                    // Read AVL from an immediate
                    // Use rs1 as a 5-bit immediate
                    rs1 as u32
                }
            };

            // Depending on the instruction, the vtype selection is different
            // See RISC-V V spec, section 6
            let vtype_bits = match inst_kind {
                ConfigKind::vsetvli => {
                    zimm11 as u32
                },
                ConfigKind::vsetivli => {
                    zimm10 as u32
                },
                ConfigKind::vsetvl => {
                    conn.sreg[rs2 as usize] 
                },
            };
            // Try to parse vtype bits
            let req_vtype = VType::decode(vtype_bits)?;

            // Calculate the maximum number of elements per register group
            // (under some configurations, e.g. Sew=8,Lmul=1/4,Vlen=32, this could be < 1 which is illegal)
            let elems_per_group = req_vtype.elems_per_group();

            let vtype_supported = elems_per_group > 0 && 
                req_vtype.vsew != Sew::e64 &&  // ELEN = 32, we don't support larger elements
                match req_vtype.vlmul {
                    Lmul::eEighth => false, // As per the spec (section 3.4.2) we aren't required to support Lmul = 1/8
                    Lmul::eHalf | Lmul::eQuarter => false, // TODO - support these lol
                    _ => true
                };

            if vtype_supported {
                self.vtype = req_vtype;
                dbg!(avl, elems_per_group);
                self.vl = min(elems_per_group, avl);

                conn.sreg[rd as usize] = self.vl;
            } else {
                self.vtype = VType::illegal();
                bail!("Valid but unsupported vtype: {:b} -> {:?}, elems_per_group {}", vtype_bits, req_vtype, elems_per_group);
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

            (LoadFP, InstructionBits::FLdStType{rd, width, rs1, rs2, vm, mew, mop, nf, ..}) => {
                // MEW = Memory Expanded Width(?)
                // Expected to be used for larger widths, because it's right next to the width field,
                // but for now it has to be 0
                if mew { bail!("LoadFP with mew = 1 is reserved") }

                // Get the element width we want to use (which is NOT the same as the one encoded in vtype)
                // EEW = Effective Element Width
                let eew = match width {
                    0b0001 | 0b0010 | 0b0011 | 0b0100 => bail!("LoadFP uses width for normal floats, not vectors"),
                    0b1000..=0b1111 => bail!("LoadFP using reserved width {}", width),

                    0b0000 => 8,
                    0b0101 => 16,
                    0b0110 => 32,
                    0b0111 => 64,

                    _ => bail!("LoadFP has impossible width {}", width)
                };

                if eew == 64 {
                    // We are allowed to reject values of EEW that aren't supported for SEW in vtype
                    // (see section 7.3 of RISC-V V spec)
                    bail!("effective element width of 64 is not supported");
                }

                // Check the effective element width is valid, given the current SEW and LMUL

                // EMUL = Effective LMUL
                // because LMULs can be as small as 1/8th, evaluate it as an integer * 8 (effectively 29.3 fixed point)
                let emul_times_8 = self.vtype.val_times_lmul_over_sew(eew * 8);

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
                let eew = match eew {
                    8  => Sew::e8,
                    16 => Sew::e16,
                    32 => Sew::e32,
                    64 => Sew::e64,
                    _ => bail!("Impossible EEW {}", eew)
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
                    0b10 => Mop::Strided(conn.sreg[rs2 as usize]),
                    0b01 => Mop::Indexed{ordered: false},
                    0b11 => Mop::Indexed{ordered: true},

                    _ => panic!("impossible mop bits {:2b}", mop)
                };

                let base_addr = conn.sreg[rs1 as usize];

                match mop {
                    Mop::UnitStride => {
                        use UnitStrideLoadOp::*;
                        let lumop = match rs2 {
                            0b00000 => Load,
                            0b01000 => WholeRegister,
                            0b01011 => ByteMaskLoad,
                            0b10000 => FaultOnlyFirst,
    
                            _ => bail!("invalid unit stride type {:05b}", rs2)
                        };

                        match lumop {
                            WholeRegister | ByteMaskLoad | FaultOnlyFirst => {
                                bail!("Variant {:?} of Vector Unit Load not implemented", lumop)
                            }
                            Load => {
                                if vm && rd == 0 {
                                    // If we're masked, we can't load over v0 as that's the mask register
                                    bail!("Masked instruction cannot load into v0");
                                }
                                if nf > 1 {
                                    bail!("Unit Segmented Load not implemented");
                                }
                                if eew != Sew::e32 {
                                    bail!("Unit Vector Load for EEW ({:?}) != 32 not implemented", eew);
                                }

                                // Effective VL
                                let evl = self.vl;//val_times_lmul_over_sew(VLEN as u32, eew, emul);

                                // i = element index in logical vector (which includes groups)
                                let mut addr = base_addr;
                                for i in self.vstart..evl {
                                    // TODO remove dumb usize cast here?
                                    let i = i as usize;
                                    // vm == 1 => mask disabled
                                    // => only check mask if vm == 0 e.g. false
                                    if (!vm) && (bits!(self.vreg[0], i:i) == 0) {
                                        // The element has been masked out
                                        continue
                                    }
                                    
                                    let val = conn.memory.load_u32(addr)?;
                                    self.store_u32_vreg(rd, i as u32, val)?;
                                    addr += 4;
                                }
                            }
                        }
                    }
                    Mop::Strided(_stride) => {
                        bail!("Vector Strided Load not implemented yet")
                    }
                    Mop::Indexed{ordered: _ordered} => {
                        if nf > 1 {
                            bail!("Indexed Segmented Load not implemented");
                        }
                        if emul_times_8 < 8 {
                            bail!("Indexed load for EMUL < 1 (emul*8 = {}) not implemented", emul_times_8);
                        }
                        bail!("Vector Indexed Load not implemented yet")
                    }
                }
            }

            (StoreFP, InstructionBits::FLdStType{rd, width, rs1, rs2, vm, mew, mop, nf, ..}) => {
                // TODO - move shared code with LoadFP somewhere common

                // MEW = Memory Expanded Width(?)
                // Expected to be used for larger widths, because it's right next to the width field,
                // but for now it has to be 0
                if mew { bail!("LoadFP with mew = 1 is reserved") }

                // Get the element width we want to use (which is NOT the same as the one encoded in vtype)
                // EEW = Effective Element Width
                let eew = match width {
                    0b0001 | 0b0010 | 0b0011 | 0b0100 => bail!("LoadFP uses width for normal floats, not vectors"),
                    0b1000..=0b1111 => bail!("LoadFP using reserved width {}", width),

                    0b0000 => 8,
                    0b0101 => 16,
                    0b0110 => 32,
                    0b0111 => 64,

                    _ => bail!("LoadFP has impossible width {}", width)
                };

                if eew == 64 {
                    // We are allowed to reject values of EEW that aren't supported for SEW in vtype
                    // (see section 7.3 of RISC-V V spec)
                    bail!("effective element width of 64 is not supported");
                }

                // Check the effective element width is valid, given the current SEW and LMUL

                // EMUL = Effective LMUL
                // because LMULs can be as small as 1/8th, evaluate it as an integer * 8 (effectively 29.3 fixed point)
                let emul_times_8 = self.vtype.val_times_lmul_over_sew(eew * 8);

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
                let eew = match eew {
                    8  => Sew::e8,
                    16 => Sew::e16,
                    32 => Sew::e32,
                    64 => Sew::e64,
                    _ => bail!("Impossible EEW {}", eew)
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
                    0b10 => Mop::Strided(conn.sreg[rs2 as usize]),
                    0b01 => Mop::Indexed{ordered: false},
                    0b11 => Mop::Indexed{ordered: true},

                    _ => panic!("impossible mop bits {:2b}", mop)
                };

                let base_addr = conn.sreg[rs1 as usize];

                match mop {
                    Mop::UnitStride => {
                        use UnitStrideStoreOp::*;
                        let sumop = match rs2 {
                            0b00000 => Store,
                            0b01000 => WholeRegister,
                            0b01011 => ByteMaskStore,
    
                            _ => bail!("invalid unit stride type {:05b}", rs2)
                        };

                        match sumop {
                            WholeRegister | ByteMaskStore => {
                                bail!("Variant {:?} of Vector Unit Store not implemented", sumop)
                            }
                            Store => {
                                if vm && rd == 0 {
                                    // If we're masked, we can't load over v0 as that's the mask register
                                    bail!("Masked instruction cannot load into v0");
                                }
                                if nf > 1 {
                                    bail!("Unit Segmented Store not implemented");
                                }
                                if eew != Sew::e32 {
                                    bail!("Unit Vector Store for EEW ({:?}) != 32 not implemented", eew);
                                }

                                // Effective VL
                                let evl = self.vl;//val_times_lmul_over_sew(VLEN as u32, eew, emul);

                                // i = element index in logical vector (which includes groups)
                                let mut addr = base_addr;
                                for i in self.vstart..evl {
                                    // TODO remove dumb usize cast here?
                                    let i = i as usize;
                                    // vm == 1 => mask disabled
                                    // => only check mask if vm == 0 e.g. false
                                    if (!vm) && (bits!(self.vreg[0], i:i) == 0) {
                                        // The element has been masked out
                                        continue
                                    }
                                    
                                    let val = self.load_u32_vreg(rd, i as u32)?;
                                    // dbg!(addr, val);
                                    conn.memory.store_u32(addr, val)?;
                                    
                                    addr += 4;
                                }
                            }
                        }
                    }
                    Mop::Strided(_stride) => {
                        bail!("Vector Strided Store not implemented yet")
                    }
                    Mop::Indexed{ordered: _ordered} => {
                        if nf > 1 {
                            bail!("Indexed Segmented Store not implemented");
                        }
                        if emul_times_8 < 8 {
                            bail!("Indexed load for EMUL < 1 (emul*8 = {}) not implemented", emul_times_8);
                        }
                        bail!("Vector Indexed Store not implemented yet")
                    }
                }
            }

            _ => bail!("Unexpected opcode/InstructionBits pair at vector unit")
        }

        Ok(())
    }

    /// Store a value in an element in a vertex register group, where EEW = 32.
    /// 
    /// Example: if VLEN=128bits (4 32-bit elements per register), `vd` = 3, `idx` = 5,
    /// this would store `val` into v4[96:64]
    fn store_u32_vreg(&mut self, vd: u8, idx: u32, val: u32) -> Result<()> {
        use std::convert::TryInto;

        const ELEMS_PER_V: u32 = (VLEN as u32)/32;
        // Convert
        let vd: u8 = (vd as u32 + (idx / ELEMS_PER_V)).try_into()
            .context(format!("calculating destination register for vd={},idx={}", vd, idx))?;
        let idx = idx % 4;

        let old_value = self.vreg[vd as usize];
        let mask = (0xFFFF_FFFF as uVLEN) << (32*idx);
        let old_value_with_element_removed = old_value & (!mask);
        let new_value = old_value_with_element_removed | ((val as uVLEN) << (32*idx));

        self.vreg[vd as usize] = new_value;

        Ok(())
    }

    /// Store a value in an element in a vertex register group, where EEW = 32.
    /// 
    /// Example: if VLEN=128bits (4 32-bit elements per register), `vd` = 3, `idx` = 5,
    /// this would store `val` into v4[96:64]
    fn load_u32_vreg(&self, vd: u8, idx: u32) -> Result<u32> {
        use std::convert::TryInto;

        const ELEMS_PER_V: u32 = (VLEN as u32)/32;
        // Convert
        let vd: u8 = (vd as u32 + (idx / ELEMS_PER_V)).try_into()
            .context(format!("calculating destination register for vd={},idx={}", vd, idx))?;
        let idx = idx % 4;

        let full_reg = self.vreg[vd as usize];
        let individual_elem = ((full_reg >> (32*idx)) & 0xFFFF_FFFF) as u32;

        Ok(individual_elem)
    }

    /// Dump vector unit state to standard output.
    pub fn dump(&self) {
        for i in 0..32 {
            println!("v{} = 0x{:032x}", i, self.vreg[i]);
        }
        println!("vl: {}\nvtype: {:?}", self.vl, self.vtype);
    }
}

/// Vector type information
/// 
/// Records the current vector state the program has requested, including element width.
/// Convertible to/from uXLEN, e.g. a register value.
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
        VType::decode(1 << (XLEN - 1)).unwrap()
    }

    /// Shorthand for [val_times_lmul_over_sew] with x = VLEN
    /// 
    /// Used for calculating the number of vector elements a vector register can hold in a given configuration.
    pub fn elems_per_group(self) -> u32 {
        self.val_times_lmul_over_sew(VLEN as u32)
    }

    /// Encode the VType structure into a uXLEN
    /// This is necessary when a program queries the vector type CSR.
    pub fn encode(&self) -> uXLEN {
        let mut val: uXLEN = 0;

        if self.vill {
            // Set top bit
            val |= 1 << (XLEN - 1);
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

    /// Attempt to decode a uXLEN vtype value (e.g. one encoded in a register value)
    /// into an actual VType.
    pub fn decode(vtype_bits: u32) -> Result<VType> {
        let vsew = match bits!(vtype_bits, 3:5) {
            0b000 => Sew::e8,
            0b001 => Sew::e16,
            0b010 => Sew::e32,
            0b011 => Sew::e64,

            invalid => bail!("Bad vtype - invalid SEW selected {:b}", invalid)
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
            invalid => bail!("Bad vtype - invalid Lmul selected {:b}", invalid),
        };

        match bits!(vtype_bits, 8:(XLEN-2)) {
            0 => {
                // As expected, all middle bits should be zero
            },
            invalid => bail!("Bad vtype - reserved middle bits nonzero: {:b}", invalid)
        }

        Ok(VType {
            vill: bits!(vtype_bits, (XLEN-1):(XLEN-1)) == 1,
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
