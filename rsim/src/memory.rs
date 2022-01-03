use thiserror::Error;
use std::convert::TryInto;

/// Simple structure representing main memory.
/// Supports loads and stores in 8, 16, 32-bit granularity.
/// Addressable by u32 only.
pub struct Memory {
    data: Vec<u8>,
}

// This library expects to address a Vec of data by u32 - usize should be at least that large
use std::mem::size_of;
const_assert!(size_of::<usize>() >= size_of::<u32>());

/// Error which can be raised when accessing memory
/// 
/// Includes ResultReturned, which is raised when the program writes a u32 to 0xF000_0000
#[derive(Error, Debug, PartialEq, Eq)]
pub enum MemError {
    #[error("Address {0:08x} misaligned")]
    AddressMisaligned(usize),
    #[error("Address {addr:08x} out-of-bounds, bounds = {max:08x}")]
    AddressOOB{ addr: usize, max: usize },
    #[error("Program returned a value: {0}")]
    ResultReturned(u32),
}

pub type Result<T> = std::result::Result<T, MemError>;

impl Memory {
    /// Generate a Memory holding the bytes from a given file, padded out with zeros.
    /// 
    /// # Arguments
    /// 
    /// * `path_s` - A path to the source file
    /// * `pad_memory_to` - The length to pad out the data to (bytes)
    pub fn new_from_file(path_s: &str, pad_memory_to: usize) -> Memory {
        use std::io::Read;
        use std::path::Path;
        use std::fs::{File,metadata};

        let path = Path::new(path_s);

        let mut f = File::open(&path).expect("no file found");
        let metadata = metadata(&path).expect("unable to read metadata");
        assert_eq!(metadata.len() % 4, 0);
        assert_eq!(metadata.len() as usize <= pad_memory_to, true);
        
        let mut buffer = vec![0; pad_memory_to];
        f.read(&mut buffer).expect("buffer overflow");

        Memory {
            data: buffer,
        }
    }

    /// Generate a Memory holding zeros.
    /// 
    /// # Arguments
    /// 
    /// * `length` - How many bytes the memory should hold
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rsim::memory::Memory;
    /// 
    /// let mem = Memory::new(4);
    /// assert_eq!(mem.load_u32(0).unwrap(), 0);
    /// assert_eq!(mem.len(), 4);
    /// ```
    pub fn new(length: usize) -> Memory {
        Memory {
            data: vec![0; length]
        }
    }

    /// Returns the size of the memory
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Reads a 32-bit value from an aligned, in-bounds address.
    /// Will return an error if the address is misaligned or out-of-bounds.
    /// 
    /// # Arguments
    /// 
    /// * `addr` - The address to load from
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rsim::memory::{Memory,MemError};
    /// let mut mem = Memory::new(256);
    /// 
    /// mem.store_u32(0x0, 512).unwrap();
    /// assert_eq!(mem.load_u32(0x0).unwrap(),      512);
    /// assert_eq!(mem.load_u32(0x03).unwrap_err(), MemError::AddressMisaligned(0x03));
    /// assert_eq!(mem.load_u32(512).unwrap_err(),  MemError::AddressOOB{ addr: 512, max: 256 });
    /// ```
    pub fn load_u32(&self, addr: u32) -> Result<u32> {
        let addr: usize = addr as usize;

        if addr & 0x03 != 0 {
            Err(MemError::AddressMisaligned(addr))
        } else if addr + 3 >= self.data.len() {
            Err(MemError::AddressOOB{addr, max: self.data.len()})
        } else {
            // Must be aligned and in-bounds
            Ok(
                ((self.data[addr+3] as u32) << 24) | 
                ((self.data[addr+2] as u32) << 16) | 
                ((self.data[addr+1] as u32) << 8) | 
                ((self.data[addr+0] as u32))
            )
        }
    }

    /// Reads a 16-bit value from an aligned, in-bounds address.
    /// Will return an error if the address is misaligned or out-of-bounds.
    /// 
    /// # Arguments
    /// 
    /// * `addr` - The address to load from
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rsim::memory::{Memory,MemError};
    /// let mut mem = Memory::new(256);
    /// 
    /// mem.store_u16(0x0, 512).unwrap();
    /// assert_eq!(mem.load_u16(0x0).unwrap(),      512);
    /// assert_eq!(mem.load_u16(0x03).unwrap_err(), MemError::AddressMisaligned(0x03));
    /// assert_eq!(mem.load_u16(512).unwrap_err(),  MemError::AddressOOB{ addr: 512, max: 256 });
    /// ```
    pub fn load_u16(&self, addr: u32) -> Result<u16> {
        let addr: usize = addr as usize;

        if addr & 0x01 != 0 {
            Err(MemError::AddressMisaligned(addr))
        } else if addr + 1 >= self.data.len() {
            Err(MemError::AddressOOB{addr, max: self.data.len()})
        } else {
            Ok(
                ((self.data[addr+1] as u16) << 8) | 
                ((self.data[addr+0] as u16))
            )
        }
    }

