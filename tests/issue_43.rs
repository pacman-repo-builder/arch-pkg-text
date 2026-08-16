//! Issue: The three `desc` queriers disagree on empty values and on duplicate fields.
//!
//! <https://github.com/pacman-repo-builder/arch-pkg-text/issues/43>

use arch_pkg_text::{
    ParsedDesc, QueryDescMut,
    desc::{ForgetfulQuerier, MemoQuerier},
    value::{Architecture, Description, Name},
};
use pretty_assertions::assert_eq;
use text_block_macros::text_block_fnl;

const EMPTY_VALUE_IN_THE_MIDDLE: &str = text_block_fnl! {
    "%NAME%"
    "foo"
    ""
    "%GROUPS%"
    ""
    "%ARCH%"
    "x86_64"
};

const EMPTY_VALUE_AT_THE_END: &str = text_block_fnl! {
    "%NAME%"
    "foo"
    ""
    "%GROUPS%"
};

const DUPLICATE_FIELD: &str = text_block_fnl! {
    "%NAME%"
    "first"
    ""
    "%ARCH%"
    "x86_64"
    ""
    "%NAME%"
    "second"
};

const DUPLICATE_FIELD_FOLLOWED_BY_ANOTHER_FIELD: &str = text_block_fnl! {
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

const DUPLICATE_FIELD_WITH_EMPTY_FIRST_OCCURRENCE: &str = text_block_fnl! {
    "%GROUPS%"
    ""
    "%NAME%"
    "foo"
    ""
    "%GROUPS%"
    "later"
};

#[test]
fn empty_value_in_the_middle() {
    fn run_assertions<'a>(querier: &mut impl QueryDescMut<'a>) {
        eprintln!("ASSERT: %NAME% precedes the empty %GROUPS%");
        assert_eq!(querier.name_mut(), Some(Name("foo")));

        eprintln!("ASSERT: %GROUPS% is empty, hence None");
        assert!(querier.groups_mut().is_none());

        eprintln!("ASSERT: %ARCH% follows the empty %GROUPS%");
        let mut architecture = querier.architecture_mut().unwrap().into_iter();
        assert_eq!(architecture.next(), Some(Architecture("x86_64")));
        assert_eq!(architecture.next(), None);
    }

    eprintln!("CASE: ParsedDesc");
    run_assertions(&mut ParsedDesc::parse(EMPTY_VALUE_IN_THE_MIDDLE).unwrap());

    eprintln!("CASE: ForgetfulQuerier");
    run_assertions(&mut ForgetfulQuerier::new(EMPTY_VALUE_IN_THE_MIDDLE));

    eprintln!("CASE: MemoQuerier");
    run_assertions(&mut MemoQuerier::new(EMPTY_VALUE_IN_THE_MIDDLE));
}

#[test]
fn empty_value_at_the_end() {
    fn run_assertions<'a>(querier: &mut impl QueryDescMut<'a>) {
        eprintln!("ASSERT: %NAME% precedes the empty %GROUPS%");
        assert_eq!(querier.name_mut(), Some(Name("foo")));

        eprintln!("ASSERT: %GROUPS% is empty and last, hence None");
        assert!(querier.groups_mut().is_none());
    }

    eprintln!("CASE: ParsedDesc");
    run_assertions(&mut ParsedDesc::parse(EMPTY_VALUE_AT_THE_END).unwrap());

    eprintln!("CASE: ForgetfulQuerier");
    run_assertions(&mut ForgetfulQuerier::new(EMPTY_VALUE_AT_THE_END));

    eprintln!("CASE: MemoQuerier");
    run_assertions(&mut MemoQuerier::new(EMPTY_VALUE_AT_THE_END));
}

#[test]
fn duplicate_field() {
    fn run_assertions<'a>(querier: &mut impl QueryDescMut<'a>) {
        eprintln!("ASSERT: %NAME% occurs twice, the first occurrence wins");
        assert_eq!(querier.name_mut(), Some(Name("first")));
    }

    eprintln!("CASE: ParsedDesc");
    run_assertions(&mut ParsedDesc::parse(DUPLICATE_FIELD).unwrap());

    eprintln!("CASE: ForgetfulQuerier");
    run_assertions(&mut ForgetfulQuerier::new(DUPLICATE_FIELD));

    eprintln!("CASE: MemoQuerier");
    run_assertions(&mut MemoQuerier::new(DUPLICATE_FIELD));
}

#[test]
fn duplicate_field_after_querying_a_later_field() {
    fn run_assertions<'a>(querier: &mut impl QueryDescMut<'a>) {
        eprintln!("ASSERT: %DESC% lies behind the second %NAME%");
        assert_eq!(querier.description_mut(), Some(Description("hello")));

        eprintln!("ASSERT: %NAME% must not change after the whole text was scanned");
        assert_eq!(querier.name_mut(), Some(Name("first")));
    }

    eprintln!("CASE: ParsedDesc");
    run_assertions(&mut ParsedDesc::parse(DUPLICATE_FIELD_FOLLOWED_BY_ANOTHER_FIELD).unwrap());

    eprintln!("CASE: ForgetfulQuerier");
    run_assertions(&mut ForgetfulQuerier::new(
        DUPLICATE_FIELD_FOLLOWED_BY_ANOTHER_FIELD,
    ));

    eprintln!("CASE: MemoQuerier");
    run_assertions(&mut MemoQuerier::new(
        DUPLICATE_FIELD_FOLLOWED_BY_ANOTHER_FIELD,
    ));
}

#[test]
fn duplicate_field_whose_first_occurrence_is_empty() {
    fn run_assertions<'a>(querier: &mut impl QueryDescMut<'a>) {
        eprintln!("ASSERT: the first %GROUPS% is empty, hence None");
        assert!(querier.groups_mut().is_none());
    }

    eprintln!("CASE: ParsedDesc");
    run_assertions(&mut ParsedDesc::parse(DUPLICATE_FIELD_WITH_EMPTY_FIRST_OCCURRENCE).unwrap());

    eprintln!("CASE: ForgetfulQuerier");
    run_assertions(&mut ForgetfulQuerier::new(
        DUPLICATE_FIELD_WITH_EMPTY_FIRST_OCCURRENCE,
    ));

    eprintln!("CASE: MemoQuerier");
    run_assertions(&mut MemoQuerier::new(
        DUPLICATE_FIELD_WITH_EMPTY_FIRST_OCCURRENCE,
    ));
}

#[test]
fn duplicate_field_whose_first_occurrence_is_empty_after_querying_another_field() {
    fn run_assertions<'a>(querier: &mut impl QueryDescMut<'a>) {
        eprintln!("ASSERT: %NAME% lies between the 2 occurrences of %GROUPS%");
        assert_eq!(querier.name_mut(), Some(Name("foo")));

        eprintln!("ASSERT: the first %GROUPS% is empty, hence None");
        assert!(querier.groups_mut().is_none());
    }

    eprintln!("CASE: ParsedDesc");
    run_assertions(&mut ParsedDesc::parse(DUPLICATE_FIELD_WITH_EMPTY_FIRST_OCCURRENCE).unwrap());

    eprintln!("CASE: ForgetfulQuerier");
    run_assertions(&mut ForgetfulQuerier::new(
        DUPLICATE_FIELD_WITH_EMPTY_FIRST_OCCURRENCE,
    ));

    eprintln!("CASE: MemoQuerier");
    run_assertions(&mut MemoQuerier::new(
        DUPLICATE_FIELD_WITH_EMPTY_FIRST_OCCURRENCE,
    ));
}
