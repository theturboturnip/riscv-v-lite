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
    #[error("Address {addr:08x} not mapped")]
    AddressUnmapped{ addr: usize },
    #[error("Program returned a value = 0x{got:08X} (expected 0x{expected:08X}) = 0b{got:016b}")]
    ResultReturned{got: u32, expected: u32},
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum CapOrRegister {
    Cap(Cc128Cap),
    Reg(u8)
}

/// Enum for capability-related exceptions.
/// Modelled after [https://github.com/CTSRD-CHERI/sail-cheri-riscv/blob/master/src/cheri_types.sail].
/// Contains commented-out variants that are currently unused.
#[derive(Debug,Clone,PartialEq,Eq,Error)]
pub enum CapabilityException {
    #[error("Expected cap {cap:?} address to be in-bounds for a {size}-byte data type")]
    BoundsViolation{ cap: CapOrRegister, size: usize },
    #[error("Expected cap {cap:?} to be tagged")]
    TagViolation{ cap: CapOrRegister },
    #[error("Expected cap {cap:?} to be unsealed")]
    SealViolation{ cap: CapOrRegister },
    // TypeViolation{ cap: CapOrRegister },
    // CallTrap{ cap: CapOrRegister },
    // ReturnTrap{ cap: CapOrRegister },
    // TSSUnderFlow{ cap: CapOrRegister },
    // UserDefViolation{ cap: CapOrRegister },
    // InexactBounds{ cap: CapOrRegister },
    // UnalignedBase{ cap: CapOrRegister },
    // GlobalViolation{ cap: CapOrRegister },
    #[error("Expected cap {cap:?} to have permissions {perms:b}")]
    PermissionViolation{ cap: CapOrRegister, perms: u32 }
    // AccessSystemRegsViolation{ cap: CapOrRegister },
    // PermitCInvokeViolation{ cap: CapOrRegister },
    // AccessCInvokeIDCViolation{ cap: CapOrRegister },
    // PermitUnsealViolation{ cap: CapOrRegister },
    // PermitSetCIDViolation{ cap: CapOrRegister }
}