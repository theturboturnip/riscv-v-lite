use std::ops::Range;
use std::convert::TryInto;
use std::collections::HashSet;

use crate::processor::exceptions::MemoryException;
use crate::processor::elements::memory::*;
use super::capability::*;

fn cap_bounds_range(cap: Cc128Cap) -> Range<u64> {
    let b = cap.bounds();
    Range { start: b.0, end: b.1.try_into().unwrap() }
}

/// Wrapper for AggregateMemory64 that keeps tags, supports MemoryOf<SafeTaggedCap> for reading/writing capabilities.
/// All other Memory variants clear associated tag bits on write.
pub struct CheriAggregateMemory {
    base_mem: AggregateMemory64,
    // Store tags in a hash-set
    // Less complicated, likely less memory intensive than storing
    // a bool for each 64-bits in the valid address range
    tag_mem: HashSet<u64>
}

impl CheriAggregateMemory {
    fn check_capability<TData>(&self, cap: Cc128Cap, expected_perms: u32) -> MemoryResult<()> {
        let size = std::mem::size_of::<TData>() as u64;
        if !cap.tag() {
            bail!(MemoryException::CapabilityInvalid{ cap })
        } else if cap.permissions() & expected_perms != expected_perms {
            bail!(MemoryException::CapabilityPermission { cap, perms: expected_perms })
        } else if !cap_bounds_range(cap).contains(&cap.address()) 
            || !cap_bounds_range(cap).contains(&(cap.address() + size - 1)) {
            bail!(MemoryException::AddressOobCapability { cap, size: size as usize, addr: cap.address() as usize })
        } else if cap.is_sealed() {
            bail!(MemoryException::CapabilitySealed{ cap })
        } else {
            Ok(())
        }
    }

    pub fn fetch_inst_u32(&mut self, cap: Cc128Cap) -> MemoryResult<u32> {
        self.check_capability::<u32>(cap, Cc128::PERM_LOAD | Cc128::PERM_EXECUTE)?;

        self.base_mem.load_u32(cap.address())
    }
    pub fn load_maybe_cap(&mut self, addr: Cc128Cap) -> MemoryResult<SafeTaggedCap> {
        <Self as MemoryOf<SafeTaggedCap, Cc128Cap>>::read(self, addr)
    }
    pub fn store_maybe_cap(&mut self, addr: Cc128Cap, val: SafeTaggedCap) -> MemoryResult<()> {
        <Self as MemoryOf<SafeTaggedCap, Cc128Cap>>::write(self, addr, val)
    }

