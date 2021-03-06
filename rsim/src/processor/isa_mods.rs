//! All RISC-V ISA extensions implemented as objects implementing [IsaMod].

use crate::processor::decode::{InstructionBits, Opcode};
use crate::processor::elements::memory::Memory;
use crate::processor::elements::registers::RegisterFile;
use crate::processor::exceptions::ProcessorResult;
use num_traits::Num;

/// Trait for all ISA modules.
///
/// Uses a `TConn` value to connect to processor state: for example,
/// the [Rv64im] module takes a connection holding a reference to a 64-bit register file,
/// and a reference to main memory.
pub trait IsaMod<TConn> {
    type Pc;

    /// Return true if this ISA module should handle the given instruction
    fn will_handle(&self, opcode: Opcode, inst: InstructionBits) -> bool;
    /// Execute the given instruction, returning the new PC (or None if continuing as normal)
    fn execute(
        &mut self,
        opcode: Opcode,
        inst: InstructionBits,
        inst_bits: u32,
        conn: TConn,
    ) -> ProcessorResult<Option<Self::Pc>>;
}

/// Trait for possible XLEN values.
/// Used to make elements generic over (u32 or u64).
pub trait PossibleXlen:
    // Supports 0, 1, addition, multiplication, etc.
    Num
    // Can be printed
    + std::fmt::LowerHex
    // Trivially copyable
    + Copy
    // Can be used as a u64 for the memory subsystem
    + Into<u64>
    // Can generate one from the lowest common type - u32
    + From<u32>
    {}
impl PossibleXlen for u32 {}
impl PossibleXlen for u64 {}

/// Connection to a RISC-V I+M ISA module.
#[allow(non_camel_case_types)]
pub struct RvimConn<'a, uXLEN: PossibleXlen> {
    pub pc: uXLEN,
    pub sreg: &'a mut dyn RegisterFile<uXLEN>,
    pub memory: &'a mut dyn Memory,
}
pub type Rv32imConn<'a> = RvimConn<'a, u32>;
pub type Rv64imConn<'a> = RvimConn<'a, u64>;

mod rv32im;
pub use rv32im::Rv32im;
mod rv64im;
pub use rv64im::Rv64im;

mod csrs;
pub use csrs::{CSRProvider, Zicsr32, Zicsr32Conn, Zicsr64, Zicsr64Conn};
pub mod vector;
pub use vector::{CheriVectorRegisterFile, IntVectorRegisterFile, Rv32v, Rv64Cheriv, Rv64v};
mod cheri;
pub use cheri::{Rv64imCapabilityMode, XCheri64, XCheri64Conn};
