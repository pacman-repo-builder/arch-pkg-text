//! Issue: The three `desc` queriers disagree on empty values and on duplicate fields.
//!
//! <https://github.com/pacman-repo-builder/arch-pkg-text/issues/43>

use arch_pkg_text::{
    ParsedDesc, QueryDescMut,
    desc::{ForgetfulQuerier, MemoQuerier},
    value::{Architecture, Description, Name},
};
use pipe_trait::Pipe;
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
    EMPTY_VALUE_IN_THE_MIDDLE
        .pipe(ParsedDesc::parse)
        .unwrap()
        .pipe_mut(run_assertions);

    eprintln!("CASE: ForgetfulQuerier");
    EMPTY_VALUE_IN_THE_MIDDLE
        .pipe(ForgetfulQuerier::new)
        .pipe_mut(run_assertions);

    eprintln!("CASE: MemoQuerier");
    EMPTY_VALUE_IN_THE_MIDDLE
        .pipe(MemoQuerier::new)
        .pipe_mut(run_assertions);
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
    EMPTY_VALUE_AT_THE_END
        .pipe(ParsedDesc::parse)
        .unwrap()
        .pipe_mut(run_assertions);

    eprintln!("CASE: ForgetfulQuerier");
    EMPTY_VALUE_AT_THE_END
        .pipe(ForgetfulQuerier::new)
        .pipe_mut(run_assertions);

    eprintln!("CASE: MemoQuerier");
    EMPTY_VALUE_AT_THE_END
        .pipe(MemoQuerier::new)
        .pipe_mut(run_assertions);
}

#[test]
fn duplicate_field() {
    fn run_assertions<'a>(querier: &mut impl QueryDescMut<'a>) {
        eprintln!("ASSERT: %NAME% occurs twice, the first occurrence wins");
        assert_eq!(querier.name_mut(), Some(Name("first")));
    }

    eprintln!("CASE: ParsedDesc");
    DUPLICATE_FIELD
        .pipe(ParsedDesc::parse)
        .unwrap()
        .pipe_mut(run_assertions);

    eprintln!("CASE: ForgetfulQuerier");
    DUPLICATE_FIELD
        .pipe(ForgetfulQuerier::new)
        .pipe_mut(run_assertions);

    eprintln!("CASE: MemoQuerier");
    DUPLICATE_FIELD
        .pipe(MemoQuerier::new)
        .pipe_mut(run_assertions);
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
    DUPLICATE_FIELD_FOLLOWED_BY_ANOTHER_FIELD
        .pipe(ParsedDesc::parse)
        .unwrap()
        .pipe_mut(run_assertions);

    eprintln!("CASE: ForgetfulQuerier");
    DUPLICATE_FIELD_FOLLOWED_BY_ANOTHER_FIELD
        .pipe(ForgetfulQuerier::new)
        .pipe_mut(run_assertions);

    eprintln!("CASE: MemoQuerier");
    DUPLICATE_FIELD_FOLLOWED_BY_ANOTHER_FIELD
        .pipe(MemoQuerier::new)
        .pipe_mut(run_assertions);
}

#[test]
fn duplicate_field_whose_first_occurrence_is_empty() {
    fn run_assertions<'a>(querier: &mut impl QueryDescMut<'a>) {
        eprintln!("ASSERT: the first %GROUPS% is empty, hence None");
        assert!(querier.groups_mut().is_none());
    }

    eprintln!("CASE: ParsedDesc");
    DUPLICATE_FIELD_WITH_EMPTY_FIRST_OCCURRENCE
        .pipe(ParsedDesc::parse)
        .unwrap()
        .pipe_mut(run_assertions);

    eprintln!("CASE: ForgetfulQuerier");
    DUPLICATE_FIELD_WITH_EMPTY_FIRST_OCCURRENCE
        .pipe(ForgetfulQuerier::new)
        .pipe_mut(run_assertions);

    eprintln!("CASE: MemoQuerier");
    DUPLICATE_FIELD_WITH_EMPTY_FIRST_OCCURRENCE
        .pipe(MemoQuerier::new)
        .pipe_mut(run_assertions);
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
    DUPLICATE_FIELD_WITH_EMPTY_FIRST_OCCURRENCE
        .pipe(ParsedDesc::parse)
        .unwrap()
        .pipe_mut(run_assertions);

    eprintln!("CASE: ForgetfulQuerier");
    DUPLICATE_FIELD_WITH_EMPTY_FIRST_OCCURRENCE
        .pipe(ForgetfulQuerier::new)
        .pipe_mut(run_assertions);

    eprintln!("CASE: MemoQuerier");
    DUPLICATE_FIELD_WITH_EMPTY_FIRST_OCCURRENCE
        .pipe(MemoQuerier::new)
        .pipe_mut(run_assertions);
}
