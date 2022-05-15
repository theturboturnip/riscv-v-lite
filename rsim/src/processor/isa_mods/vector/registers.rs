use crate::processor::elements::cheri::SafeTaggedCap;
use std::ops::Range;
use super::types::*;
use std::convert::TryInto;
use anyhow::{Context,Result};

/// Returns (register idx, bit range) for an element of a given width `eew` 
/// in register `idx_from_base` in a register group starting at `vd_base`
fn bit_range_for_element(eew: Sew, vd_base: u8, idx_from_base: u32) -> Result<(u8, Range<usize>)> {
    let elem_width = (eew.width_in_bytes() * 8) as u32;

    // TODO refactor to use shifting
    let elems_per_v = (VLEN as u32)/elem_width;
    let vd: u8 = (vd_base as u32 + (idx_from_base / elems_per_v)).try_into()
        .context(format!("calculating destination register for vd_base={},idx_from_base={},eew={:?}", vd_base, idx_from_base, eew))?;
    let idx = idx_from_base % elems_per_v;

    Ok((
        vd,
        Range {
            start: (idx*elem_width) as usize,
            end: ((idx + 1)*elem_width - 1) as usize,
        }
    ))
}

/// Function that replaces the bits of a value in a specific range with the bits at the bottom of a new value.
/// The Range is expected in Verilog-style, i.e. all-inclusive *unlike typical usages of Range*.
/// Panics if new_data has 1s outside of the range specified by `bits`
/// 
/// ```
/// assert_eq!(
///     replace_bits(0, 0xf, 12..15),
///     0xf000
/// );
/// assert_eq!(
///     replace_bits(0xffff_ffff, 0b1011, 28..31),
///     0xbfff_ffff
/// );
/// ```
fn replace_bits(original: u128, new_data: u128, bits: Range<usize>) -> u128 {
    assert!(bits.end >= bits.start);
    let data_length_bits = bits.end - bits.start + 1;
    // Mask of (data_length_bits) 1s, starting at bit 0
    let data_length_mask = (u128::MAX << (128 - data_length_bits)) >> (128 - data_length_bits);
    assert_eq!(new_data, new_data & data_length_mask);
    // Mask applied to the original value to make a hole where the new_data is placed
    // All 1s except for the range defined by bits, where it is 0
    let original_mask = !(data_length_mask << bits.start);

    (original & original_mask) | (new_data & data_length_mask) << bits.start
}
/// Complementary function to [replace_bits]
/// 
/// Grabs the bits of `original` in range `bits`
/// Expects a Verilog-style i.e. all-inclusive bits range.
///```
/// assert_eq!(
///     extract_bits(0xf000, 12..15),
///     0xf
/// );
/// assert_eq!(
///     extract_bits(0xbfff_ffff, 28..31),
///     0b1011
/// );
/// ```
fn extract_bits(original: u128, bits: Range<usize>) -> u128 {
    assert!(bits.end >= bits.start);
    let data_length_bits = bits.end - bits.start + 1;
    // Mask of (data_length_bits) 1s, starting at bit 0
    let data_length_mask = (u128::MAX << (128 - data_length_bits)) >> (128 - data_length_bits);
    
    // Shift down so the bits we want are at the bottom
    // Mask off all but the bits we want
    (original >> bits.start) & data_length_mask
}

/// Trait for a vector register file where VLEN=128, ELEN=128.
/// Data is stored in TElem, which can be plain integers or a SafeTaggedCap (which implicitly adds 1-bit)
pub trait VectorRegisterFile<TElem> {
    /// Load a value from an element in a vertex register group, with specified EEW
    /// Requires the type of the value to store matches the EEW.
    /// 
    /// Example: if EEW=32bits, VLEN=128bits (4 32-bit elements per register), `vd` = 3, `idx` = 5,
    /// the actual `vd` = 3 + (idx_from_base / 4) = 4, and
    /// the actual `idx` = idx_from_base % 4 = 1.
    /// this would return v4\[64:32\] (element 1 of v4)
    fn load_vreg_elem(&self, eew: Sew, vd_base: u8, idx_from_base: u32) -> Result<TElem>;
    fn load_vreg_elem_int(&self, eew: Sew, vd_base: u8, idx_from_base: u32) -> Result<u128>;

