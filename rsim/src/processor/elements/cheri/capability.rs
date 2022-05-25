use crate::processor::exceptions::{CapOrRegister, CapabilityException};
use anyhow::Result;
pub use rust_cheri_compressed_cap::{
    wrappers::CheriRVFuncs, Cc128, Cc128Cap, CompressedCapability,
};
use std::convert::TryInto;
use std::ops::Range;

/// Enumeration that stores either raw data or a valid capability.
/// The capability inside ValidCap(Cc128Cap) will always have its tag bit = True *as long as [SafeTaggedCap::ValidCap] is not created manually*.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SafeTaggedCap {
    RawData { top: u64, bot: u64 },
    ValidCap(Cc128Cap),
}
impl SafeTaggedCap {
    pub fn from_integer(data: u128) -> Self {
        SafeTaggedCap::RawData {
            top: (data >> 64) as u64,
            bot: data as u64,
        }
    }
    pub fn to_integer(&self) -> u128 {
        let (top, bot) = match *self {
            SafeTaggedCap::RawData { top, bot } => (top, bot),
            SafeTaggedCap::ValidCap(cap) => {
                let pebst = Cc128::compress_raw(&cap);
                let cursor = cap.address();

                (pebst, cursor)
            }
        };

        ((top as u128) << 64) | (bot as u128)
    }

    pub fn from_tagged_mem(top: u64, bot: u64, tag: bool) -> Self {
        if tag {
            let pebst = top;
            let addr = bot;
            SafeTaggedCap::ValidCap(Cc128::decompress_mem(pebst, addr, tag))
        } else {
            SafeTaggedCap::RawData { top, bot }
        }
    }
    /// Converts a capability into a SafeTaggedCap.
    /// If the capability is valid (has tag bit set), returns ValidCap
    /// If the capability is invalid, returns RawData containing the compressed-raw representation
    pub fn from_cap(cap: Cc128Cap) -> Self {
        if cap.tag() {
            SafeTaggedCap::ValidCap(cap)
        } else {
            let pebst = Cc128::compress_raw(&cap);
            SafeTaggedCap::RawData {
                top: pebst,
                bot: cap.address(),
            }
        }
    }
    /// Converts a SafeTaggedCap to a Cc128Cap.
    /// If ValidCap, just returns the cap.
    /// If RawData, decompresses using raw representation and tag=false.
    pub fn to_cap(&self) -> Cc128Cap {
        match self {
            SafeTaggedCap::ValidCap(cap) => *cap,
            SafeTaggedCap::RawData { top, bot } => Cc128::decompress_raw(*top, *bot, false),
        }
    }

    /// Converts a SafeTaggedCap to a Cc128Cap with tag bit = true.
    /// Panics if the SafeTaggedCap is not a ValidCap.
    pub fn unwrap_cap(&self) -> Cc128Cap {
        match self {
            SafeTaggedCap::ValidCap(cap) => *cap,
            SafeTaggedCap::RawData { .. } => panic!("unwrap_cap called on RawData"),
        }
    }
}
impl Default for SafeTaggedCap {
    fn default() -> Self {
        SafeTaggedCap::RawData { top: 0, bot: 0 }
    }
}

/// Return the range of addresses you can access with a capability
pub fn cap_bounds_range(cap: Cc128Cap) -> Range<u64> {
    let b = cap.bounds();
    Range {
        start: b.0,
        end: b.1.try_into().unwrap(),
    }
}
/// Checks that a capability allows access to a `TData`-sized object at the current cursor
pub fn check_capability<TData>(cap: Cc128Cap, expected_perms: u32) -> Result<()> {
    check_obj_bounds_against_capability::<TData>(cap.address(), cap, expected_perms)
}
/// Checks that a capability allows access to a `TData`-sized object at the given `addr`
pub fn check_obj_bounds_against_capability<TData>(
    addr: u64,
    cap: Cc128Cap,
    expected_perms: u32,
) -> Result<()> {
    let size = std::mem::size_of::<TData>() as u64;
    check_bounds_against_capability(
        Range {
            start: addr,
            end: addr + size,
        },
        cap,
        expected_perms,
    )
}
/// Checks that a capability allows access to a given range of byte addresses
pub fn check_bounds_against_capability(
    bounds: Range<u64>,
    cap: Cc128Cap,
    expected_perms: u32,
) -> Result<()> {
    if !cap.tag() {
        bail!(CapabilityException::TagViolation {
            cap: CapOrRegister::Cap(cap)
        })
    } else if cap.permissions() & expected_perms != expected_perms {
        bail!(CapabilityException::PermissionViolation {
            cap: CapOrRegister::Cap(cap),
            perms: expected_perms
        })
    } else if !cap_bounds_range(cap).contains(&bounds.start)
        || !cap_bounds_range(cap).contains(&(bounds.end - 1))
    {
        // ^ check end - 1 because bounds is an exclusive range - we're only going to access the byte at (end - 1)
        bail!(CapabilityException::BoundsViolation {
            cap: CapOrRegister::Cap(cap),
            size: (bounds.end - bounds.start) as usize
        })
    } else if cap.is_sealed() {
        bail!(CapabilityException::SealViolation {
            cap: CapOrRegister::Cap(cap)
        })
    } else {
        Ok(())
    }
}
