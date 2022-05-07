use crate::processor::elements::registers::*;
use super::capability::*;

/// Register file for 64-bit RISC-V that can hold tagged 128-bit capabilities.
/// 
/// Implements [RegisterFile<SafeTaggedCap>] for capability-mode access, [RegisterFile<u64>] for integer-mode.
pub struct CheriRV64RegisterFile {
    regs: [SafeTaggedCap; 31],
}
impl CheriRV64RegisterFile {
    pub fn read_u64(&mut self, idx: u8) -> Result<u64, RegisterFileError> {
        <Self as RegisterFile<u64>>::read(self, idx)
    }
    pub fn write_u64(&mut self, idx: u8, val: u64) -> Result<(), RegisterFileError> {
        <Self as RegisterFile<u64>>::write(self, idx, val)
    }
    /// Reads a valid capability from the register file.
    pub fn read_maybe_cap(&mut self, idx: u8) -> Result<SafeTaggedCap, RegisterFileError> {
        <Self as RegisterFile<SafeTaggedCap>>::read(self, idx)
    }
    pub fn write_maybe_cap(&mut self, idx: u8, val: SafeTaggedCap) -> Result<(), RegisterFileError> {
        <Self as RegisterFile<SafeTaggedCap>>::write(self, idx, val)
    }

    pub fn dump(&self) {
        const REGISTER_NAMES: [&str; 32] = [
            "zero", "ra", "sp", "gp",
            "tp", "t0", "t1", "t2",
            "fp", "s1", "a0", "a1",
            "a2", "a3", "a4", "a5",
            "a6", "a7", "s2", "s3",
            "s4", "s5", "s6", "s7",
            "s8", "s9", "s10", "s11",
            "t3", "t4", "t5", "t6"
        ];

        println!("x{} = {} = 0x{:016x}", 0, REGISTER_NAMES[0], 0);
        for i in 1..32 {
            match self.regs[i - 1] {
                SafeTaggedCap::RawData{ top, bot } => {
                    println!("x{} = {} = 0x{:08x}{:08x}", i, REGISTER_NAMES[i], top, bot);
                }
                SafeTaggedCap::ValidCap(cap) => {
                    println!("x{} = {} = {:x?}", i, REGISTER_NAMES[i], cap);
                }
            };
        }
    }
    pub fn reset(&mut self) {
        // TR-951 $3.6.1 
        // If the arch-specific approach is to extend existing integer registers
        // to also hold tagged capabilties, can instead init to hold untagged values:
        // Unset Tag bit, offset=0, base=0, length=2^XLEN(?), otype=2^(XLEN)-1(??)
        // Right now, initing them to 0
        self.regs = [SafeTaggedCap::RawData{top: 0, bot: 0}; 31];
    }
}
impl RegisterFile<SafeTaggedCap> for CheriRV64RegisterFile {
    fn read(&mut self, idx: u8) -> Result<SafeTaggedCap, RegisterFileError> {
        let val = match idx {
            0    => Ok(SafeTaggedCap::RawData{top: 0, bot: 0}),
            1..=31 => Ok(self.regs[(idx - 1) as usize]),
            _ => Err(RegisterFileError::InvalidIndex(idx))
        }?;

        Ok(val)
    }
    fn write(&mut self, idx: u8, val: SafeTaggedCap) -> Result<(), RegisterFileError> {
        match idx {
            0    => Ok(()),
            1..=31 => {
                self.regs[(idx - 1) as usize] = val;
                Ok(())
            },
            _ => Err(RegisterFileError::InvalidIndex(idx))
        }?;

        Ok(())
    }
}
/// Interface used by normal RV32 instructions
impl RegisterFile<u64> for CheriRV64RegisterFile {
    fn read(&mut self, idx: u8) -> Result<u64, RegisterFileError> {
        let val = match idx {
            0    => Ok(0),
            1..=31 => match self.regs[(idx - 1) as usize] {
                // Return just the bottom part of raw data - the top is capability metadata
                SafeTaggedCap::RawData{top: _, bot} => Ok(bot),
                SafeTaggedCap::ValidCap(cap) => Ok(cap.address())
            },
            _ => Err(RegisterFileError::InvalidIndex(idx))
        }?;

        Ok(val as u64)
    }
    fn write(&mut self, idx: u8, val: u64) -> Result<(), RegisterFileError> {
        // In integer mode, assume writes zero out the top bit and remove tag
        // TR-951$5.3.6 states "the upper XLEN bits and tag bit [...] will be ignored"
        let val = SafeTaggedCap::RawData{top: 0, bot: val};
        match idx {
            0    => Ok(()),
            1..=31 => {
                self.regs[(idx - 1) as usize] = val;
                Ok(())
            },
            _ => Err(RegisterFileError::InvalidIndex(idx))
        }?;

        Ok(())
    }
}
impl Default for CheriRV64RegisterFile {
    fn default() -> Self {
        CheriRV64RegisterFile {
            regs: [SafeTaggedCap::RawData{ top: 0, bot: 0 }; 31],
        }
    }
}