    /// Store a value in an element in a vertex register group, with specified EEW.
    /// Requires the type of the value to store matches the EEW.
    /// 
    /// Example: if EEW=32bits, VLEN=128bits (4 32-bit elements per register), `vd_base` = 3, `idx_from_base` = 5,
    /// the actual `vd` = 3 + (idx_from_base / 4) = 4, and
    /// the actual `idx` = idx_from_base % 4 = 1.
    /// This would store `val` into v4\[64:32\] (element 1 of v4)
    fn store_vreg_elem(&mut self, eew: Sew, vd_base: u8, idx_from_base: u32, val: TElem) -> Result<()>;
    fn store_vreg_elem_int(&mut self, eew: Sew, vd_base: u8, idx_from_base: u32, val: u128) -> Result<()>;

    fn load_vreg(&mut self, vs: u8) -> Result<TElem>;
    fn load_vreg_int(&mut self, vs: u8) -> Result<u128>;
    fn store_vreg(&mut self, vs: u8, val: TElem) -> Result<()>;
    fn store_vreg_int(&mut self, vs: u8, val: u128) -> Result<()>;

    /// Returns true if the mask is enabled and element `i` has been masked *out*, e.g. that it should not be touched.
    fn seg_masked_out(&self, vm: bool, i: u32) -> bool;
    fn dump(&self);
    fn reset(&mut self);
}

pub struct IntVectorRegisterFile {
    vreg: [u128; 32]
}
impl VectorRegisterFile<u128> for IntVectorRegisterFile {
    fn load_vreg_elem(&self, eew: Sew, vd_base: u8, idx_from_base: u32) -> Result<u128> {
        let (vd, bits) = bit_range_for_element(eew, vd_base, idx_from_base)?;

        let full_reg = self.vreg[vd as usize];

        // Convert the element to the expected type and return
        Ok(extract_bits(full_reg, bits))
    }
    fn load_vreg_elem_int(&self, eew: Sew, vd_base: u8, idx_from_base: u32) -> Result<u128> {
        self.load_vreg_elem(eew, vd_base, idx_from_base)
    }

    fn store_vreg_elem(&mut self, eew: Sew, vd_base: u8, idx_from_base: u32, val: u128) -> Result<()> {
        let (vd, bits) = bit_range_for_element(eew, vd_base, idx_from_base)?;

        let full_reg = self.vreg[vd as usize];

        self.vreg[vd as usize] = replace_bits(full_reg, val, bits);

        Ok(())
    }
    fn store_vreg_elem_int(&mut self, eew: Sew, vd_base: u8, idx_from_base: u32, val: u128) -> Result<()> {
        self.store_vreg_elem(eew, vd_base, idx_from_base, val)
    }

    fn load_vreg(&mut self, vs: u8) -> Result<u128> {
        Ok(self.vreg[vs as usize])
    }
    fn load_vreg_int(&mut self, vd: u8) -> Result<u128> {
        Ok(self.vreg[vd as usize])
    }
    fn store_vreg(&mut self, vd: u8, val: u128) -> Result<()> {
        self.vreg[vd as usize] = val;
        Ok(())
    }
    fn store_vreg_int(&mut self, vd: u8, val: u128) -> Result<()> {
        self.vreg[vd as usize] = val;
        Ok(())
    }

    fn seg_masked_out(&self, vm: bool, i: u32) -> bool {
        let i = i as usize;
        // vm == 1 for mask disabled, 0 for mask enabled
        (!vm) && (bits!(self.vreg[0], i:i) == 0)
    }

