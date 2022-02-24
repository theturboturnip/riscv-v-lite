/// TODO come up with a way to handle extensions elegantly
/// 
/// Want to have a good way to enable/disable CHERI, conditionally support CSRs, conditionally support Vectors etc.
/// Some sort of "decode stack" and "execution stack"?
/// "decode stack" = expand RV-C and decode full instructions, produce (Opcode, InstructionBits)
/// "execution stack" = ordered list of extensions that 

use crate::processor::exceptions::ProcessorResult;
use crate::processor::decode::{Opcode, InstructionBits};

pub trait IsaModConn: Sized {}
pub trait IsaMod<TConn: IsaModConn> {
    type Pc;

    /// Return true if this ISA module should handle the given instruction
    fn will_handle(&self, opcode: Opcode, inst: InstructionBits) -> bool;
    /// Execute the given instruction, returning the new PC (or None if continuing as normal)
    fn execute(&mut self, opcode: Opcode, inst: InstructionBits, inst_bits: u32, conn: TConn) -> ProcessorResult<Option<Self::Pc>>;
}

mod rv32i;
pub use rv32i::{Rv32i,Rv32iConn};
mod csrs;
pub use csrs::{Zicsr, ZicsrConn, CSRProvider};
mod vector;
pub use vector::{Rvv,RvvConn};
// mod cheri;
// pub use cheri::{CheriRv32i,CheriRv32iConn};

// struct CheriRV32 {
//     // TODO Special Capability Registers - How to handle PCC?

// }
// struct CheriRV32Connection {
//     sreg: &mut CheriRV32RegisterFile,
//     memory: &mut CheriMemory,
// }
// impl ProcessorExtention for CheriRV32Connection {

// }