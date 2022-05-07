use crate::processor::elements::cheri::CheriMemory;
use crate::processor::elements::cheri::IntegerModeCheriAggregateMemory;
use crate::processor::elements::cheri::SafeTaggedCap;
use crate::Cc128Cap;
use crate::processor::elements::cheri::check_obj_bounds_against_capability;
use crate::processor::elements::cheri::check_bounds_against_capability;
use std::marker::PhantomData;
use crate::processor::isa_mods::PossibleXlen;
use std::ops::Range;
use std::convert::{TryInto};
use anyhow::Result;

use super::types::*;
use super::decode::MemOpDir;
use crate::processor::elements::cheri::{CheriRV64RegisterFile,CheriAggregateMemory,Cc128,CompressedCapability};
use crate::processor::elements::{registers::RegisterFile,memory::Memory};

/// The vector unit needs to support connections to
/// - RegisterFile<u32>, Memory
/// - RegisterFile<u64>, Memory
/// - RegisterFile<u64>, CHERI memory (integer-mode CHERI)
/// - RegisterFile<SafeTaggedCap>, CHERI memory (capability-mode CHERI)
/// 
/// Each version implements VecRegInterface and VecMemInterface, depending on which register file and memory are used.

struct BaseRvvConn<'a, TReg, TMem, uXLEN: PossibleXlen, TElem> {
    pub sreg: TReg,
    pub memory: TMem,
    _phantom_xlen: PhantomData<uXLEN>,
    _phantom_elem: PhantomData<TElem>,
}

/// References to all scalar resources touched by the vector unit.
pub struct RvvConn<'a, uXLEN: PossibleXlen, TElem>(BaseRvvConn<'a, &'a mut dyn RegisterFile<uXLEN>, &'a mut dyn Memory, uXLEN, TElem>);
pub type Rv32vConn<'a> = RvvConn<'a, u32, u128>;
pub type Rv64vConn<'a> = RvvConn<'a, u64, u128>;

/// Capability-mode connection from CHERI to the vector unit
/// Can read out capabilities from the register file, address width = 64, can manipulate SafeTaggedCap
pub type Rv64vCheriConn<'a> = BaseRvvConn<'a,
    &'a mut CheriRV64RegisterFile,
    &'a mut CheriAggregateMemory,
    u64,
    SafeTaggedCap,
>;
/// Integer-mode connection from CHERI to the vector unit
/// Reads out plain addresses from the register file, sends them to an integer-mode aggregate memory
/// TODO: This should be able to read out capabilities?
pub type Rv64vCheriIntegerConn<'a> = BaseRvvConn<'a,
    &'a mut dyn RegisterFile<u64>,
    &'a mut IntegerModeCheriAggregateMemory<'a>,
    u64,
    u128,
>;

// Check all traits are implemented for each kind of Conn
// If VecInterface<uXLEN, TElem> is satisfied for a BaseRvvConn implementation, then the Reg and Mem interfaces are correct
pub trait VecInterface<uXLEN: PossibleXlen, TElem>: VecRegInterface<uXLEN> + VecMemInterface<uXLEN, TElem> {}
impl<'a> VecInterface<u32, u128> for Rv32vConn<'a> {}
impl<'a> VecInterface<u64, u128> for Rv64vConn<'a> {}
impl<'a> VecInterface<u64, SafeTaggedCap> for Rv64vCheriConn<'a> {}
impl<'a> VecInterface<u64, u128> for Rv64vCheriIntegerConn<'a> {}

/// Struct indicating the providence of a pointer.
#[derive(Debug,Copy,Clone)]
pub struct Provenance {
    cap: Option<Cc128Cap>
}

/// Common trait for register file interfaces
/// 
pub trait VecRegInterface<uXLEN: PossibleXlen> {
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
}

/// Common trait for interfaces to memory
pub trait VecMemInterface<uXLEN: PossibleXlen, TElem> {
        /// Use an address, provenance pair to read a vector element from memory
    fn load_from_memory(&mut self, eew: Sew, addr_provenance: (u64, Provenance)) -> Result<TElem>;
    /// Use an address, provenance pair to write a vector element to memory
    fn store_to_memory(&mut self, eew: Sew, val: TElem, addr_provenance: (u64, Provenance)) -> Result<()>;
}

