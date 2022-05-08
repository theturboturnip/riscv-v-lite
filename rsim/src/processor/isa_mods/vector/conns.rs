use crate::processor::elements::cheri::IntegerModeCheriRV64RegisterFile;
use crate::memory::AggregateMemory;
use crate::processor::elements::registers::RvRegisterFile;
use crate::processor::elements::cheri::IntegerModeCheriAggregateMemory;
use crate::Cc128Cap;
use crate::processor::elements::cheri::CheriMemory;
use crate::processor::elements::cheri::SafeTaggedCap;
use crate::processor::isa_mods::PossibleXlen;
use std::ops::Range;
use std::convert::{TryInto};
use anyhow::Result;

use super::types::*;
use super::decode::MemOpDir;
use crate::processor::elements::cheri::{CheriRV64RegisterFile,CheriAggregateMemory,Cc128,CompressedCapability,check_bounds_against_capability,check_obj_bounds_against_capability};
use crate::processor::elements::{registers::RegisterFile,memory::Memory};

/// The vector unit needs to support connections to
/// - RegisterFile<u32>, Memory
/// - RegisterFile<u64>, Memory
/// - RegisterFile<u64>, CHERI memory (integer-mode CHERI)
/// - RegisterFile<SafeTaggedCap>, CHERI memory (capability-mode CHERI)

/// Struct indicating the providence of a pointer.
pub type Provenance = Option<Cc128Cap>;

pub trait VecRegInterface<uXLEN: PossibleXlen> {
    /// Read a value XLEN from a register, not an address
    fn sreg_read_xlen(&mut self, reg: u8) -> Result<uXLEN>;
    /// Write a value XLEN to a register, not an address
    fn sreg_write_xlen(&mut self, reg: u8, val: uXLEN) -> Result<()>;
    /// Get a raw address value + the provenance of that address from a register.
    /// The address can be changed, and then [load_from_memory] and [store_to_memory] 
    /// can reuse the providence with the modified address
    fn get_addr_provenance(&mut self, reg: u8) -> Result<(u64, Provenance)>;

    // TODO move the below into VecMemInterface

    /// Check if the provenance `p` allows accesses to the specified address range in the specified direction (load or store).
    /// Note: This addr_range is at the 1-byte level - if you want to access a range of u64s,
    /// make sure the range includes the end of the last u64
    fn check_addr_range_against_provenance(&mut self, addr_range: Range<u64>, prov: Provenance, dir: MemOpDir) -> Result<()>;
    /// As for check_addr_range_against_provenance, but for a single element of given width
    fn check_elem_bounds_against_provenance(&mut self, eew: Sew, addr_provenance: (u64, Provenance), dir: MemOpDir) -> Result<()>;    
}

/// Common trait for interfaces to memory
pub trait VecMemInterface<uXLEN: PossibleXlen, TElem> {
    /// Use an address, provenance pair to read a vector element from memory
    fn load_from_memory(&mut self, eew: Sew, addr_provenance: (u64, Provenance)) -> Result<TElem>;
    /// Use an address, provenance pair to write a vector element to memory
    fn store_to_memory(&mut self, eew: Sew, val: TElem, addr_provenance: (u64, Provenance)) -> Result<()>;
}

impl<'a, uXLEN: PossibleXlen> VecRegInterface<uXLEN> for RvRegisterFile<uXLEN> {
    fn sreg_read_xlen(&mut self, reg: u8) -> Result<uXLEN> {
        Ok(self.read(reg)?)
    }
    fn sreg_write_xlen(&mut self, reg: u8, val: uXLEN) -> Result<()> {
        Ok(self.write(reg, val)?)
    }
    fn get_addr_provenance(&mut self, reg: u8) -> Result<(u64, Provenance)> {
        Ok((self.read(reg)?.into(), None))
    }
    fn check_addr_range_against_provenance(&mut self, _addr_range: Range<u64>, _prov: Provenance, _dir: MemOpDir) -> Result<()> {
        // Nothing to check
        Ok(())
    }
    fn check_elem_bounds_against_provenance(&mut self, _eew: Sew, _addr_provenance: (u64, Provenance), _dir: MemOpDir) -> Result<()> {
        // Nothing to check
        Ok(())
    }
}

