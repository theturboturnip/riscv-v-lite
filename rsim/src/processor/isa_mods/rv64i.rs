use crate::processor::elements::memory::Memory64;
use crate::processor::elements::registers::RegisterFile;
use crate::processor::isa_mods::*;

use crate::processor::exceptions::IllegalInstructionException::UnsupportedParam;

pub struct Rv64iConn<'a> {
    pub pc: u64,
    pub sreg: &'a mut dyn RegisterFile<u64>,
    pub memory: &'a mut dyn Memory64,
}
impl<'a> IsaModConn for Rv64iConn<'a> {}

pub struct Rv64i {}
impl IsaMod<Rv64iConn<'_>> for Rv64i {
    type Pc = u64;

    fn will_handle(&self, opcode: Opcode, _inst: InstructionBits) -> bool {
        use crate::processor::decode::Opcode::*;
        match opcode {
            Load | Store | OpImm | Op | OpImm32 | Op32 => true,
            AddUpperImmPC | LoadUpperImm | JumpAndLink | JumpAndLinkRegister | Branch => true,

            _ => false
        }
    }

    fn execute(&mut self, opcode: Opcode, inst: InstructionBits, _inst_bits: u32, conn: Rv64iConn) -> ProcessorResult<Option<Self::Pc>> {
        let mut next_pc = None;

        use crate::processor::decode::Opcode::*;
        match (opcode, inst) {
            (Load, InstructionBits::IType{rd, funct3, rs1, imm}) => {
                let addr = conn.sreg.read(rs1)?.wrapping_add(imm.sign_extend_u64());
                let new_val = match funct3 {
                    // LB, LH, LW sign-extend if necessary
                    0b000 => (conn.memory.load_u8(addr)? as i8) as i64 as u64, // LB
                    0b001 => (conn.memory.load_u16(addr)? as i16) as i64 as u64, // LH
                    0b010 => (conn.memory.load_u32(addr)? as i32) as i64 as u64, // LW
                    0b011 => conn.memory.load_u64(addr)? as u64, // LD
                    // LBU, LHU, LWU don't sign-extend
                    0b100 => conn.memory.load_u8(addr)? as u64, // LBU
                    0b101 => conn.memory.load_u16(addr)? as u64, // LHU
                    0b110 => conn.memory.load_u32(addr)? as u64, // LWU

                    _ => bail!(UnsupportedParam(format!("Load funct3 {:03b}", funct3)))
                };
                conn.sreg.write(rd, new_val)?;
            }
            (Store, InstructionBits::SType{funct3, rs1, rs2, imm}) => {
                let addr = conn.sreg.read(rs1)?.wrapping_add(imm.sign_extend_u64());
                match funct3 {
                    0b000 => conn.memory.store_u8(addr, (conn.sreg.read(rs2)? & 0xFF) as u8)?,
                    0b001 => conn.memory.store_u16(addr, (conn.sreg.read(rs2)? & 0xFFFF) as u16)?,
                    0b010 => conn.memory.store_u32(addr, (conn.sreg.read(rs2)? & 0xFFFF_FFFF) as u32)?,
                    0b011 => conn.memory.store_u64(addr, (conn.sreg.read(rs2)? & 0xFFFF_FFFF_FFFF_FFFF) as u64)?,
                    
                    _ => bail!(UnsupportedParam(format!("Store funct3 {:03b}", funct3)))
                };
            }

            (OpImm, InstructionBits::IType{rd, funct3, rs1, imm}) => {
                let imm = imm.sign_extend_u64();
                let input = conn.sreg.read(rs1)?;
                let new_val = match (imm, funct3) {
                    (imm, 0b000) => input.wrapping_add(imm), // ADDI
                    (imm, 0b010) => if (input as i64) < (imm as i64) { 1 } else { 0 }, // SLTI
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
                            ((input as i64) >> shamt) as u64
                        } else {
                            // SRLI
                            input >> shamt
                        }
                    }

                    _ => unreachable!("OpImm funct3 {:03b}", funct3)
                };
                conn.sreg.write(rd, new_val)?;
            }
            (OpImm32, InstructionBits::IType{rd, funct3, rs1, imm}) => {
                // Op32 = do all operations with 32-bit values, then sign-extend the 32-bit version up to 64-bit
                let input = conn.sreg.read(rs1)? as u32;
                let imm = imm.sign_extend_u32();
                let new_val = match (imm, funct3) {
                    (imm, 0b000) => input.wrapping_add(imm), // ADDIW

                    (imm, 0b001) => {
                        // SLLIW
                        // shamt = bottom-five-bits
                        if imm & 0x10 != 0 {
                            bail!(UnsupportedParam(format!("OpImm32 shamt has bit 5 set")))
                        }
                        let shamt = imm & 0x0F;
                        input << shamt
                    },
                    (imm, 0b101) => {
                        // Check top bits of imm to see if arithmetic or logical
                        // shamt = bottom-five-bits
                        if imm & 0x10 != 0 {
                            bail!(UnsupportedParam(format!("OpImm32 shamt has bit 5 set")))
                        }
                        let shamt = imm & 0x0F;
                        if ((imm >> 10) & 1) == 1 {
                            // SRAIW
                            // input as i32 => shift will be arithmetic
                            // cast back to u32 afterwards
                            ((input as i32) >> shamt) as u32
                        } else {
                            // SRLIW
                            input >> shamt
                        }
                    }

                    _ => unreachable!("OpImm funct3 {:03b}", funct3)
                };
                // Sign-extend 32-bit value to 64-bit
                conn.sreg.write(rd, new_val as i32 as i64 as u64)?;
            }

            (Op, InstructionBits::RType{rd, funct3, rs1, rs2, funct7}) => {
                const ALT: u8 = 0b0100000;
                let x = conn.sreg.read(rs1)?;
                let y = conn.sreg.read(rs2)?;
                let new_val = match (funct7, funct3) {
                    (0, 0b000) => x.wrapping_add(y), // ADD
                    
                    (ALT, 0b000) => x.wrapping_sub(y), // SUB

                    (0, 0b001) => x << y, // SLL
                    
                    (0, 0b010) => if (x as i64) < (y as i64) { 1 } else { 0 }, // SLT
                    (0, 0b011) => if x < y { 1 } else { 0 }, // SLTU

                    (0, 0b100) => x ^ y, // XOR 
                    (0, 0b101) => x >> y, // SRL
                    (ALT, 0b101) => ((x as i64) >> y) as u64, // SRA
                    (0, 0b110) => x | y, // OR
                    (0, 0b111) => x & y, // AND

                    _ => bail!(UnsupportedParam(format!("Op funct7/3: {:07b}, {:03b}", funct7, funct3)))
                };
                conn.sreg.write(rd, new_val)?;
            }
            (Op32, InstructionBits::RType{rd, funct3, rs1, rs2, funct7}) => {
                // Op32 = do all operations with 32-bit values, then sign-extend the 32-bit version up to 64-bit
                const ALT: u8 = 0b0100000;
                let x = conn.sreg.read(rs1)? as u32;
                let y = conn.sreg.read(rs2)? as u32;
                let new_val = match (funct7, funct3) {
                    (0, 0b000) => x.wrapping_add(y), // ADDW

                    (0, 0b001) => x << y, // SLLW
                    (0, 0b101) => x >> y, // SRLW

                    (ALT, 0b000) => x.wrapping_sub(y), // SUBW
                    (ALT, 0b101) => ((x as i32) >> y) as u32, // SRAW

                    _ => bail!(UnsupportedParam(format!("Op funct7/3: {:07b}, {:03b}", funct7, funct3)))
                };
                conn.sreg.write(rd, (new_val as i32) as i64 as u64)?;
            }

            (AddUpperImmPC, InstructionBits::UType{rd, imm}) => {
                let addr = (imm.sign_extend_u64()) + conn.pc;
                conn.sreg.write(rd, addr)?;
            }

            (LoadUpperImm, InstructionBits::UType{rd, imm}) => {
                conn.sreg.write(rd, imm.sign_extend_u64())?;
            }

            (JumpAndLink, InstructionBits::JType{rd, imm}) => {
                conn.sreg.write(rd, conn.pc + 4)?;
                next_pc = Some(conn.pc.wrapping_add(imm.sign_extend_u64()));
            }
            (JumpAndLinkRegister, InstructionBits::IType{rd, funct3: 0b000, rs1, imm}) => {
                // Read rs1, add immediate, unset bottom bit
                next_pc = Some(conn.sreg.read(rs1)?.wrapping_add(imm.sign_extend_u64()) & (!1));

                conn.sreg.write(rd, conn.pc + 4)?;
            }

            (Branch, InstructionBits::BType{funct3, rs1, rs2, imm}) => {
                let src1 = conn.sreg.read(rs1)?;
                let src2 = conn.sreg.read(rs2)?;

                let take_branch = match funct3 {
                    0b000 => src1 == src2, // BEQ
                    0b001 => src1 != src2, // BNE
                    0b100 => (src1 as i64) < (src2 as i64), // BLT
                    0b101 => (src1 as i64) > (src2 as i64), // BGE
                    0b110 => (src1 as u64) < (src2 as u64), // BLTU
                    0b111 => (src1 as u64) > (src2 as u64), // BGEU

                    _ => bail!(UnsupportedParam(format!("funct3 for branch {:03b}", funct3)))
                };

                if take_branch {
                    next_pc = Some(conn.pc.wrapping_add(imm.sign_extend_u64()));
                }
            }
            _ => bail!("Invalid opcode/instruction pair passed to RV32I")
        }

        Ok(next_pc)
    }
}