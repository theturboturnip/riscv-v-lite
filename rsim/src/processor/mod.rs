use std::mem::size_of;
use anyhow::{Context,Result};

use bitutils::sign_extend32;

use super::memory::Memory;

pub mod decode;
use decode::{decode, InstructionBits};

pub mod vector;
use vector::{VectorUnit, VectorUnitConnection};

/// Scalar register length in bits
pub const XLEN: usize = 32;

/// Unsigned type of length [XLEN]
/// 
/// ```
/// use rsim::processor::{uXLEN, XLEN};
/// use std::mem::size_of;
/// 
/// assert_eq!(size_of::<uXLEN>() * 8, XLEN);
/// ```
#[allow(non_camel_case_types)]
pub type uXLEN = u32;
const_assert!(size_of::<uXLEN>() * 8 == XLEN);

/// The processor.
/// Holds scalar registers and configuration, all vector-related stuff is in [VectorUnit]. 
pub struct Processor {
    pub running: bool,
    pub memory: Memory,
    pc: uXLEN,
    sreg: [uXLEN; 32],
}

impl Processor {
    /// Create a new processor and vector unit which operates on given memory.
    ///
    /// # Arguments
    /// 
    /// * `mem` - The memory the processor should hold. Currently a value, not a reference.
    pub fn new(mem: Memory) -> (Processor, VectorUnit) {
        let mut p = Processor {
            running: false,
            memory: mem,
            pc: 0,
            sreg: [0; 32]
        };
        let mut v = VectorUnit::new();

        p.reset(&mut v);

        (p, v)
    }

