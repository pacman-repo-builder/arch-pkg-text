use super::{Query, QueryMut};
use crate::db::field::{FieldName, ParsedField, RawField};
use core::str::Lines;
use pipe_trait::Pipe;

macro_rules! def_struct {
    ($(
        $(#[$attrs:meta])*
        $name:ident, $name_mut:ident = $field_name:ident;
    )*) => {
        /// Parsed data of a package description text.
        ///
        /// Every function call in [`Query`] and [`QueryMut`] is constant time.
        #[derive(Debug, Default, Clone, Copy)]
        pub struct EagerQuerier<'a> {$(
            $(#[$attrs])*
            $name: Option<&'a str>,
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
            fn parse(text: &'a str) -> Option<(Self, usize)> {
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
                    FieldName::$field_name => self.$name,
                )*}
            }

            /// Add a raw value into the querier.
            fn set_raw_value(&mut self, field_name: FieldName, raw_value: &'a str) {
                match field_name {$(
                    FieldName::$field_name => self.$name = Some(raw_value),
                )*}
            }
        }
    };
}

def_struct! {
    file_name, file_name_mut = FileName;
    name, name_mut = Name;
    base, base_mut = Base;
    version, version_mut = Version;
    description, description_mut = Description;
    groups, groups_mut = Groups;
    compressed_size, compressed_size_mut = CompressedSize;
    installed_size, installed_size_mut = InstalledSize;
    md5_checksum, md5_checksum_mut = Md5Checksum;
    sha256_checksum, sha256_checksum_mut = Sha256Checksum;
    pgp_signature, pgp_signature_mut = PgpSignature;
    url, url_mut = Url;
    license, license_mut = License;
    architecture, architecture_mut = Architecture;
    build_date, build_date_mut = BuildDate;
    packager, packager_mut = Packager;
    dependencies, dependencies_mut = Dependencies;
    make_dependencies, make_dependencies_mut = MakeDependencies;
    check_dependencies, check_dependencies_mut = CheckDependencies;
    opt_dependencies, opt_dependencies_mut = OptionalDependencies;
    provides, provides_mut = Provides;
    conflicts, conflicts_mut = Conflicts;
    replaces, replaces_mut = Replaces;
}

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
