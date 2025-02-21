use super::{Hex128, SkipOrHex128, parse_hex::ParseHex};

impl Hex128<'_> {
    /// Convert the hex string into a 128-bit unsigned integer.
    pub fn u128(self) -> Option<u128> {
        let (invalid, value) = ParseHex::parse_hex(self.0);
        invalid.is_empty().then_some(value)
    }
}

impl SkipOrHex128<'_> {
    /// Convert the hex string into a 128-bit unsigned integer.
    pub fn u128(self) -> Option<Option<u128>> {
        if self.as_str() == "SKIP" {
            return Some(None);
        }
        let (invalid, value) = ParseHex::parse_hex(self.0);
        invalid.is_empty().then_some(Some(value))
    }
}
