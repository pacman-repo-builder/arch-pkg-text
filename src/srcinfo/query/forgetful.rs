use super::{
    ChecksumValue, Checksums, ChecksumsMut, Query, QueryChecksumItem, QueryMut, QueryRawTextItem,
    Section,
    utils::{non_blank_trimmed_lines, parse_line},
};
use crate::{
    srcinfo::{
        field::{FieldName, ParsedField, RawField},
        misc::{False, ReuseAdvice},
    },
    value::{Architecture, Name},
};
use iter_scan::IterScan;
use pipe_trait::Pipe;

/// [Query] without a cache.
#[derive(Debug, Clone, Copy)]
pub struct ForgetfulQuerier<'a>(&'a str);

impl<'a> ForgetfulQuerier<'a> {
    /// Query the `text` without cache.
    pub const fn new(text: &'a str) -> Self {
        ForgetfulQuerier(text)
    }

    /// List all items of known fields.
    fn all_known_items(
        &self,
    ) -> impl Iterator<Item = (Section<'a>, (ParsedField<&'a str>, &'a str))> {
        self.0
            .pipe(non_blank_trimmed_lines)
            .map_while(parse_line)
            .filter_map(known_field)
            .filter(|(_, value)| !value.is_empty())
            .scan_state_copy(Section::Base, scan_section)
    }
}

impl<'a> Query<'a> for ForgetfulQuerier<'a> {
    fn query_raw_text(&self, field_name: FieldName) -> impl Iterator<Item = QueryRawTextItem<'a>> {
        self.all_known_items()
            .filter(move |(_, (field, _))| *field.name() == field_name)
            .map(|(section, (field, value))| {
                QueryRawTextItem::from_tuple3((
                    value,
                    section,
                    field.architecture_str().map(Architecture::new),
                ))
            })
    }
}

impl<'a> QueryMut<'a> for ForgetfulQuerier<'a> {
    fn query_raw_text_mut(
        &mut self,
        field_name: FieldName,
    ) -> impl Iterator<Item = QueryRawTextItem<'a>> {
        self.query_raw_text(field_name)
    }
}

impl<'a> Checksums<'a> for ForgetfulQuerier<'a> {
    fn checksums(&self) -> impl Iterator<Item = QueryChecksumItem<'a>> {
        self.all_known_items()
            .filter_map(|(section, (field, value))| {
                ChecksumValue::try_from_field_name(*field.name(), value)
                    .map(|value| (value, section, field.architecture_str().map(Architecture)))
            })
            .map(QueryChecksumItem::from_tuple3)
    }
}

impl<'a> ChecksumsMut<'a> for ForgetfulQuerier<'a> {
    fn checksums_mut(&mut self) -> impl Iterator<Item = QueryChecksumItem<'a>> {
        self.checksums()
    }
}

/// Callback function to pass to `.filter_map` to filter out unknown fields.
fn known_field<'a, Architecture, Acquaintance>(
    (field, acquaintance): (RawField<'a>, Acquaintance),
) -> Option<(ParsedField<Architecture>, Acquaintance)>
where
    &'a str: TryInto<Architecture>,
{
    field
        .to_parsed::<FieldName, Architecture>()
        .map(|field| (field, acquaintance))
        .ok()
}

/// Callback function to pass to `.scan_state_*` to attach sections to items.
fn scan_section<'a>(
    section: Section<'a>,
    (field, value): (ParsedField<&'a str>, &'a str),
) -> (Section<'a>, (ParsedField<&'a str>, &'a str)) {
    match field.name() {
        FieldName::Name => (Section::Derivative(Name(value)), (field, value)),
        _ => (section, (field, value)),
    }
}

impl ReuseAdvice for ForgetfulQuerier<'_> {
    /// Whilst [`ForgetfulQuerier`] costs nothing to construct, performing a
    /// lookup on it costs O(n) time complexity (n being text length).
    ///
    /// This struct is best used to lookup once.
    type ShouldReuse = False;
}

impl<'a> From<&'a str> for ForgetfulQuerier<'a> {
    fn from(value: &'a str) -> Self {
        ForgetfulQuerier::new(value)
    }
}
