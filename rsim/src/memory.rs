use thiserror::Error;
use anyhow::{Result};

use std::convert::TryInto;
use std::io::Read;
use std::path::Path;
use std::fs::{File,metadata};

pub struct Memory {
    _data: Vec<u8>,
}

#[derive(Error, Debug)]
enum MemError {
    #[error("Address {0:08x} misaligned or OOB")]
    AddressInvalid(usize),
}

impl Memory {
    pub fn new_from_file(path_s: &str, pad_memory_to: u64) -> Memory {
        let path = Path::new(path_s);

        let mut f = File::open(&path).expect("no file found");
        let metadata = metadata(&path).expect("unable to read metadata");
        assert_eq!(metadata.len() % 4, 0);
        assert_eq!(metadata.len() <= pad_memory_to, true);
        
        let mut buffer = vec![0; pad_memory_to as usize];
        f.read(&mut buffer).expect("buffer overflow");

        // let mut buffer32 = vec![0; metadata.len() as usize / 4];
        // for i in 0..buffer32.len() {
        //     buffer32[i] = 
        //         ((buffer[i * 4 + 3] as u32) << 24) |
        //         ((buffer[i * 4 + 2] as u32) << 16) |
        //         ((buffer[i * 4 + 1] as u32) << 8)  |
        //         ((buffer[i * 4 + 0] as u32) << 0)
        //     ;
        // }

        Memory {
            _data: buffer,
        }
    }

    pub fn len(&self) -> usize {
        self._data.len()
    }

    pub fn load_u32(&self, addr: u32) -> Result<u32> {
        let addr: usize = addr.try_into()?;
        if addr & 0x03 == 0 // Aligned address
            && addr + 3 < self._data.len() { // In-bounds
            Ok(
                ((self._data[addr+3] as u32) << 24) | 
                ((self._data[addr+2] as u32) << 16) | 
                ((self._data[addr+1] as u32) << 8) | 
                ((self._data[addr+0] as u32))
            )
        } else {
            Err(MemError::AddressInvalid(addr))?
        }
    }
    pub fn load_u16(&self, addr: u32) -> Result<u16> {
        let addr: usize = addr.try_into()?;
        if addr & 0x01 == 0 // Aligned address
            && addr + 1 < self._data.len() { // In-bounds
                Ok(
                    ((self._data[addr+1] as u16) << 8) | 
                    ((self._data[addr+0] as u16))
                )
        } else {
            Err(MemError::AddressInvalid(addr))?
        }
    }
    pub fn load_u8(&self, addr: u32) -> Result<u8> {
        let addr: usize = addr.try_into()?;
        if addr < self._data.len() { // In-bounds
            Ok(self._data[addr])
        } else {
            Err(MemError::AddressInvalid(addr))?
        }
    }

    pub fn store_u32(&mut self, addr: u32, data: u32) -> Result<()> {
        let addr: usize = addr.try_into()?;
        if addr == 0xf000_0000 {
            // Special case
            println!("RESULT = 0x{:08x} = {}", data, data);
            Ok(())
        } else if addr & 0x03 == 0 // Aligned address
            && addr + 3 < self._data.len() { // In-bounds

            self._data[addr + 3] = ((data >> 24) & 0xff).try_into().unwrap();
            self._data[addr + 2] = ((data >> 16) & 0xff).try_into().unwrap();
            self._data[addr + 1] = ((data >> 8) & 0xff).try_into().unwrap();
            self._data[addr + 0] = ((data) & 0xff).try_into().unwrap();
            Ok(())
        } else {
            Err(MemError::AddressInvalid(addr))?
        }
    }

    pub fn store_u16(&mut self, addr: u32, data: u16) -> Result<()> {
        let addr: usize = addr.try_into()?;
        if addr & 0x01 == 0 // Aligned address
            && addr + 1 < self._data.len() { // In-bounds

            self._data[addr + 1] = ((data >> 8) & 0xff).try_into().unwrap();
            self._data[addr + 0] = ((data) & 0xff).try_into().unwrap();
            Ok(())
        } else {
            Err(MemError::AddressInvalid(addr))?
        }
    }

    pub fn store_u8(&mut self, addr: u32, data: u8) -> Result<()> {
        let addr: usize = addr.try_into()?;
        if addr < self._data.len() { // In-bounds
            self._data[addr] = data;
            Ok(())
        } else {
            Err(MemError::AddressInvalid(addr))?
        }
    }
}
