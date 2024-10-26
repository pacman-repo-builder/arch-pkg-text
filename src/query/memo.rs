use super::Query;
use crate::{
    collections::{GetEntry, OrInsertWith},
    field::{ParsedField, RawField},
};
use core::str::Lines;
use pipe_trait::Pipe;

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
            .or_insert_with(|| -> Option<&str> {
                let mut lines_with_end_offset = self.lines.by_ref().map(|line| {
                    (
                        line,
                        line.as_ptr() as usize + line.len() - self.text.as_ptr() as usize,
                    )
                });

                let (_, value_start_offset) =
                    lines_with_end_offset.by_ref().find(|(line, _)| {
                        line.trim()
                            .pipe(RawField::try_from)
                            .ok()
                            .map(|x| x.name_str() == field.name_str())
                            .unwrap_or(false)
                    })?;

                let (_, value_end_offset) = lines_with_end_offset
                    .take_while(|(line, _)| RawField::try_from(line.trim()).is_err())
                    .last()?; // no last means empty iterator, which means no content

                let value =
                    self.text[value_start_offset..value_end_offset].trim_matches(['\n', '\r']);

                if value.is_empty() {
                    None
                } else {
                    Some(value)
                }
            })
            .as_deref()
    }
}
