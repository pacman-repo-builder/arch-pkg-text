//! Issue: `ParsedDesc::parse_with_issues` loops forever or corrupts every value
//! when the handler tolerates an issue.
//!
//! <https://github.com/pacman-repo-builder/arch-pkg-text/issues/42>

use arch_pkg_text::{
    ParsedDesc, QueryDesc,
    parse::DescParseIssue,
    value::{Architecture, Description, FileName, Name},
};
use core::convert::Infallible;
use pretty_assertions::assert_eq;

const DESC: &str = include_str!("fixtures/gnome-shell.desc");

/// Name of the variant of a [`DescParseIssue`], to be recorded by the issue handlers below.
fn issue_name(issue: DescParseIssue<'_>) -> &'static str {
    match issue {
        DescParseIssue::EmptyInput => "EmptyInput",
        DescParseIssue::FirstLineIsNotAField(_, _) => "FirstLineIsNotAField",
        DescParseIssue::UnknownField(_) => "UnknownField",
    }
}

/// Number of issues after which the parser is deemed to not terminate.
const MAX_ISSUES: usize = 1000;

/// Parse `text` with a handler that tolerates every issue,
/// returning the parsed data alongside the issues that were reported.
fn parse_tolerantly(text: &str) -> (ParsedDesc<'_>, Vec<&'static str>) {
    let mut issues = Vec::new();
    let parsed = ParsedDesc::parse_with_issues(text, |issue| -> Result<(), Infallible> {
        issues.push(issue_name(issue));
        if issues.len() > MAX_ISSUES {
            panic!("The issue handler was called more than {MAX_ISSUES} times");
        }
        Ok(())
    })
    .try_into_complete()
    .unwrap();
    (parsed, issues)
}

fn assert_query<'a>(querier: &impl QueryDesc<'a>) {
    assert_eq!(querier.name(), Some(Name("gnome-shell")));
    assert_eq!(
        querier.file_name(),
        Some(FileName("gnome-shell-1:46.2-1-x86_64.pkg.tar.zst")),
    );
    assert_eq!(
        querier.description(),
        Some(Description("Next generation desktop shell")),
    );
    assert_eq!(
        querier.installed_size().map(|value| value.as_str()),
        Some("14190669"),
    );

    let mut architecture = querier.architecture().unwrap().into_iter();
    assert_eq!(architecture.next(), Some(Architecture("x86_64")));
    assert_eq!(architecture.next(), None);
}

/// An empty input has nothing left to parse, so a tolerated `EmptyInput` must end the parse.
#[test]
fn empty_input_terminates() {
    let (parsed, issues) = parse_tolerantly("");
    assert_eq!(issues, ["EmptyInput"]);
    assert_eq!(parsed.name(), None);
}

/// An input without any field line must terminate, too.
#[test]
fn input_without_field_terminates() {
    let (parsed, issues) = parse_tolerantly("not a field\nneither is this\n");
    assert_eq!(
        issues,
        ["FirstLineIsNotAField", "FirstLineIsNotAField", "EmptyInput"],
    );
    assert_eq!(parsed.name(), None);
}

/// A skipped leading line must not shift the offsets of the values that follow it.
#[test]
fn leading_garbage_does_not_shift_values() {
    let text = format!("garbage line\n{DESC}");
    let (parsed, issues) = parse_tolerantly(&text);
    assert_eq!(issues, ["FirstLineIsNotAField"]);
    assert_query(&parsed);
}

/// Several skipped leading lines must not shift the offsets either.
#[test]
fn multiple_leading_garbage_lines_do_not_shift_values() {
    let text = format!("garbage line\nanother garbage line\n\n{DESC}");
    let (parsed, issues) = parse_tolerantly(&text);
    assert_eq!(
        issues,
        [
            "FirstLineIsNotAField",
            "FirstLineIsNotAField",
            "FirstLineIsNotAField",
        ],
    );
    assert_query(&parsed);
}

/// A shifted offset used to land inside a multi-byte character and panic.
#[test]
fn non_ascii_leading_garbage_does_not_panic() {
    let text = format!("日本語のゴミ\n{DESC}");
    let (parsed, issues) = parse_tolerantly(&text);
    assert_eq!(issues, ["FirstLineIsNotAField"]);
    assert_query(&parsed);
}

/// The issue handler may still stop the parsing process by returning an `Err`.
#[test]
fn intolerant_handler_still_stops_the_parse() {
    let (parsed, error) =
        ParsedDesc::parse_with_issues("", |issue| Err::<(), _>(issue_name(issue))).into_partial();
    assert_eq!(error, Some("EmptyInput"));
    assert_eq!(parsed.name(), None);

    let (parsed, error) =
        ParsedDesc::parse_with_issues("not a field\n", |issue| Err::<(), _>(issue_name(issue)))
            .into_partial();
    assert_eq!(error, Some("FirstLineIsNotAField"));
    assert_eq!(parsed.name(), None);
}

/// Tolerating an issue must not change the result of an input that has none.
#[test]
fn well_formed_input_is_unaffected() {
    let (parsed, error) =
        ParsedDesc::parse_with_issues(DESC, |_| Ok::<(), Infallible>(())).into_partial();
    assert!(error.is_none());
    assert_query(&parsed);
    assert_eq!(ParsedDesc::parse(DESC).unwrap().name(), parsed.name());
}
