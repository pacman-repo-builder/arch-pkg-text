use super::{common::query_raw_text_from, Query};
use crate::{
    collections::{GetEntry, OrInsertWith},
    field::ParsedField,
};
use core::str::Lines;

/// [Query] with a cache.
#[derive(Debug, Clone)]
pub struct MemoQuerier<'a, Cache> {
    text: &'a str,
    lines: Lines<'a>,
    cache: Cache,
}

impl<'a, Cache> MemoQuerier<'a, Cache> {
    /// Query the `text` without cache.
    pub fn new(text: &'a str) -> Self
    where
        Cache: Default,
    {
        MemoQuerier {
            text,
            lines: text.lines(),
            cache: Cache::default(),
        }
    }
}

impl<'a, Cache> Query<'a> for &'a mut MemoQuerier<'a, Cache>
where
    &'a mut Cache: GetEntry<ParsedField>,
    <&'a mut Cache as GetEntry<ParsedField>>::Entry: OrInsertWith<'a, Item = Option<&'a str>>,
{
    fn query_raw_text(self, field: ParsedField) -> Option<&'a str> {
        self.cache
            .get_entry(field)
            .or_insert_with(|| query_raw_text_from(self.lines.by_ref(), self.text, field))
            .as_deref()
    }
}
