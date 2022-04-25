#[macro_use]
extern crate bitutils;

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate static_assertions;

pub mod processor;

pub use processor::models;
pub mod memory {
    pub use crate::processor::elements::memory::{AggregateMemory,MemoryBacking,IOMemory};
    pub use crate::processor::elements::cheri::{CheriAggregateMemory};
}
pub use crate::processor::elements::cheri::{Cc128,Cc128Cap,CheriRVFuncs,CompressedCapability};