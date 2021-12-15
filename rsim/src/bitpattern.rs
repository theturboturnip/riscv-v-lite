pub struct BitPattern {
    expected: u64,
    mask: u64,
}

impl BitPattern {
    /// Accepts a bit pattern as a string literal.
    /// - '0' matches a 0 bit
    /// - '1' matches a 1 bit
    /// - any other char means "ignore this bit"
    pub const fn new(s: &str) -> Self {
        let mut expected = 0;
        let mut mask = !0;

        let mut cur_bit = 1 << (s.len() - 1);

        let mut i = 0;
        while i < s.len() {
            let val: u8 = s.as_bytes()[i];
            i += 1;

            if val == b'1' {
                expected |= cur_bit;
            } else if val == b'0' {
                // do nothing
            } else {
                mask &= !cur_bit;
            }

            // TODO: With panic!() in const functions we could restrict
            // this to a more limited subset of characters.

            cur_bit >>= 1;
        }

        Self { expected, mask }
    }
    pub const fn matches(&self, val: u64) -> bool {
        (val & self.mask) == self.expected
    }
}