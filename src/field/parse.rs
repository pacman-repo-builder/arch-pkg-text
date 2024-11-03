use super::{Field, RawField};
use derive_more::{Display, Error};
use pipe_trait::Pipe;

/// Error when attempt to parse a [`Field`] with [`TryFrom`].
#[derive(Debug, Display, Clone, Copy, Error)]
pub enum ParseFieldError<ParseNameError> {
    RawField(ParseRawFieldError),
    Name(ParseNameError),
}

impl<Name> Field<Name> {
    /// Parse a [`Field`] from [`str`].
    /// ```
    /// # use parse_arch_pkg_desc::field::{FieldName, ParsedField};
    /// # use pretty_assertions::assert_eq;
    /// let parsed_field = ParsedField::parse("%NAME%").unwrap();
    /// assert_eq!(parsed_field.name(), &FieldName::Name);
    /// ```
    pub fn parse<'a>(value: &'a str) -> Result<Self, <Self as TryFrom<&'a str>>::Error>
    where
        &'a str: TryInto<Name>,
    {
        Self::try_from(value)
    }
}

/// Parse a [`Field`] from [`str`].
impl<'a, Name> TryFrom<&'a str> for Field<Name>
where
    &'a str: TryInto<Name>,
{
    type Error = ParseFieldError<<&'a str as TryInto<Name>>::Error>;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        value
            .pipe(RawField::parse_raw)
            .map_err(ParseFieldError::RawField)?
            .into_name()
            .pipe(TryInto::<Name>::try_into)
            .map_err(ParseFieldError::Name)
            .map(Field)
    }
}

/// Error when attempt to parse a [`RawField`] with [`RawField::parse_raw`].
#[derive(Debug, Display, Clone, Copy, Error)]
pub enum ParseRawFieldError {
    #[display("Input doesn't start with '%'")]
    IncorrectStartingCharacter,
    #[display("Input doesn't end with '%'")]
    IncorrectEndingCharacter,
    #[display("Field name is empty")]
    Empty,
    #[display("Found invalid character {_1:?} at index {_0} which isn't ASCII uppercase")]
    NotAsciiUppercase(usize, char),
}

impl<'a> RawField<'a> {
    /// Parse a [`RawField`] from a [`str`].
    ///
    /// ```
    /// # use parse_arch_pkg_desc::field::RawField;
    /// # use pretty_assertions::assert_eq;
    /// let raw_field = RawField::parse_raw("%NAME%").unwrap();
    /// assert_eq!(raw_field.name_str(), "NAME");
    /// ```
    pub fn parse_raw(input: &'a str) -> Result<Self, ParseRawFieldError> {
        let field_name = input
            .strip_prefix('%')
            .ok_or(ParseRawFieldError::IncorrectStartingCharacter)?
            .strip_suffix('%')
            .ok_or(ParseRawFieldError::IncorrectEndingCharacter)?;

        if field_name.is_empty() {
            return Err(ParseRawFieldError::Empty);
        }

        if let Some((index, char)) = field_name
            .char_indices()
            .find(|(_, x)| !x.is_ascii_uppercase())
        {
            return Err(ParseRawFieldError::NotAsciiUppercase(index, char));
        }

        Ok(Field(field_name))
    }

    /// Try converting a [`RawField`] into a [`Field<Name>`].
    ///
    /// ```
    /// # use parse_arch_pkg_desc::field::{FieldName, ParsedField, RawField};
    /// # use pretty_assertions::assert_eq;
    /// let raw_field = RawField::parse_raw("%NAME%").unwrap();
    /// let parsed_field: ParsedField = raw_field.to_parsed().unwrap();
    /// assert_eq!(parsed_field.name(), &FieldName::Name);
    /// ```
    pub fn to_parsed<Name>(&self) -> Result<Field<Name>, <&'a str as TryInto<Name>>::Error>
    where
        &'a str: TryInto<Name>,
    {
        self.name_str().pipe(TryInto::<Name>::try_into).map(Field)
    }
}
