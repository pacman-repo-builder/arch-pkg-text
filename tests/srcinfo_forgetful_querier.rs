pub mod _utils;
pub use _utils::*;

use parse_arch_pkg_desc::{
    srcinfo::query::{ChecksumArray, Checksums, ForgetfulQuerier, Query, QueryItem, Section},
    value::{
        Architecture, Base, Dependency, Description, License, Name, SkipOrArray, Source,
        UpstreamVersion,
    },
};
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;

const COMPLEX: &str = include_str!("fixtures/complex/.SRCINFO");
const SIMPLE: &str = include_str!("fixtures/simple/.SRCINFO");
const HAS_EMPTY_VALUES: &str = include_str!("fixtures/has-empty-values/.SRCINFO");
const MULTIPLE_CHECKSUM_TYPES: &str = include_str!("fixtures/multiple-checksum-types/.SRCINFO");

/// Run assertions for srcinfo similar to [`COMPLEX`].
fn assert_complex(querier: &ForgetfulQuerier) {
    assert_eq!(querier.base_name(), Some(Base("complex-example-bin")));
    assert_eq!(querier.version(), Some(UpstreamVersion("12.34.56.r789")));
    assert_eq!(querier.release().unwrap().parse().ok(), Some(2));
    assert_eq!(querier.epoch().unwrap().parse().ok(), Some(3));
    assert_eq!(
        querier
            .description()
            .map(QueryItem::into_tuple2)
            .collect::<Vec<_>>(),
        [
            (Description("Description under pkgbase"), Section::Base),
            (
                Description("Description under foo-bin"),
                Section::Derivative(Name("foo-bin")),
            ),
            (
                Description("Description under bar-bin"),
                Section::Derivative(Name("bar-bin")),
            ),
        ]
    );
    assert_eq!(
        querier
            .architecture()
            .map(QueryItem::into_tuple2)
            .collect::<Vec<_>>(),
        [
            (Architecture("x86_64"), Section::Base),
            (Architecture("aarch64"), Section::Base),
            (Architecture("i686"), Section::Derivative(Name("foo-bin"))),
        ],
    );
    assert_eq!(
        querier
            .dependencies()
            .map(QueryItem::into_tuple3)
            .collect::<Vec<_>>(),
        [
            (Dependency("glibc>=2.0"), Section::Base, None),
            (Dependency("coreutils"), Section::Base, None),
            (Dependency("linux"), Section::Base, None),
            (
                Dependency("aarch64-compatibility"),
                Section::Base,
                Some(Architecture("aarch64")),
            ),
            (
                Dependency("x86_64-compatibility-for-foo"),
                Section::Derivative(Name("foo-bin")),
                Some(Architecture("x86_64")),
            ),
            (
                Dependency("i686-compatibility-for-foo"),
                Section::Derivative(Name("foo-bin")),
                Some(Architecture("i686")),
            ),
            (
                Dependency("extra-depend-for-foo"),
                Section::Derivative(Name("foo-bin")),
                None,
            ),
            (
                Dependency("x86_64-compatibility-for-bar"),
                Section::Derivative(Name("bar-bin")),
                Some(Architecture("x86_64")),
            ),
            (
                Dependency("extra-depend-for-bar"),
                Section::Derivative(Name("bar-bin")),
                None,
            ),
        ],
    );
    assert_eq!(
        querier
            .sha1_checksums()
            .map(QueryItem::into_tuple3)
            .map(|(value, section, architecture)| (value.u8_array(), section, architecture))
            .collect::<Vec<_>>(),
        [
            (
                Some(SkipOrArray::Array([
                    72, 8, 192, 29, 45, 169, 186, 138, 31, 13, 166, 3, 210, 13, 81, 94, 62, 122,
                    103, 230,
                ])),
                Section::Base,
                None,
            ),
            (
                Some(SkipOrArray::Skip),
                Section::Base,
                Some(Architecture("x86_64")),
            ),
            (
                Some(SkipOrArray::Skip),
                Section::Base,
                Some(Architecture("aarch64")),
            ),
            (
                Some(SkipOrArray::Skip),
                Section::Derivative(Name("foo-bin")),
                None,
            ),
            (
                Some(SkipOrArray::Skip),
                Section::Derivative(Name("bar-bin")),
                None,
            ),
        ],
    );
    assert_eq!(
        querier
            .checksums()
            .map(QueryItem::into_tuple3)
            .map(|(value, section, architecture)| (value.u8_array(), section, architecture))
            .collect::<Vec<_>>(),
        [
            (
                Some(ChecksumArray::Sha1([
                    72, 8, 192, 29, 45, 169, 186, 138, 31, 13, 166, 3, 210, 13, 81, 94, 62, 122,
                    103, 230,
                ])),
                Section::Base,
                None,
            ),
            (
                Some(ChecksumArray::Skip),
                Section::Base,
                Some(Architecture("x86_64")),
            ),
            (
                Some(ChecksumArray::Skip),
                Section::Base,
                Some(Architecture("aarch64")),
            ),
            (
                Some(ChecksumArray::Skip),
                Section::Derivative(Name("foo-bin")),
                None,
            ),
            (
                Some(ChecksumArray::Skip),
                Section::Derivative(Name("bar-bin")),
                None,
            ),
        ],
    );
}

