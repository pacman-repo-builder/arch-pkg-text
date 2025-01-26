use crate::{
    srcinfo::field::FieldName,
    value::{self, ParseArray, SkipOrArray},
};
use pipe_trait::Pipe;

macro_rules! def_enum {
    ($(
        $checksum_variant:ident($checksum_content:ident, $size:literal) = $field_variant:ident,
    )*) => {
        /// Type of checksum.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum ChecksumType {$(
            $checksum_variant,
        )*}

        impl ChecksumType {
            /// All recognized checksum types.
            pub const TYPES: &[Self] = &[$(ChecksumType::$checksum_variant,)*];

            /// All recognized checksum types.
            pub fn all_types() -> impl Iterator<Item = &'static Self> {
                ChecksumType::TYPES.iter()
            }

            /// Convert a [`ChecksumType`] into a corresponding [`FieldName`].
            pub const fn into_field_name(self) -> FieldName {
                match self {$(
                    ChecksumType::$checksum_variant => FieldName::$field_variant,
                )*}
            }

            /// Try converting a [`FieldName`] into a corresponding [`ChecksumType`].
            pub const fn try_from_field_name(field_name: FieldName) -> Option<Self> {
                match field_name {
                    $(FieldName::$field_variant => Some(ChecksumType::$checksum_variant),)*
                    _ => None,
                }
            }
        }

        /// Value of checksum.
        #[derive(Debug, Clone, Copy)]
        pub enum ChecksumValue<'a> {$(
            $checksum_variant(value::$checksum_content<'a>),
        )*}

        impl<'a> ChecksumValue<'a> {
            /// Create a [`ChecksumValue`] from a [`ChecksumType`] and a raw value.
            pub fn new(checksum_type: ChecksumType, raw_value: &'a str) -> Self {
                match checksum_type {$(
                    ChecksumType::$checksum_variant => {
                        raw_value.pipe(value::$checksum_content).pipe(ChecksumValue::$checksum_variant)
                    }
                )*}
            }

            /// Attempt to create a [`ChecksumValue`] from a [`FieldName`] and a raw value.
            pub(super) fn try_from_field_name(field_name: FieldName, raw_value: &'a str) -> Option<Self> {
                let checksum_type = ChecksumType::try_from_field_name(field_name)?;
                Some(ChecksumValue::new(checksum_type, raw_value))
            }
        }

        /// [Array type](ParseArray::Array) of [`ChecksumValue`].
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum ChecksumArray {
            Skip,
            $($checksum_variant([u8; $size]),)*
        }

        impl<'a> ChecksumValue<'a> {
            /// Convert the hex string into an array of 8-bit unsigned integers.
            pub fn u8_array(self) -> Option<ChecksumArray> {
                Some(match self {$(
                    ChecksumValue::$checksum_variant(hex_string) => match hex_string.u8_array()? {
                        SkipOrArray::Skip => ChecksumArray::Skip,
                        SkipOrArray::Array(array) => ChecksumArray::$checksum_variant(array),
                    }
                )*})
            }
        }

        impl ChecksumArray {
            /// Get a slice of the internal array if it wasn't [`ChecksumArray::Skip`].
            pub fn try_as_slice(&self) -> Option<&'_ [u8]> {
                match self {
                    ChecksumArray::Skip => None,
                    $(ChecksumArray::$checksum_variant(array) => Some(array),)*
                }
            }
        }
    };
}

def_enum! {
    Md5(SkipOrHex128, 16) = Md5Checksums,
    Sha1(SkipOrHex160, 20) = Sha1Checksums,
    Sha224(SkipOrHex224, 28) = Sha224Checksums,
    Sha256(SkipOrHex256, 32) = Sha256Checksums,
    Sha384(SkipOrHex384, 48) = Sha384Checksums,
    Sha512(SkipOrHex512, 64) = Sha512Checksums,
    Blake2b(SkipOrHex512, 64) = Blake2bChecksums,
}

impl From<ChecksumType> for FieldName {
    fn from(value: ChecksumType) -> Self {
        value.into_field_name()
    }
}

impl TryFrom<FieldName> for ChecksumType {
    type Error = ();
    fn try_from(value: FieldName) -> Result<Self, Self::Error> {
        ChecksumType::try_from_field_name(value).ok_or(())
    }
}

impl<'a> ParseArray for ChecksumValue<'a> {
    type Array = ChecksumArray;
    type Error = ();
    fn parse_array(&self) -> Result<Self::Array, Self::Error> {
        self.u8_array().ok_or(())
    }
}