    fn dump(&self) {
        for i in 0..32 {
            println!("v{} = 0x{:032x}", i, self.vreg[i]);
        }
    }

    fn reset(&mut self) {
        self.vreg = [0; 32];
    }
}
impl Default for IntVectorRegisterFile {
    fn default() -> Self {
        IntVectorRegisterFile {
            vreg: [0; 32]
        }
    }
}

pub struct CheriVectorRegisterFile {
    vreg: [SafeTaggedCap; 32]
}
impl VectorRegisterFile<SafeTaggedCap> for CheriVectorRegisterFile {
    fn load_vreg(&mut self, vs: u8) -> Result<SafeTaggedCap> {
        Ok(self.vreg[vs as usize])
    }
    fn load_vreg_int(&mut self, vs: u8) -> Result<u128> {
        Ok(self.vreg[vs as usize].to_integer())
    }
    fn store_vreg(&mut self, vd: u8, val: SafeTaggedCap) -> Result<()> {
        self.vreg[vd as usize] = val;
        Ok(())
    }
    fn store_vreg_int(&mut self, vd: u8, val: u128) -> Result<()> {
        self.vreg[vd as usize] = SafeTaggedCap::from_integer(val);
        Ok(())
    }

    fn load_vreg_elem(&self, eew: Sew, vd_base: u8, idx_from_base: u32) -> Result<SafeTaggedCap> {
        let (vd, bits) = bit_range_for_element(eew, vd_base, idx_from_base)?;

        match eew {
            Sew::e128 => {
                // Load the full register, which may be a capability
                // bits should take up the full range of the register
                assert_eq!(bits, 0..127);
                Ok(self.vreg[vd as usize])
            }
            _ => {
                let full_reg = self.vreg[vd as usize].to_integer();

                // Convert the element to the expected type and return
                Ok(SafeTaggedCap::from_integer(extract_bits(full_reg, bits)))
            }
        }
    }
    fn load_vreg_elem_int(&self, eew: Sew, vd_base: u8, idx_from_base: u32) -> Result<u128> {
        Ok(self.load_vreg_elem(eew, vd_base, idx_from_base)?.to_integer())
    }

    fn store_vreg_elem(&mut self, eew: Sew, vd_base: u8, idx_from_base: u32, val: SafeTaggedCap) -> Result<()> {
        let (vd, bits) = bit_range_for_element(eew, vd_base, idx_from_base)?;

        match eew {
            Sew::e128 => {
                // Store the full register, which may be a capability
                // bits should take up the full range of the register
                assert_eq!(bits, 0..127);
                self.vreg[vd as usize] = val;
            }
            _ => {
                let full_reg = self.vreg[vd as usize].to_integer();

                self.vreg[vd as usize] = SafeTaggedCap::from_integer(
                    replace_bits(
                        full_reg, val.to_integer(), bits
                    )
                );
            }
        };

        Ok(())
    }
    fn store_vreg_elem_int(&mut self, eew: Sew, vd_base: u8, idx_from_base: u32, val: u128) -> Result<()> {
        self.store_vreg_elem(eew, vd_base, idx_from_base, SafeTaggedCap::from_integer(val))
    }

    fn seg_masked_out(&self, vm: bool, i: u32) -> bool {
        let i = i as usize;
        // vm == 1 for mask disabled, 0 for mask enabled
        (!vm) && (bits!(self.vreg[0].to_integer(), i:i) == 0)
    }

    fn dump(&self) {
        for i in 0..32 {
            println!("v{} = 0x{:?}", i, self.vreg[i]);
        }
    }

    fn reset(&mut self) {
        self.vreg = [Default::default(); 32];
    }
}
impl Default for CheriVectorRegisterFile {
    fn default() -> Self {
        CheriVectorRegisterFile {
            vreg: [Default::default(); 32]
        }
    }
}
