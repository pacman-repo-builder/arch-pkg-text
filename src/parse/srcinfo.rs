mod data;

use crate::{
    srcinfo::{
        field::{FieldName, ParsedField},
        query::{
            utils::{non_blank_trimmed_lines, parse_line},
            Section,
        },
    },
    value,
};
use data::EagerDerivativeSectionEntry;
use derive_more::{Display, Error};
use indexmap::IndexMap;
use pipe_trait::Pipe;

pub use data::{
    EagerBaseAlreadySetError, EagerBaseSection, EagerDerivativeAlreadySetError,
    EagerDerivativeSection,
};

#[derive(Debug, Default, Clone)]
pub struct ParsedSrcinfo<'a> {
    pub base: EagerBaseSection<'a>,
    pub derivatives: IndexMap<value::Name<'a>, EagerDerivativeSection<'a>>,
}

enum EagerSectionMut<'a, 'r> {
    Base(&'r mut EagerBaseSection<'a>),
    Derivative(EagerDerivativeSectionEntry<'a, 'r>),
}

impl<'a> ParsedSrcinfo<'a> {
    fn get_or_insert(&mut self, section: Section<'a>) -> EagerSectionMut<'a, '_> {
        match section {
            Section::Base => self.base.pipe_mut(EagerSectionMut::Base),
            Section::Derivative(name) => self
                .derivatives
                .entry(name)
                .or_default()
                .pipe(|data| EagerDerivativeSectionEntry::new(name, data))
                .pipe(EagerSectionMut::Derivative),
        }
    }
}

/// Private error type for control flow.
enum AddFailure<'a> {
    MeetHeader(value::Name<'a>),
    Error(SrcinfoParseError<'a>),
}

impl<'a, 'r> EagerSectionMut<'a, 'r> {
    fn add(&mut self, field: ParsedField<&'a str>, value: &'a str) -> Result<(), AddFailure<'a>> {
        match self {
            EagerSectionMut::Base(section) => section.add(field, value),
            EagerSectionMut::Derivative(section) => section.add(field, value),
        }
    }

    fn shrink_to_fit(&mut self) {
        match self {
            EagerSectionMut::Base(section) => section.shrink_to_fit(),
            EagerSectionMut::Derivative(section) => section.shrink_to_fit(),
        }
    }
}

#[derive(Debug, Display, Error, Clone, Copy)]
pub enum SrcinfoParseError<'a> {
    #[display("Failed to insert value to the pkgbase section: {_0}")]
    BaseFieldAlreadySet(#[error(not(source))] EagerBaseAlreadySetError<'a>),
    #[display("Failed to insert value to the pkgname section named {_0}: {_1}")]
    DerivativeFieldAlreadySet(value::Name<'a>, EagerDerivativeAlreadySetError<'a>),
    #[display("Invalid line: {_0:?}")]
    InvalidLine(#[error(not(source))] &'a str),
}

/// Return type of [`ParsedSrcinfo::parse`].
#[derive(Debug, Clone)]
pub struct SrcinfoParseReturn<'a> {
    /// The result of the parsing process, either partial or complete.
    parsed: ParsedSrcinfo<'a>,
    /// Possible error encountered during parsing.
    error: Option<SrcinfoParseError<'a>>,
}

impl<'a> SrcinfoParseReturn<'a> {
    /// Return an `Ok` of [`ParsedSrcinfo`] if there was no error.
    ///
    /// Otherwise, return an `Err` of [`SrcinfoParseError`].
    pub fn into_complete(self) -> Result<ParsedSrcinfo<'a>, SrcinfoParseError<'a>> {
        match self.error {
            Some(error) => Err(error),
            None => Ok(self.parsed),
        }
    }

    /// Return both the parsed querier and the error regardless of whether there was an error.
    pub fn into_partial(self) -> (ParsedSrcinfo<'a>, Option<SrcinfoParseError<'a>>) {
        (self.parsed, self.error)
    }

    /// Return a reference to the stored error if there was one.
    pub fn error(&self) -> Option<&'_ SrcinfoParseError<'a>> {
        self.error.as_ref()
    }
}

impl<'a> ParsedSrcinfo<'a> {
    pub fn new(text: &'a str) -> Self {
        text.pipe(ParsedSrcinfo::parse).into_partial().0
    }

    pub fn parse(text: &'a str) -> SrcinfoParseReturn<'a> {
        let mut parsed = ParsedSrcinfo::default();
        let lines = non_blank_trimmed_lines(text);
        let mut section_mut = parsed.get_or_insert(Section::Base);

        for line in lines {
            let Some((field, value)) = parse_line(line) else {
                return SrcinfoParseReturn {
                    parsed,
                    error: line.pipe(SrcinfoParseError::InvalidLine).pipe(Some),
                };
            };
            let Ok(field) = field.to_parsed::<FieldName, &str>() else {
                continue;
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
                Err(AddFailure::Error(error)) => {
                    return SrcinfoParseReturn {
                        parsed,
                        error: Some(error),
                    };
                }
            }
        }

        SrcinfoParseReturn {
            parsed,
            error: None,
        }
    }
}
