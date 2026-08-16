//! Issue: The three `desc` queriers disagree on empty values and on duplicate fields.
//!
//! <https://github.com/pacman-repo-builder/arch-pkg-text/issues/43>

use arch_pkg_text::{
    ParsedDesc, QueryDescMut,
    desc::{ForgetfulQuerier, MemoQuerier},
    value::{ArchitectureList, GroupList},
};
use pretty_assertions::assert_eq;
use text_block_macros::text_block_fnl;

/// Run the same assertions against all 3 queriers of a `desc` file text.
///
/// Every querier is constructed anew, so that the assertions of one querier
/// don't interfere with the assertions of another.
fn assert_all_queriers<'a>(text: &'a str, assert: impl Fn(&str, &mut dyn QueryDescMut<'a>)) {
    assert("ParsedDesc", &mut ParsedDesc::parse(text).unwrap());
    assert("ForgetfulQuerier", &mut ForgetfulQuerier::new(text));
    assert("MemoQuerier", &mut MemoQuerier::new(text));
}

/// List the groups as strings.
fn groups(list: GroupList<'_>) -> Vec<&'_ str> {
    list.into_iter().map(|item| item.as_str()).collect()
}

/// List the architectures as strings.
fn architectures(list: ArchitectureList<'_>) -> Vec<&'_ str> {
    list.into_iter().map(|item| item.as_str()).collect()
}

#[test]
fn empty_value_in_the_middle() {
    let text = text_block_fnl! {
        "%NAME%"
        "foo"
        ""
        "%GROUPS%"
        ""
        "%ARCH%"
        "x86_64"
    };
    assert_all_queriers(text, |querier_name, querier| {
        assert_eq!(
            querier.name_mut().map(|value| value.as_str()),
            Some("foo"),
            "{querier_name}: %NAME% precedes the empty %GROUPS%",
        );
        assert_eq!(
            querier.groups_mut().map(groups),
            None,
            "{querier_name}: %GROUPS% is empty, hence None",
        );
        assert_eq!(
            querier.architecture_mut().map(architectures),
            Some(vec!["x86_64"]),
            "{querier_name}: %ARCH% follows the empty %GROUPS%",
        );
    });
}

#[test]
fn empty_value_at_the_end() {
    let text = text_block_fnl! {
        "%NAME%"
        "foo"
        ""
        "%GROUPS%"
    };
    assert_all_queriers(text, |querier_name, querier| {
        assert_eq!(
            querier.name_mut().map(|value| value.as_str()),
            Some("foo"),
            "{querier_name}: %NAME% precedes the empty %GROUPS%",
        );
        assert_eq!(
            querier.groups_mut().map(groups),
            None,
            "{querier_name}: %GROUPS% is empty and last, hence None",
        );
    });
}

#[test]
fn duplicate_field() {
    let text = text_block_fnl! {
        "%NAME%"
        "first"
        ""
        "%ARCH%"
        "x86_64"
        ""
        "%NAME%"
        "second"
    };
    assert_all_queriers(text, |querier_name, querier| {
        assert_eq!(
            querier.name_mut().map(|value| value.as_str()),
            Some("first"),
            "{querier_name}: %NAME% occurs twice, the first occurrence wins",
        );
    });
}

#[test]
fn duplicate_field_after_querying_a_later_field() {
    let text = text_block_fnl! {
        "%NAME%"
        "first"
        ""
        "%ARCH%"
        "x86_64"
        ""
        "%NAME%"
        "second"
        ""
        "%DESC%"
        "hello"
    };
    assert_all_queriers(text, |querier_name, querier| {
        assert_eq!(
            querier.description_mut().map(|value| value.as_str()),
            Some("hello"),
            "{querier_name}: %DESC% lies behind the second %NAME%",
        );
        assert_eq!(
            querier.name_mut().map(|value| value.as_str()),
            Some("first"),
            "{querier_name}: %NAME% must not change after the whole text was scanned",
        );
    });
}

#[test]
fn duplicate_field_whose_first_occurrence_is_empty() {
    let text = text_block_fnl! {
        "%GROUPS%"
        ""
        "%NAME%"
        "foo"
        ""
        "%GROUPS%"
        "later"
    };

    assert_all_queriers(text, |querier_name, querier| {
        assert_eq!(
            querier.groups_mut().map(groups),
            None,
            "{querier_name}: the first %GROUPS% is empty, hence None",
        );
    });

    assert_all_queriers(text, |querier_name, querier| {
        assert_eq!(
            querier.name_mut().map(|value| value.as_str()),
            Some("foo"),
            "{querier_name}: %NAME% queried before %GROUPS%",
        );
        assert_eq!(
            querier.groups_mut().map(groups),
            None,
            "{querier_name}: the first %GROUPS% is empty, hence None",
        );
    });
}
