use crate::db::{
    field::{FieldName, ParsedField, RawField},
    query::{Query, QueryMut},
};
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

impl<'a> ParsedDb<'a> {
    /// Parse a package description text.
    pub fn new(text: &'a str) -> Self {
        ParsedDb::parse(text).unwrap_or_default()
    }

    /// Parse lines of package description text.
    pub fn parse(text: &'a str) -> Option<Self> {
        let mut lines = text.lines_inclusive();
        let mut processed_length = 0;

        // parse the first field
        let first_line = lines.next()?;
        let first_field = first_line.trim().pipe(RawField::parse_raw).ok()?;

        // parse the remaining values and fields.
        let mut querier = ParsedDb::default();
        let mut current_field = Some((first_field, first_line));
        while let Some((field, field_line)) = current_field {
            let (value_length, next_field) = ParsedDb::parse_next(&mut lines);
            let value_start_offset = processed_length + field_line.len();
            let value_end_offset = value_start_offset + value_length;
            if let Ok(field) = field.to_parsed::<FieldName>() {
                let value = text[value_start_offset..value_end_offset].trim();
                querier.set_raw_value(*field.name(), value);
            }
            processed_length = value_end_offset;
            current_field = next_field;
        }

        Some(querier)
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
