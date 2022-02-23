use rust_cheri_compressed_cap::Cc64Cap;
use thiserror::Error;

#[derive(Debug,Clone,PartialEq,Eq,Error)]
pub enum IllegalInstructionException {
    #[error("Unhandled Opcode {0:07b}")]
    UnknownOpcode(u8),
    /// Indended use: `UnsupportedParam(format!("Load funct3 {:03b}", funct3))`
    #[error("Unsupported parameter: {0}")]
    UnsupportedParam(String),
    #[error("Haven't implemented instruction {0} yet")]
    UnimplementedInstruction(&'static str),
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
    /// For when an address is dereferenced in integer mode, with respect to a default capability (DDC/PCC)
    #[error("Address {addr:08x} dereferenced, but out of bounds from default capability {cap:?}")]
    AddressOobDefaultCapability { addr: usize, cap: Cc64Cap },
    #[error("Capability permission violated: required permission 0b{perm:b} not set in capability {cap:?}")]
    CapabilityPermission { perm: u32, cap: Cc64Cap },
    #[error("Program returned a value = 0x{got:08X} (expected 0x{expected:08X}) = 0b{got:016b}")]
    ResultReturned{got: u32, expected: u32},
}