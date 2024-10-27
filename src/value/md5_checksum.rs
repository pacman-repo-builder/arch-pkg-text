use super::{hex::ParseHex, Md5Checksum};

impl<'a> Md5Checksum<'a> {
    /// Convert the hex string into a 128-bit unsigned integer.
    pub fn u128(self) -> Option<u128> {
        let (invalid, value) = ParseHex::parse_hex(self.0);
        invalid.is_empty().then_some(value)
    }
}
