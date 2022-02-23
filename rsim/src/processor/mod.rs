use crate::processor::IllegalInstructionException::MiscDecodeException;
use crate::processor::IllegalInstructionException::UnsupportedParam;
use std::mem::size_of;
use anyhow::{Context,Result};

use bitutils::sign_extend32;

pub mod exceptions;
use exceptions::{IllegalInstructionException,MemoryException};

pub mod decode;
use decode::{decode, InstructionBits};

pub mod elements;
use elements::{AggregateMemory,ProcessorMemory,RV32RegisterFile,RegisterFile,RegisterTracking};

pub mod isa_mods;
use isa_mods::{IsaMod, Rvv, RvvConn, Zicsr, ZicsrConn, CSRProvider};

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
    pub memory: AggregateMemory,
    pc: uXLEN,
    sreg: RV32RegisterFile,
    csrs: ProcessorCSRs,
}
pub struct ProcessorModules {
    rvv: Option<Rvv>,
    zicsr: Option<Zicsr>
}

impl Processor {
    /// Create a new processor and vector unit which operates on given memory.
    ///
    /// # Arguments
    /// 
    /// * `mem` - The memory the processor should hold. Currently a value, not a reference.
    pub fn new(mem: AggregateMemory) -> (Processor, ProcessorModules) {
        let mut p = Processor {
            running: false,
            memory: mem,
            pc: 0,
            sreg: RV32RegisterFile::default(),
            csrs: ProcessorCSRs{}
        };
        let mut mods = ProcessorModules {
            rvv: Some(Rvv::new()),
            zicsr: Some(Zicsr{})
        };

        p.reset(&mut mods);

        (p, mods)
    }

    /// Get a short-lived connection to scalar resources, usable by the vector unit.
    /// This connection holds mutable references to fields in the Processor.
    /// 
    /// # Associated Lifetimes
    /// 
    /// * `'a` - The lifetime of the Processor
    /// * `'b` - The lifetime of the references held in the RvvConn
    /// 
    /// `'a : 'b` => `'a` outlives `'b`, e.g. the Processor will live longer than the references to its fields.
    /// Rust needs this guarantee.
    /// 
    /// Because Rust isn't smart enough to understand *which* fields in the processor are referenced,
    /// and the references inside the [RvvConn] are mutable,
    /// holding a [RvvConn] is equivalent to holding a *mutable reference to the Processor and all its fields*.
    /// This means you couldn't, say, store the [VectorUnit] inside the Processor and do `processor.v_unit.exec_inst(connection)`,
    /// because [VectorUnit::exec_inst()] tries to take a mutable reference to [VectorUnit], but the `connection` holds that reference already.
    fn vector_conn<'a,'b>(&'a mut self) -> RvvConn<'b> where 'a: 'b {
        RvvConn {
            sreg: &mut self.sreg,
            memory: &mut self.memory,
        }
    }

    fn zicsr_conn<'a,'b>(&'a mut self, rvv: &'a mut Option<Rvv>) -> ZicsrConn<'b> where 'a: 'b {
        let mut csr_providers = vec![&mut self.csrs as &mut dyn CSRProvider];
        if let Some(rvv) = rvv.as_mut() {
            csr_providers.push(rvv as &mut dyn CSRProvider)
        }
        ZicsrConn {
            sreg: &mut self.sreg,
            csr_providers
        }
    }

    /// Reset the processor and associated vector unit
    pub fn reset(&mut self, mods: &mut ProcessorModules) {
        self.running = false;
        self.pc = 0;
        self.sreg.reset();

        if let Some(v_unit) = mods.rvv.as_mut() {
            v_unit.reset();
        }
    }

