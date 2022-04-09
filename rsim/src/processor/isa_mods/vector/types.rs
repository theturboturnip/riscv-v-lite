#![allow(non_camel_case_types)]

use std::mem::size_of;

/// Unsigned type of length [ELEN]
/// 
/// ```
/// use rsim::processor::vector::{uELEN, ELEN};
/// use std::mem::size_of;
/// 
/// assert_eq!(size_of::<uELEN>() * 8, ELEN);
/// ```
pub type uELEN = u32;


/// Unsigned type of length [VLEN]
/// 
/// Used for storing vector registers
/// 
/// ```
/// use rsim::processor::vector::{uVLEN, VLEN};
/// use std::mem::size_of;
/// 
/// assert_eq!(size_of::<uVLEN>() * 8, VLEN);
/// ```
pub type uVLEN = u128;

/// Vector register length in bits
pub const VLEN: usize = size_of::<uVLEN>() * 8; // ELEN * 4
const_assert!(size_of::<uVLEN>() % size_of::<uELEN>() == 0);


/// Trait for possible XLEN values
/// We need to be able to turn it into u64s (used for talking to memory subsystem), and getting it from a u32 (the type used for Vlen)
pub trait PossibleXlen: Into<u64> + From<u32> {}
impl PossibleXlen for u64 {}
impl PossibleXlen for u32 {}