/// Run assertions for srcinfo similar to [`SIMPLE`].
fn assert_simple(querier: &ForgetfulQuerier) {
    assert_eq!(querier.base_name(), Some(Base("simple-example-bin")));
    assert_eq!(querier.version(), Some(UpstreamVersion("12.34.56.r789")));
    assert_eq!(querier.release().unwrap().parse().ok(), Some(1));
    assert!(querier.epoch().is_none());
    assert_eq!(
        querier
            .description()
            .map(QueryItem::into_tuple2)
            .collect::<Vec<_>>(),
        [(Description("Simple .SRCINFO example"), Section::Base),]
    );
    assert_eq!(
        querier
            .architecture()
            .map(QueryItem::into_tuple2)
            .collect::<Vec<_>>(),
        [(Architecture("any"), Section::Base)],
    );
    assert_eq!(
        querier
            .dependencies()
            .map(QueryItem::into_tuple3)
            .collect::<Vec<_>>(),
        [
            (Dependency("glibc>=2.0"), Section::Base, None),
            (Dependency("coreutils"), Section::Base, None),
            (Dependency("linux"), Section::Base, None),
        ],
    );
    assert_eq!(
        querier
            .sha1_checksums()
            .map(QueryItem::into_tuple3)
            .map(|(value, section, architecture)| (value.u8_array(), section, architecture))
            .collect::<Vec<_>>(),
        [
            (
                Some(SkipOrArray::Array([
                    72, 8, 192, 29, 45, 169, 186, 138, 31, 13, 166, 3, 210, 13, 81, 94, 62, 122,
                    103, 230,
                ])),
                Section::Base,
                None,
            ),
            (Some(SkipOrArray::Skip), Section::Base, None),
            (Some(SkipOrArray::Skip), Section::Base, None),
        ],
    );
    assert_eq!(
        querier
            .checksums()
            .map(QueryItem::into_tuple3)
            .map(|(value, section, architecture)| (value.u8_array(), section, architecture))
            .collect::<Vec<_>>(),
        [
            (
                Some(ChecksumArray::Sha1([
                    72, 8, 192, 29, 45, 169, 186, 138, 31, 13, 166, 3, 210, 13, 81, 94, 62, 122,
                    103, 230,
                ])),
                Section::Base,
                None,
            ),
            (Some(ChecksumArray::Skip), Section::Base, None),
            (Some(ChecksumArray::Skip), Section::Base, None),
        ],
    );
}

#[test]
fn query_complex() {
    COMPLEX.pipe(ForgetfulQuerier::new).pipe_ref(assert_complex);
}

#[test]
fn query_simple() {
    SIMPLE.pipe(ForgetfulQuerier::new).pipe_ref(assert_simple);
}

#[test]
fn query_no_indent() {
    eprintln!("CASE: complex srcinfo");
    COMPLEX
        .pipe(remove_indent)
        .pipe_as_ref(ForgetfulQuerier::new)
        .pipe_ref(assert_complex);

    eprintln!("CASE: simple srcinfo");
    SIMPLE
        .pipe(remove_indent)
        .pipe_as_ref(ForgetfulQuerier::new)
        .pipe_ref(assert_simple);
}

#[test]
fn query_uneven_indent() {
    eprintln!("CASE: complex srcinfo");
    COMPLEX
        .pipe(uneven_indent)
        .pipe_as_ref(ForgetfulQuerier::new)
        .pipe_ref(assert_complex);

    eprintln!("CASE: simple srcinfo");
    SIMPLE
        .pipe(uneven_indent)
        .pipe_as_ref(ForgetfulQuerier::new)
        .pipe_ref(assert_simple);
}

#[test]
fn filter_out_empty_values() {
    fn run_assertions(querier: &ForgetfulQuerier) {
        assert_eq!(
            querier.derivative_names().collect::<Vec<_>>(),
            [Name("parallel-disk-usage")],
        );
        assert_eq!(
            querier
                .license()
                .map(QueryItem::into_tuple2)
                .collect::<Vec<_>>(),
            [(License("Apache-2.0"), Section::Base)],
        );
        assert_eq!(
            querier
                .source()
                .map(QueryItem::into_tuple3)
                .collect::<Vec<_>>(),
            [(
                Source("parallel-disk-usage-0.11.0.tar.gz::https://github.com/KSXGitHub/parallel-disk-usage/archive/0.11.0.tar.gz"),
                Section::Base,
                None,
            )],
        );
    }

    eprintln!("CASE: no weird format");
    HAS_EMPTY_VALUES
        .pipe(ForgetfulQuerier::new)
        .pipe_ref(run_assertions);

    eprintln!("CASE: trailing whitespaces");
    HAS_EMPTY_VALUES
        .pipe(trailing_whitespaces)
        .pipe_as_ref(ForgetfulQuerier::new)
        .pipe_ref(run_assertions);
}

#[test]
fn multiple_checksum_types() {
    assert_eq!(
        MULTIPLE_CHECKSUM_TYPES
            .pipe(ForgetfulQuerier::new)
            .checksums()
            .map(QueryItem::into_tuple3)
            .map(|(value, section, architecture)| (value.u8_array(), section, architecture))
            .collect::<Vec<_>>(),
        [
            (
                Some(ChecksumArray::Sha1([
                    238, 21, 212, 200, 111, 145, 178, 150, 50, 122, 197, 82, 197, 178, 20, 225,
                    226, 16, 42, 56,
                ])),
                Section::Base,
                None,
            ),
            (
                Some(ChecksumArray::Sha1([
                    227, 58, 153, 73, 214, 32, 106, 121, 154, 37, 218, 242, 16, 86, 118, 17, 25,
                    200, 34, 126,
                ])),
                Section::Base,
                None,
            ),
            (
                Some(ChecksumArray::Md5([
                    85, 228, 106, 159, 222, 52, 186, 188, 135, 255, 41, 206, 254, 199, 250, 135,
                ])),
                Section::Base,
                None,
            ),
            (
                Some(ChecksumArray::Md5([
                    61, 175, 17, 122, 139, 193, 112, 13, 153, 124, 160, 68, 187, 179, 134, 204,
                ])),
                Section::Base,
                None,
            ),
        ],
    );
}
