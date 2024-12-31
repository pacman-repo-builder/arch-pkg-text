use super::{
    utils::{parse_line, trimmed_line_is_blank},
    Query, QueryMut, QueryRawTextItem, Section,
};
use crate::{
    srcinfo::field::{FieldName, ParsedField, RawField},
    value::{Architecture, Name},
};
use iter_scan::IterScan;

/// [Query] without a cache.
#[derive(Debug, Clone, Copy)]
pub struct ForgetfulQuerier<'a>(&'a str);

impl<'a> ForgetfulQuerier<'a> {
    /// Query the `text` without cache.
    pub const fn new(text: &'a str) -> Self {
        ForgetfulQuerier(text)
    }
}

impl<'a> Query<'a> for ForgetfulQuerier<'a> {
    fn query_raw_text(&self, field_name: FieldName) -> impl Iterator<Item = QueryRawTextItem<'a>> {
        self.0
            .lines()
            .map(str::trim)
            .filter(|line| !trimmed_line_is_blank(line))
            .map_while(parse_line)
            .filter_map(known_field)
            .scan_state_copy(Section::Base, |section, (field, value)| {
                match field.name() {
                    FieldName::Name => (Section::Derivative(Name(value)), (field, value)),
                    _ => (section, (field, value)),
                }
            })
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