    pub fn get_full_range_cap(&self) -> Cc128Cap {
        // Generate a capability covering the range of mapped addresses
        let full_range = self.range();
        let full_range_len = full_range.end - full_range.start;
        // Find the minimum alignment and length we need to apply to the memory range,
        // in order to represent it exactly as a capability
        let required_align_mask = Cc128::get_alignment_mask(full_range_len as u128);
        let required_len = Cc128::get_representable_length(full_range_len as u128);

        let full_range_cap_base = (full_range.start as u64) & (required_align_mask as u64);
        let full_range_cap = Cc128::make_max_perms_cap(
            full_range_cap_base, // base
            full_range_cap_base, // put the address at the base
            full_range_cap_base as u128 + required_len as u128 // top
        );

        full_range_cap
    }
}
/// Implement MemoryOf<TData> addressed by Cc128Cap, which does all necessary validity checks,
/// for every TData in {u8,u16,u32,u64}
impl<TData> MemoryOf<TData, Cc128Cap> for CheriAggregateMemory where AggregateMemory64: MemoryOf<TData, u64> {
    fn read(&mut self, cap: Cc128Cap) -> MemoryResult<TData> {
        self.check_capability::<TData>(cap, Cc128::PERM_LOAD)?;

        self.base_mem.read(cap.address())
    }
    fn write(&mut self, cap: Cc128Cap, val: TData) -> MemoryResult<()> {
        self.check_capability::<TData>(cap, Cc128::PERM_STORE)?;

        // Do the write, which also does an alignment check
        self.base_mem.write(cap.address(), val)?;

        // Set the tag on the 128-bit range containing (addr) to false
        self.tag_mem.remove(&(cap.address() / 16));
        // Assert the numerical type cannot extend over multiple tagged regions.
        // We know the address was aligned to size_of<TData>, so as long as that size
        // is smaller than 128-bits we're fine.
        assert!(std::mem::size_of::<TData>() <= 16);

        Ok(())
    } 
}
/// Now we've defined MemoryOf<u8,u16,u32>, combine them into a single Memory trait
impl Memory32<Cc128Cap> for CheriAggregateMemory {
    fn range(&self) -> Range<usize> {
        self.base_mem.range().clone()
    }
}
impl Memory64<Cc128Cap> for CheriAggregateMemory {}
/// Impl a capability-aware view of memory for CHERI instructions
/// e.g. a CHERI Load instruction, which is allowed to load capabilities, would use this version.
impl MemoryOf<SafeTaggedCap, Cc128Cap> for CheriAggregateMemory {
    // read/write funcs that set correct tag bits on reads/writes
    fn read(&mut self, cap: Cc128Cap) -> MemoryResult<SafeTaggedCap> {
        // We don't require PERM_LOAD_CAP, because if it isn't set we just clear the tag (see below).
        // Spec = TR951$8.4, [C]LC
        self.check_capability::<u128>(cap, Cc128::PERM_LOAD)?;
        let addr = cap.address();

        check_alignment_range::<u128>(addr, &self.base_mem.range())?;

        let base_mem = &mut self.base_mem as &mut dyn MemoryOf<u64>;
        // Must be aligned and in-bounds
        // This is a valid capability if (it's tagged) AND (we have permission to load capabilities)
        let tag = self.tag_mem.contains(&(addr / 16)) && (cap.permissions() & Cc128::PERM_LOAD_CAP != 0);
        let data_top = base_mem.read(addr + 8)?;
        let data_bot = base_mem.read(addr)?;
        Ok(SafeTaggedCap::from_tagged_mem(data_top, data_bot, tag))
    }
    fn write(&mut self, cap: Cc128Cap, val: SafeTaggedCap) -> MemoryResult<()> {
        self.check_capability::<u128>(cap, Cc128::PERM_STORE | Cc128::PERM_STORE_CAP)?;
        let addr = cap.address();

        check_alignment_range::<u128>(addr, &self.base_mem.range())?;

        // TODO - this shouldn't have to be a dyn object, right?
        let base_mem = &mut self.base_mem as &mut dyn MemoryOf<u64>;
        match val {
            SafeTaggedCap::RawData{ top, bot } => {
                base_mem.write(addr + 8, top)?;
                base_mem.write(addr, bot)?;
                self.tag_mem.remove(&(addr / 16));
            }
            SafeTaggedCap::ValidCap(cap) => {
                let cap_pebst = Cc128::compress_mem(&cap);
                base_mem.write(addr + 8,    cap_pebst)?;
                base_mem.write(addr,        cap.address())?;
                self.tag_mem.insert(addr / 16);
            }
        }
        Ok(())
    }
}
impl From<AggregateMemory64> for CheriAggregateMemory {
    fn from(base_mem: AggregateMemory64) -> Self {
        CheriAggregateMemory {
            base_mem,
            tag_mem: HashSet::default()
        }
    }
}

/// Wrapper for a reference to CheriAggregateMemory that allows integer-mode accesses.
/// Exposes MemoryOf<{u8,u16,u32,u64}, TAddr=u64>.
pub struct IntegerModeCheriAggregateMemory<'a> {
    base_mem: &'a mut CheriAggregateMemory,
    base_cap: Cc128Cap
}
impl<'a> IntegerModeCheriAggregateMemory<'a> {
    pub fn wrap(base_mem: &'a mut CheriAggregateMemory, base_cap: Cc128Cap) -> Self {
        IntegerModeCheriAggregateMemory {
            base_mem,
            base_cap
        }
    }
}
/// Reimplement basic MemoryOf<TData, TAddr=u64> for the integer mode memory.
/// Integer addresses are passed to the underlying [CheriAggregateMemory] inside the base capability,
/// and all checking/security is done inside the [CheriAggregateMemory].
impl<'a, TData> MemoryOf<TData> for IntegerModeCheriAggregateMemory<'a> where CheriAggregateMemory: MemoryOf<TData, Cc128Cap> {
    fn read(&mut self, addr: u64) -> MemoryResult<TData> {
        self.base_cap.set_address_unchecked(addr);
        self.base_mem.read(self.base_cap)
    }
    fn write(&mut self, addr: u64, val: TData) -> MemoryResult<()> {
        self.base_cap.set_address_unchecked(addr);
        self.base_mem.write(self.base_cap, val)
    }
}
impl<'a> Memory32 for IntegerModeCheriAggregateMemory<'a> {
    fn range(&self) -> Range<usize> {
        self.base_mem.range()
    }
}
impl<'a> Memory64 for IntegerModeCheriAggregateMemory<'a> {}