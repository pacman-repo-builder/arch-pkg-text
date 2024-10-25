use super::Query;
use crate::field::{ParsedField, RawField};
use pipe_trait::Pipe;

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

        let mut lines_with_end_offset = text.lines().map(|line| {
            (
                line,
                line.as_ptr() as usize + line.len() - text.as_ptr() as usize,
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

        let value = text[value_start_offset..value_end_offset].trim_matches(['\n', '\r']);

        if value.is_empty() {
            None
        } else {
            Some(value)
        }
    }
}
