use crate::processor::exceptions::IllegalInstructionException::UnknownOpcode;
use anyhow::Result;
use std::convert::TryInto;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    Load,
    Store,
    LoadFP,
    StoreFP,
    OpImm,
    Op,
    OpImm32,
    Op32,
    AddUpperImmPC,
    LoadUpperImm,
    JumpAndLink,
    JumpAndLinkRegister,
    Branch,
    Vector,
    System,
    MiscMem,
    Custom2CHERI,
}

impl TryInto<Opcode> for u8 {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Opcode> {
        Ok(match self {
            0b00_000_11 => Opcode::Load,
            0b01_000_11 => Opcode::Store,
            0b00_001_11 => Opcode::LoadFP,
            0b01_001_11 => Opcode::StoreFP,

            0b00_011_11 => Opcode::MiscMem,

            0b00_100_11 => Opcode::OpImm,
            0b01_100_11 => Opcode::Op,

            0b00_110_11 => Opcode::OpImm32,
            0b01_110_11 => Opcode::Op32,

            0b00_101_11 => Opcode::AddUpperImmPC,
            0b01_101_11 => Opcode::LoadUpperImm,

            0b11_001_11 => Opcode::JumpAndLinkRegister,
            0b11_011_11 => Opcode::JumpAndLink,
            0b11_000_11 => Opcode::Branch,

            0b10_101_11 => Opcode::Vector,

            0b11_100_11 => Opcode::System,

            0b10_110_11 => Opcode::Custom2CHERI,

            _ => bail!(UnknownOpcode(self)),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Imm {
    // Source bits used to generate the immediate
    data: u32,
    // The bit width of the immediate
    width: u32,
}
impl Imm {
    /// Generates an immediate from data + width
    pub fn new(data: u32, width: u32) -> Imm {
        assert!(width <= 32);
        // Check if provided data was too-wide
        assert_eq!(bits!(data, 0:(width as usize - 1)), data);
        Imm { data, width }
    }

    /// Return the immediate data as a u32
    ///
    /// ```
    /// use rsim::processor::decode::Imm;
    /// let i = Imm::new(0xF0FF, 16);
    /// assert_eq!(i.no_extend_u32(), 0xF0FF);
    /// ```
    #[inline]
    pub fn no_extend_u32(&self) -> u32 {
        self.data
    }
    /// Return the immediate data as a u64
    ///
    /// ```
    /// use rsim::processor::decode::Imm;
    /// let i = Imm::new(0xF0FF, 16);
    /// assert_eq!(i.no_extend_u64(), 0xF0FF);
    /// ```
    #[inline]
    pub fn no_extend_u64(&self) -> u64 {
        self.data as u64
    }
    /// Sign-extend the immediate value and return as a u32
    ///
    /// ```
    /// use rsim::processor::decode::Imm;
    /// let i = Imm::new(0xF0FF, 16);
    /// assert_eq!(i.sign_extend_u32(), 0xFFFF_F0FF);
    /// assert_eq!(Imm::new(0x00FF_0000, 32).sign_extend_u32(), 0x00FF_0000);
    /// ```
    #[inline]
    pub fn sign_extend_u32(&self) -> u32 {
        // Chop the top bits off a full sign-extended u64
        self.sign_extend_i32() as u32
    }
    /// Sign-extend the immediate value and return as a u64
    ///
    /// ```
    /// use rsim::processor::decode::Imm;
    /// let i = Imm::new(0xF0FF, 16);
    /// assert_eq!(i.sign_extend_u64(), 0xFFFF_FFFF_FFFF_F0FF);
    /// ```
    #[inline]
    pub fn sign_extend_u64(&self) -> u64 {
        self.sign_extend_i64() as u64
    }
    /// Sign-extend the immediate value and return as a i32
    ///
    /// ```
    /// use rsim::processor::decode::Imm;
    /// let i = Imm::new(0xFFFF, 16);
    /// assert_eq!(i.sign_extend_i32(), -1);
    /// ```
    #[inline]
    pub fn sign_extend_i32(&self) -> i32 {
        let sign = self.data & (1 << (self.width - 1));
        if self.width == 32 || sign == 0 {
            self.data as i32
        } else {
            (-1i32 << self.width) | self.data as i32
        }
    }
    /// Sign-extend the immediate value and return as a i64
    ///
    /// ```
    /// use rsim::processor::decode::Imm;
    /// let i = Imm::new(0xFFFF, 16);
    /// assert_eq!(i.sign_extend_i64(), -1);
    /// ```
    #[inline]
    pub fn sign_extend_i64(&self) -> i64 {
        self.sign_extend_i32() as i64
    }
}

/// TODO - Right now this does sign extension up to 32-bits.
/// These should really all be 64-bit, now that we could be decoding 32 or 64-bit instructions.
/// TODO - Make each of these a separate struct? Then we can combine variants in enums, e.g. type ROrIType = (RType, IType).
#[derive(Debug, Clone, Copy)]
pub enum InstructionBits {
    RType {
        rd: u8,
        funct3: u8,
        rs1: u8,
        rs2: u8,
        funct7: u8,
    },
    IType {
        rd: u8,
        funct3: u8,
        rs1: u8,
        imm: Imm,
    },
    ROrIType {
        rd: u8,
        funct3: u8,
        rs1: u8,

        // R-Type only
        rs2: u8,
        funct7: u8,

        // I-Type only
        imm: Imm,
    },
    SType {
        funct3: u8,
        rs1: u8,
        rs2: u8,
        imm: Imm,
    },
    UType {
        rd: u8,
        imm: Imm,
    },
    JType {
        rd: u8,
        imm: Imm,
    },
    BType {
        funct3: u8,
        rs1: u8,
        rs2: u8,
        imm: Imm,
    },
    VType {
        funct3: u8,
        rs1: u8,
        rs2: u8,
        rd: u8,

        // Only for use in arithmetic
        funct6: u8,
        vm: bool,

        // Only for use in configuration
        zimm11: u16,
        zimm10: u16,
    },
    FLdStType {
        rd: u8,
        width: u8,
        rs1: u8,
        rs2: u8,

        // Float only
        funct7: u8,

        // Vector only
        vm: bool,
        mew: bool,
        mop: u8,
        nf: u8,
    },
}

impl InstructionBits {
    pub fn get_opcode(inst: u32) -> Result<Opcode> {
        (bits!(inst, 0:6) as u8).try_into()
    }

    pub fn from_r(inst: u32) -> InstructionBits {
        InstructionBits::RType {
            rd: (bits!(inst, 7:11) as u8),
            funct3: (bits!(inst, 12:14) as u8),
            rs1: (bits!(inst, 15:19) as u8),
            rs2: (bits!(inst, 20:24) as u8),
            funct7: (bits!(inst, 25:31) as u8),
        }
    }

    pub fn from_i(inst: u32) -> InstructionBits {
        InstructionBits::IType {
            rd: (bits!(inst, 7:11) as u8),
            funct3: (bits!(inst, 12:14) as u8),
            rs1: (bits!(inst, 15:19) as u8),
            imm: Imm::new(bits!(inst, 20:31), 12),
        }
    }

    pub fn from_r_or_i(inst: u32) -> InstructionBits {
        InstructionBits::ROrIType {
            rd: (bits!(inst, 7:11) as u8),
            funct3: (bits!(inst, 12:14) as u8),
            rs1: (bits!(inst, 15:19) as u8),

            rs2: (bits!(inst, 20:24) as u8),
            funct7: (bits!(inst, 25:31) as u8),

            imm: Imm::new(bits!(inst, 20:31), 12),
        }
    }

    pub fn from_s(inst: u32) -> InstructionBits {
        let imm_bits: u32 = (bits!(inst, 7:11) as u32) | ((bits!(inst, 25:31) as u32) << 5);

        InstructionBits::SType {
            funct3: (bits!(inst, 12:14) as u8),
            rs1: (bits!(inst, 15:19) as u8),
            rs2: (bits!(inst, 20:24) as u8),
            imm: Imm::new(imm_bits, 12),
        }
    }

    pub fn from_u(inst: u32) -> InstructionBits {
        let imm_bits = bits!(inst, 12:31);

        InstructionBits::UType {
            rd: (bits!(inst, 7:11) as u8),
            imm: Imm::new(imm_bits << 12, 32),
        }
    }

    pub fn from_j(inst: u32) -> InstructionBits {
        let imm = (bits!(inst, 21:30) << 1)
            | (bits!(inst, 20:20) << 11)
            | (bits!(inst, 12:19) << 12)
            | (bits!(inst, 31:31) << 20);

        InstructionBits::JType {
            rd: (bits!(inst, 7:11) as u8),
            imm: Imm::new(imm, 21),
        }
    }

    pub fn from_b(inst: u32) -> InstructionBits {
        let imm = (bits!(inst, 8:11) << 1)
            | (bits!(inst, 25:30) << 5)
            | (bits!(inst, 7:7) << 11)
            | (bits!(inst, 31:31) << 12);

        InstructionBits::BType {
            funct3: (bits!(inst, 12:14) as u8),
            rs1: (bits!(inst, 15:19) as u8),
            rs2: (bits!(inst, 20:24) as u8),
            imm: Imm::new(imm, 13),
        }
    }

    pub fn from_v(inst: u32) -> InstructionBits {
        InstructionBits::VType {
            rd: (bits!(inst, 7:11) as u8),
            funct3: (bits!(inst, 12:14) as u8),
            rs1: (bits!(inst, 15:19) as u8),
            rs2: (bits!(inst, 20:24) as u8),

            // arithmetic
            funct6: (bits!(inst, 26:31) as u8),
            vm: bits!(inst, 25:25) == 1,

            // configuration
            zimm11: bits!(inst, 20:30) as u16,
            zimm10: bits!(inst, 20:29) as u16,
        }
    }

    pub fn from_f_ld_st(inst: u32) -> InstructionBits {
        InstructionBits::FLdStType {
            rd: (bits!(inst, 7:11) as u8),
            width: (bits!(inst, 12:14) as u8),
            rs1: (bits!(inst, 15:19) as u8),
            rs2: (bits!(inst, 20:24) as u8),

            // Float only
            funct7: (bits!(inst, 25:31) as u8),

            // Vector only
            vm: bits!(inst, 25:25) == 1,
            mop: bits!(inst, 26:27) as u8,
            mew: bits!(inst, 28:28) == 1,
            nf: bits!(inst, 29:31) as u8,
        }
    }
}

pub fn decode(inst: u32) -> Result<(Opcode, InstructionBits)> {
    let opcode = InstructionBits::get_opcode(inst)?;

    use Opcode::*;
    let instr = match opcode {
        Load => InstructionBits::from_i(inst),
        Store => InstructionBits::from_s(inst),
        LoadFP => InstructionBits::from_f_ld_st(inst),
        StoreFP => InstructionBits::from_f_ld_st(inst),
        MiscMem => InstructionBits::from_i(inst),
        OpImm => InstructionBits::from_i(inst),
        Op => InstructionBits::from_r(inst),
        OpImm32 => InstructionBits::from_i(inst),
        Op32 => InstructionBits::from_r(inst),
        AddUpperImmPC => InstructionBits::from_u(inst),
        LoadUpperImm => InstructionBits::from_u(inst),
        JumpAndLink => InstructionBits::from_j(inst),
        JumpAndLinkRegister => InstructionBits::from_i(inst),
        Branch => InstructionBits::from_b(inst),
        Vector => InstructionBits::from_v(inst),
        System => InstructionBits::from_i(inst),
        Custom2CHERI => InstructionBits::from_r_or_i(inst),
    };

    Ok((opcode, instr))
}
