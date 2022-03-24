pub use rust_cheri_compressed_cap::{CompressedCapability, Cc128, Cc128Cap, wrappers::CheriRVFuncs};

/// Enumeration that stores either raw data or a valid capability.
/// The [Cc128Cap] struct has its own tag bit - this will always be True *as long as [SafeTaggedCap::ValidCap] is not created manually*.
#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum SafeTaggedCap {
    RawData{ top: u64, bot: u64 },
    ValidCap(Cc128Cap)
}
impl SafeTaggedCap {
    pub fn from_tagged_mem(top: u64, bot: u64, tag: bool) -> Self {
        if tag {
            let pebst = top;
            let addr = bot;
            SafeTaggedCap::ValidCap(Cc128::decompress_mem(pebst, addr, tag))
        } else {
            SafeTaggedCap::RawData{ top, bot }
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
            SafeTaggedCap::RawData{ top: pebst, bot: cap.address() }
        }
    }
    /// Converts a SafeTaggedCap to a Cc128Cap.
    /// If ValidCap, just returns the cap.
    /// If RawData, decompresses using raw representation and tag=false.
    pub fn to_cap(&self) -> Cc128Cap {
        match self {
            SafeTaggedCap::ValidCap(cap) => *cap,
            SafeTaggedCap::RawData{ top, bot } => Cc128::decompress_raw(*top, *bot, false)
        }
    }
}

