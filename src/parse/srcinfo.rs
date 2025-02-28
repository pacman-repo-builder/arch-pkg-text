mod checksums;
mod data;

use super::{ParseWithIssues, PartialParse, PartialParseResult};
use crate::{
    srcinfo::{
        Field, FieldName, ParsedField, RawField, Section,
        misc::{ReuseAdvice, True},
        utils::{non_blank_trimmed_lines, parse_line},
    },
    value,
};
use derive_more::{Display, Error};
use indexmap::IndexMap;
use pipe_trait::Pipe;

pub use data::{
    ParsedSrcinfoBaseSection, ParsedSrcinfoBaseUniqueFieldDuplicationError,
    ParsedSrcinfoDerivativeSection, ParsedSrcinfoDerivativeUniqueFieldDuplicationError,
};

/// Parsed information of `.SRCINFO`.
#[derive(Debug, Default, Clone)]
pub struct ParsedSrcinfo<'a> {
    /// The section under `pkgbase`.
    pub base: ParsedSrcinfoBaseSection<'a>,
    /// The sections under `pkgname`.
    pub derivatives: IndexMap<value::Name<'a>, ParsedSrcinfoDerivativeSection<'a>>,
}

/// Write cursor of the sections in [`ParsedSrcinfo`].
enum ParsedSrcinfoSectionMut<'a, 'r> {
    /// Write to the `pkgbase` section.
    Base(&'r mut ParsedSrcinfoBaseSection<'a>),
    /// Write to a `pkgname` section.
    Derivative(ParsedSrcinfoDerivativeSectionEntryMut<'a, 'r>),
}

impl<'a> ParsedSrcinfo<'a> {
    /// Get a section or create one if didn't exist.
    fn get_or_insert(&mut self, section: Section<'a>) -> ParsedSrcinfoSectionMut<'a, '_> {
        match section {
            Section::Base => self.base.pipe_mut(ParsedSrcinfoSectionMut::Base),
            Section::Derivative(name) => self
                .derivatives
                .entry(name)
                .or_default()
                .pipe(|data| ParsedSrcinfoDerivativeSectionEntryMut::new(name, data))
                .pipe(ParsedSrcinfoSectionMut::Derivative),
        }
    }
}

/// Private error type for control flow.
enum AddFailure<'a> {
    /// Meet an entry with field `pkgname`.
    MeetHeader(value::Name<'a>),
    /// Meet an issue.
    Issue(SrcinfoParseIssue<'a>),
}

/// Create an [`Err`] of an [`AddFailure::Issue`] of a [`SrcinfoParseIssue::UnknownField`] of a [`ParsedField`].
fn unknown_field_from_parsed(field: ParsedField<&str>) -> Result<(), AddFailure<'_>> {
    Field::blank()
        .with_name(field.name_str())
        .with_architecture(field.architecture_str())
        .pipe(SrcinfoParseIssue::UnknownField)
        .pipe(AddFailure::Issue)
        .pipe(Err)
}

impl<'a> ParsedSrcinfoSectionMut<'a, '_> {
    /// Add an entry to a `pkgbase` or `pkgname` section.
    fn add(&mut self, field: ParsedField<&'a str>, value: &'a str) -> Result<(), AddFailure<'a>> {
        match self {
            ParsedSrcinfoSectionMut::Base(section) => section.add(field, value),
            ParsedSrcinfoSectionMut::Derivative(section) => section.add(field, value),
        }
    }

    /// Shrink all internal containers' capacities to fit.
    fn shrink_to_fit(&mut self) {
        match self {
            ParsedSrcinfoSectionMut::Base(section) => section.shrink_to_fit(),
            ParsedSrcinfoSectionMut::Derivative(section) => section.shrink_to_fit(),
        }
    }
}

