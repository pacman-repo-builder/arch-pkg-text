use super::{hex::ParseHex, Hex128, SkipOrHex128};

impl<'a> Hex128<'a> {
    /// Convert the hex string into a 128-bit unsigned integer.
    pub fn u128(self) -> Option<u128> {
        let (invalid, value) = ParseHex::parse_hex(self.0);
        invalid.is_empty().then_some(value)
    }
}

impl<'a> SkipOrHex128<'a> {
    /// Convert the hex string into a 128-bit unsigned integer.
    pub fn u128(self) -> Option<Option<u128>> {
        if self.as_str() == "SKIP" {
            return Some(None);
        }
        let (invalid, value) = ParseHex::parse_hex(self.0);
        invalid.is_empty().then_some(Some(value))
    }
}
