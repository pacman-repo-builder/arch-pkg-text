use super::{ParsedSrcinfo, ParsedSrcinfoBaseSection, ParsedSrcinfoDerivativeSection};
use crate::{
    srcinfo::{ChecksumType, ChecksumValue, Checksums, ChecksumsMut, Query, QueryChecksumItem},
    value,
};
use pipe_trait::Pipe;

macro_rules! def_impl {
    ($(
        $checksum_variant:ident($checksum_content:ident) <- $query_method:ident;
    )*) => {
        /// Private [iterator](Iterator) type to be used as the underlying return types in [`Checksums`] and [`ChecksumsMut`].
        enum ChecksumsIter<$($checksum_variant,)*> {$(
            $checksum_variant($checksum_variant),
        )*}

        impl<'a, $($checksum_variant,)*> Iterator for ChecksumsIter<$($checksum_variant,)*>
        where
            $($checksum_variant: Iterator<Item = QueryChecksumItem<'a>>,)*
        {
            type Item = QueryChecksumItem<'a>;
            fn next(&mut self) -> Option<Self::Item> {
                match self {$(
                    ChecksumsIter::$checksum_variant(iter) => iter.next(),
                )*}
            }
        }

        impl<'a> ParsedSrcinfo<'a> {
            /// Query checksums from all sections of a single [`ChecksumType`].
            fn checksums_single_type(&self, checksum_type: ChecksumType) -> impl Iterator<Item = QueryChecksumItem<'a>> + '_ {
                match checksum_type {$(
                    ChecksumType::$checksum_variant => {
                        self.$query_method()
                            .map(|item| item.map(ChecksumValue::$checksum_variant))
                            .pipe(ChecksumsIter::$checksum_variant)
                    }
                )*}
            }
        }

        impl<'a> Checksums<'a> for ParsedSrcinfo<'a> {
            fn checksums(&self) -> impl Iterator<Item = QueryChecksumItem<'a>> {
                ChecksumType::all_types()
                    .flat_map(|checksum_type| self.checksums_single_type(*checksum_type))
            }
        }

        impl<'a> ChecksumsMut<'a> for ParsedSrcinfo<'a> {
            fn checksums_mut(&mut self) -> impl Iterator<Item = QueryChecksumItem<'a>> {
                self.checksums()
            }
        }

        /// Slice of list of checksums.
        enum SectionChecksumsView<'a, 'r> {$(
            $checksum_variant(&'r [(value::$checksum_content<'a>, Option<value::Architecture<'a>>)]),
        )*}

        /// Private [iterator](Iterator) type to be used as the underlying return types in
        /// [`ParsedSrcinfoBaseSection::checksums`] and [`ParsedSrcinfoDerivativeSection::checksums`].
        struct SectionChecksumsIter<'a, 'r> {
            index: usize,
            view: SectionChecksumsView<'a, 'r>,
        }

        impl<'a, 'r> SectionChecksumsIter<'a, 'r> {
            /// Create a new iterator.
            fn new(view: SectionChecksumsView<'a, 'r>) -> Self {
                SectionChecksumsIter { index: 0, view }
            }
        }

        impl<'a, 'r> Iterator for SectionChecksumsIter<'a, 'r> {
            type Item = (ChecksumValue<'a>, Option<value::Architecture<'a>>);
            fn next(&mut self) -> Option<Self::Item> {
                let value = match &self.view {$(
                    SectionChecksumsView::$checksum_variant(view) => {
                        let (value, architecture) = view.get(self.index)?;
                        (ChecksumValue::$checksum_variant(*value), *architecture)
                    }
                )*};
                self.index += 1;
                Some(value)
            }
        }

        impl<'a> ParsedSrcinfoBaseSection<'a> {
            /// Query checksums of a single [`ChecksumType`].
            fn checksums_single_type(&self, checksum_type: ChecksumType) -> impl Iterator<Item = (ChecksumValue<'a>, Option<value::Architecture<'a>>)> + '_ {
                let view = match checksum_type {$(
                    ChecksumType::$checksum_variant => SectionChecksumsView::$checksum_variant(self.$query_method()),
                )*};
                SectionChecksumsIter::new(view)
            }
        }

        impl<'a> ParsedSrcinfoDerivativeSection<'a> {
            /// Query checksums of a single [`ChecksumType`].
            fn checksums_single_type(&self, checksum_type: ChecksumType) -> impl Iterator<Item = (ChecksumValue<'a>, Option<value::Architecture<'a>>)> + '_ {
                let view = match checksum_type {$(
                    ChecksumType::$checksum_variant => SectionChecksumsView::$checksum_variant(self.$query_method()),
                )*};
                SectionChecksumsIter::new(view)
            }
        }
    };
}

def_impl! {
    Md5(SkipOrHex128) <- md5_checksums;
    Sha1(SkipOrHex160) <- sha1_checksums;
    Sha224(SkipOrHex224) <- sha224_checksums;
    Sha256(SkipOrHex256) <- sha256_checksums;
    Sha384(SkipOrHex384) <- sha384_checksums;
    Sha512(SkipOrHex512) <- sha512_checksums;
    Blake2b(SkipOrHex512) <- blake2b_checksums;
}

impl<'a> ParsedSrcinfoBaseSection<'a> {
    /// List checksums of all [`ChecksumType`].
    pub fn checksums(
        &self,
    ) -> impl Iterator<Item = (ChecksumValue<'a>, Option<value::Architecture<'a>>)> + '_ {
        ChecksumType::all_types()
            .flat_map(|checksum_type| self.checksums_single_type(*checksum_type))
    }
}

impl<'a> ParsedSrcinfoDerivativeSection<'a> {
    /// List checksums of all [`ChecksumType`].
    pub fn checksums(
        &self,
    ) -> impl Iterator<Item = (ChecksumValue<'a>, Option<value::Architecture<'a>>)> + '_ {
        ChecksumType::all_types()
            .flat_map(|checksum_type| self.checksums_single_type(*checksum_type))
    }
}