impl<'a> ParsedSrcinfoBaseSection<'a> {
    /// Add a value to a unique entry.
    fn add_value_to_option<Value: Copy>(
        target: &mut Option<Value>,
        value: &'a str,
        make_value: impl FnOnce(&'a str) -> Value,
        make_error: impl FnOnce(Value) -> ParsedSrcinfoBaseUniqueFieldDuplicationError<'a>,
    ) -> Result<(), AddFailure<'a>> {
        let Some(old_value) = target else {
            *target = Some(make_value(value));
            return Ok(());
        };
        (*old_value)
            .pipe(make_error)
            .pipe(SrcinfoParseIssue::BaseUniqueFieldDuplication)
            .pipe(AddFailure::Issue)
            .pipe(Err)
    }
}

/// A pair of [`value::Name`] and [`ParsedSrcinfoDerivativeSection`].
struct ParsedSrcinfoDerivativeSectionEntryMut<'a, 'r> {
    name: value::Name<'a>,
    data: &'r mut ParsedSrcinfoDerivativeSection<'a>,
}

impl<'a, 'r> ParsedSrcinfoDerivativeSectionEntryMut<'a, 'r> {
    /// Create a new pair.
    fn new(name: value::Name<'a>, data: &'r mut ParsedSrcinfoDerivativeSection<'a>) -> Self {
        ParsedSrcinfoDerivativeSectionEntryMut { name, data }
    }

    /// Add a value to a unique entry.
    fn add_value_to_option<Value: Copy>(
        name: value::Name<'a>,
        target: &mut Option<Value>,
        value: &'a str,
        make_value: impl FnOnce(&'a str) -> Value,
        make_error: impl FnOnce(Value) -> ParsedSrcinfoDerivativeUniqueFieldDuplicationError<'a>,
    ) -> Result<(), AddFailure<'a>> {
        let Some(old_value) = target else {
            *target = Some(make_value(value));
            return Ok(());
        };
        (*old_value)
            .pipe(make_error)
            .pipe(move |error| SrcinfoParseIssue::DerivativeUniqueFieldDuplication(name, error))
            .pipe(AddFailure::Issue)
            .pipe(Err)
    }

    /// Shrink all internal containers' capacities to fit.
    fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit()
    }
}

/// Error type of [`ParsedSrcinfo::parse`].
#[derive(Debug, Display, Error, Clone, Copy)]
pub enum SrcinfoParseError<'a> {
    #[display("Failed to insert value to the pkgbase section: {_0}")]
    BaseUniqueFieldDuplication(
        #[error(not(source))] ParsedSrcinfoBaseUniqueFieldDuplicationError<'a>,
    ),
    #[display("Failed to insert value to the pkgname section named {_0}: {_1}")]
    DerivativeUniqueFieldDuplication(
        value::Name<'a>,
        ParsedSrcinfoDerivativeUniqueFieldDuplicationError<'a>,
    ),
    #[display("Invalid line: {_0:?}")]
    InvalidLine(#[error(not(source))] &'a str),
}

/// Return type of [`ParsedSrcinfo::parse`].
pub type SrcinfoParseReturn<'a> = PartialParseResult<ParsedSrcinfo<'a>, SrcinfoParseError<'a>>;

