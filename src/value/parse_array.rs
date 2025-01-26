/// Types that implement this trait can parse its content into an array.
pub trait ParseArray {
    /// Type of array when parsing succeeds.
    type Array;
    /// Type of error when parsing fails.
    type Error;
    /// Parse an array.
    fn parse_array(&self) -> Result<Self::Array, Self::Error>;
}
