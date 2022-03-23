use rust_cheri_compressed_cap::Cc128Cap;
use thiserror::Error;

pub type ProcessorResult<T> = anyhow::Result<T>;

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
/// 
/// TODO move capability-related exceptions out into a CapabilityException?
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
    #[error("Address {addr:08x} dereferenced for {size}-byte data type, but out of bounds from capability {cap:?}")]
    AddressOobCapability { addr: usize, size: usize, cap: Cc128Cap },
    #[error("Capability permission violated: required permissions 0b{perms:b} not set in capability {cap:?}")]
    CapabilityPermission { perms: u32, cap: Cc128Cap },
    #[error("Tried to access memory through an invalid (tag=0) capability {cap:?}")]
    CapabilityInvalid { cap: Cc128Cap },
    #[error("Tried to access memory through a sealed capability {cap:?}")]
    CapabilitySealed { cap: Cc128Cap },
    #[error("Program returned a value = 0x{got:08X} (expected 0x{expected:08X}) = 0b{got:016b}")]
    ResultReturned{got: u32, expected: u32},
}