    /// Get a short-lived connection to scalar resources, usable by the vector unit
    fn vector_conn<'a,'b>(&'a mut self) -> VectorUnitConnection<'b> where 'a: 'b {
        VectorUnitConnection {
            sreg: &mut self.sreg,
            memory: &mut self.memory,
        }
    }

    /// Reset the processor and associated vector unit
    pub fn reset(&mut self, v_unit: &mut VectorUnit) {
        self.running = false;
        self.pc = 0;
        self.sreg = [0; 32];

        v_unit.reset();
    }

    /// Process an instruction, returning the new PC value or any execution error
    /// 
    /// # Arguments
    /// 
    /// * `v_unit` - The associated vector unit, which will execute vector instructions if they are found.
    /// * `inst_bits` - The raw instruction bits
    /// * `opcode` - The major opcode of the decoded instruction
    /// * `inst` - The fields of the decoded instruction
    fn process_inst(&mut self, v_unit: &mut VectorUnit, inst_bits: u32, opcode: decode::Opcode, inst: InstructionBits) -> Result<u32> {
        let mut next_pc = self.pc + 4;
        
        use decode::Opcode::*;
        match (opcode, inst) {
            (Load, InstructionBits::IType{rd, funct3, rs1, imm}) => {
                let addr = self.sreg[rs1 as usize] + imm;
                self.sreg[rd as usize] = match funct3 {
                    // LB, LH, LW sign-extend if necessary
                    0b000 => sign_extend32(self.memory.load_u8(addr)? as u32, 8) as u32, // LB
                    0b001 => sign_extend32(self.memory.load_u16(addr)? as u32, 16) as u32, // LH
                    0b010 => self.memory.load_u32(addr)?, // LW
                    // LBU, LHU don't sign-extend
                    0b100 => self.memory.load_u8(addr)? as u32, // LBU
                    0b101 => self.memory.load_u16(addr)? as u32, // LBU

                    _ => bail!("Unexpected Load funct3 {:03b}", funct3)
                };
            }
            (Store, InstructionBits::SType{funct3, rs1, rs2, imm}) => {
                let addr = self.sreg[rs1 as usize] + imm;
                match funct3 {
                    0b000 => self.memory.store_u8(addr, (self.sreg[rs2 as usize] & 0xFF) as u8)?,
                    0b001 => self.memory.store_u16(addr, (self.sreg[rs2 as usize] & 0xFFFF) as u16)?,
                    0b010 => self.memory.store_u32(addr, self.sreg[rs2 as usize])?,
                    
                    _ => bail!("Unexpected Store funct3 {:03b}", funct3)
                };
            }

            (OpImm, InstructionBits::IType{rd, funct3, rs1, imm}) => {
                let input = self.sreg[rs1 as usize];
                self.sreg[rd as usize] = match (imm, funct3) {
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

                    _ => bail!("Unexpected OpImm funct3 {:03b}", funct3)
                };
            }

            (Op, InstructionBits::RType{rd, funct3, rs1, rs2, funct7}) => {
                const ALT: u8 = 0b0100000;
                let x = self.sreg[rs1 as usize];
                let y = self.sreg[rs2 as usize];
                self.sreg[rd as usize] = match (funct7, funct3) {
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

                    _ => bail!("Unexpected Op funct7/3: {:07b}, {:03b}", funct7, funct3)
                };
            }

            (AddUpperImmPC, InstructionBits::UType{rd, imm}) => {
                let addr = imm + self.pc;
                self.sreg[rd as usize] = addr;
            }

            (LoadUpperImm, InstructionBits::UType{rd, imm}) => {
                self.sreg[rd as usize] = imm;
            }

            (JumpAndLink, InstructionBits::JType{rd, imm}) => {
                self.sreg[rd as usize] = self.pc + 4;
                next_pc = self.pc.wrapping_add(imm);
            }
            (JumpAndLinkRegister, InstructionBits::IType{rd, funct3: 0b000, rs1, imm}) => {
                next_pc = self.sreg[rs1 as usize].wrapping_add(imm);
                // Unset bottom bit
                next_pc = next_pc & 0xFFFF_FFFE;

                self.sreg[rd as usize] = self.pc + 4;
            }

            (Branch, InstructionBits::BType{funct3, rs1, rs2, imm}) => {
                let src1 = self.sreg[rs1 as usize];
                let src2 = self.sreg[rs2 as usize];

                let take_branch = match funct3 {
                    0b000 => src1 == src2, // BEQ
                    0b001 => src1 != src2, // BNE
                    0b100 => (src1 as i32) < (src2 as i32), // BLT
                    0b101 => (src1 as i32) > (src2 as i32), // BGE
                    0b110 => (src1 as u32) < (src2 as u32), // BLTU
                    0b111 => (src1 as u32) > (src2 as u32), // BGEU

                    _ => bail!("Unexpected funct3 for branch {:03b}", funct3)
                };

                if take_branch {
                    next_pc = self.pc.wrapping_add(imm);
                }
            }

            (Vector, inst) => v_unit.exec_inst(opcode, inst, inst_bits, self.vector_conn())?,

            (LoadFP | StoreFP, InstructionBits::FLdStType{width, mew, ..}) => {
                if mew { bail!("LoadFP/StoreFP with mew = 1 is reserved") }

                match width {
                    0b0001 | 0b0010 | 0b0011 | 0b0100 => bail!("LoadFP/StoreFP uses width for actual floats, not supported"),
                    0b1000..=0b1111 => bail!("LoadFP/StoreFP using reserved width {}", width),

                    _ => v_unit.exec_inst(opcode, inst, inst_bits, self.vector_conn())?
                }
            },

            _ => bail!("Unexpected opcode/InstructionBits pair")
        }

        Ok(next_pc)
    }

    /// Run a fetch-decode-execute step on the processor, executing a single instruction
    /// 
    /// # Arguments
    /// 
    /// * `v_unit` - The associated vector unit, which will execute vector instructions if they are found.
    pub fn exec_step(&mut self, v_unit: &mut VectorUnit) -> Result<()> {
        self.running = true;

        // Fetch
        let inst_bits = self.memory.load_u32(self.pc).context("Couldn't load next instruction")?;

        // Decode
        let (opcode, inst) = decode(inst_bits)
            .with_context(|| format!("Failed to decode instruction {:08x}", inst_bits))?;

        // Execute
        let next_pc = self.process_inst(v_unit, inst_bits, opcode, inst)
            .with_context(|| format!("Failed to execute decoded instruction {:?} {:?}", opcode, inst))
            ?;

        // Restore x0 => 0
        self.sreg[0] = 0;

        // Increment PC
        if next_pc % 4 != 0 {
            bail!("PC was set to a misaligned value {:?}", next_pc);
        }
        self.pc = next_pc;

        Ok(())
    }

    /// Dump processor and vector unit state to standard output.
    pub fn dump(&self, v_unit: &mut VectorUnit) {
        const REGISTER_NAMES: [&str; 32] = [
            "zero", "ra", "sp", "gp",
            "tp", "t0", "t1", "t2",
            "fp", "s1", "a0", "a1",
            "a2", "a3", "a4", "a5",
            "a6", "a7", "s2", "s3",
            "s4", "s5", "s6", "s7",
            "s8", "s9", "s10", "s11",
            "t3", "t4", "t5", "t6"
        ];

        println!("running: {:?}\npc: 0x{:08x}", self.running, self.pc);
        for i in 0..32 {
            println!("x{} = {} = 0x{:08x}", i, REGISTER_NAMES[i], self.sreg[i]);
        }
        v_unit.dump();
    }
}