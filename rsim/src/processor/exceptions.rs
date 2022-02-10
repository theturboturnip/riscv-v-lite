use thiserror::Error;

#[derive(Debug,Clone,PartialEq,Eq,Error)]
pub enum IllegalInstructionException {
    #[error("Unhandled Opcode {0:07b}")]
    UnknownOpcode(u8),
    #[error("Unsupported {kind} parameter {param} 0x{value:X}")]
    UnsupportedParam{kind: &'static str, param: &'static str, value: u32},
    #[error("Haven't implemented instruction {name} yet")]
    UnimplementedInstruction{name: &'static str},
    #[error("{0}")]
    MiscDecodeException(String)
}

/// Error which can be raised when accessing memory
/// 
/// Includes ResultReturned, which is raised when the program writes a u32 to 0xF000_0000
#[derive(Debug,Clone,PartialEq,Eq,Error)]
pub enum MemoryException {
    #[error("Address {addr:08x} misaligned, expected alignment on {expected}")]
    AddressMisaligned{ addr: usize, expected: usize },
    #[error("Jump target address {addr:08x} misaligned, expected alignment on {expected}")]
    JumpMisaligned{addr: usize, expected: usize},
    // #[error("Address {addr:08x} out-of-bounds, bounds = {max:08x}")]
    // AddressOOB{ addr: usize, max: usize },
    #[error("Address {addr:08x} not mapped")]
    AddressUnmapped{ addr: usize }, 

    #[error("Program returned a value = 0x{0:04X} (expected 0x3FFF) = 0b{0:016b}")]
    ResultReturned(u32),
}