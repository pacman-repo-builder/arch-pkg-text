mod cache;

use super::{
    utils::{parse_line, trimmed_line_is_blank},
    ChecksumType, ChecksumValue, ChecksumsMut, QueryChecksumItem, QueryMut, QueryRawTextItem,
    Section,
};
use crate::{
    srcinfo::field::FieldName,
    value::{Architecture, Name},
};
use cache::Cache;
use core::str::Lines;
use pipe_trait::Pipe;

/// [Query](QueryMut) with a cache.
#[derive(Debug, Clone)]
pub struct MemoQuerier<'a> {
    remaining_lines: Lines<'a>,
    current_section: Section<'a>,
    cache: Cache<'a>,
}

impl<'a> MemoQuerier<'a> {
    /// Query the fields of a `.SRCINFO` file with a cache.
    pub fn new(srcinfo: &'a str) -> Self {
        MemoQuerier {
            remaining_lines: srcinfo.lines(),
            current_section: Section::Base,
            cache: Cache::default(),
        }
    }

    /// Shrink the cache's capacity to fit its length.
    pub fn shrink_cache_to_fit(&mut self) {
        self.cache.shrink_to_fit();
    }

    /// Private function for testing the internal cache.
    #[doc(hidden)]
    pub fn __has_cache(&self, field_name: FieldName, index: usize) -> bool {
        self.cache.get(field_name, index).is_some()
    }

    /// Parse the next key-value pair, save it the cache and return it.
    fn next_entry(&mut self) -> Option<(FieldName, QueryRawTextItem<'a>)> {
        let line = self.remaining_lines.next()?.trim();
        if trimmed_line_is_blank(line) {
            return self.next_entry();
        }
        let (raw_field, value) = parse_line(line)?;
        let Ok(field) = raw_field.to_parsed::<FieldName, &str>() else {
            return self.next_entry();
        };
        if value.is_empty() {
            return self.next_entry();
        }
        let architecture = field.architecture_str().map(Architecture);
        if *field.name() == FieldName::Name && architecture.is_none() {
            self.current_section = value.pipe(Name).pipe(Section::Derivative);
        }
        let item = QueryRawTextItem::from_tuple3((value, self.current_section, architecture));
        self.cache.add(*field.name(), item);
        Some((*field.name(), item))
    }
}

/// Return type of [`QueryMut::query_raw_text_mut`] on an instance of [`MemoQuerier`].
struct QueryIter<'a, 'r> {
    querier: &'r mut MemoQuerier<'a>,
    field_name: FieldName,
    index: usize,
}

impl<'a, 'r> QueryIter<'a, 'r> {
    /// Create an iterator that queries `field_name` from `querier`.
    fn new(querier: &'r mut MemoQuerier<'a>, field_name: FieldName) -> Self {
        QueryIter {
            querier,
            field_name,
            index: 0,
        }
    }
}

impl<'a> Iterator for QueryIter<'a, '_> {
    type Item = QueryRawTextItem<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let QueryIter {
            querier,
            field_name,
            index,
        } = self;
        loop {
            if let Some(item) = querier.cache.get(*field_name, *index) {
                *index += 1;
                return Some(item);
            } else {
                querier.next_entry()?;
                continue;
            }
        }
    }
}

impl<'a> QueryMut<'a> for MemoQuerier<'a> {
    fn query_raw_text_mut(
        &mut self,
        field_name: FieldName,
    ) -> impl Iterator<Item = QueryRawTextItem<'a>> {
        QueryIter::new(self, field_name)
    }
}

/// Return type of [`ChecksumsMut::checksums_mut`] on an instance of [`MemoQuerier`].
struct ChecksumIter<'a, 'r> {
    querier: &'r mut MemoQuerier<'a>,
    checksum_type_id: usize,
    checksum_index: usize,
}

impl<'a, 'r> ChecksumIter<'a, 'r> {
    /// Create an iterator that queries all checksums from `querier`.
    fn new(querier: &'r mut MemoQuerier<'a>) -> Self {
        ChecksumIter {
            querier,
            checksum_type_id: 0,
            checksum_index: 0,
        }
    }
}

impl<'a> Iterator for ChecksumIter<'a, '_> {
    type Item = QueryChecksumItem<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let ChecksumIter {
            querier,
            checksum_type_id,
            checksum_index,
        } = self;
        loop {
            let checksum_type = *ChecksumType::TYPES.get(*checksum_type_id)?;
            let field_name = checksum_type.into_field_name();
            if let Some(item) = querier.cache.get(field_name, *checksum_index) {
                *checksum_index += 1;
                return item
                    .map(move |value| ChecksumValue::new(checksum_type, value))
                    .pipe(Some);
            } else if querier.next_entry().is_none() {
                *checksum_type_id += 1;
                *checksum_index = 0;
            }
        }
    }
}

impl<'a> ChecksumsMut<'a> for MemoQuerier<'a> {
    fn checksums_mut(&mut self) -> impl Iterator<Item = QueryChecksumItem<'a>> {
        ChecksumIter::new(self)
    }
}
