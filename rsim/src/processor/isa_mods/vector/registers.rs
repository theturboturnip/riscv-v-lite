use super::types::*;
use std::convert::TryInto;
use anyhow::{Context,Result};

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
    fn store_vreg(&mut self, vs: u8, val: TElem) -> Result<()>;
    fn store_vreg_int(&mut self, vs: u8, val: u128) -> Result<()>;

    /// Returns true if the mask is enabled and element `i` has been masked *out*, e.g. that it should not be touched.
    fn seg_masked_out(&self, vm: bool, i: usize) -> bool;
    fn dump(&self);
    fn reset(&mut self);
}

pub struct IntVectorRegisterFile {
    vreg: [u128; 32]
}
impl VectorRegisterFile<u128> for IntVectorRegisterFile {
    fn load_vreg_elem(&self, eew: Sew, vd_base: u8, idx_from_base: u32) -> Result<u128> {
        let (elem_width_mask, elem_width) : (u128, u32) = match eew {
            Sew::e8  => (0xFF, 8),
            Sew::e16 => (0xFFFF, 16),
            Sew::e32 => (0xFFFF_FFFF, 32),
            Sew::e64 => (0xFFFF_FFFF_FFFF_FFFF, 64),
            Sew::e128 => (0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF, 128),
        };

        // TODO refactor to use shifting
        let elems_per_v: u32 = (VLEN as u32)/elem_width;
        let vd: u8 = (vd_base as u32 + (idx_from_base / elems_per_v)).try_into()
            .context(format!("calculating destination register for vd_base={},idx_from_base={},eew={:?}", vd_base, idx_from_base, eew))?;
        let idx = idx_from_base % elems_per_v;

        let full_reg = self.vreg[vd as usize];
        // Shift the register down so the new element is at the bottom,
        // and mask off the other elements
        let individual_elem = (full_reg >> (elem_width*idx)) & (elem_width_mask as uVLEN);

        // Convert the element to the expected type and return
        Ok(individual_elem)
    }
    fn load_vreg_elem_int(&self, eew: Sew, vd_base: u8, idx_from_base: u32) -> Result<u128> {
        self.load_vreg_elem(eew, vd_base, idx_from_base)
    }

    fn store_vreg_elem(&mut self, eew: Sew, vd_base: u8, idx_from_base: u32, val: u128) -> Result<()> {
        let (elem_width_mask, elem_width) : (u128, u32) = match eew {
            Sew::e8  => (0xFF, 8),
            Sew::e16 => (0xFFFF, 16),
            Sew::e32 => (0xFFFF_FFFF, 32),
            Sew::e64 => (0xFFFF_FFFF_FFFF_FFFF, 64),
            Sew::e128 => (0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF, 128),
        };
        // Assert the value doesn't have more data
        assert_eq!(val & (!elem_width_mask), 0);

        // TODO refactor to use shifting
        let elems_per_v: u32 = (VLEN as u32)/elem_width;
        let vd: u8 = (vd_base as u32 + (idx_from_base / elems_per_v)).try_into()
            .context(format!("calculating destination register for vd_base={},idx_from_base={},eew={:?}", vd_base, idx_from_base, eew))?;
        let idx = idx_from_base % elems_per_v;

        // Get the previous value for the vector
        let old_value = self.vreg[vd as usize];
        // Mask off the element we want to write
        let mask = (elem_width_mask as uVLEN) << (elem_width*idx);
        let old_value_with_element_removed = old_value & (!mask);
        // Create a uVLEN value with just the new element, shifted into the right place
        let new_element_shifted = (val as uVLEN) << (elem_width*idx);
        // Combine (old value sans element) with (new element)
        let new_value = old_value_with_element_removed | new_element_shifted;

        self.vreg[vd as usize] = new_value;

        Ok(())
    }
    fn store_vreg_elem_int(&mut self, eew: Sew, vd_base: u8, idx_from_base: u32, val: u128) -> Result<()> {
        self.store_vreg_elem(eew, vd_base, idx_from_base, val)
    }

    fn load_vreg(&mut self, vs: u8) -> Result<u128> {
        Ok(self.vreg[vs as usize])
    }
    fn store_vreg(&mut self, vd: u8, val: u128) -> Result<()> {
        self.vreg[vd as usize] = val;
        Ok(())
    }
    fn store_vreg_int(&mut self, vd: u8, val: u128) -> Result<()> {
        self.vreg[vd as usize] = val;
        Ok(())
    }

    fn seg_masked_out(&self, vm: bool, i: usize) -> bool {
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

// pub struct CheriVectorRegisterFile {
//     vreg: [SafeTaggedCap; 32]
// }
// impl VectorRegisterFile<SafeTaggedCap> for CheriVectorRegisterFile {}
// impl Default for CheriVectorRegisterFile {}
