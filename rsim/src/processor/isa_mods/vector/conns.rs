use std::convert::{TryInto};
use anyhow::Result;

use super::types::*;
use crate::processor::elements::cheri::{CheriRV64RegisterFile,CheriAggregateMemory};
use crate::processor::elements::{registers::RegisterFile,memory::Memory32};

/// References to all scalar resources touched by the vector unit.
pub struct Rv32vConn<'a> {
    pub sreg: &'a mut dyn RegisterFile<u32>,
    pub memory: &'a mut dyn Memory32,
}

pub struct Rv64vCheriConn<'a> {
    pub sreg: &'a mut CheriRV64RegisterFile,
    pub memory: &'a mut CheriAggregateMemory,
}

/// Struct indicating the providence of a pointer.
/// Here, for CHERI purposes, we assume the provenance is stored in a register.
#[derive(Debug,Copy,Clone)]
pub struct Provenance {
    reg: u8
}

/// Common trait for interfaces to memory
pub trait VecMemInterface<uXLEN> where uXLEN: PossibleXlen {
    /// Read a value XLEN from a register, not an address
    fn sreg_read_xlen(&mut self, reg: u8) -> Result<uXLEN>;
    /// Write a value XLEN to a register, not an address
    fn sreg_write_xlen(&mut self, reg: u8, val: uXLEN) -> Result<()>;
    /// Get a raw address value + the provenance of that address from a register.
    /// The address can be changed, and then [load_from_memory] and [store_to_memory] 
    /// can reuse the providence with the modified address
    fn get_addr_provenance(&mut self, reg: u8) -> Result<(u64, Provenance)>;
    /// Use an address, provenance pair to read a vector element from memory
    fn load_from_memory(&mut self, eew: Sew, addr_provenance: (u64, Provenance)) -> Result<uELEN>;
    /// Use an address, provenance pair to write a vector element to memory
    fn store_to_memory(&mut self, eew: Sew, val: uELEN, addr_provenance: (u64, Provenance)) -> Result<()>;
}
impl<'a> VecMemInterface<u32> for Rv32vConn<'a> {
    fn sreg_read_xlen(&mut self, reg: u8) -> Result<u32> {
        Ok(self.sreg.read(reg)?)
    }
    fn sreg_write_xlen(&mut self, reg: u8, val: u32) -> Result<()> {
        Ok(self.sreg.write(reg, val)?)
    }
    fn get_addr_provenance(&mut self, reg: u8) -> Result<(u64, Provenance)> {
        Ok((self.sreg.read(reg)? as u64, Provenance{ reg }))
    }
    fn load_from_memory(&mut self, eew: Sew, addr_provenance: (u64, Provenance)) -> Result<uELEN> {
        let (addr, _) = addr_provenance;
        let val = match eew {
            Sew::e8 => {
                self.memory.load_u8(addr)? as u32
            }
            Sew::e16 => {
                self.memory.load_u16(addr)? as u32
            }
            Sew::e32 => {
                self.memory.load_u32(addr)? as u32
            }
            Sew::e64 => { bail!("load_from_memory {:?} unsupported", eew) }
        };
        Ok(val)
    }
    fn store_to_memory(&mut self, eew: Sew, val: uELEN, addr_provenance: (u64, Provenance)) -> Result<()> {
        let (addr, _) = addr_provenance;
        match eew {
            Sew::e8 => {
                self.memory.store_u8(addr, val.try_into()?)?
            }
            Sew::e16 => {
                self.memory.store_u16(addr, val.try_into()?)?
            }
            Sew::e32 => {
                self.memory.store_u32(addr, val.try_into()?)?
            }
            Sew::e64 => { bail!("store_to_memory {:?} unsupported", eew) }
        }
        Ok(())
    }
}
impl<'a> VecMemInterface<u64> for Rv64vCheriConn<'a> {
    fn sreg_read_xlen(&mut self, reg: u8) -> Result<u64> {
        Ok(self.sreg.read(reg)?)
    }
    fn sreg_write_xlen(&mut self, reg: u8, val: u64) -> Result<()> {
        Ok(self.sreg.write(reg, val)?)
    }
    fn get_addr_provenance(&mut self, reg: u8) -> Result<(u64, Provenance)> {
        Ok((self.sreg.read(reg)?, Provenance{ reg }))
    }
    fn load_from_memory(&mut self, eew: Sew, addr_provenance: (u64, Provenance)) -> Result<uELEN> {
        let (addr, prov) = addr_provenance;
        let mut cap = self.sreg.read_maybe_cap(prov.reg)?.to_cap();
        cap.set_address_unchecked(addr);
        let val = match eew {
            Sew::e8 => {
                self.memory.load_u8(cap)? as u32
            }
            Sew::e16 => {
                self.memory.load_u16(cap)? as u32
            }
            Sew::e32 => {
                self.memory.load_u32(cap)? as u32
            }
            Sew::e64 => { bail!("load_from_memory {:?} unsupported", eew) }
        };
        Ok(val)
    }
    fn store_to_memory(&mut self, eew: Sew, val: uELEN, addr_provenance: (u64, Provenance)) -> Result<()> {
        let (addr, prov) = addr_provenance;
        let mut cap = self.sreg.read_maybe_cap(prov.reg)?.to_cap();
        cap.set_address_unchecked(addr);
        match eew {
            Sew::e8 => {
                self.memory.store_u8(cap, val.try_into()?)?
            }
            Sew::e16 => {
                self.memory.store_u16(cap, val.try_into()?)?
            }
            Sew::e32 => {
                self.memory.store_u32(cap, val.try_into()?)?
            }
            Sew::e64 => { bail!("store_to_memory {:?} unsupported", eew) }
        }
        Ok(())
    }
}