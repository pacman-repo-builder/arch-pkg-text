use super::{Query, QueryMut};
use crate::field::{ParsedField, RawField};
use pipe_trait::Pipe;

/// [Query] without a cache.
#[derive(Debug, Clone, Copy)]
pub struct ForgetfulQuerier<'a>(&'a str);

impl<'a> ForgetfulQuerier<'a> {
    /// Query the `text` without cache.
    pub const fn new(text: &'a str) -> Self {
        ForgetfulQuerier(text)
    }
}

impl<'a> Query<'a> for ForgetfulQuerier<'a> {
    fn query_raw_text(&self, field: ParsedField) -> Option<&'a str> {
        let mut lines_with_end_offset = self.0.lines().map(|line| {
            (
                line,
                line.as_ptr() as usize + line.len() - self.0.as_ptr() as usize,
            )
        });

        let (_, value_start_offset) = lines_with_end_offset.by_ref().find(|(line, _)| {
            line.trim()
                .pipe(RawField::try_from)
                .ok()
                .map(|x| x.name_str() == field.name_str())
                .unwrap_or(false)
        })?;

        let (_, value_end_offset) = lines_with_end_offset
            .take_while(|(line, _)| RawField::try_from(line.trim()).is_err())
            .last()?; // no last means empty iterator, which means no content

        let value = self.0[value_start_offset..value_end_offset].trim_matches(['\n', '\r']);

        if value.is_empty() {
            None
        } else {
            Some(value)
        }
    }
}

impl<'a> QueryMut<'a> for ForgetfulQuerier<'a> {
    fn query_raw_text_mut(&mut self, field: ParsedField) -> Option<&'a str> {
        self.query_raw_text(field)
    }
}
