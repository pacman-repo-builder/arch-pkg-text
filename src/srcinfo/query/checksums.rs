use crate::{srcinfo::field::FieldName, value};
use pipe_trait::Pipe;

macro_rules! def_enum {
    ($(
        $checksum_variant:ident($checksum_content:ident) = $field_variant:ident,
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
    };
}

def_enum! {
    Md5(SkipOrHex128) = Md5Checksums,
    Sha1(SkipOrHex160) = Sha1Checksums,
    Sha224(SkipOrHex224) = Sha224Checksums,
    Sha256(SkipOrHex256) = Sha256Checksums,
    Sha384(SkipOrHex384) = Sha384Checksums,
    Sha512(SkipOrHex512) = Sha512Checksums,
    Blake2b(SkipOrHex512) = Blake2bChecksums,
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
