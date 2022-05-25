use crate::processor::elements::cheri::capability::check_capability;
use std::any::Any;
use std::collections::HashSet;
use std::ops::Range;

use super::capability::*;
use crate::processor::elements::memory::*;

/// Memory holding tags for capabilities.
/// Implements MemoryOf<bool>, which checks if the supplied address is a multiple of 16-bytes i.e. the size of a capability.
pub struct TagMemory {
    internal_mem: HashSet<u64>,
}
impl TagMemory {
    pub fn new() -> Self {
        TagMemory {
            internal_mem: HashSet::default(),
        }
    }
    pub fn range(&self) -> Range<usize> {
        Range {
            start: 0,
            end: usize::MAX,
        }
    }
}
impl MemoryOf<bool> for TagMemory {
    fn read(&mut self, addr: u64) -> MemoryResult<bool> {
        // Should only ask for tags on aligned values
        check_alignment_range::<u128>(addr, &self.range())?;

        let tag_line = addr / (std::mem::size_of::<u128>() as u64);
        Ok(self.internal_mem.contains(&tag_line))
    }
    fn write(&mut self, addr: u64, val: bool) -> MemoryResult<()> {
        check_alignment_range::<u128>(addr, &self.range())?;

        let tag_line = addr / (std::mem::size_of::<u128>() as u64);
        if val {
            self.internal_mem.insert(tag_line);
        } else {
            self.internal_mem.remove(&tag_line);
        }
        Ok(())
    }
}

pub trait CheriMemory<TAddr>: Memory<TAddr> + MemoryOf<SafeTaggedCap, TAddr> {
    fn load_maybe_cap(&mut self, addr: TAddr) -> MemoryResult<SafeTaggedCap> {
        <Self as MemoryOf<SafeTaggedCap, TAddr>>::read(self, addr)
    }
    fn store_maybe_cap(&mut self, addr: TAddr, val: SafeTaggedCap) -> MemoryResult<()> {
        <Self as MemoryOf<SafeTaggedCap, TAddr>>::write(self, addr, val)
    }
    fn store_cap(&mut self, addr: TAddr, val: Cc128Cap) -> MemoryResult<()> {
        <Self as MemoryOf<SafeTaggedCap, TAddr>>::write(self, addr, SafeTaggedCap::from_cap(val))
    }
}

/// Wrapper for AggregateMemory64 that keeps tags, supports MemoryOf<SafeTaggedCap> for reading/writing capabilities.
/// All other Memory variants clear associated tag bits on write.
pub struct CheriAggregateMemory {
    base_mem: AggregateMemory,
    tag_mem: TagMemory,
}
impl CheriAggregateMemory {
    pub fn from_base(base_mem: AggregateMemory) -> CheriAggregateMemory {
        CheriAggregateMemory {
            base_mem,
            tag_mem: TagMemory::new(),
        }
    }

    pub fn fetch_inst_u32(&mut self, cap: Cc128Cap) -> MemoryResult<u32> {
        check_capability::<u32>(cap, Cc128::PERM_LOAD | Cc128::PERM_EXECUTE)?;

        self.base_mem.load_u32(cap.address())
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
            full_range_cap_base,                                // base
            full_range_cap_base,                                // put the address at the base
            full_range_cap_base as u128 + required_len as u128, // top
        );

