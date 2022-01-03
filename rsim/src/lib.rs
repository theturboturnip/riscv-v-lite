#[macro_use]
extern crate bitutils;

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate static_assertions;

pub mod memory;
pub mod processor;

pub use memory::Memory;
pub use processor::Processor;