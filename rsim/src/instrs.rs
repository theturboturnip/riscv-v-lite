use std::convert::TryInto;
use bitutils::sign_extend32;
use anyhow::Result;

#[derive(Debug,Clone,Copy)]
pub enum Opcode {
    Load,
    Store,
    OpImm,
    Op,
    AddUpperImmPC,
    LoadUpperImm,
    // LoadFP,
    // StoreFP,
    JumpAndLink,
    JumpAndLinkRegister,
    MiscMem,
    Branch,
}

impl TryInto<Opcode> for u8 {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Opcode> {
        Ok(match self {
            0b00_000_11 => Opcode::Load,
            0b01_000_11 => Opcode::Store,
            // 0b00_001_11 => Opcode::LoadFP,
            // 0b01_001_11 => Opcode::StoreFP,

            0b00_100_11 => Opcode::OpImm,
            0b01_100_11 => Opcode::Op,

            0b00_101_11 => Opcode::AddUpperImmPC,
            0b01_101_11 => Opcode::LoadUpperImm,

            0b11_001_11 => Opcode::JumpAndLinkRegister,
            0b11_011_11 => Opcode::JumpAndLink,
            0b11_000_11 => Opcode::Branch,
            0b00_011_11 => Opcode::MiscMem,

            _ => bail!("unhandled opcode {:07b}", self),
        })
    }
}

#[derive(Debug,Clone,Copy)]
pub enum Instruction {
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
}

impl Instruction {
    pub fn get_opcode(inst: u32) -> Result<Opcode> {
        (bits!(inst, 0:6) as u8).try_into()
    }

    pub fn from_r(inst: u32) -> Instruction {
        Instruction::RType {
            rd:     ((bits!(inst, 7:11) as u8)),
            funct3: ((bits!(inst, 12:14) as u8)),
            rs1:    ((bits!(inst, 15:19) as u8)),
            rs2:    ((bits!(inst, 20:24) as u8)),
            funct7: ((bits!(inst, 25:31) as u8)),
        }
    }

    pub fn from_i(inst: u32) -> Instruction {
        Instruction::IType {
            rd:     ((bits!(inst, 7:11) as u8)),
            funct3: ((bits!(inst, 12:14) as u8)),
            rs1:    ((bits!(inst, 15:19) as u8)),
            imm:    (sign_extend32((bits!(inst, 20:31) as u16).into(), 12) as u32),
        }
    }

    pub fn from_s(inst: u32) -> Instruction {
        let imm_bits: u16 = (bits!(inst, 7:11) as u16) | ((bits!(inst, 25:31) as u16) << 5);

        Instruction::SType {
            funct3: ((bits!(inst, 12:14) as u8)),
            rs1:    ((bits!(inst, 15:19) as u8)),
            rs2:    ((bits!(inst, 20:24) as u8)),
            imm:    (sign_extend32(imm_bits.into(), 12) as u32),
        }
    }

    pub fn from_u(inst: u32) -> Instruction {
        let imm_bits = bits!(inst, 12:31);

        Instruction::UType {
            rd:     ((bits!(inst, 7:11) as u8)),
            imm:    imm_bits << 12
        }
    }

    pub fn from_j(inst: u32) -> Instruction {
        let imm = 
            (bits!(inst, 21:30) << 1) |
            (bits!(inst, 20:20) << 11) |
            (bits!(inst, 12:19) << 12) |
            (bits!(inst, 31:31) << 20);

        Instruction::JType {
            rd:     ((bits!(inst, 7:11) as u8)),
            imm:    (sign_extend32(imm, 20) as u32),
        }
    }

    pub fn from_b(inst: u32) -> Instruction {
        let imm = 
            (bits!(inst, 8:11) << 1) |
            (bits!(inst, 25:30) << 5) |
            (bits!(inst, 7:7) << 11) |
            (bits!(inst, 31:31) << 12);

        Instruction::BType {
            funct3: ((bits!(inst, 12:14) as u8)),
            rs1:    ((bits!(inst, 15:19) as u8)),
            rs2:    ((bits!(inst, 20:24) as u8)),
            imm:    (sign_extend32(imm.into(), 12) as u32),
        }
    }
}

pub fn decode(inst: u32) -> Result<(Opcode, Instruction)> {
    let opcode = Instruction::get_opcode(inst)?;

    use Opcode::*;
    match opcode {
        Load => Ok((opcode, Instruction::from_i(inst))),
        Store => Ok((opcode, Instruction::from_s(inst))),
        OpImm => Ok((opcode, Instruction::from_i(inst))),
        Op => Ok((opcode, Instruction::from_r(inst))),
        AddUpperImmPC => Ok((opcode, Instruction::from_u(inst))),
        LoadUpperImm => Ok((opcode, Instruction::from_u(inst))),
        JumpAndLink => Ok((opcode, Instruction::from_j(inst))),
        JumpAndLinkRegister => Ok((opcode, Instruction::from_i(inst))),
        Branch => Ok((opcode, Instruction::from_b(inst))),

        _ => bail!("opcode {:?} not decoded yet", opcode)
    }
}