/// Issue that may arise during parsing.
#[derive(Debug, Clone, Copy)]
pub enum SrcinfoParseIssue<'a> {
    UnknownField(RawField<'a>),
    BaseUniqueFieldDuplication(ParsedSrcinfoBaseUniqueFieldDuplicationError<'a>),
    DerivativeUniqueFieldDuplication(
        value::Name<'a>,
        ParsedSrcinfoDerivativeUniqueFieldDuplicationError<'a>,
    ),
    InvalidLine(&'a str),
}

impl<'a> SrcinfoParseIssue<'a> {
    /// Return `Ok(())` if the issue was [`SrcinfoParseIssue::UnknownField`],
    /// or return an `Err` of [`SrcinfoParseError`] otherwise.
    fn ignore_unknown_field(self) -> Result<(), SrcinfoParseError<'a>> {
        Err(match self {
            SrcinfoParseIssue::UnknownField(_) => return Ok(()),
            SrcinfoParseIssue::BaseUniqueFieldDuplication(error) => {
                SrcinfoParseError::BaseUniqueFieldDuplication(error)
            }
            SrcinfoParseIssue::DerivativeUniqueFieldDuplication(name, error) => {
                SrcinfoParseError::DerivativeUniqueFieldDuplication(name, error)
            }
            SrcinfoParseIssue::InvalidLine(line) => SrcinfoParseError::InvalidLine(line),
        })
    }
}

impl<'a> ParsedSrcinfo<'a> {
    /// Parse `.SRCINFO` text, unknown fields are ignored.
    pub fn parse(text: &'a str) -> SrcinfoParseReturn<'a> {
        ParsedSrcinfo::parse_with_issues(text, SrcinfoParseIssue::ignore_unknown_field)
    }

    /// Parse `.SRCINFO` with a callback that handles [parsing issues](SrcinfoParseIssue).
    pub fn parse_with_issues<HandleIssue, Error>(
        text: &'a str,
        mut handle_issue: HandleIssue,
    ) -> PartialParseResult<ParsedSrcinfo<'a>, Error>
    where
        HandleIssue: FnMut(SrcinfoParseIssue<'a>) -> Result<(), Error>,
    {
        let mut parsed = ParsedSrcinfo::default();
        let lines = non_blank_trimmed_lines(text);
        let mut section_mut = parsed.get_or_insert(Section::Base);

        macro_rules! return_or_continue {
            ($issue:expr) => {
                match handle_issue($issue) {
                    Err(error) => return PartialParseResult::new_partial(parsed, error),
                    Ok(()) => continue,
                }
            };
        }

        for line in lines {
            let Some((field, value)) = parse_line(line) else {
                return_or_continue!(SrcinfoParseIssue::InvalidLine(line));
            };
            let Ok(field) = field.to_parsed::<FieldName, &str>() else {
                return_or_continue!(SrcinfoParseIssue::UnknownField(field));
            };
            if value.is_empty() {
                continue;
            }
            match section_mut.add(field, value) {
                Ok(()) => {}
                Err(AddFailure::MeetHeader(name)) => {
                    section_mut.shrink_to_fit();
                    section_mut = parsed.get_or_insert(Section::Derivative(name));
                }
                Err(AddFailure::Issue(issue)) => {
                    return_or_continue!(issue);
                }
            }
        }

        PartialParseResult::new_complete(parsed)
    }
}

impl<'a> PartialParse<&'a str> for ParsedSrcinfo<'a> {
    type Error = SrcinfoParseError<'a>;
    fn partial_parse(input: &'a str) -> PartialParseResult<Self, Self::Error> {
        ParsedSrcinfo::parse(input)
    }
}

impl<'a, HandleIssue, Error> ParseWithIssues<&'a str, HandleIssue, Error> for ParsedSrcinfo<'a>
where
    HandleIssue: FnMut(SrcinfoParseIssue<'a>) -> Result<(), Error>,
{
    fn parse_with_issues(
        input: &'a str,
        handle_issue: HandleIssue,
    ) -> PartialParseResult<Self, Error> {
        ParsedSrcinfo::parse_with_issues(input, handle_issue)
    }
}

/// Try parsing a `.SRCINFO` text, unknown fields are ignored, partial success means error.
impl<'a> TryFrom<&'a str> for ParsedSrcinfo<'a> {
    /// Error that occurs when parsing fails or incomplete.
    type Error = SrcinfoParseError<'a>;
    /// Try parsing a `.SRCINFO` text, unknown fields are ignored, partial success means error.
    fn try_from(text: &'a str) -> Result<Self, Self::Error> {
        ParsedSrcinfo::parse(text).try_into_complete()
    }
}

impl ReuseAdvice for ParsedSrcinfo<'_> {
    /// [`ParsedSrcinfo`] costs O(n) time to construct (n being text length).
    /// Performing a lookup on it costs O(1) time.
    ///
    /// This struct is designed to be reused.
    type ShouldReuse = True;
}
