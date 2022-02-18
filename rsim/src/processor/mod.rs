use crate::processor::IllegalInstructionException::MiscDecodeException;
use crate::processor::IllegalInstructionException::UnsupportedParam;
use std::mem::size_of;
use anyhow::{Context,Result};

use bitutils::sign_extend32;

pub mod memory;
pub use memory::Memory;

pub mod exceptions;
use exceptions::{IllegalInstructionException,MemoryException};

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

/// Defines if Zicsr extension (CSR instructions) is enabled
#[allow(non_upper_case_globals)]
const EXT_Zicsr: bool = true;

pub trait CSRProvider {
    /// Does the Provider provide access to a given CSR?
    fn has_csr(&self, csr: u32) -> bool;
    
    /// Atomic Read/Write a CSR
    /// 
    /// Will only be called on `csr` if `has_csr(csr) == true`
    /// 
    /// Reads can have side-effects, and some variants of the instruction disable that.
    /// If `need_read == false', it won't perform a read or any side-effects, and will return an Ok(None).
    /// Otherwise will return an Ok(Some()) with the previous CSR value, and execute side-effects.
    fn csr_atomic_read_write(&mut self, csr: u32, need_read: bool, write_val: u32) -> Result<Option<u32>>;

    /// Atomically read and set specific bits in a CSR
    /// 
    /// Will only be called on `csr` if `has_csr(csr) == true`
    /// 
    /// If `set_bits == None', no write will be performed (thus no side-effects of the write will happen).
    /// The CSR will always be read, and those side-effects will always be applied.
    fn csr_atomic_read_set(&mut self, csr: u32, set_bits: Option<u32>) -> Result<u32>;