    /// Process an instruction, returning the new PC value or any execution error
    /// 
    /// # Arguments
    /// 
    /// * `v_unit` - The associated vector unit, which will execute vector instructions if they are found.
    /// * `inst_bits` - The raw instruction bits
    /// * `opcode` - The major opcode of the decoded instruction
    /// * `inst` - The fields of the decoded instruction
    fn process_inst(&mut self, mods: &mut ProcessorModules, inst_bits: u32, opcode: decode::Opcode, inst: InstructionBits) -> Result<u32> {
        let mut next_pc = self.pc + 4;
        
        if let Some(zicsr) = mods.zicsr.as_mut() {
            if zicsr.will_handle(opcode, inst) {
                let requested_pc = zicsr.execute(opcode, inst, inst_bits, self.zicsr_conn(&mut mods.rvv))?;
                if let Some(requested_pc) = requested_pc {
                    next_pc = requested_pc;
                }
                return Ok(next_pc);
            }
        }
        if let Some(rvv) = mods.rvv.as_mut() {
            if rvv.will_handle(opcode, inst) {
                let requested_pc = rvv.execute(opcode, inst, inst_bits, self.vector_conn())?;
                if let Some(requested_pc) = requested_pc {
                    next_pc = requested_pc;
                }
                return Ok(next_pc);
            }
        }

        use decode::Opcode::*;
        match (opcode, inst) {
            (Load, InstructionBits::IType{rd, funct3, rs1, imm}) => {
                let addr = self.sreg.read(rs1)?.wrapping_add(imm);
                let new_val = match funct3 {
                    // LB, LH, LW sign-extend if necessary
                    0b000 => sign_extend32(self.memory.load_u8(addr)? as u32, 8) as u32, // LB
                    0b001 => sign_extend32(self.memory.load_u16(addr)? as u32, 16) as u32, // LH
                    0b010 => self.memory.load_u32(addr)?, // LW
                    // LBU, LHU don't sign-extend
                    0b100 => self.memory.load_u8(addr)? as u32, // LBU
                    0b101 => self.memory.load_u16(addr)? as u32, // LBU

                    _ => bail!(UnsupportedParam(format!("Load funct3 {:03b}", funct3)))
                };
                self.sreg.write(rd, new_val)?;
            }
            (Store, InstructionBits::SType{funct3, rs1, rs2, imm}) => {
                let addr = self.sreg.read(rs1)?.wrapping_add(imm);
                match funct3 {
                    0b000 => self.memory.store_u8(addr, (self.sreg.read(rs2)? & 0xFF) as u8)?,
                    0b001 => self.memory.store_u16(addr, (self.sreg.read(rs2)? & 0xFFFF) as u16)?,
                    0b010 => self.memory.store_u32(addr, (self.sreg.read(rs2)? & 0xFFFF_FFFF) as u32)?,
                    
                    _ => bail!(UnsupportedParam(format!("Store funct3 {:03b}", funct3)))
                };
            }

            (OpImm, InstructionBits::IType{rd, funct3, rs1, imm}) => {
                let input = self.sreg.read(rs1)?;
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
                self.sreg.write(rd, new_val)?;
            }

            (Op, InstructionBits::RType{rd, funct3, rs1, rs2, funct7}) => {
                const ALT: u8 = 0b0100000;
                let x = self.sreg.read(rs1)?;
                let y = self.sreg.read(rs2)?;
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
                self.sreg.write(rd, new_val)?;
            }

            (AddUpperImmPC, InstructionBits::UType{rd, imm}) => {
                let addr = imm + self.pc;
                self.sreg.write(rd, addr)?;
            }

            (LoadUpperImm, InstructionBits::UType{rd, imm}) => {
                self.sreg.write(rd, imm)?;
            }

            (JumpAndLink, InstructionBits::JType{rd, imm}) => {
                self.sreg.write(rd, self.pc + 4)?;
                next_pc = self.pc.wrapping_add(imm);
            }
            (JumpAndLinkRegister, InstructionBits::IType{rd, funct3: 0b000, rs1, imm}) => {
                next_pc = self.sreg.read(rs1)?.wrapping_add(imm);
                // Unset bottom bit
                next_pc = next_pc & (!1);

                self.sreg.write(rd, self.pc + 4)?;
            }

            (Branch, InstructionBits::BType{funct3, rs1, rs2, imm}) => {
                let src1 = self.sreg.read(rs1)?;
                let src2 = self.sreg.read(rs2)?;

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

            _ => bail!(MiscDecodeException("Unexpected opcode/InstructionBits pair".to_string()))
        }

        Ok(next_pc)
    }

    /// Run a fetch-decode-execute step on the processor, executing a single instruction
    /// 
    /// # Arguments
    /// 
    /// * `v_unit` - The associated vector unit, which will execute vector instructions if they are found.
    pub fn exec_step(&mut self, mods: &mut ProcessorModules) -> Result<()> {
        self.running = true;

        self.sreg.start_tracking()?;

        let next_pc_res: Result<u32> = {
            // Fetch
            let inst_bits = self.memory.load_u32(self.pc).context("Couldn't load next instruction")?;

            // Decode
            let (opcode, inst) = decode(inst_bits)
                .with_context(|| format!("Failed to decode instruction {:08x}", inst_bits))?;

            // Execute
            let next_pc = self.process_inst(mods, inst_bits, opcode, inst)
                .with_context(|| format!("Failed to execute decoded instruction {:?} {:?}", opcode, inst))?;

            if next_pc % 4 != 0 {
                Err(MemoryException::JumpMisaligned{addr: next_pc as usize, expected: 4})?
            } else {
                Ok(next_pc)
            }
        };

        // TODO use this for something
        let _register_file_actions = self.sreg.end_tracking()?;

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

        // Increment PC
        self.pc = next_pc;

        Ok(())
    }

    /// Dump processor and vector unit state to standard output.
    pub fn dump(&self, mods: &mut ProcessorModules) {
        println!("running: {:?}\npc: 0x{:08x}", self.running, self.pc);
        self.sreg.dump();
        if let Some(rvv) = mods.rvv.as_mut() {
            rvv.dump();
        }
    }
}

struct ProcessorCSRs {}
impl CSRProvider for ProcessorCSRs {
    fn has_csr(&self, _csr: u32) -> bool {
        false
    }
    fn csr_atomic_read_write(&mut self, _csr: u32, _need_read: bool, _write_val: u32) -> Result<Option<u32>> { todo!() }
    fn csr_atomic_read_set(&mut self, _csr: u32, _set_bits: Option<u32>) -> Result<u32> { todo!() }
    fn csr_atomic_read_clear(&mut self, _csr: u32, _clear_bits: Option<u32>) -> Result<u32> { todo!() }
}
