use super::{Hex128, SkipOrHex128, parse_hex::ParseHex};

impl<Text: AsRef<str>> Hex128<Text> {
    /// Convert the hex string into a 128-bit unsigned integer.
    pub fn u128(self) -> Option<u128> {
        let (invalid, value) = ParseHex::parse_hex(self.0.as_ref());
        invalid.is_empty().then_some(value)
    }
}

impl<Text: AsRef<str>> SkipOrHex128<Text> {
    /// Convert the hex string into a 128-bit unsigned integer.
    pub fn u128(self) -> Option<Option<u128>> {
        if self.0.as_ref() == "SKIP" {
            return Some(None);
        }
        let (invalid, value) = ParseHex::parse_hex(self.0.as_ref());
        invalid.is_empty().then_some(Some(value))
    }
}
