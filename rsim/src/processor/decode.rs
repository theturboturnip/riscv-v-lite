use std::convert::TryInto;
use bitutils::sign_extend32;
use anyhow::Result;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Opcode {
    Load,
    Store,
    LoadFP,
    StoreFP,
    OpImm,
    Op,
    AddUpperImmPC,
    LoadUpperImm,
    JumpAndLink,
    JumpAndLinkRegister,
    Branch,
    Vector,
    System
}

impl TryInto<Opcode> for u8 {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Opcode> {
        Ok(match self {
            0b00_000_11 => Opcode::Load,
            0b01_000_11 => Opcode::Store,
            0b00_001_11 => Opcode::LoadFP,
            0b01_001_11 => Opcode::StoreFP,

            0b00_100_11 => Opcode::OpImm,
            0b01_100_11 => Opcode::Op,

            0b00_101_11 => Opcode::AddUpperImmPC,
            0b01_101_11 => Opcode::LoadUpperImm,

            0b11_001_11 => Opcode::JumpAndLinkRegister,
            0b11_011_11 => Opcode::JumpAndLink,
            0b11_000_11 => Opcode::Branch,

            0b10_101_11 => Opcode::Vector,

            0b11_100_11 => Opcode::System,

            _ => bail!("unhandled opcode {:07b}", self),
        })
    }
}

#[derive(Debug,Clone,Copy)]
pub enum InstructionBits {
    RType {
        rd: u8,
        funct3: u8,
        rs1: u8,
        rs2: u8,
        funct7: u8
    },
    IType {
        rd: u8,
        funct3: u8,
        rs1: u8,
        imm: u32
    },
    SType {
        funct3: u8,
        rs1: u8,
        rs2: u8,
        imm: u32
    },
    UType {
        rd: u8,
        imm: u32
    },
    JType {
        rd: u8,
        imm: u32
    },
    BType {
        funct3: u8,
        rs1: u8,
        rs2: u8,
        imm: u32
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
        zimm10: u16
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
    }
}

impl InstructionBits {
    pub fn get_opcode(inst: u32) -> Result<Opcode> {
        (bits!(inst, 0:6) as u8).try_into()
    }

    pub fn from_r(inst: u32) -> InstructionBits {
        InstructionBits::RType {
            rd:     ((bits!(inst, 7:11) as u8)),
            funct3: ((bits!(inst, 12:14) as u8)),
            rs1:    ((bits!(inst, 15:19) as u8)),
            rs2:    ((bits!(inst, 20:24) as u8)),
            funct7: ((bits!(inst, 25:31) as u8)),
        }
    }

    pub fn from_i(inst: u32, sign_extend_imm: bool) -> InstructionBits {
        let mut imm = bits!(inst, 20:31);
        if sign_extend_imm {
            imm = sign_extend32(imm, 12) as u32;
        }

        InstructionBits::IType {
            rd:     ((bits!(inst, 7:11) as u8)),
            funct3: ((bits!(inst, 12:14) as u8)),
            rs1:    ((bits!(inst, 15:19) as u8)),
            imm:    imm,
        }
    }

    pub fn from_s(inst: u32) -> InstructionBits {
        let imm_bits: u16 = (bits!(inst, 7:11) as u16) | ((bits!(inst, 25:31) as u16) << 5);

        InstructionBits::SType {
            funct3: ((bits!(inst, 12:14) as u8)),
            rs1:    ((bits!(inst, 15:19) as u8)),
            rs2:    ((bits!(inst, 20:24) as u8)),
            imm:    (sign_extend32(imm_bits.into(), 12) as u32),
        }
    }

    pub fn from_u(inst: u32) -> InstructionBits {
        let imm_bits = bits!(inst, 12:31);

        InstructionBits::UType {
            rd:     ((bits!(inst, 7:11) as u8)),
            imm:    imm_bits << 12
        }
    }

    pub fn from_j(inst: u32) -> InstructionBits {
        let imm = 
            (bits!(inst, 21:30) << 1) |
            (bits!(inst, 20:20) << 11) |
            (bits!(inst, 12:19) << 12) |
            (bits!(inst, 31:31) << 20);

        InstructionBits::JType {
            rd:     ((bits!(inst, 7:11) as u8)),
            imm:    (sign_extend32(imm, 20) as u32),
        }
    }

    pub fn from_b(inst: u32) -> InstructionBits {
        let imm = 
            (bits!(inst, 8:11) << 1) |
            (bits!(inst, 25:30) << 5) |
            (bits!(inst, 7:7) << 11) |
            (bits!(inst, 31:31) << 12);

        InstructionBits::BType {
            funct3: ((bits!(inst, 12:14) as u8)),
            rs1:    ((bits!(inst, 15:19) as u8)),
            rs2:    ((bits!(inst, 20:24) as u8)),
            imm:    (sign_extend32(imm.into(), 12) as u32),
        }
    }

    pub fn from_v(inst: u32) -> InstructionBits {
        InstructionBits::VType {
            rd:     ((bits!(inst, 7:11) as u8)),
            funct3: ((bits!(inst, 12:14) as u8)),
            rs1:    ((bits!(inst, 15:19) as u8)),
            rs2:    ((bits!(inst, 20:24) as u8)),

            // arithmetic
            funct6: ((bits!(inst, 26:31) as u8)),
            vm: bits!(inst, 25:25) == 1,

            // configuration
            zimm11: bits!(inst, 20:30) as u16,
            zimm10: bits!(inst, 20:29) as u16,
        }
    }

    pub fn from_f_ld_st(inst: u32) -> InstructionBits {
        InstructionBits::FLdStType {
            rd:     ((bits!(inst, 7:11) as u8)),
            width: ((bits!(inst, 12:14) as u8)),
            rs1:    ((bits!(inst, 15:19) as u8)),
            rs2:    ((bits!(inst, 20:24) as u8)),
            
            // Float only
            funct7: ((bits!(inst, 25:31) as u8)),

            // Vector only
            vm: bits!(inst, 25:25) == 1,
            mop: bits!(inst, 26:27) as u8,
            mew: bits!(inst, 28:28) == 1,
            nf: bits!(inst, 29:31) as u8
        }
    }
}

pub fn decode(inst: u32) -> Result<(Opcode, InstructionBits)> {
    let opcode = InstructionBits::get_opcode(inst)?;

    use Opcode::*;
    let instr = match opcode {
        Load =>             InstructionBits::from_i(inst, true),
        Store =>            InstructionBits::from_s(inst),
        LoadFP =>           InstructionBits::from_f_ld_st(inst),
        StoreFP =>          InstructionBits::from_f_ld_st(inst),
        OpImm =>            InstructionBits::from_i(inst, true),
        Op =>               InstructionBits::from_r(inst),
        AddUpperImmPC =>    InstructionBits::from_u(inst),
        LoadUpperImm =>     InstructionBits::from_u(inst),
        JumpAndLink =>      InstructionBits::from_j(inst),
        JumpAndLinkRegister => InstructionBits::from_i(inst, true),
        Branch =>           InstructionBits::from_b(inst),
        Vector =>           InstructionBits::from_v(inst),
        System =>           InstructionBits::from_i(inst, false),
    };

    Ok((opcode, instr))
}