use crate::field::{ParsedField, RawField};
use pipe_trait::Pipe;

/// Query raw text from the remaining lines of the original text.
///
/// **NOTE:** `remaining_lines` must have originated from `original_text`.
pub fn query_raw_text_from<'a>(
    remaining_lines: impl Iterator<Item = &'a str>,
    original_text: &'a str,
    field: ParsedField,
) -> Option<&'a str> {
    let mut lines_with_end_offset = remaining_lines.map(|line| {
        (
            line,
            line.as_ptr() as usize + line.len() - original_text.as_ptr() as usize,
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

    let value = original_text[value_start_offset..value_end_offset].trim_matches(['\n', '\r']);

    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}
