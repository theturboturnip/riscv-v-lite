/// TODO come up with a way to handle extensions elegantly
/// 
/// Want to have a good way to enable/disable CHERI, conditionally support CSRs, conditionally support Vectors etc.
/// Some sort of "decode stack" and "execution stack"?
/// "decode stack" = expand RV-C and decode full instructions, produce (Opcode, InstructionBits)
/// "execution stack" = ordered list of extensions that 

use crate::processor::exceptions::ProcessorResult;
use crate::processor::decode::{Opcode, InstructionBits};

pub trait IsaMod<TConn> {
    type Pc;

    /// Return true if this ISA module should handle the given instruction
    fn will_handle(&self, opcode: Opcode, inst: InstructionBits) -> bool;
    /// Execute the given instruction, returning the new PC (or None if continuing as normal)
    fn execute(&mut self, opcode: Opcode, inst: InstructionBits, inst_bits: u32, conn: TConn) -> ProcessorResult<Option<Self::Pc>>;
}

mod rv32im;
pub use rv32im::{Rv32im,Rv32imConn};
mod rv64im;
pub use rv64im::{Rv64im,Rv64imConn};
mod csrs;
pub use csrs::{Zicsr32, Zicsr32Conn, Zicsr64, Zicsr64Conn, CSRProvider};
mod vector;
pub use vector::{Rv32v,Rv32vConn,Rv64v,Rv64vCheriConn};
mod cheri;
pub use cheri::{XCheri64,XCheri64Conn};
