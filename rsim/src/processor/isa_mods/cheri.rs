use crate::processor::isa_mods::*;
use crate::processor::elements::cheri::{Cc128Cap,SafeTaggedCap,CheriAggregateMemory};
use crate::processor::elements::cheri::CheriRV64RegisterFile;

pub struct XCheri64Conn<'a> {
    pub pc: Cc128Cap,
    pub sreg: &'a mut CheriRV64RegisterFile,
    pub memory: &'a mut CheriAggregateMemory,
}
impl<'a> IsaModConn for XCheri64Conn<'a> {}

pub struct XCheri64 {}
impl IsaMod<XCheri64Conn<'_>> for XCheri64 {
    type Pc = Cc128Cap;

    fn will_handle(&self, opcode: Opcode, _inst: InstructionBits) -> bool {
        use crate::processor::decode::Opcode::*;
        match opcode {
            _ => false
        }
    }

    fn execute(&mut self, opcode: Opcode, inst: InstructionBits, _inst_bits: u32, conn: XCheri64Conn) -> ProcessorResult<Option<Self::Pc>> {
        Ok(None)
    }
}
