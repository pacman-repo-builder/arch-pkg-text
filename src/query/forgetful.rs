use super::{common::query_raw_text_from, Query};
use crate::field::ParsedField;

/// [Query] without a cache.
#[derive(Debug, Clone, Copy)]
pub struct ForgetfulQuerier<Text>(Text);

impl<Text> ForgetfulQuerier<Text> {
    /// Query the `text` without cache.
    pub const fn new(text: Text) -> Self {
        ForgetfulQuerier(text)
    }
}

impl<'a, Text> Query<'a> for &'a ForgetfulQuerier<Text>
where
    Text: AsRef<str>,
{
    fn query_raw_text(self, field: ParsedField) -> Option<&'a str> {
        let text = self.0.as_ref();
        query_raw_text_from(text.lines(), text, field)
    }
}
