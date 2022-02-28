#[macro_use]
extern crate bitutils;

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate static_assertions;

pub mod processor;

pub use processor::{Processor};
pub mod memory {
    pub use crate::processor::elements::{AggregateMemory32,AggregateMemory64,MemoryBacking,IOMemory};
}