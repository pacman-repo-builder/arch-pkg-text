pub mod _utils;
pub use _utils::*;

use hex_literal::hex;
use parse_arch_pkg_desc::{
    srcinfo::query::{ChecksumArray, Checksums, ForgetfulQuerier, Query, QueryItem, Section},
    value::{
        Architecture, Base, Dependency, Description, License, Name, SkipOrArray, Source,
        UpstreamVersion,
    },
};
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;

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
    let checksums: Vec<_> = MULTIPLE_CHECKSUM_TYPES
        .pipe(ForgetfulQuerier::new)
        .checksums()
        .map(QueryItem::into_tuple3)
        .map(|(value, section, architecture)| (value.u8_array(), section, architecture))
        .collect();

    eprintln!("ASSERT: checksum variants");
    assert_eq!(
        checksums,
        [
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
            hex!("ee15d4c86f91b296327ac552c5b214e1e2102a38").as_slice(),
            hex!("e33a9949d6206a799a25daf21056761119c8227e").as_slice(),
            hex!("55e46a9fde34babc87ff29cefec7fa87").as_slice(),
            hex!("3daf117a8bc1700d997ca044bbb386cc").as_slice(),
        ],
    );
}
