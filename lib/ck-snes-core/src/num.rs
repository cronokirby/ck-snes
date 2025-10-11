/// A 24 bit unsigned integer.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct U24(u32);

impl U24 {
    /// 16 + 8 -> 24.
    pub fn from_lo_hi(lo: u16, hi: u8) -> Self {
        Self(lo as u32 | ((hi as u32) << 16))
    }

    /// 24 -> 16 + 8.
    pub fn into_lo_hi(self) -> (u16, u8) {
        (self.0 as u16, (self.0 >> 16) as u8)
    }
}

impl From<(u16, u8)> for U24 {
    fn from((lo, hi): (u16, u8)) -> Self {
        Self::from_lo_hi(lo, hi)
    }
}

impl From<U24> for (u16, u8) {
    fn from(x: U24) -> Self {
        x.into_lo_hi()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_from_lo_hi_roundtrip(lo: u16, hi: u8) {
            assert_eq!((lo, hi), U24::from((lo, hi)).into());
        }
    }
}
