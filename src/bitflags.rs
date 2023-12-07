//! Bitflags for use with Brogue Scanner.

/// Holds 16-bit bitflags.  Bits are indexed just like those of vectors (starting at 0).
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct BitFlags16(pub u16);

impl BitFlags16 {
    /// Returns new (empty) instance.
    pub fn new() -> Self {
        Self::empty()
    }
    /// Returns empty `BitFlags16` (with value of 0).
    #[inline]
    pub fn empty() -> Self {
        Self(0)
    } 
    /// Returns new instance using specified index.  Only 16 indexes allowed (0-15).
    #[inline]
    pub fn from_index(index: usize) -> Self {
        assert!(index < 16, "up to 16 unique flags allowed for BitFlags16");
        Self(2_u16.pow(index as u32))
    }            
    /// Returns true if current flags contain _at least one_ of the incoming flags.
    #[inline]
    pub fn intersects(&self, other: Self) -> bool {
        (self.0 & other.0) > 0
    }
    /// Inserts flags into current `BitFlags16` (bitwise OR).
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 = self.0 | other.0;
    }
}

impl std::fmt::Display for BitFlags16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Written with LSB on the left.  UTF-8 (48) is '0'; UTF-8 (49) is '1'
        let mut bits = self.0;
        let mut bit_ix = 0;
        
        let mut bytes: [u8; 16] = [48; 16];

        while bits != 0 {
            if bits & 1 == 1 {
                bits >>= 1;
                bytes[bit_ix] = 49;
                bit_ix += 1;
                continue;
            }
            bits >>= 1;
            bit_ix += 1;
        }

        // Note: this will not fail, so `unwrap()` is fine
        write!(f, "BitFlags[{}]", std::str::from_utf8(&bytes).unwrap())
    }
}
