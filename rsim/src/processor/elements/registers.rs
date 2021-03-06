use crate::processor::isa_mods::PossibleXlen;
use thiserror::Error;

pub trait RegisterFile<TData> {
    fn read(&mut self, idx: u8) -> Result<TData, RegisterFileError>;
    fn write(&mut self, idx: u8, val: TData) -> Result<(), RegisterFileError>;
}

#[derive(Debug, Error, Copy, Clone, PartialEq, Eq)]
pub enum RegisterFileError {
    #[error("Tried to access nonexistant register {0}")]
    InvalidIndex(u8),
}

pub struct RvRegisterFile<T: PossibleXlen> {
    regs: [T; 31],
}
impl<T> RvRegisterFile<T>
where
    T: PossibleXlen,
{
    pub fn dump(&self) {
        const REGISTER_NAMES: [&str; 32] = [
            "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "fp", "s1", "a0", "a1", "a2", "a3",
            "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
            "t3", "t4", "t5", "t6",
        ];

        for i in 0..32 {
            println!(
                "x{} = {} = 0x{:08x}",
                i,
                REGISTER_NAMES[i],
                match i {
                    0 => T::zero(),
                    _ => self.regs[i - 1],
                }
            );
        }
    }
    pub fn reset(&mut self) {
        self.regs = [T::zero(); 31];
    }
}
impl<T> RegisterFile<T> for RvRegisterFile<T>
where
    T: PossibleXlen,
{
    fn read(&mut self, idx: u8) -> Result<T, RegisterFileError> {
        let val = match idx {
            0 => Ok(T::zero()),
            1..=31 => Ok(self.regs[(idx - 1) as usize]),
            _ => Err(RegisterFileError::InvalidIndex(idx)),
        }?;

        Ok(val)
    }
    fn write(&mut self, idx: u8, val: T) -> Result<(), RegisterFileError> {
        match idx {
            0 => Ok(()),
            1..=31 => {
                self.regs[(idx - 1) as usize] = val;
                Ok(())
            }
            _ => Err(RegisterFileError::InvalidIndex(idx)),
        }?;

        Ok(())
    }
}
impl<T> Default for RvRegisterFile<T>
where
    T: PossibleXlen,
{
    fn default() -> Self {
        RvRegisterFile {
            regs: [T::zero(); 31],
        }
    }
}
