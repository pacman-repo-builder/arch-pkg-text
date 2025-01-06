mod cache;

use super::{
    utils::{parse_line, trimmed_line_is_blank},
    QueryMut, QueryRawTextItem, Section,
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
struct Iter<'a, 'r> {
    querier: &'r mut MemoQuerier<'a>,
    field_name: FieldName,
    index: usize,
}

impl<'a, 'r> Iter<'a, 'r> {
    /// Create an iterator that queries `field_name` from `querier`.
    fn new(querier: &'r mut MemoQuerier<'a>, field_name: FieldName) -> Self {
        Iter {
            querier,
            field_name,
            index: 0,
        }
    }
}

impl<'a, 'r> Iterator for Iter<'a, 'r> {
    type Item = QueryRawTextItem<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let Iter {
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
        Iter::new(self, field_name)
    }
}
