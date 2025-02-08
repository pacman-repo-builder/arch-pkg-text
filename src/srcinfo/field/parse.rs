use super::{Field, RawField};
use derive_more::{Display, Error};

/// Error when attempt to parse a [`Field`] with [`TryFrom`].
#[derive(Debug, Display, Clone, Copy, Error)]
pub enum ParseFieldError<ParseNameError, ParseArchitectureError> {
    Name(ParseNameError),
    Architecture(ParseArchitectureError),
}

type ParseError<'a, Name, Architecture> =
    ParseFieldError<<&'a str as TryInto<Name>>::Error, <&'a str as TryInto<Architecture>>::Error>;

type ParseResult<'a, Name, Architecture> =
    Result<Field<Name, Architecture>, ParseError<'a, Name, Architecture>>;

impl<Name, Architecture> Field<Name, Architecture> {
    /// Parse a [`Field`] from [`str`].
    /// ```
    /// # use arch_pkg_text::srcinfo::{Field, FieldName, ParsedField};
    /// # use pretty_assertions::assert_eq;
    /// let parsed_field: ParsedField<&str> = Field::parse("source_x86_64").unwrap();
    /// assert_eq!(parsed_field.name(), &FieldName::Source);
    /// assert_eq!(parsed_field.architecture_str(), Some("x86_64"));
    /// ```
    pub fn parse<'a>(input: &'a str) -> ParseResult<'a, Name, Architecture>
    where
        &'a str: TryInto<Name> + TryInto<Architecture>,
    {
        RawField::parse_raw(input).to_parsed()
    }
}

/// Parse a [`Field`] from [`str`].
impl<'a, Name, Architecture> TryFrom<&'a str> for Field<Name, Architecture>
where
    &'a str: TryInto<Name> + TryInto<Architecture>,
{
    type Error = ParseError<'a, Name, Architecture>;
    fn try_from(value: &'a str) -> ParseResult<'a, Name, Architecture> {
        RawField::parse_raw(value).to_parsed()
    }
}

impl<'a> RawField<'a> {
    /// Parse a [`RawField`] from a [`str`].
    ///
    /// **Without architecture:**
    ///
    /// ```
    /// # use arch_pkg_text::srcinfo::RawField;
    /// # use pretty_assertions::assert_eq;
    /// let raw_field = RawField::parse_raw("source");
    /// assert_eq!(raw_field.name_str(), "source");
    /// assert_eq!(raw_field.architecture_str(), None);
    /// ```
    ///
    /// **With architecture:**
    ///
    /// ```
    /// # use arch_pkg_text::srcinfo::RawField;
    /// # use pretty_assertions::assert_eq;
    /// let raw_field = RawField::parse_raw("source_x86_64");
    /// assert_eq!(raw_field.name_str(), "source");
    /// assert_eq!(raw_field.architecture_str(), Some("x86_64"));
    /// ```
    pub fn parse_raw(input: &'a str) -> Self {
        let (name, architecture) = match input.split_once('_') {
            Some((name, architecture)) => (name, Some(architecture)),
            None => (input, None),
        };
        RawField { name, architecture }
    }

    /// Try converting a [`RawField`] into a [`Field<Name, Architecture>`].
    ///
    /// ```
    /// # use arch_pkg_text::srcinfo::{FieldName, ParsedField, RawField};
    /// # use pretty_assertions::assert_eq;
    /// let raw_field = RawField::parse_raw("source_x86_64");
    /// let parsed_field: ParsedField<&str> = raw_field.to_parsed().unwrap();
    /// assert_eq!(parsed_field.name(), &FieldName::Source);
    /// assert_eq!(parsed_field.architecture_str(), Some("x86_64"));
    /// ```
    pub fn to_parsed<Name, Architecture>(&self) -> ParseResult<'a, Name, Architecture>
    where
        &'a str: TryInto<Name> + TryInto<Architecture>,
    {
        let &RawField { name, architecture } = self;
        let name: Name = name.try_into().map_err(ParseFieldError::Name)?;
        let architecture: Option<Architecture> = architecture
            .map(TryInto::try_into)
            .transpose()
            .map_err(ParseFieldError::Architecture)?;
        Ok(Field { name, architecture })
    }
}
