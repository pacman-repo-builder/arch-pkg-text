//! Issue: `ForgetfulQuerier` and `MemoQuerier` leak whitespace into `desc` values,
//! disagreeing with `ParsedDesc`.
//!
//! <https://github.com/pacman-repo-builder/arch-pkg-text/issues/47>

pub mod _utils;
pub use _utils::*;

use arch_pkg_text::{
    ParsedDesc, QueryDesc, QueryDescMut,
    desc::{ForgetfulQuerier, MemoQuerier},
    value::{Description, Name},
};
use pretty_assertions::assert_eq;

/// Assert that the three queriers agree on `%NAME%` and `%DESC%`.
#[track_caller]
fn assert_agreement(text: &str, name: Option<&str>, description: Option<&str>) {
    let name = name.map(Name);
    let description = description.map(Description);

    eprintln!("CASE: ParsedDesc");
    let parsed = ParsedDesc::parse(text).unwrap();
    assert_eq!(parsed.name(), name);
    assert_eq!(parsed.description(), description);

    eprintln!("CASE: ForgetfulQuerier");
    let forgetful = ForgetfulQuerier::new(text);
    assert_eq!(forgetful.name(), name);
    assert_eq!(forgetful.description(), description);

    eprintln!("CASE: MemoQuerier");
    let mut memo = MemoQuerier::new(text);
    assert_eq!(memo.name_mut(), name);
    assert_eq!(memo.description_mut(), description);
}

#[test]
fn whitespace_on_the_field_line() {
    eprintln!("ASSERT: a space after the field name doesn't belong to the value");
    assert_agreement("%NAME% \nfoo\n", Some("foo"), None);

    eprintln!("ASSERT: a tab after the field name doesn't belong to the value");
    assert_agreement("%NAME%\t\nfoo\n", Some("foo"), None);

    eprintln!("ASSERT: an indent before the field name doesn't belong to the value");
    assert_agreement("  %NAME%\nfoo\n", Some("foo"), None);

    eprintln!("ASSERT: the indent of a field doesn't belong to the preceding value");
    assert_agreement("%NAME%\nfoo\n  %DESC%\nbar\n", Some("foo"), Some("bar"));
}

#[test]
fn whitespace_on_the_value_line() {
    eprintln!("ASSERT: a trailing space doesn't belong to the value");
    assert_agreement("%NAME%\nfoo \n", Some("foo"), None);

    eprintln!("ASSERT: a leading space doesn't belong to the value");
    assert_agreement("%NAME%\n foo\n", Some("foo"), None);

    eprintln!("ASSERT: a separator line of a single space doesn't belong to the value");
    assert_agreement("%NAME%\nfoo\n \n%DESC%\nbar\n", Some("foo"), Some("bar"));

    eprintln!("ASSERT: a value of nothing but whitespaces is empty, hence None");
    assert_agreement("%NAME%\n \n%DESC%\nbar\n", None, Some("bar"));
}

#[test]
fn whitespace_everywhere() {
    eprintln!("ASSERT: both causes at once");
    assert_agreement(
        "%NAME% \nfoo \n \n%DESC% \nbar \n",
        Some("foo"),
        Some("bar"),
    );
}

#[test]
fn numeric_value_still_parses() {
    let text = include_str!("fixtures/gnome-shell.desc").trailing_whitespaces();

    eprintln!("CASE: ParsedDesc");
    let parsed = ParsedDesc::parse(&text).unwrap();
    assert_eq!(parsed.installed_size().unwrap().as_str(), "14190669");
    assert_eq!(parsed.installed_size().unwrap().parse(), Ok(14190669));

    eprintln!("CASE: ForgetfulQuerier");
    let forgetful = ForgetfulQuerier::new(&text);
    assert_eq!(forgetful.installed_size().unwrap().as_str(), "14190669");
    assert_eq!(forgetful.installed_size().unwrap().parse(), Ok(14190669));

    eprintln!("CASE: MemoQuerier");
    let mut memo = MemoQuerier::new(&text);
    assert_eq!(memo.installed_size_mut().unwrap().as_str(), "14190669");
    assert_eq!(memo.installed_size_mut().unwrap().parse(), Ok(14190669));
}
