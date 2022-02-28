use crate::processor::exceptions::MemoryException;
use std::ops::Range;

pub trait MemoryOf<TData> where TData: Sized {
    fn read(&mut self, addr: u64) -> Result<TData, MemoryException>;
    fn write(&mut self, addr: u64, val: TData) -> Result<(), MemoryException>;
}
pub trait Memory32: MemoryOf<u8> + MemoryOf<u16> + MemoryOf<u32> {
    /// The mapped address range for this Memory.
    /// All addresses passed to read,write must be within this range.
    /// Guaranteed to be at least 4 bytes in size, both ends will be 4-byte aligned.
    fn range(&self) -> Range<usize>;
    fn load_u8(&mut self, addr: u64) -> Result<u8, MemoryException> {
        <Self as MemoryOf<u8>>::read(self, addr)
    }
    fn load_u16(&mut self, addr: u64) -> Result<u16, MemoryException> {
        <Self as MemoryOf<u16>>::read(self, addr)
    }
    fn load_u32(&mut self, addr: u64) -> Result<u32, MemoryException> {
        <Self as MemoryOf<u32>>::read(self, addr)
    }
    fn store_u8(&mut self, addr: u64, val: u8) -> Result<(), MemoryException> {
        <Self as MemoryOf<u8>>::write(self, addr, val)
    }
    fn store_u16(&mut self, addr: u64, val: u16) -> Result<(), MemoryException> {
        <Self as MemoryOf<u16>>::write(self, addr, val)
    }
    fn store_u32(&mut self, addr: u64, val: u32) -> Result<(), MemoryException> {
        <Self as MemoryOf<u32>>::write(self, addr, val)
    }
}
pub trait Memory64: Memory32 + MemoryOf<u64> {
    fn load_u32(&mut self, addr: u64) -> Result<u64, MemoryException> {
        <Self as MemoryOf<u64>>::read(self, addr)
    }
    fn store_u32(&mut self, addr: u64, val: u64) -> Result<(), MemoryException> {
        <Self as MemoryOf<u64>>::write(self, addr, val)
    }
}

// Memory expects to address a Vec of data by u64 - usize should be at least that large
use std::mem::size_of;
const_assert!(size_of::<usize>() >= size_of::<u64>());

/// I/O Memory
/// Defines an address range of a single u32
/// Reads from this address return 0
/// Writes to this address through MemoryException::ResultReturned
pub struct IOMemory {
    range: Range<usize>,
    expected: u32,
}
impl IOMemory {
    pub fn return_address(addr: usize, expected: u32) -> IOMemory {
        IOMemory{
            range: Range{ start: addr, end: addr+8 },
            expected
        }
    }
}
impl MemoryOf<u8> for IOMemory {
    fn read(&mut self, _: u64) -> Result<u8, MemoryException> { Ok(0) }
    fn write(&mut self, _: u64, val: u8) -> Result<(), MemoryException> {
        Err(MemoryException::ResultReturned{
            got: val as u32,
            expected: self.expected
        })
    }
}
impl MemoryOf<u16> for IOMemory {
    fn read(&mut self, _: u64) -> Result<u16, MemoryException> { Ok(0) }
    fn write(&mut self, _: u64, val: u16) -> Result<(), MemoryException> {
        Err(MemoryException::ResultReturned{
            got: val as u32,
            expected: self.expected
        })
    }
}
impl MemoryOf<u32> for IOMemory {
    fn read(&mut self, _: u64) -> Result<u32, MemoryException> { Ok(0) }
    fn write(&mut self, _: u64, val: u32) -> Result<(), MemoryException> {
        Err(MemoryException::ResultReturned{
            got: val as u32,
            expected: self.expected
        })
    }
}
impl MemoryOf<u64> for IOMemory {
    fn read(&mut self, _: u64) -> Result<u64, MemoryException> { Ok(0) }
    fn write(&mut self, _: u64, val: u64) -> Result<(), MemoryException> {
        Err(MemoryException::ResultReturned{
            got: val as u32,
            expected: self.expected
        })
    }
}
impl Memory32 for IOMemory {
    fn range(&self) -> Range<usize> {
        self.range.clone()
    }
}
impl Memory64 for IOMemory {}