    /// Atomically read and clear specific bits in a CSR
    /// 
    /// Will only be called on `csr` if `has_csr(csr) == true`
    /// 
    /// If `clear_bits == None', no write will be performed (thus no side-effects of the write will happen).
    /// The CSR will always be read, and those side-effects will always be applied.
    fn csr_atomic_read_clear(&mut self, csr: u32, clear_bits: Option<u32>) -> Result<u32>;
}

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

    /// Get a short-lived connection to scalar resources, usable by the vector unit.
    /// This connection holds mutable references to fields in the Processor.
    /// 
    /// # Associated Lifetimes
    /// 
    /// * `'a` - The lifetime of the Processor
    /// * `'b` - The lifetime of the references held in the VectorUnitConnection
    /// 
    /// `'a : 'b` => `'a` outlives `'b`, e.g. the Processor will live longer than the references to its fields.
    /// Rust needs this guarantee.
    /// 
    /// Because Rust isn't smart enough to understand *which* fields in the processor are referenced,
    /// and the references inside the [VectorUnitConnection] are mutable,
    /// holding a [VectorUnitConnection] is equivalent to holding a *mutable reference to the Processor and all its fields*.
    /// This means you couldn't, say, store the [VectorUnit] inside the Processor and do `processor.v_unit.exec_inst(connection)`,
    /// because [VectorUnit::exec_inst()] tries to take a mutable reference to [VectorUnit], but the `connection` holds that reference already.
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
                let addr = self.sreg[rs1 as usize].wrapping_add(imm);
                self.sreg[rd as usize] = match funct3 {
                    // LB, LH, LW sign-extend if necessary
                    0b000 => sign_extend32(self.memory.load_u8(addr)? as u32, 8) as u32, // LB
                    0b001 => sign_extend32(self.memory.load_u16(addr)? as u32, 16) as u32, // LH
                    0b010 => self.memory.load_u32(addr)?, // LW
                    // LBU, LHU don't sign-extend
                    0b100 => self.memory.load_u8(addr)? as u32, // LBU
                    0b101 => self.memory.load_u16(addr)? as u32, // LBU

                    _ => bail!(UnsupportedParam(format!("Load funct3 {:03b}", funct3)))
                };
            }
            (Store, InstructionBits::SType{funct3, rs1, rs2, imm}) => {
                let addr = self.sreg[rs1 as usize].wrapping_add(imm);
                match funct3 {
                    0b000 => self.memory.store_u8(addr, (self.sreg[rs2 as usize] & 0xFF) as u8)?,
                    0b001 => self.memory.store_u16(addr, (self.sreg[rs2 as usize] & 0xFFFF) as u16)?,
                    0b010 => self.memory.store_u32(addr, self.sreg[rs2 as usize])?,
                    
                    _ => bail!(UnsupportedParam(format!("Store funct3 {:03b}", funct3)))
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

                    _ => unreachable!("OpImm funct3 {:03b}", funct3)
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

                    _ => bail!(UnsupportedParam(format!("Op funct7/3: {:07b}, {:03b}", funct7, funct3)))
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

                    _ => bail!(UnsupportedParam(format!("funct3 for branch {:03b}", funct3)))
                };

                if take_branch {
                    next_pc = self.pc.wrapping_add(imm);
                }
            }

            (System, InstructionBits::IType{rd, funct3, rs1, imm}) => {

                if funct3 == 0b000 {
                    bail!("Non-CSR System instructions not supported")
                } else if EXT_Zicsr {
                    let csr = imm;

                    let is_imm_instruction = (funct3 & 0b100) != 0;
                    // rs1 can be an immediate value if the top bit of funct3 is set
                    // Take the "rs1 value" as either the immediate or the register[rs1]
                    let rs1_val = if is_imm_instruction {
                        rs1 as uXLEN
                    } else {
                        self.sreg[rs1 as usize]
                    };

                    match funct3 {
                        0b001 | 0b101 => {
                            // Atomic Read-Write CSR
                            let need_read = rd == 0;
                            let write_val = rs1_val;

                            // Perform the read/write
                            let old_csr_val = {
                                if self.has_csr(csr) {
                                    self.csr_atomic_read_write(csr, need_read, write_val)?
                                } else if v_unit.has_csr(csr) {
                                    v_unit.csr_atomic_read_write(csr, need_read, write_val)?
                                } else {
                                    bail!("No provider found for Read/Write of CSR 0x{:04x}", csr)
                                }
                            };

                            if need_read {
                                self.sreg[rd as usize] = old_csr_val.unwrap();
                            }
                        }

                        0b010 | 0b110 => {
                            // Atomic Read-Set CSR
                            // If the register index (or immediate value) is equal to 0, then no write is performed
                            let write_val = if rs1 == 0 {
                                None
                            } else {
                                Some(rs1_val)
                            };

                            // Perform the read/write
                            let old_csr_val = {
                                if self.has_csr(csr) {
                                    self.csr_atomic_read_set(csr, write_val)?
                                } else if v_unit.has_csr(csr) {
                                    v_unit.csr_atomic_read_set(csr, write_val)?
                                } else {
                                    bail!("No provider found for Read/Set of CSR 0x{:04x}", csr)
                                }
                            };

                            self.sreg[rd as usize] = old_csr_val;
                        }
                        0b011 | 0b111 => {
                            // Atomic Read-Clear CSR
                            // If the register index (or immediate value) is equal to 0, then no write is performed
                            let write_val = if rs1 == 0 {
                                None
                            } else {
                                Some(rs1_val)
                            };

                            // Perform the read/write
                            let old_csr_val = {
                                if self.has_csr(csr) {
                                    self.csr_atomic_read_clear(csr, write_val)?
                                } else if v_unit.has_csr(csr) {
                                    v_unit.csr_atomic_read_clear(csr, write_val)?
                                } else {
                                    bail!("No provider found for Read/Clear of CSR 0x{:04x}", csr)
                                }
                            };

                            self.sreg[rd as usize] = old_csr_val;
                        }

                        0b000 | _ => unreachable!("impossible funct3")
                    }
                } else {
                    // funct3 != 0 but EXT_Zicsr not enabled
                    bail!("CSR extension not enabled, CSR instruction won't work");
                }
                
            }

            // Delegate all instructions under the Vector opcode to the vector unit
            (Vector, inst) => v_unit.exec_inst(opcode, inst, inst_bits, self.vector_conn())?,

            (LoadFP | StoreFP, InstructionBits::FLdStType{width, ..}) => {
                // Check the access width
                match width {
                    0b0001 | 0b0010 | 0b0011 | 0b0100 => 
                        bail!(UnsupportedParam(format!("LoadFP/StoreFP uses width for actual floats, not supported"))),
                    0b1000..=0b1111 => 
                        bail!(UnsupportedParam(format!("LoadFP/StoreFP using reserved width {}", width))),

                    // This width corresponds to a vector, delegate this instruction to the vector unit
                    _ => v_unit.exec_inst(opcode, inst, inst_bits, self.vector_conn())?
                }
            },

            _ => bail!(MiscDecodeException("Unexpected opcode/InstructionBits pair".to_string()))
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

        let next_pc_res: Result<u32> = {
            // Fetch
            let inst_bits = self.memory.load_u32(self.pc).context("Couldn't load next instruction")?;

            // Decode
            let (opcode, inst) = decode(inst_bits)
                .with_context(|| format!("Failed to decode instruction {:08x}", inst_bits))?;

            // Execute
            let next_pc = self.process_inst(v_unit, inst_bits, opcode, inst)
                .with_context(|| format!("Failed to execute decoded instruction {:?} {:?}", opcode, inst))?;

            if next_pc % 4 != 0 {
                anyhow!(MemoryException::JumpMisaligned{addr: next_pc as usize, expected: 4})
            } else {
                Ok(next_pc)
            }
        };

        let next_pc = match next_pc_res {
            Ok(val) => val,
            Err(err) => {
                if let Some(_iie) = err.downcast_ref::<IllegalInstructionException>() {
                    // TODO - trap, return new PC
                    println!("Found Illegal Instruction error");
                    return Err(err)
                } else if let Some(_mem) = err.downcast_ref::<MemoryException>() {
                    // TODO - trap, return new PC
                    println!("Found Memory error");
                    return Err(err)
                } else {
                    println!("Untrappable error");
                    return Err(err)
                }
            }
        };

        // Restore x0 => 0
        self.sreg[0] = 0;

        // Increment PC
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

impl CSRProvider for Processor {
    fn has_csr(&self, _csr: u32) -> bool {
        false
    }
    fn csr_atomic_read_write(&mut self, _csr: u32, _need_read: bool, _write_val: u32) -> Result<Option<u32>> { todo!() }
    fn csr_atomic_read_set(&mut self, _csr: u32, _set_bits: Option<u32>) -> Result<u32> { todo!() }
    fn csr_atomic_read_clear(&mut self, _csr: u32, _clear_bits: Option<u32>) -> Result<u32> { todo!() }
}
