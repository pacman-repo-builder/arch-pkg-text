use super::{common::query_raw_text_from, Query, QueryMut};
use crate::field::ParsedField;

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
        query_raw_text_from(self.0.lines(), self.0, field)
    }
}

impl<'a> QueryMut<'a> for ForgetfulQuerier<'a> {
    fn query_raw_text_mut(&mut self, field: ParsedField) -> Option<&'a str> {
        self.query_raw_text(field)
    }
}
