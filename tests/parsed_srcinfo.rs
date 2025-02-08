#![cfg(feature = "std")]

pub mod _utils;
pub use _utils::*;

use hex_literal::hex;
use parse_arch_pkg_desc::{
    parse::{
        ParsedSrcinfo, ParsedSrcinfoBaseUniqueFieldDuplicationError,
        ParsedSrcinfoDerivativeUniqueFieldDuplicationError, SrcinfoParseError,
    },
    srcinfo::query::{ChecksumArray, Checksums, Query, QueryItem, Section},
    value::{
        Architecture, Base, Dependency, Description, License, Name, SkipOrArray, Source,
        UpstreamVersion,
    },
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
        self.try_into().unwrap()
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
        .map(|(dependency, architecture)| (Dependency(dependency), architecture.map(Architecture))),
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
        .map(|(dependency, architecture)| (Dependency(dependency), architecture.map(Architecture))),
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
        .map(|(dependency, architecture)| (Dependency(dependency), architecture.map(Architecture))),
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
    dbg!(querier);

    eprintln!("STEP: pkgbase");
    let base = dbg!(&querier.base);
    assert_eq!(base.base_name(), Some(Base("simple-example-bin")));
    assert_eq!(base.version(), Some(UpstreamVersion("12.34.56.r789")));
    assert_eq!(base.release().unwrap().parse().ok(), Some(1));
    assert!(base.epoch().is_none());
    assert_eq!(
        base.description(),
        Some(Description("Simple .SRCINFO example")),
    );
    assert_eq!(base.architecture(), ["any"].map(Architecture));
    assert_eq!(
        base.dependencies(),
        ["glibc>=2.0", "coreutils", "linux"].map(|dependency| (Dependency(dependency), None)),
    );
    assert_eq!(
        base.sha1_checksums()
            .iter()
            .map(|(value, architecture)| (value.u8_array(), *architecture))
            .collect::<Vec<_>>(),
        [
            SkipOrArray::Array(hex!("4808c01d2da9ba8a1f0da603d20d515e3e7a67e6")),
            SkipOrArray::Skip,
            SkipOrArray::Skip,
        ]
        .map(|value| (Some(value), None)),
    );
    assert_eq!(
        base.checksums()
            .map(|(value, architecture)| (value.u8_array(), architecture))
            .collect::<Vec<_>>(),
        [
            ChecksumArray::Sha1(hex!("4808c01d2da9ba8a1f0da603d20d515e3e7a67e6")),
            ChecksumArray::Skip,
            ChecksumArray::Skip,
        ]
        .map(|value| (Some(value), None)),
    );

    eprintln!("STEP: pkgname = simple-example-bin");
    let derivative = querier
        .derivatives
        .get(&Name("simple-example-bin"))
        .unwrap();
    dbg!(&derivative);
    assert_eq!(derivative.description(), None);
    assert_eq!(derivative.architecture(), []);
    assert_eq!(derivative.dependencies(), []);
    assert!(derivative.checksums().next().is_none());

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

#[test]
fn filter_out_empty_values() {
    fn run_assertions(querier: &ParsedSrcinfo) {
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
        .parse_srcinfo_unwrap()
        .pipe_ref(run_assertions);

    eprintln!("CASE: trailing whitespaces");
    HAS_EMPTY_VALUES
        .pipe(trailing_whitespaces)
        .parse_srcinfo_unwrap()
        .pipe_ref(run_assertions);
}

#[test]
fn multiple_checksum_types() {
    let querier = dbg!(MULTIPLE_CHECKSUM_TYPES.parse_srcinfo_unwrap());

    eprintln!("STEP: pkgbase");
    let base = dbg!(&querier.base);
    let checksums: Vec<_> = base
        .checksums()
        .map(|(value, architecture)| (value.u8_array(), architecture))
        .collect();
    assert_eq!(
        checksums,
        [
            ChecksumArray::Md5(hex!("55e46a9fde34babc87ff29cefec7fa87")),
            ChecksumArray::Md5(hex!("3daf117a8bc1700d997ca044bbb386cc")),
            ChecksumArray::Sha1(hex!("ee15d4c86f91b296327ac552c5b214e1e2102a38")),
            ChecksumArray::Sha1(hex!("e33a9949d6206a799a25daf21056761119c8227e")),
        ]
        .map(|value| (Some(value), None))
    );

    eprintln!("STEP: pkgname = multiple-checksum-types");
    let derivative = querier
        .derivatives
        .get(&Name("multiple-checksum-types"))
        .unwrap();
    dbg!(&derivative);
    let checksums: Vec<_> = derivative
        .checksums()
        .map(|(value, architecture)| (value.u8_array(), architecture))
        .collect();
    assert_eq!(checksums, []);

    eprintln!("STEP: query");
    let checksums: Vec<_> = querier
        .checksums()
        .map(QueryItem::into_tuple3)
        .map(|(value, section, architecture)| (value.u8_array(), section, architecture))
        .collect();

    eprintln!("ASSERT: checksum variants");
    assert_eq!(
        checksums,
        [
            (
                Some(ChecksumArray::Md5(hex!("55e46a9fde34babc87ff29cefec7fa87"))),
                Section::Base,
                None,
            ),
            (
                Some(ChecksumArray::Md5(hex!("3daf117a8bc1700d997ca044bbb386cc"))),
                Section::Base,
                None,
            ),
            (
                Some(ChecksumArray::Sha1(hex!(
                    "ee15d4c86f91b296327ac552c5b214e1e2102a38"
                ))),
                Section::Base,
                None,
            ),
            (
                Some(ChecksumArray::Sha1(hex!(
                    "e33a9949d6206a799a25daf21056761119c8227e"
                ))),
                Section::Base,
                None,
            ),
        ],
    );

    eprintln!("ASSERT: checksum slices");
    let slices: Vec<&[u8]> = checksums
        .as_slice()
        .iter()
        .flat_map(|(value, _, _)| value.as_ref())
        .flat_map(|value| value.try_as_slice())
        .collect();
    assert_eq!(
        slices,
        [
            hex!("55e46a9fde34babc87ff29cefec7fa87").as_slice(),
            hex!("3daf117a8bc1700d997ca044bbb386cc").as_slice(),
            hex!("ee15d4c86f91b296327ac552c5b214e1e2102a38").as_slice(),
            hex!("e33a9949d6206a799a25daf21056761119c8227e").as_slice(),
        ],
    );
}

#[test]
fn duplicated_single_fields() {
    eprintln!("CASE: duplicated pkgbase under pkgbase");
    let srcinfo = insert_line_under(SIMPLE, "pkgbase", "pkgbase = duplicated");
    let result = dbg!(ParsedSrcinfo::try_from(srcinfo.as_str()));
    assert!(matches!(
        result,
        Err(SrcinfoParseError::BaseUniqueFieldDuplication(
            ParsedSrcinfoBaseUniqueFieldDuplicationError::Base(Base("simple-example-bin")),
        )),
    ));
    assert_eq!(
        result.unwrap_err().to_string(),
        "Failed to insert value to the pkgbase section: Field pkgbase is already set",
    );

    eprintln!("CASE: duplicated pkgdesc under pkgbase");
    let srcinfo = insert_line_under(SIMPLE, "pkgdesc", "pkgdesc = duplicated");
    let result = dbg!(ParsedSrcinfo::try_from(srcinfo.as_str()));
    assert!(matches!(
        result,
        Err(SrcinfoParseError::BaseUniqueFieldDuplication(
            ParsedSrcinfoBaseUniqueFieldDuplicationError::Description(Description(
                "Simple .SRCINFO example"
            )),
        )),
    ));
    assert_eq!(
        result.unwrap_err().to_string(),
        "Failed to insert value to the pkgbase section: Field pkgdesc is already set",
    );

    eprintln!("CASE: duplicated pkgver under pkgbase");
    let srcinfo = insert_line_under(SIMPLE, "pkgver", "pkgver = 0.1.2");
    let result = dbg!(ParsedSrcinfo::try_from(srcinfo.as_str()));
    assert!(matches!(
        result,
        Err(SrcinfoParseError::BaseUniqueFieldDuplication(
            ParsedSrcinfoBaseUniqueFieldDuplicationError::Version(UpstreamVersion("12.34.56.r789")),
        )),
    ));
    assert_eq!(
        result.unwrap_err().to_string(),
        "Failed to insert value to the pkgbase section: Field pkgver is already set",
    );

    eprintln!("CASE: duplicated pkgdesc under pkgname");
    let srcinfo = insert_line_under(
        COMPLEX,
        "pkgdesc = Description under foo-bin",
        "pkgdesc = duplicated",
    );
    let result = dbg!(ParsedSrcinfo::try_from(srcinfo.as_str()));
    assert!(matches!(
        result,
        Err(SrcinfoParseError::DerivativeUniqueFieldDuplication(
            Name("foo-bin"),
            ParsedSrcinfoDerivativeUniqueFieldDuplicationError::Description(Description(
                "Description under foo-bin"
            )),
        )),
    ));
    assert_eq!(
        result.unwrap_err().to_string(),
        "Failed to insert value to the pkgname section named foo-bin: Field pkgdesc is already set",
    );
}
