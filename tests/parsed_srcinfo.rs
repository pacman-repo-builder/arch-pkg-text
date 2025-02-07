#![cfg(feature = "std")]

pub mod _utils;
pub use _utils::*;

use hex_literal::hex;
use parse_arch_pkg_desc::{
    parse::ParsedSrcinfo,
    srcinfo::query::{ChecksumArray, Checksums, Query, QueryItem, Section},
    value::{Architecture, Base, Dependency, Description, Name, SkipOrArray, UpstreamVersion},
};
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;

/// Convenient methods to convert string to [`ParsedSrcinfo`].
trait SrcinfoParsingUtils {
    /// Try parsing [`ParsedSrcinfo`] from a string, panic on error.
    fn parse_srcinfo_unwrap(&self) -> ParsedSrcinfo<'_>;
}

impl SrcinfoParsingUtils for str {
    fn parse_srcinfo_unwrap(&self) -> ParsedSrcinfo<'_> {
        ParsedSrcinfo::try_from(self).unwrap()
    }
}

/// Run assertions for srcinfo similar to [`COMPLEX`].
fn assert_complex(querier: &ParsedSrcinfo) {
    dbg!(querier);

    eprintln!("STEP: pkgbase");
    let base = dbg!(&querier.base);
    assert_eq!(base.base_name(), Some(Base("complex-example-bin")));
    assert_eq!(base.version(), Some(UpstreamVersion("12.34.56.r789")));
    assert_eq!(base.release().unwrap().parse().ok(), Some(2));
    assert_eq!(base.epoch().unwrap().parse().ok(), Some(3));
    assert_eq!(
        base.description(),
        Some(Description("Description under pkgbase")),
    );
    assert_eq!(base.architecture(), ["x86_64", "aarch64"].map(Architecture),);
    assert_eq!(
        base.dependencies(),
        [
            ("glibc>=2.0", None),
            ("coreutils", None),
            ("linux", None),
            ("aarch64-compatibility", Some("aarch64")),
        ]
        .map(|(name, architecture)| (Dependency(name), architecture.map(Architecture))),
    );
    assert_eq!(
        base.sha1_checksums()
            .iter()
            .map(|(value, architecture)| (value.u8_array(), *architecture))
            .collect::<Vec<_>>(),
        [
            (
                SkipOrArray::Array(hex!("4808c01d2da9ba8a1f0da603d20d515e3e7a67e6")),
                None,
            ),
            (SkipOrArray::Skip, Some("x86_64")),
            (SkipOrArray::Skip, Some("aarch64")),
        ]
        .map(|(value, architecture)| (Some(value), architecture.map(Architecture))),
    );
    assert_eq!(
        base.checksums()
            .map(|(value, architecture)| (value.u8_array(), architecture))
            .collect::<Vec<_>>(),
        [
            (
                ChecksumArray::Sha1(hex!("4808c01d2da9ba8a1f0da603d20d515e3e7a67e6")),
                None,
            ),
            (ChecksumArray::Skip, Some("x86_64")),
            (ChecksumArray::Skip, Some("aarch64")),
        ]
        .map(|(value, architecture)| (Some(value), architecture.map(Architecture))),
    );

    eprintln!("STEP: pkgname");
    let derivatives = dbg!(&querier.derivatives);
    assert_eq!(
        derivatives.keys().copied().collect::<Vec<_>>(),
        &["foo-bin", "bar-bin"].map(Name),
    );

    eprintln!("STEP: pkgname = foo-bin");
    let derivative = dbg!(derivatives.get(&Name("foo-bin")).unwrap());
    assert_eq!(
        derivative.description(),
        Some(Description("Description under foo-bin")),
    );
    assert_eq!(derivative.architecture(), ["i686"].map(Architecture));
    assert_eq!(
        derivative.dependencies(),
        [
            ("x86_64-compatibility-for-foo", Some("x86_64")),
            ("i686-compatibility-for-foo", Some("i686")),
            ("extra-depend-for-foo", None),
        ]
        .map(|(name, architecture)| (Dependency(name), architecture.map(Architecture))),
    );
    assert_eq!(
        derivative
            .sha1_checksums()
            .iter()
            .map(|(value, architecture)| (value.u8_array(), *architecture))
            .collect::<Vec<_>>(),
        [(SkipOrArray::Skip, None)]
            .map(|(value, architecture)| (Some(value), architecture.map(Architecture))),
    );
    assert_eq!(
        derivative
            .checksums()
            .map(|(value, architecture)| (value.u8_array(), architecture))
            .collect::<Vec<_>>(),
        [(ChecksumArray::Skip, None)]
            .map(|(value, architecture)| (Some(value), architecture.map(Architecture))),
    );

    eprintln!("STEP: pkgname = bar-bin");
    let derivative = dbg!(derivatives.get(&Name("bar-bin")).unwrap());
    assert_eq!(
        derivative.description(),
        Some(Description("Description under bar-bin")),
    );
    assert_eq!(derivative.architecture(), []);
    assert_eq!(
        derivative.dependencies(),
        [
            ("x86_64-compatibility-for-bar", Some("x86_64")),
            ("extra-depend-for-bar", None),
        ]
        .map(|(name, architecture)| (Dependency(name), architecture.map(Architecture))),
    );
    assert_eq!(
        derivative
            .sha1_checksums()
            .iter()
            .map(|(value, architecture)| (value.u8_array(), *architecture))
            .collect::<Vec<_>>(),
        [(SkipOrArray::Skip, None)]
            .map(|(value, architecture)| (Some(value), architecture.map(Architecture))),
    );
    assert_eq!(
        derivative
            .checksums()
            .map(|(value, architecture)| (value.u8_array(), architecture))
            .collect::<Vec<_>>(),
        [(ChecksumArray::Skip, None)]
            .map(|(value, architecture)| (Some(value), architecture.map(Architecture))),
    );

    eprintln!("STEP: query");
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
                Some(SkipOrArray::Array(hex!(
                    "4808c01d2da9ba8a1f0da603d20d515e3e7a67e6"
                ))),
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
                Some(ChecksumArray::Sha1(hex!(
                    "4808c01d2da9ba8a1f0da603d20d515e3e7a67e6"
                ))),
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
fn assert_simple(querier: &ParsedSrcinfo) {
    eprintln!("STEP: query");
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
                Some(SkipOrArray::Array(hex!(
                    "4808c01d2da9ba8a1f0da603d20d515e3e7a67e6"
                ))),
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
                Some(ChecksumArray::Sha1(hex!(
                    "4808c01d2da9ba8a1f0da603d20d515e3e7a67e6"
                ))),
                Section::Base,
                None,
            ),
            (Some(ChecksumArray::Skip), Section::Base, None),
            (Some(ChecksumArray::Skip), Section::Base, None),
        ],
    );
}

#[test]
fn complex() {
    COMPLEX.parse_srcinfo_unwrap().pipe_ref(assert_complex);
}

#[test]
fn simple() {
    SIMPLE.parse_srcinfo_unwrap().pipe_ref(assert_simple);
}

#[test]
fn query_no_indent() {
    eprintln!("CASE: complex srcinfo");
    COMPLEX
        .pipe(remove_indent)
        .parse_srcinfo_unwrap()
        .pipe_ref(assert_complex);

    eprintln!("CASE: simple srcinfo");
    SIMPLE
        .pipe(remove_indent)
        .parse_srcinfo_unwrap()
        .pipe_ref(assert_simple);
}
