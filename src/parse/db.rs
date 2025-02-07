use super::PartialParseResult;
use crate::db::{
    field::{FieldName, ParseRawFieldError, ParsedField, RawField},
    query::{Query, QueryMut},
};
use derive_more::{Display, Error};
use lines_inclusive::{LinesInclusive, LinesInclusiveIter};
use pipe_trait::Pipe;

macro_rules! def_struct {
    ($(
        $(#[$attrs:meta])*
        $field:ident $(,)? $(;)?
    )*) => {
        /// Parsed data of a package description text.
        ///
        /// Every function call in [`Query`] and [`QueryMut`] is constant time.
        #[derive(Debug, Default, Clone, Copy)]
        #[allow(non_snake_case, reason = "We don't access the field names directly, keep it simple.")]
        pub struct ParsedDb<'a> {$(
            $(#[$attrs])*
            $field: Option<&'a str>,
        )*}

        impl<'a> ParsedDb<'a> {
            /// Get a raw value from the querier.
            fn get_raw_value(&self, field_name: FieldName) -> Option<&'a str> {
                match field_name {$(
                    FieldName::$field => self.$field,
                )*}
            }

            /// Add a raw value into the querier.
            fn set_raw_value(&mut self, field_name: FieldName, raw_value: &'a str) {
                match field_name {$(
                    FieldName::$field => self.$field = Some(raw_value),
                )*}
            }
        }
    };
}

def_struct!(
    FileName Name Base Version Description Groups
    CompressedSize InstalledSize Md5Checksum Sha256Checksum
    PgpSignature Url License Architecture BuildDate Packager
    Dependencies CheckDependencies MakeDependencies OptionalDependencies
    Provides Conflicts Replaces
);

#[derive(Debug, Display, Error, Clone, Copy)]
pub enum DbParseError<'a> {
    #[display("Input is empty")]
    EmptyInput,
    #[display("Receive a value without field: {_0:?}")]
    ValueWithoutField(#[error(not(source))] &'a str),
}

/// Issue that may arise during parsing.
#[derive(Debug, Clone, Copy)]
pub enum DbParseIssue<'a> {
    EmptyInput,
    FirstLineIsNotAField(&'a str, ParseRawFieldError),
    UnknownField(RawField<'a>),
}

impl<'a> DbParseIssue<'a> {
    /// Return `Ok(())` if the issue was [`DbParseIssue::UnknownField`],
    /// or return an `Err` of [`DbParseError`] otherwise.
    fn ignore_unknown_field(self) -> Result<(), DbParseError<'a>> {
        Err(match self {
            DbParseIssue::EmptyInput => DbParseError::EmptyInput,
            DbParseIssue::FirstLineIsNotAField(line, _) => DbParseError::ValueWithoutField(line),
            DbParseIssue::UnknownField(_) => return Ok(()),
        })
    }
}

impl<'a> ParsedDb<'a> {
    /// Parse a package description text, unknown fields are ignored.
    pub fn parse(text: &'a str) -> Result<Self, DbParseError<'a>> {
        ParsedDb::parse_with_issues(text, DbParseIssue::ignore_unknown_field).try_into_complete()
    }

    /// Parse a package description text with a callback that handle [parsing issues](DbParseIssue).
    pub fn parse_with_issues<HandleIssue, Error>(
        text: &'a str,
        mut handle_issue: HandleIssue,
    ) -> PartialParseResult<ParsedDb<'a>, Error>
    where
        HandleIssue: FnMut(DbParseIssue<'a>) -> Result<(), Error>,
    {
        let mut parsed = ParsedDb::default();
        let mut lines = text.lines_inclusive();
        let mut processed_length = 0;

        macro_rules! return_or_continue {
            ($issue:expr) => {
                match handle_issue($issue) {
                    Err(error) => return PartialParseResult::new_partial(parsed, error),
                    Ok(()) => continue,
                }
            };
        }

        // parse the first field
        let (first_line, first_field) = loop {
            let Some(first_line) = lines.next() else {
                return_or_continue!(DbParseIssue::EmptyInput);
            };
            let first_field = match first_line.trim().pipe(RawField::parse_raw) {
                Ok(first_field) => first_field,
                Err(error) => {
                    return_or_continue!(DbParseIssue::FirstLineIsNotAField(first_line, error))
                }
            };
            break (first_line, first_field);
        };

        // parse the remaining values and fields.
        let mut current_field = Some((first_field, first_line));
        while let Some((field, field_line)) = current_field {
            let (value_length, next_field) = ParsedDb::parse_next(&mut lines);
            let value_start_offset = processed_length + field_line.len();
            let value_end_offset = value_start_offset + value_length;
            if let Ok(field) = field.to_parsed::<FieldName>() {
                let value = text[value_start_offset..value_end_offset].trim();
                parsed.set_raw_value(*field.name(), value);
            } else {
                return_or_continue!(DbParseIssue::UnknownField(field));
            }
            processed_length = value_end_offset;
            current_field = next_field;
        }

        PartialParseResult::new_complete(parsed)
    }

    /// Parse a value until the end of input or when a [`RawField`] is found.
    ///
    /// This function returns a tuple of the length of the value and the next field.
    fn parse_next(
        remaining_lines: &mut LinesInclusiveIter<'a>,
    ) -> (usize, Option<(RawField<'a>, &'a str)>) {
        let mut value_length = 0;

        for line in remaining_lines {
            if let Ok(field) = line.trim().pipe(RawField::parse_raw) {
                return (value_length, Some((field, line)));
            }
            value_length += line.len();
        }

        (value_length, None)
    }
}

impl<'a> TryFrom<&'a str> for ParsedDb<'a> {
    type Error = DbParseError<'a>;
    fn try_from(text: &'a str) -> Result<Self, Self::Error> {
        ParsedDb::parse(text)
    }
}

impl<'a> Query<'a> for ParsedDb<'a> {
    fn query_raw_text(&self, field: ParsedField) -> Option<&'a str> {
        self.get_raw_value(*field.name())
    }
}

impl<'a> QueryMut<'a> for ParsedDb<'a> {
    fn query_raw_text_mut(&mut self, field: ParsedField) -> Option<&'a str> {
        self.query_raw_text(field)
    }
}
