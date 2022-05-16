//! Library for emulating various RISC-V processors.
//! Supports 32 or 64-bit ISAs, "i", "m", "Zcsr", "v", "xcheri" extensions.
//! 
//! If you're coming from my MPhil project, you probably want to see the [vector documentation](processor::isa_mods::vector)

#[macro_use]
extern crate bitutils;

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate static_assertions;

/// Main module with all processor elements, ISA modules, and models.
pub mod processor;

/// Different processor models for 32-bit, 64-bit, and 64-bit + CHERI.
pub use processor::models;

// Expose this publically for the sake of documentation
/// ISA modules for Integer, Multiply, Vector, CSR, CHERI extensions
pub use processor::isa_mods;

/// Publically exposed memory-related structures.
/// Includes capability and integer-addressed memory.
pub mod memory {
    pub use crate::processor::elements::memory::{AggregateMemory,MemoryBacking,IOMemory};
    pub use crate::processor::elements::cheri::{CheriAggregateMemory};
}

/// Publically exposed CHERI-related structures.
/// All pulled from [rust_cheri_compressed_cap].
pub use crate::processor::elements::cheri::{Cc128,Cc128Cap,CheriRVFuncs,CompressedCapability};