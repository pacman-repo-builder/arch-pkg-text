/// Parse result of `SkipOrHex*::u8_array` methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkipOrArray<const LEN: usize> {
    /// The source string was `"SKIP"`.
    Skip,
    /// The source string was a valid hexadecimal string.
    Array([u8; LEN]),
}

impl<const LEN: usize> SkipOrArray<LEN> {
    /// Returns `true` if the value is [`Skip`](SkipOrArray::Skip).
    #[must_use]
    pub fn is_skip(&self) -> bool {
        matches!(self, Self::Skip)
    }

    /// Returns `true` if the value is [`Array`](SkipOrArray::Array).
    #[must_use]
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    /// Try extracting an array of `u8`.
    pub fn try_into_array(self) -> Option<[u8; LEN]> {
        match self {
            SkipOrArray::Array(array) => Some(array),
            SkipOrArray::Skip => None,
        }
    }

    /// Try getting a reference to the underlying array of `u8`.
    pub fn as_array(&self) -> Option<&[u8; LEN]> {
        match self {
            SkipOrArray::Array(array) => Some(array),
            SkipOrArray::Skip => None,
        }
    }

    /// Try getting a slice of `[u8]`.
    pub fn as_slice(&self) -> Option<&[u8]> {
        self.as_array().map(|x| x.as_slice())
    }
}

impl<const LEN: usize> TryFrom<SkipOrArray<LEN>> for [u8; LEN] {
    type Error = ();
    fn try_from(value: SkipOrArray<LEN>) -> Result<Self, Self::Error> {
        value.try_into_array().ok_or(())
    }
}
