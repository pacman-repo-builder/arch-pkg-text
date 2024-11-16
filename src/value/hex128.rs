use super::{hex::ParseHex, Hex128};

impl<'a> Hex128<'a> {
    /// Convert the hex string into a 128-bit unsigned integer.
    pub fn u128(self) -> Option<u128> {
        let (invalid, value) = ParseHex::parse_hex(self.0);
        invalid.is_empty().then_some(value)
    }
}
