use crate::processor::uXLEN;
use crate::processor::RegisterFile;
use crate::processor::isa_mods::*;

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
    fn csr_atomic_read_write(&mut self, csr: u32, need_read: bool, write_val: u32) -> ProcessorResult<Option<u32>>;

    /// Atomically read and set specific bits in a CSR
    /// 
    /// Will only be called on `csr` if `has_csr(csr) == true`
    /// 
    /// If `set_bits == None', no write will be performed (thus no side-effects of the write will happen).
    /// The CSR will always be read, and those side-effects will always be applied.
    fn csr_atomic_read_set(&mut self, csr: u32, set_bits: Option<u32>) -> ProcessorResult<u32>;

    /// Atomically read and clear specific bits in a CSR
    /// 
    /// Will only be called on `csr` if `has_csr(csr) == true`
    /// 
    /// If `clear_bits == None', no write will be performed (thus no side-effects of the write will happen).
    /// The CSR will always be read, and those side-effects will always be applied.
    fn csr_atomic_read_clear(&mut self, csr: u32, clear_bits: Option<u32>) -> ProcessorResult<u32>;
}

/// References to all CSR providers
pub struct ZicsrConn<'a> {
    pub sreg: &'a mut dyn RegisterFile<u32>,
    // pub csr_providers: Vec<&'a mut dyn CSRProvider>
    pub processor_csr: &'a mut dyn CSRProvider,
    pub vector_csr: &'a mut dyn CSRProvider
}
impl<'a> IsaModConn for ZicsrConn<'a> {}

use crate::processor::isa_mods::Opcode::System;

pub struct Zicsr {}
impl IsaMod<ZicsrConn<'_>> for Zicsr {
    type Pc = u32;

    fn will_handle(&self, opcode: Opcode, _inst: InstructionBits) -> bool {
        opcode == System
    }

    fn execute(&mut self, opcode: Opcode, inst: InstructionBits, _inst_bits: u32, conn: ZicsrConn) -> ProcessorResult<Option<Self::Pc>> {
        if let (System, InstructionBits::IType{rd, funct3, rs1, imm}) = (opcode, inst) {
            if funct3 == 0b000 {
                bail!("Non-CSR System instructions not supported")
            } else {
                let csr = imm;

                let is_imm_instruction = (funct3 & 0b100) != 0;
                // rs1 can be an immediate value if the top bit of funct3 is set
                // Take the "rs1 value" as either the immediate or the register[rs1]
                let rs1_val = if is_imm_instruction {
                    rs1 as uXLEN
                } else {
                    conn.sreg.read(rs1)?
                };

                match funct3 {
                    0b001 | 0b101 => {
                        // Atomic Read-Write CSR
                        let need_read = rd == 0;
                        let write_val = rs1_val;

                        // Perform the read/write
                        let old_csr_val = {
                            if conn.processor_csr.has_csr(csr) {
                                conn.processor_csr.csr_atomic_read_write(csr, need_read, write_val)?
                            } else if conn.vector_csr.has_csr(csr) {
                                conn.vector_csr.csr_atomic_read_write(csr, need_read, write_val)?
                            } else {
                                bail!("No provider found for Read/Write of CSR 0x{:04x}", csr)
                            }
                        };

                        if need_read {
                            conn.sreg.write(rd, old_csr_val.unwrap())?;
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
                            if conn.processor_csr.has_csr(csr) {
                                conn.processor_csr.csr_atomic_read_set(csr, write_val)?
                            } else if conn.vector_csr.has_csr(csr) {
                                conn.vector_csr.csr_atomic_read_set(csr, write_val)?
                            } else {
                                bail!("No provider found for Read/Set of CSR 0x{:04x}", csr)
                            }
                        };

                        conn.sreg.write(rd, old_csr_val)?;
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
                            if conn.processor_csr.has_csr(csr) {
                                conn.processor_csr.csr_atomic_read_clear(csr, write_val)?
                            } else if conn.vector_csr.has_csr(csr) {
                                conn.vector_csr.csr_atomic_read_clear(csr, write_val)?
                            } else {
                                bail!("No provider found for Read/Clear of CSR 0x{:04x}", csr)
                            }
                        };

                        conn.sreg.write(rd, old_csr_val)?;
                    }

                    0b000 | _ => unreachable!("impossible funct3")
                }
            }
        }

        Ok(None)
    }
}