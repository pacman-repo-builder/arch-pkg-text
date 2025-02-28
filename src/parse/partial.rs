/// Partially parse an input.
pub trait PartialParse<Input>: Sized {
    /// Error that stopped the parsing process.
    type Error;

    /// Parse an input until completion or when an error occurs.
    ///
    /// The complete or partial result can be recovered regardless of errors.
    fn partial_parse(input: Input) -> PartialParseResult<Self, Self::Error>;
}

/// Partially parse an input with an issue handler.
pub trait ParseWithIssues<Input, HandleIssue, Error>: Sized {
    /// Parse an input until completion or when the issue handler returns an error.
    ///
    /// The complete or partial result can be recovered regardless of errors.
    fn parse_with_issues(
        input: Input,
        handle_issue: HandleIssue,
    ) -> PartialParseResult<Self, Error>;
}

/// Result of a parsing process that may or may not be complete.
#[derive(Debug, Clone, Copy)]
pub struct PartialParseResult<Parsed, Error> {
    /// The result of the parsing process, either partial or complete.
    parsed: Parsed,
    /// Possible error encountered during parsing.
    error: Option<Error>,
}

impl<Parsed, Error> PartialParseResult<Parsed, Error> {
    /// Create a complete parse result.
    pub fn new_complete(parsed: Parsed) -> Self {
        PartialParseResult {
            parsed,
            error: None,
        }
    }

    /// Create partial parse result.
    pub fn new_partial(parsed: Parsed, error: Error) -> Self {
        PartialParseResult {
            parsed,
            error: Some(error),
        }
    }

    /// Return an `Ok` if there was no error.
    ///
    /// Otherwise, return an `Err`.
    pub fn try_into_complete(self) -> Result<Parsed, Error> {
        match self.error {
            Some(error) => Err(error),
            None => Ok(self.parsed),
        }
    }

    /// Return both the parsed object and the error regardless of whether there was an error.
    pub fn into_partial(self) -> (Parsed, Option<Error>) {
        (self.parsed, self.error)
    }

    /// Return a reference to the partially parsed object.
    pub fn parsed(&self) -> &'_ Parsed {
        &self.parsed
    }

    /// Return a reference to the stored error if there was one.
    pub fn error(&self) -> Option<&'_ Error> {
        self.error.as_ref()
    }

    /// Whether there was an error.
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
}

impl<Parsed, Error> From<PartialParseResult<Parsed, Error>> for Result<Parsed, Error> {
    fn from(value: PartialParseResult<Parsed, Error>) -> Self {
        value.try_into_complete()
    }
}

impl<Parsed, Error> From<PartialParseResult<Parsed, Error>> for (Parsed, Option<Error>) {
    fn from(value: PartialParseResult<Parsed, Error>) -> Self {
        value.into_partial()
    }
}

impl<Parsed, Error> From<(Parsed, Option<Error>)> for PartialParseResult<Parsed, Error> {
    fn from((parsed, error): (Parsed, Option<Error>)) -> Self {
        PartialParseResult { parsed, error }
    }
}