        full_range_cap
    }

    pub fn get_io_values(&self) -> Vec<Option<u64>> {
        self.base_mem.get_io_values()
    }
}
/// Implement MemoryOf<TData> addressed by Cc128Cap, which does all necessary validity checks,
/// for every TData in {u8,u16,u32,u64}
impl<TData> MemoryOf<TData, Cc128Cap> for CheriAggregateMemory
where
    AggregateMemory: MemoryOf<TData, u64>,
{
    fn read(&mut self, cap: Cc128Cap) -> MemoryResult<TData> {
        check_capability::<TData>(cap, Cc128::PERM_LOAD)?;

        self.base_mem.read(cap.address())
    }
    fn write(&mut self, cap: Cc128Cap, val: TData) -> MemoryResult<()> {
        check_capability::<TData>(cap, Cc128::PERM_STORE)?;

        // Do the write, which also does an alignment check
        self.base_mem.write(cap.address(), val)?;

        // Set the tag on the 128-bit range containing (addr) to false
        // Use an aligned address to clear the tag - otherwise it complains
        self.tag_mem.write(cap.address() & !0xFFFF, false)?;
        // Assert the numerical type cannot extend over multiple tagged regions.
        // We know the address was aligned to size_of<TData>, so as long as that size
        // is smaller than 128-bits we're fine.
        assert!(std::mem::size_of::<TData>() <= 16);

        Ok(())
    }
}
/// Now we've defined MemoryOf<u8,u16,u32,u64>, combine them into a single Memory trait
impl Memory<Cc128Cap> for CheriAggregateMemory {
    fn range(&self) -> Range<usize> {
        self.base_mem.range().clone()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
/// Combine MemoryOf<u8,u16,u32,u64> with MemoryOf<SafeTaggedCap>
impl CheriMemory<Cc128Cap> for CheriAggregateMemory {}
/// Impl a capability-aware view of memory for CHERI instructions
/// e.g. a CHERI Load instruction, which is allowed to load capabilities, would use this version.
impl MemoryOf<SafeTaggedCap, Cc128Cap> for CheriAggregateMemory {
    // read/write funcs that set correct tag bits on reads/writes
    fn read(&mut self, cap: Cc128Cap) -> MemoryResult<SafeTaggedCap> {
        // We don't require PERM_LOAD_CAP, because if it isn't set we just clear the tag (see below).
        // Spec = TR951$8.4, [C]LC
        check_capability::<u128>(cap, Cc128::PERM_LOAD)?;
        let addr = cap.address();

        check_alignment_range::<u128>(addr, &self.base_mem.range())?;

        let base_mem = &mut self.base_mem as &mut dyn MemoryOf<u64>;
        // Must be aligned and in-bounds
        // This is a valid capability if (it's tagged) AND (we have permission to load capabilities)
        let tag = self.tag_mem.read(addr)? && (cap.permissions() & Cc128::PERM_LOAD_CAP != 0);
        let data_top = base_mem.read(addr + 8)?;
        let data_bot = base_mem.read(addr)?;
        Ok(SafeTaggedCap::from_tagged_mem(data_top, data_bot, tag))
    }
    fn write(&mut self, cap: Cc128Cap, val: SafeTaggedCap) -> MemoryResult<()> {
        check_capability::<u128>(cap, Cc128::PERM_STORE | Cc128::PERM_STORE_CAP)?;
        let addr = cap.address();

        check_alignment_range::<u128>(addr, &self.base_mem.range())?;

        // TODO - this shouldn't have to be a dyn object, right?
        let base_mem = &mut self.base_mem as &mut dyn MemoryOf<u64>;
        match val {
            SafeTaggedCap::RawData { top, bot } => {
                base_mem.write(addr + 8, top)?;
                base_mem.write(addr, bot)?;
                self.tag_mem.write(addr, false)?;
            }
            SafeTaggedCap::ValidCap(cap) => {
                let cap_pebst = Cc128::compress_mem(&cap);
                base_mem.write(addr + 8, cap_pebst)?;
                base_mem.write(addr, cap.address())?;
                self.tag_mem.write(addr, true)?;
            }
        }
        Ok(())
    }
}

/// Wrapper for a reference to CheriAggregateMemory that allows integer-mode accesses.
/// Exposes MemoryOf<{u8,u16,u32,u64}, TAddr=u64>.
pub struct IntegerModeCheriAggregateMemory<'a> {
    base_mem: &'a mut CheriAggregateMemory,
    base_cap: Cc128Cap,
}
impl<'a> IntegerModeCheriAggregateMemory<'a> {
    pub fn wrap(base_mem: &'a mut CheriAggregateMemory, base_cap: Cc128Cap) -> Self {
        IntegerModeCheriAggregateMemory { base_mem, base_cap }
    }
}
/// Reimplement basic MemoryOf<TData, TAddr=u64> for the integer mode memory.
/// Integer addresses are passed to the underlying [CheriAggregateMemory] inside the base capability,
/// and all checking/security is done inside the [CheriAggregateMemory].
impl<'a, TData> MemoryOf<TData> for IntegerModeCheriAggregateMemory<'a>
where
    CheriAggregateMemory: MemoryOf<TData, Cc128Cap>,
{
    fn read(&mut self, addr: u64) -> MemoryResult<TData> {
        self.base_cap.set_address_unchecked(addr);
        self.base_mem.read(self.base_cap)
    }
    fn write(&mut self, addr: u64, val: TData) -> MemoryResult<()> {
        self.base_cap.set_address_unchecked(addr);
        self.base_mem.write(self.base_cap, val)
    }
}
impl<'a> Memory for IntegerModeCheriAggregateMemory<'a> {
    fn range(&self) -> Range<usize> {
        self.base_mem.range()
    }
    fn as_any(&self) -> &dyn Any {
        unreachable!("Should never try to devirtualize an IntegerMode wrapper")
    }
}
/// Combine MemoryOf<u8,u16,u32,u64> with MemoryOf<SafeTaggedCap>
impl<'a> CheriMemory<u64> for IntegerModeCheriAggregateMemory<'a> {}
