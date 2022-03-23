pub use rust_cheri_compressed_cap::{CompressedCapability, Cc128, Cc128Cap};

/// Enumeration that stores either raw data or a valid capability.
/// The [Cc128Cap] struct has its own tag bit - this will always be True *as long as [SafeTaggedCap::ValidCap] is not created manually*.
#[derive(Copy,Clone,PartialEq,Eq)]
pub enum SafeTaggedCap {
    RawData{ top: u64, bot: u64 },
    ValidCap(Cc128Cap)
}
impl SafeTaggedCap {
    pub fn from_tagged_data(top: u64, bot: u64, tag: bool) -> Self {
        if tag {
            let pebst = top;
            let addr = bot;
            SafeTaggedCap::ValidCap(Cc128::decompress_mem(pebst, addr, tag))
        } else {
            SafeTaggedCap::RawData{ top, bot }
        }
    }
}

