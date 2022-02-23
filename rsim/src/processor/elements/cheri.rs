use std::collections::HashSet;
use either::Either;
use super::*;
use rust_cheri_compressed_cap::{CompressedCapability, Cc64, Cc64Cap};

type TaggedCap = Either<u64, Cc64Cap>;

struct CheriRV32RegisterFile {
    regs: [TaggedCap; 31],
    tracking: Option<Vec<RegisterAction<TaggedCap>>>
}
impl CheriRV32RegisterFile {
    pub fn reset(&mut self) {
        self.regs = [Either::Left(0); 31];
        self.tracking = None;
    }
}
impl RegisterFile<TaggedCap> for CheriRV32RegisterFile {
    fn read(&mut self, idx: u8) -> Result<TaggedCap, RegisterFileError> {
        let val = match idx {
            0    => Ok(Either::Left(0)),
            1..=31 => Ok(self.regs[(idx - 1) as usize]),
            _ => Err(RegisterFileError::InvalidIndex(idx))
        }?;

        if self.tracking.is_some() {
            self.tracking.as_mut().unwrap().push(RegisterAction::Read{idx, val})
        }

        Ok(val)
    }
    fn write(&mut self, idx: u8, val: TaggedCap) -> Result<(), RegisterFileError> {
        match idx {
            0    => Ok(()),
            1..=31 => {
                self.regs[(idx - 1) as usize] = val;
                Ok(())
            },
            _ => Err(RegisterFileError::InvalidIndex(idx))
        }?;

        if self.tracking.is_some() {
            self.tracking.as_mut().unwrap().push(RegisterAction::Write{idx, val})
        }

        Ok(())
    }
}
/// Interface used by normal RV32 instructions
impl RegisterFile<u32> for CheriRV32RegisterFile {
    fn read(&mut self, idx: u8) -> Result<u32, RegisterFileError> {
        let val = match idx {
            0    => Ok(0),
            1..=31 => match self.regs[(idx - 1) as usize] {
                Either::Left(val)  => Ok(val as u32),
                Either::Right(cap) => Ok(cap.address())
            },
            _ => Err(RegisterFileError::InvalidIndex(idx))
        }?;

        if self.tracking.is_some() {
            // Track reads-as-32bit events as plain reads, even if we read them from an underlying capability
            // This better reflects the program state
            self.tracking.as_mut().unwrap().push(RegisterAction::Read{idx, val: Either::Left(val as u64)})
        }

        Ok(val as u32)
    }
    fn write(&mut self, idx: u8, val: u32) -> Result<(), RegisterFileError> {
        match idx {
            0    => Ok(()),
            1..=31 => {
                todo!("Right now we're setting the tag bit to false, and clearing the top 32 bits. Is that right?");
                self.regs[(idx - 1) as usize] = Either::Left(val as u64);
                Ok(())
            },
            _ => Err(RegisterFileError::InvalidIndex(idx))
        }?;

        if self.tracking.is_some() {
            self.tracking.as_mut().unwrap().push(RegisterAction::Write{idx, val: Either::Left(val as u64)})
        }

        Ok(())
    }
}
impl RegisterTracking<TaggedCap> for CheriRV32RegisterFile {
    fn start_tracking(&mut self) -> Result<(), RegisterFileError> {
        if self.tracking.is_some() {
            Err(RegisterFileError::AlreadyTracking)
        } else {
            self.tracking = Some(vec![]);
            Ok(())
        }
    }
    fn end_tracking(&mut self) -> Result<Vec<RegisterAction<TaggedCap>>, RegisterFileError> {
        if let Some(tracking) = self.tracking.take() {
            Ok(tracking)
        } else {
            Err(RegisterFileError::NotTracking)
        }
    }
}
impl Default for CheriRV32RegisterFile {
    fn default() -> Self {
        CheriRV32RegisterFile {
            regs: [Either::Left(0); 31],
            tracking: None,
        }
    }
}

