use super::QueryMut;
use crate::desc::{
    field::{FieldName, ParsedField, RawField},
    misc::{ReuseAdvice, True},
};

/// [Query](QueryMut) with a cache.
#[derive(Debug, Clone)]
pub struct MemoQuerier<'a> {
    text: &'a str,
    cache: Cache<'a>,
    last: Option<(&'a str, RawField<'a>)>,
}

impl<'a> MemoQuerier<'a> {
    /// Query the `text` with a cache.
    pub fn new(text: &'a str) -> Self {
        MemoQuerier {
            text,
            cache: Cache::default(),
            last: None,
        }
    }

    /// Parse the next key-value pair, save it to cache and return it.
    fn next_entry(&mut self) -> Option<(RawField<'a>, &'a str)> {
        let mut lines = self.text.lines();

        let (field_str, raw_field) = if let Some((field_str, raw_field)) = self.last {
            lines.next()?;
            (field_str, raw_field)
        } else {
            let field_str = lines.next()?.trim();
            let raw_field = RawField::parse_raw(field_str).ok()?;
            (field_str, raw_field)
        };

        let value_start_offset =
            field_str.as_ptr() as usize + field_str.len() - self.text.as_ptr() as usize;
        let next = lines.find_map(|line| -> Option<(&'a str, RawField<'a>)> {
            let field_str = line.trim();
            let raw_field = RawField::parse_raw(field_str).ok()?;
            Some((field_str, raw_field))
        });

        let Some((next_field_str, next_raw_field)) = next else {
            let value = self.text[value_start_offset..].trim_matches(['\n', '\r']);
            self.text = "";
            self.last = None;
            return Some((raw_field, value));
        };

        let value_end_offset = next_field_str.as_ptr() as usize - self.text.as_ptr() as usize;
        let value = self.text[value_start_offset..value_end_offset].trim_matches(['\n', '\r']);

        // prepare for the next call
        self.last = Some((next_field_str, next_raw_field));
        self.text = &self.text[value_end_offset..];

        Some((raw_field, value))
    }

    /// Private function for testing the internal cache.
    #[doc(hidden)]
    pub fn __has_cache(&self, field: FieldName) -> bool {
        self.cache.get(&field).is_some()
    }
}

impl<'a> QueryMut<'a> for MemoQuerier<'a> {
    fn query_raw_text_mut(&mut self, field: ParsedField) -> Option<&'a str> {
        if let Some(value) = self.cache.get(field.name()) {
            return value;
        }

        while let Some((raw_field, value)) = self.next_entry() {
            let Ok(parsed_field) = raw_field.to_parsed::<FieldName>() else {
                continue;
            };
            let value = if value.is_empty() { None } else { Some(value) };
            self.cache.add(&parsed_field, value);
            if parsed_field == field {
                return value;
            }
        }

        None
    }
}

macro_rules! def_cache {
    ($(
        $(#[$attrs:meta])*
        $field:ident $(,)? $(;)?
    )*) => {
        #[derive(Debug, Clone, Copy)]
        enum CacheErr {
            OccupiedWithNone,
            Unoccupied,
        }

        #[derive(Debug, Clone)]
        #[allow(non_snake_case, reason = "We don't access the field names directly, keep it simple.")]
        struct Cache<'a> {$(
            $(#[$attrs])*
            $field: Result<&'a str, CacheErr>, // Result<&str, CacheErr> uses less memory than Option<Option<&str>>
        )*}

        impl<'a> Cache<'a> {
            fn get(&self, field: &FieldName) -> Option<Option<&'a str>> {
                match field {$(
                    FieldName::$field => match self.$field {
                        Ok(value) => Some(Some(value)),
                        Err(CacheErr::OccupiedWithNone) => Some(None),
                        Err(CacheErr::Unoccupied) => None,
                    },
                )*}
            }

            fn add(&mut self, field: &FieldName, value: Option<&'a str>) {
                match (field, value) {$(
                    (FieldName::$field, Some(value)) => self.$field = Ok(value),
                    (FieldName::$field, None) => self.$field = Err(CacheErr::OccupiedWithNone),
                )*}
            }
        }

        impl<'a> Default for Cache<'a> {
            fn default() -> Self {
                Cache {$(
                    $field: Err(CacheErr::Unoccupied),
                )*}
            }
        }

        #[test]
        fn test_cache_fields() {$({
            use pretty_assertions::assert_eq;
            let field = &FieldName::$field;
            let mut cache = Cache::default();
            assert_eq!(cache.get(field), None);
            cache.add(field, None);
            assert_eq!(cache.get(field), Some(None));
            cache.add(field, Some("foo"));
            assert_eq!(cache.get(field), Some(Some("foo")));
        })*}
    };
}

def_cache!(
    FileName Name Base Version Description Groups
    CompressedSize InstalledSize Md5Checksum Sha256Checksum
    PgpSignature Url License Architecture BuildDate Packager
    Dependencies CheckDependencies MakeDependencies OptionalDependencies
    Provides Conflicts Replaces
);

impl ReuseAdvice for MemoQuerier<'_> {
    /// [`MemoQuerier`] costs O(1) time to construct. Performing a lookup on it
    /// costs O(n) the first time and O(1) after that.
    ///
    /// This struct is designed to be reused.
    type ShouldReuse = True;
}

impl<'a> From<&'a str> for MemoQuerier<'a> {
    fn from(value: &'a str) -> Self {
        MemoQuerier::new(value)
    }
}
