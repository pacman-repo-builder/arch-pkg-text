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
    pub fn parse<'a>(raw_field: &'a str) -> ParseResult<'a, Name, Architecture>
    where
        &'a str: TryInto<Name> + TryInto<Architecture>,
    {
        RawField::parse_raw(raw_field).to_parsed()
    }
}

/// Parse a [`Field`] from [`str`].
impl<'a, Name, Architecture> TryFrom<&'a str> for Field<Name, Architecture>
where
    &'a str: TryInto<Name> + TryInto<Architecture>,
{
    type Error = ParseError<'a, Name, Architecture>;
    fn try_from(raw_field: &'a str) -> ParseResult<'a, Name, Architecture> {
        RawField::parse_raw(raw_field).to_parsed()
    }
}

impl<'a> RawField<'a> {
    /// Parse a [`RawField`] from a [`str`].
    pub fn parse_raw(raw_field: &'a str) -> Self {
        let (name, architecture) = match raw_field.split_once('_') {
            Some((name, architecture)) => (name, Some(architecture)),
            None => (raw_field, None),
        };
        RawField { name, architecture }
    }

    /// Try converting a [`RawField`] into a [`Field<Name, Architecture>`].
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
