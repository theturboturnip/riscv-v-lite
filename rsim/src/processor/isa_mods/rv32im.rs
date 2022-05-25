use crate::processor::isa_mods::*;

use crate::processor::exceptions::IllegalInstructionException::UnsupportedParam;

/// Base ISA module for RV32 ISAs.
/// Implements Integer and Multiply extensions.
pub struct Rv32im {}
impl IsaMod<Rv32imConn<'_>> for Rv32im {
    type Pc = u32;

    fn will_handle(&self, opcode: Opcode, _inst: InstructionBits) -> bool {
        use crate::processor::decode::Opcode::*;
        match opcode {
            Load | Store | OpImm | Op => true,
            AddUpperImmPC | LoadUpperImm | JumpAndLink | JumpAndLinkRegister | Branch => true,

            _ => false,
        }
    }

    fn execute(
        &mut self,
        opcode: Opcode,
        inst: InstructionBits,
        _inst_bits: u32,
        conn: Rv32imConn,
    ) -> ProcessorResult<Option<Self::Pc>> {
        let mut next_pc = None;

        use crate::processor::decode::Opcode::*;
        match (opcode, inst) {
            (
                Load,
                InstructionBits::IType {
                    rd,
                    funct3,
                    rs1,
                    imm,
                },
            ) => {
                let addr = conn.sreg.read(rs1)?.wrapping_add(imm.sign_extend_u32());
                let new_val = match funct3 {
                    // LB, LH, LW sign-extend if necessary
                    0b000 => (conn.memory.load_u8(addr as u64)? as i8) as i32 as u32, // LB
                    0b001 => (conn.memory.load_u16(addr as u64)? as i16) as i32 as u32, // LH
                    0b010 => conn.memory.load_u32(addr as u64)?,                      // LW
                    // LBU, LHU don't sign-extend
                    0b100 => conn.memory.load_u8(addr as u64)? as u32, // LBU
                    0b101 => conn.memory.load_u16(addr as u64)? as u32, // LBU

                    _ => bail!(UnsupportedParam(format!("Load funct3 {:03b}", funct3))),
                };
                conn.sreg.write(rd, new_val)?;
            }
            (
                Store,
                InstructionBits::SType {
                    funct3,
                    rs1,
                    rs2,
                    imm,
                },
            ) => {
                let addr = conn.sreg.read(rs1)?.wrapping_add(imm.sign_extend_u32());
                match funct3 {
                    0b000 => conn
                        .memory
                        .store_u8(addr as u64, (conn.sreg.read(rs2)? & 0xFF) as u8)?,
                    0b001 => conn
                        .memory
                        .store_u16(addr as u64, (conn.sreg.read(rs2)? & 0xFFFF) as u16)?,
                    0b010 => conn
                        .memory
                        .store_u32(addr as u64, (conn.sreg.read(rs2)? & 0xFFFF_FFFF) as u32)?,

                    _ => bail!(UnsupportedParam(format!("Store funct3 {:03b}", funct3))),
                };
            }

            (
                OpImm,
                InstructionBits::IType {
                    rd,
                    funct3,
                    rs1,
                    imm,
                },
            ) => {
                let input = conn.sreg.read(rs1)?;
                let imm = imm.sign_extend_u32();
                let new_val = match (imm, funct3) {
                    (imm, 0b000) => input.wrapping_add(imm), // ADDI
                    (imm, 0b010) => {
                        if (input as i32) < (imm as i32) {
                            1
                        } else {
                            0
                        }
                    } // SLTI
                    (imm, 0b011) => {
                        if input < imm {
                            1
                        } else {
                            0
                        }
                    } // SLTU
                    (imm, 0b100) => input ^ imm,             // XORI
                    (imm, 0b110) => input | imm,             // ORI
                    (imm, 0b111) => input & imm,             // ANDI

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

                    _ => unreachable!("OpImm funct3 {:03b}", funct3),
                };
                conn.sreg.write(rd, new_val)?;
            }

            (
                Op,
                InstructionBits::RType {
                    rd,
                    funct3,
                    rs1,
                    rs2,
                    funct7,
                },
            ) => {
                const ALT: u8 = 0b0100000;
                let x = conn.sreg.read(rs1)?;
                let y = conn.sreg.read(rs2)?;
                let new_val = match (funct7, funct3) {
                    (0, 0b000) => x.wrapping_add(y),   // ADD
                    (ALT, 0b000) => x.wrapping_sub(y), // SUB

                    (0, 0b001) => x << y, // SLL

                    (0, 0b010) => {
                        if (x as i32) < (y as i32) {
                            1
                        } else {
                            0
                        }
                    } // SLT
                    (0, 0b011) => {
                        if x < y {
                            1
                        } else {
                            0
                        }
                    } // SLTU

                    (0, 0b100) => x ^ y,                      // XOR
                    (0, 0b101) => x >> y,                     // SRL
                    (ALT, 0b101) => ((x as i32) >> y) as u32, // SRA
                    (0, 0b110) => x | y,                      // OR
                    (0, 0b111) => x & y,                      // AND

                    (1, funct3) => match funct3 {
                        0b000 => x * y, // MUL
                        // MULH
                        // Widen x,y to i64, multiply, shift down so we take top 32 bits of result
                        0b001 => ((x as i32 as i64) * (y as i32 as i64) >> 32) as i32 as u32,
                        // MULHSU
                        // Widen x to i64, y to u64 *then* i64 (doesn't sign-extend), multiply + shift
                        0b010 => ((x as i32 as i64) * (y as u64 as i64) >> 32) as i32 as u32,
                        // MULHU
                        // Widen x,y to u64, multiply+shift
                        0b011 => ((x as u64) * (y as u64) >> 32) as u32,
                        // TODO overflow semantics are not done correctly here
                        // DIV (signed divide)
                        0b100 => match (x, y) {
                            // Divide-by-zero (doesn't raise exception)
                            (_, 0) => (-1 as i32) as u32,
                            // Overflow = most-negative divided by -1
                            (0x8000_0000, 0xFFFF_FFFF) => 0x8000_0000,
                            // Normal signed divide - cast bits to i32, div, then cast to u32
                            (x, y) => ((x as i32) / (y as i32)) as u32,
                        },
                        // DIVU (unsigned divide)
                        0b101 => match (x, y) {
                            // Divide-by-zero (doesn't raise exception)
                            (_, 0) => (-1 as i32) as u32,
                            // Normal unsigned divide
                            (x, y) => x / y,
                        },
                        // REM (signed remainder)
                        0b110 => match (x, y) {
                            // Divide-by-zero (doesn't raise exception)
                            (x, 0) => x,
                            // Overflow = most-negative divided by -1
                            (0x8000_0000, 0xFFFF_FFFF) => 0,
                            // Normal signed remainder - cast bits to i32, div, then cast to u32
                            (x, y) => ((x as i32) % (y as i32)) as u32,
                        },
                        // REMU (unsigned remainder)
                        0b111 => match (x, y) {
                            // Divide-by-zero (doesn't raise exception)
                            (x, 0) => x,
                            // Normal unsigned remainder
                            (x, y) => x % y,
                        },
                        _ => unreachable!("Impossible funct3"),
                    },

                    _ => bail!(UnsupportedParam(format!(
                        "Op funct7/3: {:07b}, {:03b}",
                        funct7, funct3
                    ))),
                };
                conn.sreg.write(rd, new_val)?;
            }

            (AddUpperImmPC, InstructionBits::UType { rd, imm }) => {
                let addr = imm.sign_extend_u32().wrapping_add(conn.pc);
                conn.sreg.write(rd, addr)?;
            }

            (LoadUpperImm, InstructionBits::UType { rd, imm }) => {
                conn.sreg.write(rd, imm.sign_extend_u32())?;
            }

            (JumpAndLink, InstructionBits::JType { rd, imm }) => {
                conn.sreg.write(rd, conn.pc + 4)?;
                next_pc = Some(conn.pc.wrapping_add(imm.sign_extend_u32()));
            }
            (
                JumpAndLinkRegister,
                InstructionBits::IType {
                    rd,
                    funct3: 0b000,
                    rs1,
                    imm,
                },
            ) => {
                // Read rs1, add immediate, unset bottom bit
                next_pc = Some(conn.sreg.read(rs1)?.wrapping_add(imm.sign_extend_u32()) & (!1));

                conn.sreg.write(rd, conn.pc + 4)?;
            }

            (
                Branch,
                InstructionBits::BType {
                    funct3,
                    rs1,
                    rs2,
                    imm,
                },
            ) => {
                let src1 = conn.sreg.read(rs1)?;
                let src2 = conn.sreg.read(rs2)?;

                let take_branch = match funct3 {
                    0b000 => src1 == src2,                  // BEQ
                    0b001 => src1 != src2,                  // BNE
                    0b100 => (src1 as i32) < (src2 as i32), // BLT
                    0b101 => (src1 as i32) > (src2 as i32), // BGE
                    0b110 => (src1 as u32) < (src2 as u32), // BLTU
                    0b111 => (src1 as u32) > (src2 as u32), // BGEU

                    _ => bail!(UnsupportedParam(format!(
                        "funct3 for branch {:03b}",
                        funct3
                    ))),
                };

                if take_branch {
                    next_pc = Some(conn.pc.wrapping_add(imm.sign_extend_u32()));
                }
            }
            _ => bail!("Invalid opcode/instruction pair passed to RV32I"),
        }

        Ok(next_pc)
    }
}