/// Array-backed memory
/// 
/// Defines a valid address range - all addresses passed into read/write must be within this range
/// 
/// Implements MemoryOf<u64>, MemoryOf<u32>, MemoryOf<u16>, MemoryOf<u8>
/// 
/// Fields
/// - `data` - Backing vector. Guaranteed to be the same length as `range`
/// - `range` - Address range backed by the vector. Contains at least 4 elements, addresses aligned to 4-bytes
pub struct MemoryBacking {
    data: Vec<u8>,      // len(data) = (range.end - range.start)
    range: Range<usize> // Always not empty, aligned to 4 bytes
}
impl MemoryBacking {
    /// Generate a vector of zeros and map it to an address range.
    pub fn zeros(range: Range<usize>) -> MemoryBacking {
        assert!(!range.is_empty());
        if range.start % 4 != 0 || range.end % 4 != 0 {
            panic!("Input range {:?} for MemoryBacking not aligned", range);
        }
        MemoryBacking {
            data: vec![0; range.end - range.start],
            range
        }
    }
    /// Read bytes from a file and map them to an address range.
    /// The file data will be read into the start of the range,
    /// any empty space between the end of the file data and the end of the range will be zero-padded. 
    pub fn from_file(path_s: &str, range: Range<usize>) -> MemoryBacking {
        assert!(!range.is_empty());
        if range.start % 4 != 0 || range.end % 4 != 0 {
            panic!("Input range {:?} for MemoryBacking not aligned", range);
        }

        use std::io::Read;
        use std::path::Path;
        use std::fs::{File,metadata};

        let path = Path::new(path_s);

        let pad_memory_to = range.end - range.start;

        let mut f = File::open(&path).expect("no file found");
        let metadata = metadata(&path).expect("unable to read metadata");
        assert_eq!(metadata.len() % 4, 0);
        assert_eq!(metadata.len() as usize <= pad_memory_to, true);
        
        let mut buffer = vec![0; pad_memory_to];
        f.read(&mut buffer).expect("buffer overflow");

        MemoryBacking {
            data: buffer,
            range
        }
    }

}
impl MemoryOf<u64> for MemoryBacking {
    fn read(&mut self, addr: u64) -> Result<u64, MemoryException> {
        let addr = addr as usize;
        if addr % 8 != 0 {
            Err(MemoryException::AddressMisaligned{addr, expected: 8})
        } else if !self.range.contains(&addr) || !self.range.contains(&(addr + 7)) {
            Err(MemoryException::AddressUnmapped{addr})
        } else {
            let addr = addr - self.range.start;
            // Must be aligned and in-bounds
            Ok(
                ((self.data[addr+3] as u64) << 56) | 
                ((self.data[addr+3] as u64) << 48) | 
                ((self.data[addr+3] as u64) << 40) | 
                ((self.data[addr+3] as u64) << 32) | 
                ((self.data[addr+3] as u64) << 24) | 
                ((self.data[addr+2] as u64) << 16) | 
                ((self.data[addr+1] as u64) << 8) | 
                ((self.data[addr+0] as u64))
            )
        }
    }
    fn write(&mut self, addr: u64, val: u64) -> Result<(), MemoryException> {
        let addr = addr as usize;
        if addr % 8 != 0 {
            Err(MemoryException::AddressMisaligned{addr, expected: 8})
        } else if !self.range.contains(&addr) || !self.range.contains(&(addr + 7)) {
            Err(MemoryException::AddressUnmapped{addr})
        } else {
            let addr = addr - self.range.start;
            self.data[addr + 7] = (val >> 56) as u8;
            self.data[addr + 6] = (val >> 48) as u8;
            self.data[addr + 5] = (val >> 40) as u8;
            self.data[addr + 4] = (val >> 32) as u8;
            self.data[addr + 3] = (val >> 24) as u8;
            self.data[addr + 2] = (val >> 16) as u8;
            self.data[addr + 1] = (val >> 8) as u8;
            self.data[addr + 0] = (val) as u8;
            Ok(())
        }
    }
}
impl MemoryOf<u32> for MemoryBacking {
    fn read(&mut self, addr: u64) -> Result<u32, MemoryException> {
        let addr = addr as usize;
        if addr % 4 != 0 {
            Err(MemoryException::AddressMisaligned{addr, expected: 4})
        } else if !self.range.contains(&addr) || !self.range.contains(&(addr + 3)) {
            Err(MemoryException::AddressUnmapped{addr})
        } else {
            let addr = addr - self.range.start;
            // Must be aligned and in-bounds
            Ok(
                ((self.data[addr+3] as u32) << 24) | 
                ((self.data[addr+2] as u32) << 16) | 
                ((self.data[addr+1] as u32) << 8) | 
                ((self.data[addr+0] as u32))
            )
        }
    }
    fn write(&mut self, addr: u64, val: u32) -> Result<(), MemoryException> {
        let addr = addr as usize;
        if addr % 4 != 0 {
            Err(MemoryException::AddressMisaligned{addr, expected: 4})
        } else if !self.range.contains(&addr) || !self.range.contains(&(addr + 3)) {
            Err(MemoryException::AddressUnmapped{addr})
        } else {
            let addr = addr - self.range.start;
            self.data[addr + 3] = (val >> 24) as u8;
            self.data[addr + 2] = (val >> 16) as u8;
            self.data[addr + 1] = (val >> 8) as u8;
            self.data[addr + 0] = (val) as u8;
            Ok(())
        }
    }
}
impl MemoryOf<u16> for MemoryBacking {
    fn read(&mut self, addr: u64) -> Result<u16, MemoryException> {
        let addr = addr as usize;
        if addr % 2 != 0 {
            Err(MemoryException::AddressMisaligned{addr, expected: 2})
        } else if !self.range.contains(&addr) || !self.range.contains(&(addr + 1)) {
            Err(MemoryException::AddressUnmapped{addr})
        } else {
            let addr = addr - self.range.start;
            // Must be aligned and in-bounds
            Ok(
                ((self.data[addr+1] as u16) << 8) | 
                ((self.data[addr+0] as u16))
            )
        }
    }
    fn write(&mut self, addr: u64, val: u16) -> Result<(), MemoryException> {
        let addr = addr as usize;
        if addr % 2 != 0 {
            Err(MemoryException::AddressMisaligned{addr, expected: 2})
        } else if !self.range.contains(&addr) || !self.range.contains(&(addr + 1)) {
            Err(MemoryException::AddressUnmapped{addr})
        } else {
            let addr = addr - self.range.start;
            self.data[addr + 1] = (val >> 8) as u8;
            self.data[addr + 0] = (val) as u8;
            Ok(())
        }
    }
}
impl MemoryOf<u8> for MemoryBacking {
    fn read(&mut self, addr: u64) -> Result<u8, MemoryException> {
        let addr = addr as usize;
        if !self.range.contains(&addr) {
            Err(MemoryException::AddressUnmapped{addr})
        } else {
            let addr = addr - self.range.start;
            // Must be aligned and in-bounds
            Ok(
                self.data[addr]
            )
        }
    }
    fn write(&mut self, addr: u64, val: u8) -> Result<(), MemoryException> {
        let addr = addr as usize;
        if !self.range.contains(&addr) {
            Err(MemoryException::AddressUnmapped{addr})
        } else {
            let addr = addr - self.range.start;
            self.data[addr] = val;
            Ok(())
        }
    }
}
impl Memory32 for MemoryBacking {
    fn range(&self) -> Range<usize> {
        self.range.clone()
    }
}
impl Memory64 for MemoryBacking {}

