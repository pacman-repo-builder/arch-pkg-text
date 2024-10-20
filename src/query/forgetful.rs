use super::Query;
use crate::field::{ParsedField, RawField};
use iter_scan::IterScan;
use pipe_trait::Pipe;

/// [Query] without a cache.
#[derive(Debug, Clone, Copy)]
pub struct ForgetfulQuerier<Text>(Text);

impl<'a, Text> Query<'a> for &'a ForgetfulQuerier<Text>
where
    Text: AsRef<str>,
{
    fn query_raw_text(self, field: ParsedField) -> Option<&'a str> {
        let text = self.0.as_ref();

        let mut lines_with_end_offset = text
            .split('\n')
            .scan_copy(("", 0), |(_, end_offset), line| {
                (line, end_offset + line.len() + '\n'.len_utf8())
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

        let value = text[value_start_offset..value_end_offset].trim_matches('\n');

        if value.is_empty() {
            None
        } else {
            Some(value)
        }
    }
}
