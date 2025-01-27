use crate::srcinfo::field::RawField;
use pipe_trait::Pipe;

/// Parse a non-blank trimmed line.
pub fn parse_line(line: &str) -> Option<(RawField<'_>, &'_ str)> {
    let (field, value) = line.split_once('=')?;
    let field = field.trim_end().pipe(RawField::parse_raw);
    let value = value.trim_start();
    Some((field, value))
}

/// This function is intended for use in `.filter` to filter out lines to parse.
pub fn trimmed_line_is_blank(trimmed_line: &str) -> bool {
    trimmed_line.is_empty() || trimmed_line.starts_with('#')
}
