use std::ops::Range;
use std::convert::{TryInto};
use anyhow::Result;

use super::types::*;
use super::decode::MemOpDir;
use crate::processor::elements::cheri::{CheriRV64RegisterFile,CheriAggregateMemory,Cc128,CompressedCapability};
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
    /// Check if the provenance `p` allows accesses to the specified address range in the specified direction (load or store).
    /// Note: This addr_range is at the 1-byte level - if you want to access a range of u64s,
    /// make sure the range includes the end of the last u64
    fn check_addr_range_against_provenance(&mut self, addr_range: Range<u64>, prov: Provenance, dir: MemOpDir) -> Result<()>;
    /// As for check_addr_range_against_provenance, but for a single element of given width
    fn check_elem_bounds_against_provenance(&mut self, eew: Sew, addr_provenance: (u64, Provenance), dir: MemOpDir) -> Result<()>;

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
    fn check_addr_range_against_provenance(&mut self, _addr_range: Range<u64>, _prov: Provenance, _dir: MemOpDir) -> Result<()> {
        // Nothing to check
        Ok(())
    }
    fn check_elem_bounds_against_provenance(&mut self, _eew: Sew, _addr_provenance: (u64, Provenance), _dir: MemOpDir) -> Result<()> {
        // Nothing to check
        Ok(())
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
    fn check_addr_range_against_provenance(&mut self, addr_range: Range<u64>, prov: Provenance, dir: MemOpDir) -> Result<()> {
        let cap = self.sreg.read_maybe_cap(prov.reg)?.to_cap();
        let expected_perms = match dir {
            MemOpDir::Load => Cc128::PERM_LOAD,
            MemOpDir::Store => Cc128::PERM_STORE,
        };
        self.memory.check_bounds_against_capability(addr_range, cap, expected_perms)
    }
    fn check_elem_bounds_against_provenance(&mut self, eew: Sew, addr_provenance: (u64, Provenance), dir: MemOpDir) -> Result<()> {
        let (addr, prov) = addr_provenance;
        let cap = self.sreg.read_maybe_cap(prov.reg)?.to_cap();
        let expected_perms = match dir {
            MemOpDir::Load => Cc128::PERM_LOAD,
            MemOpDir::Store => Cc128::PERM_STORE,
        };
        match eew {
            Sew::e8 => {
                self.memory.check_obj_bounds_against_capability::<u8>(addr, cap, expected_perms)
            }
            Sew::e16 => {
                self.memory.check_obj_bounds_against_capability::<u16>(addr, cap, expected_perms)
            }
            Sew::e32 => {
                self.memory.check_obj_bounds_against_capability::<u32>(addr, cap, expected_perms)
            }
            Sew::e64 => { bail!("check_elem_bounds_against_provenance {:?} unsupported", eew) }
        }
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