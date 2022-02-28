use std::marker::PhantomData;
use crate::processor::RegisterFile;
use crate::processor::isa_mods::*;

pub trait CSRProvider<T> {
    /// Does the Provider provide access to a given CSR?
    fn has_csr(&self, csr: u32) -> bool;
    
    /// Atomic Read/Write a CSR
    /// 
    /// Will only be called on `csr` if `has_csr(csr) == true`
    /// 
    /// Reads can have side-effects, and some variants of the instruction disable that.
    /// If `need_read == false', it won't perform a read or any side-effects, and will return an Ok(None).
    /// Otherwise will return an Ok(Some()) with the previous CSR value, and execute side-effects.
    fn csr_atomic_read_write(&mut self, csr: u32, need_read: bool, write_val: T) -> ProcessorResult<Option<T>>;

    /// Atomically read and set specific bits in a CSR
    /// 
    /// Will only be called on `csr` if `has_csr(csr) == true`
    /// 
    /// If `set_bits == None', no write will be performed (thus no side-effects of the write will happen).
    /// The CSR will always be read, and those side-effects will always be applied.
    fn csr_atomic_read_set(&mut self, csr: u32, set_bits: Option<T>) -> ProcessorResult<T>;

    /// Atomically read and clear specific bits in a CSR
    /// 
    /// Will only be called on `csr` if `has_csr(csr) == true`
    /// 
    /// If `clear_bits == None', no write will be performed (thus no side-effects of the write will happen).
    /// The CSR will always be read, and those side-effects will always be applied.
    fn csr_atomic_read_clear(&mut self, csr: u32, clear_bits: Option<T>) -> ProcessorResult<T>;
}

/// References to all CSR providers
pub struct ZicsrConn<'a,T> where T: From<u8> {
    pub sreg: &'a mut dyn RegisterFile<T>,
    pub csr_providers: Vec<&'a mut dyn CSRProvider<T>>
}
impl<'a,T> ZicsrConn<'a,T> where T: From<u8> {
    /// Explanation of return type: OK, so this is really annoying.
    /// We want to return a reference to one of the mutrefs inside csr_providers.
    /// However, we can't just copy the mutref out, because that would mean two copies exist - one in csr_providers, and one in our return value.
    /// => we have to return a reference to the mutref - `&(&'a mut dyn CSRProvider)`.
    /// Because we want to modify the thing at the end of the mutref, Rust complains and wants the *outer* reference to be mutable as well.
    /// => `& mut (&'a mut dyn CSRProvider)`
    /// And, because we may not find anything, wrap the whole thing in an Option
    /// => `Option<& mut &'a mut dyn CSRProvider>`
    fn provider_of_csr(&mut self, csr: u32) -> Option<& mut &'a mut dyn CSRProvider<T>> {
        self.csr_providers.iter_mut().find(|provider| provider.has_csr(csr))
    }
}
impl<'a,T> IsaModConn for ZicsrConn<'a,T> where T: From<u8> {}

use crate::processor::isa_mods::Opcode::System;

pub struct Zicsr<T> where T: From<u8> {
    _phantom: PhantomData<T>,
}
impl<T> Default for Zicsr<T> where T: From<u8> {
    fn default() -> Self {
        Zicsr{
            _phantom: PhantomData
        }
    }
}
impl<T> IsaMod<ZicsrConn<'_,T>> for Zicsr<T> where T: From<u8> {
    type Pc = T;

    fn will_handle(&self, opcode: Opcode, _inst: InstructionBits) -> bool {
        opcode == System
    }

    fn execute(&mut self, opcode: Opcode, inst: InstructionBits, _inst_bits: u32, mut conn: ZicsrConn<T>) -> ProcessorResult<Option<Self::Pc>> {
        if let (System, InstructionBits::IType{rd, funct3, rs1, imm}) = (opcode, inst) {
            if funct3 == 0b000 {
                bail!("Non-CSR System instructions not supported")
            } else {
                let csr = imm;

                let is_imm_instruction = (funct3 & 0b100) != 0;
                // rs1 can be an immediate value if the top bit of funct3 is set
                // Take the "rs1 value" as either the immediate or the register[rs1]
                let rs1_val = if is_imm_instruction {
                    rs1.into()
                } else {
                    conn.sreg.read(rs1)?
                };

                match funct3 {
                    0b001 | 0b101 => {
                        // Atomic Read-Write CSR
                        let need_read = rd == 0;
                        let write_val = rs1_val;

                        // Perform the read/write
                        match conn.provider_of_csr(csr) {
                            None => bail!("No provider found for Read/Write of CSR 0x{:04x}", csr),
                            Some(csr_provider) => {
                                let old_csr_val = csr_provider.csr_atomic_read_write(csr, need_read, write_val)?;

                                if need_read {
                                    conn.sreg.write(rd, old_csr_val.unwrap())?;
                                }
                            }
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
                        match conn.provider_of_csr(csr) {
                            None => bail!("No provider found for Read/Set of CSR 0x{:04x}", csr),
                            Some(csr_provider) => {
                                let old_csr_val = csr_provider.csr_atomic_read_set(csr, write_val)?;
                                
                                conn.sreg.write(rd, old_csr_val)?;
                            }
                        }
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
                        match conn.provider_of_csr(csr) {
                            None => bail!("No provider found for Read/Clear of CSR 0x{:04x}", csr),
                            Some(csr_provider) => {
                                let old_csr_val = csr_provider.csr_atomic_read_clear(csr, write_val)?;
                                
                                conn.sreg.write(rd, old_csr_val)?;
                            }
                        }
                    }

                    0b000 | _ => unreachable!("impossible funct3")
                }
            }
        }

        Ok(None)
    }
}

pub type Zicsr32 = Zicsr<u32>;
pub type Zicsr64 = Zicsr<u64>;
pub type Zicsr32Conn<'a> = ZicsrConn<'a,u32>;
pub type Zicsr64Conn<'a> = ZicsrConn<'a,u64>;