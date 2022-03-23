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
mod rv64i;
pub use rv64i::{Rv64i,Rv64iConn};
mod csrs;
pub use csrs::{Zicsr32, Zicsr32Conn, Zicsr64, Zicsr64Conn, CSRProvider};
mod vector32;
pub use vector32::{Rv32v,Rv32vConn};
mod cheri;
pub use cheri::{XCheri64,XCheri64Conn};