/// Struct that combines a set of array-backed memory mappings.
/// The mapped address ranges may not overlap.
///
/// `T` should be `dyn Memory64` or `dyn Memory32`.
/// Because T is stored in a Box<>, it doesn't need to be Sized
pub struct AggregateMemory<T: Memory32 + ?Sized> {
    mappings: Vec<Box<T>>, // Guaranteed to not have overlapping ranges, have at least one mapping
    full_range: Range<usize> // Guaranteed to not be empty, be 4-byte-aligned
}
impl<T: Memory32 + ?Sized> AggregateMemory<T> {
    /// Take a set of mappings, verify they do not overlap, and turn them into an `AggregateMemory`.
    /// Panics if any mappings overlap.
    pub fn from_mappings(mappings: Vec<Box<T>>) -> Self {
        assert!(mappings.len() >= 1);
        let mut full_range = mappings[0].range().clone();

        for mapping_a in mappings.iter() {
            use std::cmp::{min,max};
            
            let range_a = mapping_a.range();
            // Expand the full_range to include this data
            full_range.start = min(range_a.start, full_range.start);
            full_range.end = max(range_a.end, full_range.end);

            // Check we don't have overlapping ranges
            for mapping_b in mappings.iter() {
                let range_b = mapping_b.range();
                use std::ptr::eq;
                // For each permutations of two mappings (A,B) in mappings
                //  (where A,B are references, not copies)
                // If mapping_a and mapping_b are not the same,
                // and mapping_a contains either end of mapping_b's range, then there's an overlap.
                // The other way around will be tested, because (B,A) is another permutation
                if !eq(mapping_a, mapping_b) && 
                    (
                        range_a.contains(&range_b.start) ||
                        // The end of these ranges are exclusive
                        range_a.contains(&(range_b.end - 1))
                    )
                {
                    panic!("Mappings have overlapping ranges {:?} and {:?}", range_a, range_b)
                }
            }
        }
        assert!(!full_range.is_empty());
        assert!(full_range.start % 4 == 0 && full_range.end % 4 == 0);
        AggregateMemory {
            mappings,
            full_range
        }
    }
}
/// Foreach TData, where MemoryBacking implements MemoryOf<TData>, re-implement it for AggregateMemory
/// TData = u8,u16,u32, potentially u64
impl<T: Memory32 + ?Sized,TData> MemoryOf<TData> for AggregateMemory<T> where T: MemoryOf<TData> {
    fn read(&mut self, addr: u64) -> Result<TData, MemoryException> {
        // Find a mapping which handles this address
        for mapping in self.mappings.iter_mut() {
            if mapping.range().contains(&(addr as usize)) {
                // Read from the mapping - this handles alignment checks etc
                return mapping.read(addr)
            }
        }
        // If we're here, we didn't return => we don't have a mapping for this address
        Err(MemoryException::AddressUnmapped{addr: addr as usize})
    }
    fn write(&mut self, addr: u64, val: TData) -> Result<(), MemoryException> {
        // Find a mapping which handles this address
        for mapping in self.mappings.iter_mut() {
            if mapping.range().contains(&(addr as usize)) {
                // Write to the mapping - this handles alignment checks etc
                return mapping.write(addr, val)
            }
        }
        // If we're here, we didn't return => we don't have a mapping for this address
        Err(MemoryException::AddressUnmapped{addr: addr as usize})
    }
}
impl<T: Memory32 + ?Sized> Memory32 for AggregateMemory<T> {
    fn range(&self) -> Range<usize> {
        self.full_range.clone()
    }
}
impl Memory64 for AggregateMemory<dyn Memory64> {}
/// For convenience, allow a single MemoryBacking to be converted directly to an AggregateMemory
impl From<MemoryBacking> for AggregateMemory<dyn Memory32> {
    fn from(backing: MemoryBacking) -> Self {
        AggregateMemory {
            full_range: backing.range.clone(),
            mappings: vec![Box::new(backing)]
        }
    }
}
impl From<MemoryBacking> for AggregateMemory<dyn Memory64> {
    fn from(backing: MemoryBacking) -> Self {
        AggregateMemory {
            full_range: backing.range.clone(),
            mappings: vec![Box::new(backing)]
        }
    }
}
pub type AggregateMemory32 = AggregateMemory<dyn Memory32>;
pub type AggregateMemory64 = AggregateMemory<dyn Memory64>;