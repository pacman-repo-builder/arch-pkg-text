use super::{
    QueryBaseField, QueryBaseFieldMut, QueryDerivativeField, QueryDerivativeFieldMut, QueryField,
    QueryFieldMut, QueryRawTextItem, QuerySection, QuerySectionAssoc, QuerySectionMut,
};
use crate::{
    srcinfo::field::{FieldName, ParsedField},
    value::{Base, Name},
};
use pipe_trait::Pipe;

/// [Query the sections](QuerySection) of a `.SRCINFO` text without a cache.
#[derive(Debug, Clone, Copy)]
pub struct ForgetfulSectionQuerier<'a> {
    /// Name of the `pkgbase`.
    base_name: Base<'a>,
    /// Content of `.SRCINFO` right under the `pkgbase` header.
    under_base_header: &'a str,
}

impl<'a> ForgetfulSectionQuerier<'a> {
    /// Create a querier for the sections of `.SRCINFO`.
    pub fn new(srcinfo: &'a str) -> Option<Self> {
        let (base_name, under_base_header) = create_section(srcinfo, FieldName::Base, Base)?;
        Some(ForgetfulSectionQuerier {
            base_name,
            under_base_header,
        })
    }
}

impl<'a> QuerySectionAssoc for ForgetfulSectionQuerier<'a> {
    type BaseSection = ForgetfulBaseSection<'a>;
    type DerivativeSection = ForgetfulDerivativeSection<'a>;
}

impl<'a> QuerySection<'a, Base<'a>, Name<'a>> for ForgetfulSectionQuerier<'a> {
    fn base(&self) -> Self::BaseSection {
        ForgetfulBaseSection {
            name: self.base_name,
            under_header: self.under_base_header,
        }
    }

    fn derivative(&self, name: Name<'a>) -> Option<Self::DerivativeSection> {
        let (_, header_line) = self
            .under_base_header
            .pipe(derivative_headers)
            .find(|(value, _)| *value == name.as_str())?;
        let under_header = derivative_under_header(self.under_base_header, header_line);
        Some(ForgetfulDerivativeSection { name, under_header })
    }

    fn derivatives(&self) -> impl IntoIterator<Item = Self::DerivativeSection> {
        self.under_base_header
            .pipe(derivative_headers)
            .map(|(name, header_line)| {
                (
                    Name(name),
                    derivative_under_header(self.under_base_header, header_line),
                )
            })
            .map(|(name, under_header)| ForgetfulDerivativeSection { name, under_header })
    }
}

impl<'a> QuerySectionMut<'a, Base<'a>, Name<'a>> for ForgetfulSectionQuerier<'a> {
    fn base_mut(&mut self) -> Self::BaseSection {
        self.base()
    }

    fn derivative_mut(&mut self, name: Name<'a>) -> Option<Self::DerivativeSection> {
        self.derivative(name)
    }

    fn derivatives_mut(&mut self) -> impl IntoIterator<Item = Self::DerivativeSection> {
        self.derivatives()
    }
}

macro_rules! def_section {
    ($(
        $(#[$attrs:meta])*
        $name:ident {
            field = $field:ident,
            header = $header:ident,
            query = $query:ident,
            query_mut = $query_mut:ident,
        }
    )*) => {$(
        $(#[$attrs])*
        #[derive(Debug, Clone, Copy)]
        pub struct $name<'a> {
            /// Name of the section.
            name: $header<'a>,
            /// Part of the `.SRCINFO` from right under the header to the end of the text.
            under_header: &'a str,
        }

        impl<'a> $query<'a, $header<'a>> for $name<'a> {
            fn name(&self) -> $header<'a> {
                self.name
            }
        }

        impl<'a> $query_mut<'a, $header<'a>> for $name<'a> {
            fn name_mut(&mut self) -> $header<'a> {
                self.name()
            }
        }

        impl<'a> QueryField<'a> for $name<'a> {
            fn query_raw_text(
                &self,
                field_name: FieldName,
            ) -> impl IntoIterator<Item = QueryRawTextItem<'a>> {
                query_raw_text(self.under_header, field_name)
            }
        }

        impl<'a> QueryFieldMut<'a> for $name<'a> {
            fn query_raw_text_mut(
                &mut self,
                field_name: FieldName,
            ) -> impl IntoIterator<Item = QueryRawTextItem<'a>> {
                self.query_raw_text(field_name)
            }
        }

    )*};
}

def_section! {
    /// Query information under the `pkgbase` section of a `.SRCINFO` file.
    ForgetfulBaseSection {
        field = Base,
        header = Base,
        query = QueryBaseField,
        query_mut = QueryBaseFieldMut,
    }

    /// Query information under a `pkgname` section of a `.SRCINFO` file.
    ForgetfulDerivativeSection {
        field = Name,
        header = Name,
        query = QueryDerivativeField,
        query_mut = QueryDerivativeFieldMut,
    }
}

fn parse_line(line: &str) -> Option<(ParsedField<&'_ str>, &'_ str)> {
    let (field, value) = line.split_once('=')?;
    let field = field.trim_end().pipe(ParsedField::parse).ok()?;
    let value = value.trim_start();
    Some((field, value))
}

/// Extract a header.
fn create_section<'a, Name, CreateName>(
    text: &'a str,
    header_field_name: FieldName,
    create_name: CreateName,
) -> Option<(Name, &'a str)>
where
    CreateName: FnOnce(&'a str) -> Name,
{
    let header = text.lines().next()?;
    let content = text[header.len()..].trim();
    let (field, value) = parse_line(header)?;
    (*field.name() == header_field_name).then_some(())?;
    field.architecture().is_none().then_some(())?;
    let name = value.trim_start().pipe(create_name);
    Some((name, content))
}

/// List all headers of `pkgname`.
///
/// Each item is a tuple of `(name, line)` where `name` is the trimmed string after `pkgname =`
/// and `line` is the line that contains `pkgname = {name}`.
fn derivative_headers(under_base_header: &str) -> impl Iterator<Item = (&'_ str, &'_ str)> {
    under_base_header
        .lines()
        .map(|line| (line.trim(), line))
        .filter(|(trimmed_line, _)| !trimmed_line.is_empty())
        .map_while(|(trimmed_line, line)| {
            parse_line(trimmed_line).map(|(field, value)| (field, value, line))
        })
        .filter(|(field, _, _)| *field.name() == FieldName::Name)
        .map(|(_, value, line)| (value, line))
}

/// Get the section of `under_base_header` under the `header_line`,
/// given that `header_line` is a line of `under_base_header`.
fn derivative_under_header<'a>(under_base_header: &'a str, header_line: &'a str) -> &'a str {
    let start_offset =
        header_line.as_ptr() as usize + header_line.len() - under_base_header.as_ptr() as usize;
    &under_base_header[start_offset..]
}

/// List all lines that match `field_name` from the start of `text` to the next `pkgname` or the end of `text`.
///
/// The `text` is assumed to be the part of `.SRCINFO` right after a line of `pkgbase` or `pkgname`.
///
/// This function does not list items from other sections.
fn query_raw_text(text: &str, field_name: FieldName) -> impl Iterator<Item = QueryRawTextItem<'_>> {
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map_while(parse_line)
        .take_while(|(field, _)| *field.name() != FieldName::Name)
        .filter(move |(field, _)| *field.name() == field_name)
        .map(|(field, value)| (field.architecture().copied(), value))
        .map(QueryRawTextItem::from_tuple)
}
