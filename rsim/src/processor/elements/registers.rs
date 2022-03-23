use num_traits::Num;
use thiserror::Error;

pub trait RegisterFile<TData> {
    fn read(&mut self, idx: u8) -> Result<TData, RegisterFileError>;
    fn write(&mut self, idx: u8, val: TData) -> Result<(), RegisterFileError>;
}

pub trait RegisterTracking<TData> {
    // Access Tracking
    // Used to track the register accesses done per tick, as necessary for TestRIG
    fn start_tracking(&mut self) -> Result<(), RegisterFileError>;
    fn end_tracking(&mut self) -> Result<Vec<RegisterAction<TData>>, RegisterFileError>;
}

pub enum RegisterAction<TData> {
    Read{idx: u8, val: TData},
    Write{idx: u8, val: TData}
}

#[derive(Debug,Error,Copy,Clone,PartialEq,Eq)]
pub enum RegisterFileError {
    #[error("Not currently tracking accesses")]
    NotTracking,
    #[error("Already tracking accesses")]
    AlreadyTracking,
    #[error("Tried to access nonexistant register {0}")]
    InvalidIndex(u8),
}

/// Internal trait, used to implement RvRegisterFile as a generic over u32 and u64
/// TODO - should not be pub, but has to be: RvRegisterFile32 exposes RvRegisterFile<T> as public, which requires RegisterNumT to be public as well.
pub trait RegisterNumT: Num + std::fmt::LowerHex + Copy {}
impl RegisterNumT for u32 {}
impl RegisterNumT for u64 {}

pub struct RvRegisterFile<T: RegisterNumT> {
    regs: [T; 31],
    tracking: Option<Vec<RegisterAction<T>>>
}
impl<T> RvRegisterFile<T> where T: RegisterNumT {
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

        for i in 0..32 {
            println!("x{} = {} = 0x{:08x}", i, REGISTER_NAMES[i], match i {
                0 => T::zero(),
                _ => self.regs[i - 1]
            });
        }
    }
    pub fn reset(&mut self) {
        self.regs = [T::zero(); 31];
        self.tracking = None;
    }
}
impl<T> RegisterFile<T> for RvRegisterFile<T> where T: RegisterNumT {    
    fn read(&mut self, idx: u8) -> Result<T, RegisterFileError> {
        let val = match idx {
            0    => Ok(T::zero()),
            1..=31 => Ok(self.regs[(idx - 1) as usize]),
            _ => Err(RegisterFileError::InvalidIndex(idx))
        }?;

        if self.tracking.is_some() {
            self.tracking.as_mut().unwrap().push(RegisterAction::Read{idx, val})
        }

        Ok(val)
    }
    fn write(&mut self, idx: u8, val: T) -> Result<(), RegisterFileError> {
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
impl<T> RegisterTracking<T> for RvRegisterFile<T> where T: RegisterNumT {
    fn start_tracking(&mut self) -> Result<(), RegisterFileError> {
        if self.tracking.is_some() {
            Err(RegisterFileError::AlreadyTracking)
        } else {
            self.tracking = Some(vec![]);
            Ok(())
        }
    }
    fn end_tracking(&mut self) -> Result<Vec<RegisterAction<T>>, RegisterFileError> {
        if let Some(tracking) = self.tracking.take() {
            Ok(tracking)
        } else {
            Err(RegisterFileError::NotTracking)
        }
    }
}
impl<T> Default for RvRegisterFile<T> where T: RegisterNumT {
    fn default() -> Self {
        RvRegisterFile {
            regs: [T::zero(); 31],
            tracking: None,
        }
    }
}

pub type RvRegisterFile32 = RvRegisterFile<u32>;
pub type RvRegisterFile64 = RvRegisterFile<u64>;