/// Wrapper for AggregateMemory that keeps tags, supports Memory<TaggedCap> for reading/writing capabilities.
/// All other Memory variants clear associated tag bits on write.
/// 
/// If base_cap is set, the memory is in Integer mode - all accesses will be checked against base_cap
/// Otherwise, memory is in Capability mode - all accesses are assumed to have been checked before.
struct CheriAggregateMemory {
    base_mem: AggregateMemory,
    base_cap: Option<Cc64Cap>,
    // Store tags in a hash-set
    // Less complicated, likely less memory intensive than storing
    // a bool for each 64-bits in the valid address range
    tag_mem: HashSet<u32>
}
impl CheriAggregateMemory {
    fn enter_integer_mode(&mut self, base_cap: Cc64Cap) {
        // TODO - check base_cap is actually valid in some way
        self.base_cap = Some(base_cap);
    }
    fn enter_capability_mode(&mut self) {
        self.base_cap = None;
    }
}
/// Reimplement basic Memory<TData> 
impl<TData> Memory<TData> for CheriAggregateMemory where AggregateMemory: Memory<TData> {
    fn range(&self) -> Range<usize> {
        self.base_mem.full_range.clone()
    }
    fn read(&mut self, addr: u32) -> Result<TData, MemoryException> {
        // If we're reading from a raw address, we may need to check against a base capability
        if let Some(base_cap) = self.base_cap {
            // If the addr is below the base of the capability range,
            // or the topmost byte of TData is above the top of the capability range,
            // return an exception
            if (base_cap.permissions() & Cc64::PERM_LOAD) == 0 {
                return Err(MemoryException::CapabilityPermission{perm: Cc64::PERM_LOAD, cap: base_cap})
            }
            if !base_cap.addr_in_bounds(addr, std::mem::size_of::<TData>() as u32) {
                return Err(MemoryException::AddressOobDefaultCapability{ addr: addr as usize, cap: base_cap });
            }
        }

        self.base_mem.read(addr)
    }
    fn write(&mut self, addr: u32, val: TData) -> Result<(), MemoryException> {
        // If we're reading from a raw address, we may need to check against a base capability
        if let Some(base_cap) = self.base_cap {
            // If the addr is below the base of the capability range,
            // or the topmost byte of TData is above the top of the capability range,
            // return an exception
            if (base_cap.permissions() & Cc64::PERM_STORE) == 0 {
                return Err(MemoryException::CapabilityPermission{perm: Cc64::PERM_STORE, cap: base_cap})
            }
            if !base_cap.addr_in_bounds(addr, std::mem::size_of::<TData>() as u32) {
                return Err(MemoryException::AddressOobDefaultCapability{ addr: addr as usize, cap: base_cap });
            }
        }

        // Set the tag on the 64-byte range containing (addr) to false
        self.tag_mem.remove(&(addr / 8));
        // We don't need to overwrite any other tags - tags are aligned to 8-byte boundaries, 
        // and TData is u8,u16,u32 which are aligned to 1,2,4 respectively.
        // There's no possibility of a write 
        assert!(std::mem::size_of::<TData>() <= std::mem::size_of::<u64>());

        self.base_mem.write(addr, val)
    }
}
/// Impl a capability-aware view of memory for CHERI instructions
/// e.g. a CHERI Load instruction, which is allowed to load capabilities, would use this version.
impl Memory<TaggedCap> for CheriAggregateMemory {
    fn range(&self) -> Range<usize> {
        self.base_mem.full_range.clone()
    }
    // read/write funcs that set correct tag bits on reads/writes
    fn read(&mut self, addr: u32) -> Result<TaggedCap, MemoryException> {
        // If we're reading and writing actual capabilities, we need to be in "capability mode"
        // This assumes that all reads/writes are checked before we get there, so we shouldn't have a base_cap
        assert_eq!(self.base_cap, None);

        let addr = addr as usize;
        if addr % 8 != 0 {
            Err(MemoryException::AddressMisaligned{addr, expected: 8})
        } else if !self.base_mem.full_range.contains(&addr) || !self.base_mem.full_range.contains(&(addr + 3)) {
            Err(MemoryException::AddressUnmapped{addr})
        } else {
            let base_mem = &mut self.base_mem as &mut dyn Memory<u32>;
            let addr = addr as u32;
            // Must be aligned and in-bounds
            let tag = self.tag_mem.contains(&(addr / 8));
            if tag {
                todo!("what order do we store capabilities in");
                let cap_pebst = base_mem.read(addr + 4)?;
                let cap_addr = base_mem.read(addr)?;
                Ok(Either::Right(
                    Cc64::decompress_mem(cap_pebst, cap_addr, tag)
                ))
            } else {
                Ok(Either::Left(
                    ((base_mem.read(addr+4)? as u64) << 32) | 
                    (base_mem.read(addr+0)? as u64)
                ))
            }
        }
    }
    fn write(&mut self, addr: u32, val: TaggedCap) -> Result<(), MemoryException> {
        // If we're reading and writing actual capabilities, we need to be in "capability mode"
        // This assumes that all reads/writes are checked before we get there, so we shouldn't have a base_cap
        assert_eq!(self.base_cap, None);

        let addr = addr as usize;
        if addr % 8 != 0 {
            Err(MemoryException::AddressMisaligned{addr, expected: 8})
        } else if !self.base_mem.full_range.contains(&addr) || !self.base_mem.full_range.contains(&(addr + 7)) {
            Err(MemoryException::AddressUnmapped{addr})
        } else {
            // TODO - this shouldn't have to be a dyn object, right?
            let base_mem = &mut self.base_mem as &mut dyn Memory<u32>;
            let addr = addr as u32;
            match val {
                Either::Left(val) => {
                    base_mem.write(addr + 4, (val >> 32) as u32)?;
                    base_mem.write(addr, (val) as u32)?;
                    self.tag_mem.remove(&(addr / 8));
                }
                Either::Right(cap) => {
                    let cap_pebst = Cc64::compress_mem(&cap);
                    todo!("what order do we store capabilities in");
                    base_mem.write(addr + 4,    cap_pebst)?;
                    base_mem.write(addr,        cap.address())?;
                    self.tag_mem.insert(addr / 8);
                }
            }
            Ok(())
        }
    }
}
impl From<AggregateMemory> for CheriAggregateMemory {
    fn from(base_mem: AggregateMemory) -> Self {
        CheriAggregateMemory {
            base_mem,
            base_cap: None,
            tag_mem: HashSet::default()
        }
    }
}