use super::types::*;
use crate::processor::elements::cheri::SafeTaggedCap;
use crate::processor::utils::{extract_bits, replace_bits};
use anyhow::{Context, Result};
use std::convert::TryInto;
use std::ops::Range;

/// Returns (register idx, bit range) for an element of a given width `eew`
/// in register `idx_from_base` in a register group starting at `vd_base`
fn bit_range_for_element(eew: Sew, vd_base: u8, idx_from_base: u32) -> Result<(u8, Range<usize>)> {
    let elem_width = (eew.width_in_bytes() * 8) as u32;

    // TODO refactor to use shifting
    let elems_per_v = (VLEN as u32) / elem_width;
    let vd: u8 = (vd_base as u32 + (idx_from_base / elems_per_v))
        .try_into()
        .context(format!(
            "calculating destination register for vd_base={},idx_from_base={},eew={:?}",
            vd_base, idx_from_base, eew
        ))?;
    let idx = idx_from_base % elems_per_v;

    Ok((
        vd,
        Range {
            start: (idx * elem_width) as usize,
            end: ((idx + 1) * elem_width - 1) as usize,
        },
    ))
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
    fn store_vreg_elem(
        &mut self,
        eew: Sew,
        vd_base: u8,
        idx_from_base: u32,
        val: TElem,
    ) -> Result<()>;
    fn store_vreg_elem_int(
        &mut self,
        eew: Sew,
        vd_base: u8,
        idx_from_base: u32,
        val: u128,
    ) -> Result<()>;

    fn load_vreg(&mut self, vs: u8) -> Result<TElem>;
    fn load_vreg_int(&mut self, vs: u8) -> Result<u128>;
    fn store_vreg(&mut self, vs: u8, val: TElem) -> Result<()>;
    fn store_vreg_int(&mut self, vs: u8, val: u128) -> Result<()>;

    /// Returns true if the mask is enabled and element `i` has been masked *out*, e.g. that it should not be touched.
    fn seg_masked_out(&self, vm: bool, i: u32) -> bool;
    fn dump(&self);
    fn reset(&mut self);
}

/// Register file which holds 128-bit integer vectors.
pub struct IntVectorRegisterFile {
    vreg: [u128; 32],
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

    fn store_vreg_elem(
        &mut self,
        eew: Sew,
        vd_base: u8,
        idx_from_base: u32,
        val: u128,
    ) -> Result<()> {
        let (vd, bits) = bit_range_for_element(eew, vd_base, idx_from_base)?;

        let full_reg = self.vreg[vd as usize];

        self.vreg[vd as usize] = replace_bits(full_reg, val, bits);

        Ok(())
    }
    fn store_vreg_elem_int(
        &mut self,
        eew: Sew,
        vd_base: u8,
        idx_from_base: u32,
        val: u128,
    ) -> Result<()> {
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
        (!vm) && (bits!(self.vreg[0], i: i) == 0)
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
        IntVectorRegisterFile { vreg: [0; 32] }
    }
}

/// Register file which holds 128-bit integer vectors OR one tagged capability per vector.
pub struct CheriVectorRegisterFile {
    vreg: [SafeTaggedCap; 32],
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
        Ok(self
            .load_vreg_elem(eew, vd_base, idx_from_base)?
            .to_integer())
    }

    fn store_vreg_elem(
        &mut self,
        eew: Sew,
        vd_base: u8,
        idx_from_base: u32,
        val: SafeTaggedCap,
    ) -> Result<()> {
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

                self.vreg[vd as usize] =
                    SafeTaggedCap::from_integer(replace_bits(full_reg, val.to_integer(), bits));
            }
        };

        Ok(())
    }
    fn store_vreg_elem_int(
        &mut self,
        eew: Sew,
        vd_base: u8,
        idx_from_base: u32,
        val: u128,
    ) -> Result<()> {
        self.store_vreg_elem(
            eew,
            vd_base,
            idx_from_base,
            SafeTaggedCap::from_integer(val),
        )
    }

    fn seg_masked_out(&self, vm: bool, i: u32) -> bool {
        let i = i as usize;
        // vm == 1 for mask disabled, 0 for mask enabled
        (!vm) && (bits!(self.vreg[0].to_integer(), i: i) == 0)
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
            vreg: [Default::default(); 32],
        }
    }
}
