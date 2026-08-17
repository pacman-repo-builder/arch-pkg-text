//! Issue: `DescParseIssue::EmptyInput` is reported for a non-empty input that contains no field.
//!
//! <https://github.com/pacman-repo-builder/arch-pkg-text/issues/46>

use arch_pkg_text::{
    ParsedDesc, QueryDesc,
    parse::{DescParseError, DescParseIssue},
    value::Name,
};
use core::convert::Infallible;
use pretty_assertions::assert_eq;

const DESC: &str = include_str!("fixtures/gnome-shell.desc");

/// Name of the variant of a [`DescParseIssue`], to be recorded by the issue handlers below.
fn issue_name(issue: DescParseIssue<'_>) -> &'static str {
    match issue {
        DescParseIssue::NoField => "NoField",
        DescParseIssue::FirstLineIsNotAField(_, _) => "FirstLineIsNotAField",
        DescParseIssue::UnknownField(_) => "UnknownField",
    }
}

/// Parse `text` with a handler that tolerates every issue,
/// returning the issues that were reported.
fn issues_of(text: &str) -> Vec<&'static str> {
    let mut issues = Vec::new();
    ParsedDesc::parse_with_issues(text, |issue| -> Result<(), Infallible> {
        issues.push(issue_name(issue));
        Ok(())
    })
    .try_into_complete()
    .unwrap();
    issues
}

/// Parse `text` with a handler that tolerates only [`DescParseIssue::FirstLineIsNotAField`],
/// returning the error that stopped the parsing process.
fn error_of(text: &str) -> Option<DescParseError<'_>> {
    ParsedDesc::parse_with_issues(text, |issue| match issue {
        DescParseIssue::FirstLineIsNotAField(_, _) => Ok(()),
        issue => issue.ignore_unknown_field(),
    })
    .into_partial()
    .1
}

/// An input whose lines are all non-fields isn't empty, so it must not be described as empty.
#[test]
fn field_less_input_is_not_described_as_empty() {
    assert_eq!(
        issues_of("not a field\n"),
        ["FirstLineIsNotAField", "NoField"],
    );
    let error = dbg!(error_of("not a field\n")).unwrap();
    assert!(matches!(error, DescParseError::NoField));
    assert_eq!(error.to_string(), "Input has no field");
}

/// Blank lines are lines, too, so an input made of them isn't empty either.
#[test]
fn blank_lines_only_input_is_not_described_as_empty() {
    assert_eq!(
        issues_of("\n\n"),
        ["FirstLineIsNotAField", "FirstLineIsNotAField", "NoField"],
    );
    let error = dbg!(error_of("\n\n")).unwrap();
    assert!(matches!(error, DescParseError::NoField));
    assert_eq!(error.to_string(), "Input has no field");
}

/// An empty input has no field either, so it reports the very same issue and error.
#[test]
fn empty_input_reports_the_same_issue() {
    assert_eq!(issues_of(""), ["NoField"]);
    let error = dbg!(error_of("")).unwrap();
    assert!(matches!(error, DescParseError::NoField));
    assert_eq!(error.to_string(), "Input has no field");
}

/// An input that does contain a field reports neither the issue nor the error,
/// no matter what precedes that field.
#[test]
fn input_with_field_reports_no_issue() {
    assert_eq!(issues_of(DESC), [] as [&str; 0]);
    let text = format!("garbage line\n{DESC}");
    assert_eq!(issues_of(&text), ["FirstLineIsNotAField"]);
    assert!(error_of(&text).is_none());
}

/// The default handler is unaffected: it stops at the first non-field line.
#[test]
fn default_handler_is_unaffected() {
    let error = dbg!(ParsedDesc::parse("not a field\n")).unwrap_err();
    assert!(matches!(
        error,
        DescParseError::ValueWithoutField("not a field\n"),
    ));
    assert_eq!(
        error.to_string(),
        r#"Receive a value without field: "not a field\n""#,
    );
    assert_eq!(
        ParsedDesc::parse(DESC).unwrap().name(),
        Some(Name("gnome-shell")),
    );
}
