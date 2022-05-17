use std::ops::Range;

/// Function that replaces the bits of a value in a specific range with the bits at the bottom of a new value.
/// The Range is expected in Verilog-style, i.e. all-inclusive *unlike typical usages of Range*.
/// Panics if new_data has 1s outside of the range specified by `bits`
/// 
/// ```
/// # use rsim::processor::utils::replace_bits;
/// assert_eq!(
///     replace_bits(0, 0xf, 12..15),
///     0xf000
/// );
/// assert_eq!(
///     replace_bits(0xffff_ffff, 0b1011, 28..31),
///     0xbfff_ffff
/// );
/// ```
pub fn replace_bits(original: u128, new_data: u128, bits: Range<usize>) -> u128 {
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
/// # use rsim::processor::utils::extract_bits;
/// assert_eq!(
///     extract_bits(0xf000, 12..15),
///     0xf
/// );
/// assert_eq!(
///     extract_bits(0xbfff_ffff, 28..31),
///     0b1011
/// );
/// ```
pub fn extract_bits(original: u128, bits: Range<usize>) -> u128 {
    assert!(bits.end >= bits.start);
    let data_length_bits = bits.end - bits.start + 1;
    // Mask of (data_length_bits) 1s, starting at bit 0
    let data_length_mask = (u128::MAX << (128 - data_length_bits)) >> (128 - data_length_bits);
    
    // Shift down so the bits we want are at the bottom
    // Mask off all but the bits we want
    (original >> bits.start) & data_length_mask
}
