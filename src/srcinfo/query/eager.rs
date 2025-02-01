mod data;

use super::{
    utils::{non_blank_trimmed_lines, parse_line},
    Section,
};
use crate::{
    srcinfo::field::{FieldName, ParsedField},
    value,
};
use data::EagerDerivativeSectionEntry;
use derive_more::{Display, Error};
use pipe_trait::Pipe;
use std::collections::HashMap;

pub use data::{
    EagerBaseAlreadySetError, EagerBaseSection, EagerDerivativeAlreadySetError,
    EagerDerivativeSection,
};

#[derive(Debug, Default, Clone)]
pub struct EagerQuerier<'a> {
    pub base: EagerBaseSection<'a>,
    pub derivatives: HashMap<value::Name<'a>, EagerDerivativeSection<'a>>,
}

enum EagerSectionMut<'a, 'r> {
    Base(&'r mut EagerBaseSection<'a>),
    Derivative(EagerDerivativeSectionEntry<'a, 'r>),
}

impl<'a> EagerQuerier<'a> {
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
    Error(EagerQuerierParseError<'a>),
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
pub enum EagerQuerierParseError<'a> {
    #[display("Failed to insert value to the pkgbase section: {_0}")]
    BaseFieldAlreadySet(#[error(not(source))] EagerBaseAlreadySetError<'a>),
    #[display("Failed to insert value to the pkgname section named {_0}: {_1}")]
    DerivativeFieldAlreadySet(value::Name<'a>, EagerDerivativeAlreadySetError<'a>),
    #[display("Invalid line: {_0:?}")]
    InvalidLine(#[error(not(source))] &'a str),
}

/// Return type of [`EagerQuerier::parse`].
#[derive(Debug, Clone)]
pub struct EagerQuerierParseReturn<'a> {
    /// The result of the parsing process, either partial or complete.
    querier: EagerQuerier<'a>,
    /// Possible error encountered during parsing.
    error: Option<EagerQuerierParseError<'a>>,
}

impl<'a> EagerQuerierParseReturn<'a> {
    /// Return an `Ok` of [`EagerQuerier`] if there was no error.
    ///
    /// Otherwise, return an `Err` of [`EagerQuerierParseError`].
    pub fn into_complete(self) -> Result<EagerQuerier<'a>, EagerQuerierParseError<'a>> {
        match self.error {
            Some(error) => Err(error),
            None => Ok(self.querier),
        }
    }

    /// Return both the parsed querier and the error regardless of whether there was an error.
    pub fn into_partial(self) -> (EagerQuerier<'a>, Option<EagerQuerierParseError<'a>>) {
        (self.querier, self.error)
    }

    /// Return a reference to the stored error if there was one.
    pub fn error(&self) -> Option<&'_ EagerQuerierParseError<'a>> {
        self.error.as_ref()
    }
}

impl<'a> EagerQuerier<'a> {
    pub fn new(text: &'a str) -> Self {
        text.pipe(EagerQuerier::parse).into_partial().0
    }

    pub fn parse(text: &'a str) -> EagerQuerierParseReturn<'a> {
        let mut querier = EagerQuerier::default();
        let lines = non_blank_trimmed_lines(text);
        let mut section_mut = querier.get_or_insert(Section::Base);

        for line in lines {
            let Some((field, value)) = parse_line(line) else {
                return EagerQuerierParseReturn {
                    querier,
                    error: line.pipe(EagerQuerierParseError::InvalidLine).pipe(Some),
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
                    section_mut = querier.get_or_insert(Section::Derivative(name));
                }
                Err(AddFailure::Error(error)) => {
                    return EagerQuerierParseReturn {
                        querier,
                        error: Some(error),
                    };
                }
            }
        }

        EagerQuerierParseReturn {
            querier,
            error: None,
        }
    }
}