impl<'a, uXLEN: PossibleXlen> VecMemInterface<uXLEN, u128> for AggregateMemory {
    fn load_from_memory(&mut self, eew: Sew, addr_provenance: (u64, Provenance)) -> Result<u128> {
        let (addr, _) = addr_provenance;
        let val = match eew {
            Sew::e8 => {
                self.load_u8(addr)? as u64
            }
            Sew::e16 => {
                self.load_u16(addr)? as u64
            }
            Sew::e32 => {
                self.load_u32(addr)? as u64
            }
            Sew::e64 => {
                self.load_u64(addr)? as u64
            }
            Sew::e128 => {
                bail!("Unsupported load width: 128");
            }
        };
        Ok(val as u128)
    }
    fn store_to_memory(&mut self, eew: Sew, val: u128, addr_provenance: (u64, Provenance)) -> Result<()> {
        let (addr, _) = addr_provenance;
        match eew {
            Sew::e8 => {
                self.store_u8(addr, val.try_into()?)?
            }
            Sew::e16 => {
                self.store_u16(addr, val.try_into()?)?
            }
            Sew::e32 => {
                self.store_u32(addr, val.try_into()?)?
            }
            Sew::e64 => {
                self.store_u64(addr, val.try_into()?)?
            }
            Sew::e128 => {
                bail!("Unsupported store width: 128");
            }
        }
        Ok(())
    }
}
impl<'a> VecRegInterface<u64> for CheriRV64RegisterFile {
    fn sreg_read_xlen(&mut self, reg: u8) -> Result<u64> {
        Ok(self.read(reg)?)
    }
    fn sreg_write_xlen(&mut self, reg: u8, val: u64) -> Result<()> {
        Ok(self.write(reg, val)?)
    }
    fn get_addr_provenance(&mut self, reg: u8) -> Result<(u64, Provenance)> {
        let cap = self.read_maybe_cap(reg)?.to_cap();
        Ok((cap.address(), Some(cap)))
    }
    fn check_addr_range_against_provenance(&mut self, addr_range: Range<u64>, prov: Provenance, dir: MemOpDir) -> Result<()> {
        let cap = prov.unwrap();
        let expected_perms = match dir {
            MemOpDir::Load => Cc128::PERM_LOAD,
            MemOpDir::Store => Cc128::PERM_STORE,
        };
        check_bounds_against_capability(addr_range, cap, expected_perms)
    }
    fn check_elem_bounds_against_provenance(&mut self, eew: Sew, addr_provenance: (u64, Provenance), dir: MemOpDir) -> Result<()> {
        let (addr, prov) = addr_provenance;
        let cap = prov.unwrap();
        let expected_perms = match dir {
            MemOpDir::Load => Cc128::PERM_LOAD,
            MemOpDir::Store => Cc128::PERM_STORE,
        };
        match eew {
            Sew::e8 => {
                check_obj_bounds_against_capability::<u8>(addr, cap, expected_perms)
            }
            Sew::e16 => {
                check_obj_bounds_against_capability::<u16>(addr, cap, expected_perms)
            }
            Sew::e32 => {
                check_obj_bounds_against_capability::<u32>(addr, cap, expected_perms)
            }
            Sew::e64 => { bail!("check_elem_bounds_against_provenance {:?} unsupported", eew) }
            Sew::e128 => {
                bail!("Unsupported check-bounds width: 128");
            }
        }
    }
}
impl<'a> VecRegInterface<u64> for IntegerModeCheriRV64RegisterFile<'a> {
    fn sreg_read_xlen(&mut self, reg: u8) -> Result<u64> {
        Ok(self.read(reg)?)
    }
    fn sreg_write_xlen(&mut self, reg: u8, val: u64) -> Result<()> {
        Ok(self.write(reg, val)?)
    }
    fn get_addr_provenance(&mut self, reg: u8) -> Result<(u64, Provenance)> {
        let cap = self.read_ddc_offset_cap(reg)?;
        Ok((cap.address(), Some(cap)))
    }
    fn check_addr_range_against_provenance(&mut self, addr_range: Range<u64>, prov: Provenance, dir: MemOpDir) -> Result<()> {
        let cap = prov.unwrap();
        let expected_perms = match dir {
            MemOpDir::Load => Cc128::PERM_LOAD,
            MemOpDir::Store => Cc128::PERM_STORE,
        };
        check_bounds_against_capability(addr_range, cap, expected_perms)
    }
    fn check_elem_bounds_against_provenance(&mut self, eew: Sew, addr_provenance: (u64, Provenance), dir: MemOpDir) -> Result<()> {
        let (addr, prov) = addr_provenance;
        let cap = prov.unwrap();
        let expected_perms = match dir {
            MemOpDir::Load => Cc128::PERM_LOAD,
            MemOpDir::Store => Cc128::PERM_STORE,
        };
        match eew {
            Sew::e8 => {
                check_obj_bounds_against_capability::<u8>(addr, cap, expected_perms)
            }
            Sew::e16 => {
                check_obj_bounds_against_capability::<u16>(addr, cap, expected_perms)
            }
            Sew::e32 => {
                check_obj_bounds_against_capability::<u32>(addr, cap, expected_perms)
            }
            Sew::e64 => { bail!("check_elem_bounds_against_provenance {:?} unsupported", eew) }
            Sew::e128 => {
                bail!("Unsupported check-bounds width: 128");
            }
        }
    }
}
impl<'a> VecMemInterface<u64, SafeTaggedCap> for CheriAggregateMemory {
    fn load_from_memory(&mut self, eew: Sew, addr_provenance: (u64, Provenance)) -> Result<SafeTaggedCap> {
        let (addr, prov) = addr_provenance;
        let mut cap = prov.unwrap();
        cap.set_address_unchecked(addr);
        let val = match eew {
            Sew::e8 => {
                SafeTaggedCap::from_integer(self.load_u8(cap)? as u128)
            }
            Sew::e16 => {
                SafeTaggedCap::from_integer(self.load_u16(cap)? as u128)
            }
            Sew::e32 => {
                SafeTaggedCap::from_integer(self.load_u32(cap)? as u128)
            }
            Sew::e64 => {
                SafeTaggedCap::from_integer(self.load_u64(cap)? as u128)
            }
            Sew::e128 => {
                self.load_maybe_cap(cap)?
            }
        };
        Ok(val)
    }
    fn store_to_memory(&mut self, eew: Sew, val: SafeTaggedCap, addr_provenance: (u64, Provenance)) -> Result<()> {
        let (addr, prov) = addr_provenance;
        let mut cap = prov.unwrap();
        cap.set_address_unchecked(addr);
        match eew {
            Sew::e8 => {
                self.store_u8(cap, val.to_integer().try_into()?)?
            }
            Sew::e16 => {
                self.store_u16(cap, val.to_integer().try_into()?)?
            }
            Sew::e32 => {
                self.store_u32(cap, val.to_integer().try_into()?)?
            }
            Sew::e64 => {
                self.store_u64(cap, val.to_integer().try_into()?)?
            }
            Sew::e128 => {
                self.store_maybe_cap(cap, val)?;
            }
        }
        Ok(())
    }
}
impl<'a> VecMemInterface<u64, SafeTaggedCap> for IntegerModeCheriAggregateMemory<'a> {
    fn load_from_memory(&mut self, eew: Sew, addr_provenance: (u64, Provenance)) -> Result<SafeTaggedCap> {
        let (addr, _) = addr_provenance;
        let val = match eew {
            Sew::e8 => {
                SafeTaggedCap::from_integer(self.load_u8(addr)? as u128)
            }
            Sew::e16 => {
                SafeTaggedCap::from_integer(self.load_u16(addr)? as u128)
            }
            Sew::e32 => {
                SafeTaggedCap::from_integer(self.load_u32(addr)? as u128)
            }
            Sew::e64 => {
                SafeTaggedCap::from_integer(self.load_u64(addr)? as u128)
            }
            Sew::e128 => {
                self.load_maybe_cap(addr)?
            }
        };
        Ok(val)
    }
    fn store_to_memory(&mut self, eew: Sew, val: SafeTaggedCap, addr_provenance: (u64, Provenance)) -> Result<()> {
        let (addr, _) = addr_provenance;
        match eew {
            Sew::e8 => {
                self.store_u8(addr, val.to_integer().try_into()?)?
            }
            Sew::e16 => {
                self.store_u16(addr, val.to_integer().try_into()?)?
            }
            Sew::e32 => {
                self.store_u32(addr, val.to_integer().try_into()?)?
            }
            Sew::e64 => {
                self.store_u64(addr, val.to_integer().try_into()?)?
            }
            Sew::e128 => {
                self.store_maybe_cap(addr, val)?;
            }
        }
        Ok(())
    }
}