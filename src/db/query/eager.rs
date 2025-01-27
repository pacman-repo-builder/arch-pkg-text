use super::{Query, QueryMut};
use crate::db::field::{FieldName, ParsedField, RawField};
use core::str::Lines;
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
        pub struct EagerQuerier<'a> {$(
            $(#[$attrs])*
            $field: Option<&'a str>,
        )*}

        impl<'a> EagerQuerier<'a> {
            /// Parse a package description text.
            pub fn new(text: &'a str) -> Self {
                match EagerQuerier::parse(text) {
                    Some((querier, _)) => querier,
                    None => EagerQuerier::default(),
                }
            }

            /// Parse lines of package description text.
            ///
            /// This function returns a tuple of the resulting querier and the length of processed input.
            pub fn parse(text: &'a str) -> Option<(Self, usize)> {
                let mut lines = text.lines();
                let mut processed_length = 0;

                // parse the first field
                let first_line = lines.next()?;
                processed_length += first_line.len();
                let first_field = first_line.trim().pipe(RawField::parse_raw).ok()?;

                // parse the remaining values and fields.
                let mut querier = EagerQuerier::default();
                let mut current_field = Some(first_field);
                while let Some(field) = current_field {
                    let (value_length, next_field) = EagerQuerier::parse_next(&mut lines);
                    let value_start_offset = processed_length;
                    let value_end_offset = value_start_offset + value_length;
                    if let Ok(field) = field.to_parsed::<FieldName>() {
                        let value = text[value_start_offset..value_end_offset].trim();
                        querier.set_raw_value(*field.name(), value);
                    }
                    processed_length = value_end_offset;
                    current_field = next_field;
                }

                Some((querier, processed_length))
            }

            /// Parse a value until the end of input or when a [`RawField`] is found.
            ///
            /// This function returns a tuple of the length of the value and the next field.
            fn parse_next(remaining_lines: &mut Lines<'a>) -> (usize, Option<RawField<'a>>) {
                let mut value_length = 0;

                while let Some(line) = remaining_lines.next() {
                    if let Ok(field) = line.trim().pipe(RawField::parse_raw) {
                        return (value_length, Some(field));
                    }
                    value_length += line.len();
                }

                (value_length, None)
            }

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

impl<'a> Query<'a> for EagerQuerier<'a> {
    fn query_raw_text(&self, field: ParsedField) -> Option<&'a str> {
        self.get_raw_value(*field.name())
    }
}

impl<'a> QueryMut<'a> for EagerQuerier<'a> {
    fn query_raw_text_mut(&mut self, field: ParsedField) -> Option<&'a str> {
        self.query_raw_text(field)
    }
}
