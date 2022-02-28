use crate::processor::elements::memory::Memory32;
use crate::processor::elements::registers::RegisterFile;
use crate::processor::isa_mods::*;

use crate::processor::exceptions::IllegalInstructionException::UnsupportedParam;
use bitutils::sign_extend32;

pub struct Rv32iConn<'a> {
    pub pc: u32,
    pub sreg: &'a mut dyn RegisterFile<u32>,
    pub memory: &'a mut dyn Memory32,
}
impl<'a> IsaModConn for Rv32iConn<'a> {}

pub struct Rv32i {}
impl IsaMod<Rv32iConn<'_>> for Rv32i {
    type Pc = u32;

    fn will_handle(&self, opcode: Opcode, _inst: InstructionBits) -> bool {
        use crate::processor::decode::Opcode::*;
        match opcode {
            Load | Store | OpImm | Op => true,
            AddUpperImmPC | LoadUpperImm | JumpAndLink | JumpAndLinkRegister | Branch => true,

            _ => false
        }
    }

    fn execute(&mut self, opcode: Opcode, inst: InstructionBits, _inst_bits: u32, conn: Rv32iConn) -> ProcessorResult<Option<Self::Pc>> {
        let mut next_pc = None;

        use crate::processor::decode::Opcode::*;
        match (opcode, inst) {
            (Load, InstructionBits::IType{rd, funct3, rs1, imm}) => {
                let addr = conn.sreg.read(rs1)?.wrapping_add(imm);
                let new_val = match funct3 {
                    // LB, LH, LW sign-extend if necessary
                    0b000 => sign_extend32(conn.memory.load_u8(addr as u64)? as u32, 8) as u32, // LB
                    0b001 => sign_extend32(conn.memory.load_u16(addr as u64)? as u32, 16) as u32, // LH
                    0b010 => conn.memory.load_u32(addr as u64)?, // LW
                    // LBU, LHU don't sign-extend
                    0b100 => conn.memory.load_u8(addr as u64)? as u32, // LBU
                    0b101 => conn.memory.load_u16(addr as u64)? as u32, // LBU

                    _ => bail!(UnsupportedParam(format!("Load funct3 {:03b}", funct3)))
                };
                conn.sreg.write(rd, new_val)?;
            }
            (Store, InstructionBits::SType{funct3, rs1, rs2, imm}) => {
                let addr = conn.sreg.read(rs1)?.wrapping_add(imm);
                match funct3 {
                    0b000 => conn.memory.store_u8(addr as u64, (conn.sreg.read(rs2)? & 0xFF) as u8)?,
                    0b001 => conn.memory.store_u16(addr as u64, (conn.sreg.read(rs2)? & 0xFFFF) as u16)?,
                    0b010 => conn.memory.store_u32(addr as u64, (conn.sreg.read(rs2)? & 0xFFFF_FFFF) as u32)?,
                    
                    _ => bail!(UnsupportedParam(format!("Store funct3 {:03b}", funct3)))
                };
            }

            (OpImm, InstructionBits::IType{rd, funct3, rs1, imm}) => {
                let input = conn.sreg.read(rs1)?;
                let new_val = match (imm, funct3) {
                    (imm, 0b000) => input.wrapping_add(imm), // ADDI
                    (imm, 0b010) => if (input as i32) < (imm as i32) { 1 } else { 0 }, // SLTI
                    (imm, 0b011) => if input < imm { 1 } else { 0 }, // SLTU
                    (imm, 0b100) => input ^ imm, // XORI
                    (imm, 0b110) => input | imm, // ORI
                    (imm, 0b111) => input & imm, // ANDI

                    (shamt, 0b001) => input << shamt, // SLLI
                    (imm, 0b101) => {
                        // Check top bits of imm to see if arithmetic or logical
                        // shamt = bottom-five-bits
                        let shamt = imm & 0x1F;
                        if ((imm >> 10) & 1) == 1 {
                            // SRAI
                            // input as i32 => shift will be arithmetic
                            // cast back to u32 afterwards
                            ((input as i32) >> shamt) as u32
                        } else {
                            // SRLI
                            input >> shamt
                        }
                    }

                    _ => unreachable!("OpImm funct3 {:03b}", funct3)
                };
                conn.sreg.write(rd, new_val)?;
            }

            (Op, InstructionBits::RType{rd, funct3, rs1, rs2, funct7}) => {
                const ALT: u8 = 0b0100000;
                let x = conn.sreg.read(rs1)?;
                let y = conn.sreg.read(rs2)?;
                let new_val = match (funct7, funct3) {
                    (0, 0b000) => x.wrapping_add(y), // ADD
                    (ALT, 0b000) => x.wrapping_sub(y), // SUB

                    (0, 0b001) => x << y, // SLL
                    
                    (0, 0b010) => if (x as i32) < (y as i32) { 1 } else { 0 }, // SLT
                    (0, 0b011) => if x < y { 1 } else { 0 }, // SLTU

                    (0, 0b100) => x ^ y, // XOR 
                    (0, 0b101) => x >> y, // SRL
                    (ALT, 0b101) => ((x as i32) >> y) as u32, // SRA
                    (0, 0b110) => x | y, // OR
                    (0, 0b111) => x & y, // AND

                    _ => bail!(UnsupportedParam(format!("Op funct7/3: {:07b}, {:03b}", funct7, funct3)))
                };
                conn.sreg.write(rd, new_val)?;
            }

            (AddUpperImmPC, InstructionBits::UType{rd, imm}) => {
                let addr = imm + conn.pc;
                conn.sreg.write(rd, addr)?;
            }

            (LoadUpperImm, InstructionBits::UType{rd, imm}) => {
                conn.sreg.write(rd, imm)?;
            }

            (JumpAndLink, InstructionBits::JType{rd, imm}) => {
                conn.sreg.write(rd, conn.pc + 4)?;
                next_pc = Some(conn.pc.wrapping_add(imm));
            }
            (JumpAndLinkRegister, InstructionBits::IType{rd, funct3: 0b000, rs1, imm}) => {
                // Read rs1, add immediate, unset bottom bit
                next_pc = Some(conn.sreg.read(rs1)?.wrapping_add(imm) & (!1));

                conn.sreg.write(rd, conn.pc + 4)?;
            }

            (Branch, InstructionBits::BType{funct3, rs1, rs2, imm}) => {
                let src1 = conn.sreg.read(rs1)?;
                let src2 = conn.sreg.read(rs2)?;

                let take_branch = match funct3 {
                    0b000 => src1 == src2, // BEQ
                    0b001 => src1 != src2, // BNE
                    0b100 => (src1 as i32) < (src2 as i32), // BLT
                    0b101 => (src1 as i32) > (src2 as i32), // BGE
                    0b110 => (src1 as u32) < (src2 as u32), // BLTU
                    0b111 => (src1 as u32) > (src2 as u32), // BGEU

                    _ => bail!(UnsupportedParam(format!("funct3 for branch {:03b}", funct3)))
                };

                if take_branch {
                    next_pc = Some(conn.pc.wrapping_add(imm));
                }
            }
            _ => bail!("Invalid opcode/instruction pair passed to RV32I")
        }

        Ok(next_pc)
    }
}