/// For all BaseRvvConns that have a plain RegisterFile<u32|u64>, define a XLEN=32|64 interface
/// Does not check provenance
impl<'a, uXLEN: PossibleXlen, TElem, TMem> VecRegInterface<uXLEN> for BaseRvvConn<'a, &'a mut dyn RegisterFile<uXLEN>, TMem, uXLEN, TElem> {
    fn sreg_read_xlen(&mut self, reg: u8) -> Result<uXLEN> {
        Ok(self.sreg.read(reg)?)
    }
    fn sreg_write_xlen(&mut self, reg: u8, val: uXLEN) -> Result<()> {
        Ok(self.sreg.write(reg, val)?)
    }
    fn get_addr_provenance(&mut self, reg: u8) -> Result<(u64, Provenance)> {
        Ok((self.sreg.read(reg)?.into(), Provenance{ cap: None }))
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

/// For all BaseRvvConns that have a plain RegisterFile<u32|u64> and a plain memory, this is the memory interface
/// TODO: Try upcasting the Memory to a CheriMemory inside load/store, so that we could 
impl<'a, uXLEN: PossibleXlen> VecMemInterface<uXLEN, u128> for BaseRvvConn<'a, &'a mut dyn RegisterFile<uXLEN>, &'a mut dyn Memory, uXLEN, u128> {
    fn load_from_memory(&mut self, eew: Sew, addr_provenance: (u64, Provenance)) -> Result<u128> {
        let (addr, _) = addr_provenance;
        let val = match eew {
            Sew::e8 => {
                self.memory.load_u8(addr)? as u64
            }
            Sew::e16 => {
                self.memory.load_u16(addr)? as u64
            }
            Sew::e32 => {
                self.memory.load_u32(addr)? as u64
            }
            Sew::e64 => {
                self.memory.load_u64(addr)? as u64
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
                self.memory.store_u8(addr, val.try_into()?)?
            }
            Sew::e16 => {
                self.memory.store_u16(addr, val.try_into()?)?
            }
            Sew::e32 => {
                self.memory.store_u32(addr, val.try_into()?)?
            }
            Sew::e64 => {
                self.memory.store_u64(addr, val.try_into()?)?
            }
            Sew::e128 => {
                bail!("Unsupported store width: 128");
            }
        }
        Ok(())
    }
}

/// For all BaseRvvConns that use a 64-bit Cheri register file, define a XLEN=64 register interface
/// This checks provenance.
impl<'a, TMem> VecRegInterface<u64> for BaseRvvConn<'a, &'a mut CheriRV64RegisterFile, TMem, u64, u128> {
    fn sreg_read_xlen(&mut self, reg: u8) -> Result<u64> {
        Ok(self.sreg.read(reg)?)
    }
    fn sreg_write_xlen(&mut self, reg: u8, val: u64) -> Result<()> {
        Ok(self.sreg.write(reg, val)?)
    }
    fn get_addr_provenance(&mut self, reg: u8) -> Result<(u64, Provenance)> {
        let cap = self.sreg.read_maybe_cap(reg)?.to_cap();
        Ok((cap.address(), Provenance{ cap: Some(cap) }))
    }
    fn check_addr_range_against_provenance(&mut self, addr_range: Range<u64>, prov: Provenance, dir: MemOpDir) -> Result<()> {
        todo!("This doesn't check if we may be loading/storing capabilities, which changes the permissions");
        let expected_perms = match dir {
            MemOpDir::Load => Cc128::PERM_LOAD,
            MemOpDir::Store => Cc128::PERM_STORE,
        };
        check_bounds_against_capability(addr_range, prov.cap.unwrap(), expected_perms)
    }
    fn check_elem_bounds_against_provenance(&mut self, eew: Sew, addr_provenance: (u64, Provenance), dir: MemOpDir) -> Result<()> {
        let (addr, prov) = addr_provenance;
        let cap = prov.cap.unwrap();
        let expected_perms = match (dir, eew) {
            (MemOpDir::Load, Sew::e128) => Cc128::PERM_LOAD_CAP,
            (MemOpDir::Load, _) => Cc128::PERM_LOAD,
            (MemOpDir::Store, Sew::e128) => Cc128::PERM_STORE_CAP,
            (MemOpDir::Store, _) => Cc128::PERM_STORE,
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
            Sew::e64 => {
                check_obj_bounds_against_capability::<u64>(addr, cap, expected_perms)
            }
            Sew::e128 => {
                check_obj_bounds_against_capability::<u128>(addr, cap, expected_perms)
            }
        }
    }
}

/// For all BaseRvvConns that use a CHERI memory and a XLEN=64 CHERI register file, this is the memory interface
impl<'a> VecMemInterface<u64, SafeTaggedCap> for BaseRvvConn<'a, &'a mut CheriRV64RegisterFile, &'a CheriAggregateMemory, u64, u128> {
    fn load_from_memory(&mut self, eew: Sew, addr_provenance: (u64, Provenance)) -> Result<SafeTaggedCap> {
        let (addr, prov) = addr_provenance;
        // Provenance will definitely have a capability, because we're using a CheriRV64RegisterFile
        let mut cap = prov.cap.unwrap();
        cap.set_address_unchecked(addr);
        let val = match eew {
            Sew::e8 => {
                SafeTaggedCap::from_integer(self.memory.load_u8(cap)? as u128)
            }
            Sew::e16 => {
                SafeTaggedCap::from_integer(self.memory.load_u16(cap)? as u128)
            }
            Sew::e32 => {
                SafeTaggedCap::from_integer(self.memory.load_u32(cap)? as u128)
            }
            Sew::e64 => {
                SafeTaggedCap::from_integer(self.memory.load_u64(cap)? as u128)
            }
            Sew::e128 => {
                self.memory.load_maybe_cap(cap)?
            }
        };
        Ok(val)
    }
    fn store_to_memory(&mut self, eew: Sew, val: SafeTaggedCap, addr_provenance: (u64, Provenance)) -> Result<()> {
        let (addr, prov) = addr_provenance;
        // Provenance will definitely have a capability, because we're using a CheriRV64RegisterFile
        let mut cap = prov.cap.unwrap();
        cap.set_address_unchecked(addr);
        match eew {
            Sew::e8 => {
                self.memory.store_u8(cap, val.to_integer().try_into()?)?
            }
            Sew::e16 => {
                self.memory.store_u16(cap, val.to_integer().try_into()?)?
            }
            Sew::e32 => {
                self.memory.store_u32(cap, val.to_integer().try_into()?)?
            }
            Sew::e64 => {
                self.memory.store_u64(cap, val.to_integer().try_into()?)?
            }
            Sew::e128 => {
                self.memory.store_maybe_cap(cap, val)?;
            }
        }
        Ok(())
    }
}