use super::{common::query_raw_text_from, Query};
use crate::field::{FieldName, ParsedField};
use core::str::Lines;

/// [Query] with a cache.
#[derive(Debug, Clone)]
pub struct MemoQuerier<'a> {
    text: &'a str,
    lines: Lines<'a>,
    cache: Cache<'a>,
}

impl<'a> MemoQuerier<'a> {
    /// Query the `text` with a cache.
    pub fn new(text: &'a str) -> Self {
        MemoQuerier {
            text,
            lines: text.lines(),
            cache: Cache::default(),
        }
    }
}

impl<'a> Query<'a> for &'a mut MemoQuerier<'a> {
    fn query_raw_text(self, field: ParsedField) -> Option<&'a str> {
        if let Some(value) = self.cache.get(field.name()) {
            return value;
        }
        let value = query_raw_text_from(self.lines.by_ref(), self.text, field);
        self.cache.add(field.name(), value);
        value
    }
}

macro_rules! def_cache {
    ($(
        $(#[$attrs:meta])*
        $field:ident $(,)? $(;)?
    )*) => {
        #[derive(Debug, Default, Clone)]
        #[allow(non_snake_case, reason = "We don't access the field names directly, keep it simple.")]
        struct Cache<'a> {$(
            $(#[$attrs])*
            $field: Option<Option<&'a str>>,
        )*}

        impl<'a> Cache<'a> {
            fn get(&self, field: &FieldName) -> Option<Option<&'a str>> {
                match field {$(
                    FieldName::$field => self.$field,
                )*}
            }

            fn add(&mut self, field: &FieldName, value: Option<&'a str>) {
                match field {$(
                    FieldName::$field => self.$field = Some(value),
                )*}
            }
        }
    };
}

def_cache!(
    FileName Name Base Version Description Groups
    CompressedSize InstalledSize Md5Sum Sha256Sum PgpSignature
    Url License Arch BuildDate Packager
    Depends CheckDepends MakeDepends OptDepends Provides Conflicts Replaces
);