    /// Reads a 8-bit value from an in-bounds address.
    /// Will return an error if the address is out-of-bounds.
    /// 
    /// # Arguments
    /// 
    /// * `addr` - The address to load from
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rsim::memory::{Memory,MemError};
    /// let mut mem = Memory::new(256);
    /// 
    /// mem.store_u8(0x0, 16).unwrap();
    /// assert_eq!(mem.load_u8(0x0).unwrap(),     16);
    /// assert_eq!(mem.load_u8(512).unwrap_err(), MemError::AddressOOB{ addr: 512, max: 256 });
    /// ```
    pub fn load_u8(&self, addr: u32) -> Result<u8> {
        let addr: usize = addr as usize;

        if addr >= self.data.len() {
            Err(MemError::AddressOOB{addr, max: self.data.len()})
        } else {
            Ok(self.data[addr])
        }
    }

    /// Stores a 32-bit value to an aligned, in-bounds address.
    /// Will return an error if the address is misaligned or out-of-bounds.
    /// 
    /// # Arguments
    /// 
    /// * `addr` - The address to store to
    /// * `data` - The value to store
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rsim::memory::{Memory,MemError};
    /// let mut mem = Memory::new(256);
    /// 
    /// mem.store_u32(0x0, 512).unwrap();
    /// assert_eq!(mem.load_u32(0x0).unwrap(),      512);
    /// assert_eq!(mem.store_u32(0x03, 512).unwrap_err(), MemError::AddressMisaligned(0x03));
    /// assert_eq!(mem.store_u32(512, 512).unwrap_err(),  MemError::AddressOOB{ addr: 512, max: 256 });
    /// 
    /// // Writing to 0xF000_0000 is how the program returns results
    /// assert_eq!(mem.store_u32(0xF000_0000, 42).unwrap_err(),  MemError::ResultReturned(42));
    /// ```
    pub fn store_u32(&mut self, addr: u32, data: u32) -> Result<()> {
        let addr: usize = addr as usize;

        if addr == 0xF000_0000 {
            // Special case
            Err(MemError::ResultReturned(data))
            // panic!("RESULT = 0x{:08x} = {}", data, data);
        } else if addr & 0x03 != 0 {
            Err(MemError::AddressMisaligned(addr))
        } else if addr + 3 >= self.data.len() {
            Err(MemError::AddressOOB{addr, max: self.data.len()})
        } else {
            self.data[addr + 3] = ((data >> 24) & 0xff).try_into().unwrap();
            self.data[addr + 2] = ((data >> 16) & 0xff).try_into().unwrap();
            self.data[addr + 1] = ((data >> 8) & 0xff).try_into().unwrap();
            self.data[addr + 0] = ((data) & 0xff).try_into().unwrap();
            Ok(())
        }
    }

    /// Stores a 16-bit value to an in-bounds address.
    /// Will return an error if the address is out-of-bounds.
    /// 
    /// # Arguments
    /// 
    /// * `addr` - The address to store to
    /// * `data` - The value to store
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rsim::memory::{Memory,MemError};
    /// let mut mem = Memory::new(256);
    /// 
    /// mem.store_u16(0x0, 16).unwrap();
    /// assert_eq!(mem.load_u16(0x0).unwrap(),           16);
    /// assert_eq!(mem.store_u16(0x03, 16).unwrap_err(), MemError::AddressMisaligned(0x03));
    /// assert_eq!(mem.store_u16(512, 16).unwrap_err(),  MemError::AddressOOB{ addr: 512, max: 256 });
    /// ```
    pub fn store_u16(&mut self, addr: u32, data: u16) -> Result<()> {
        let addr: usize = addr as usize;

        if addr & 0x01 != 0 {
            Err(MemError::AddressMisaligned(addr))
        } else if addr + 1 >= self.data.len() {
            Err(MemError::AddressOOB{addr, max: self.data.len()})
        } else {

            self.data[addr + 1] = ((data >> 8) & 0xff).try_into().unwrap();
            self.data[addr + 0] = ((data) & 0xff).try_into().unwrap();
            Ok(())
        }
    }

    /// Stores a 8-bit value to an in-bounds address.
    /// Will return an error if the address is out-of-bounds.
    /// 
    /// # Arguments
    /// 
    /// * `addr` - The address to store to
    /// * `data` - The value to store
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rsim::memory::{Memory,MemError};
    /// let mut mem = Memory::new(256);
    /// 
    /// mem.store_u8(0x0, 16).unwrap();
    /// assert_eq!(mem.load_u8(0x0).unwrap(),     16);
    /// assert_eq!(mem.store_u8(512, 16).unwrap_err(), MemError::AddressOOB{ addr: 512, max: 256 });
    /// ```
    pub fn store_u8(&mut self, addr: u32, data: u8) -> Result<()> {
        let addr: usize = addr as usize;

        if addr >= self.data.len() {
            Err(MemError::AddressOOB{addr, max: self.data.len()})
        } else {
            self.data[addr] = data;
            Ok(())
        }
    }
}
