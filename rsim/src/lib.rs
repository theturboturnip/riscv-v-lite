#[macro_use]
extern crate bitutils;

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate static_assertions;

pub mod processor;

pub use processor::{Processor,Memory};