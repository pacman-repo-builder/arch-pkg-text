//! Issue: Incorrect output for `ParsedDesc::installed_size()`.
//!
//! <https://github.com/pacman-repo-builder/arch-pkg-text/issues/40>

pub mod _utils;
pub use _utils::*;

use arch_pkg_text::{
    ParsedDesc, QueryDesc, QueryDescMut,
    desc::{ForgetfulQuerier, MemoQuerier},
};
use pretty_assertions::assert_eq;
use std::sync::LazyLock;

const MD5SUM: &str = "165f04122017ec76579594b17f15f8eb";
const SHA256SUM: &str = "3e84aac341825e2dd5f4a477ab03682d80e3e1a1a9b55abe38f9e01dd712852a";
const PGPSIG: &str = "iHUEABYKAB0WIQSDvIiJNRtd67toQW64rAhgDxCM3wUCZlKf1AAKCRC4rAhgDxCM319XAQDZW8vRCMsnOsn0GKvVAhNeoZW916fS87NpWeW/CLf3lgD/Y17FTUlh9CTXE/zg54ltntRedOrKXwgJ2zL3kd+mpA0=";

/// The fixture lacks an `%MD5SUM%` field, add one to cover it as well.
static TEXT: LazyLock<String> = LazyLock::new(|| {
    include_str!("fixtures/gnome-shell.desc").insert_above_line(
        |line| line.contains("%SHA256SUM%"),
        &format!("%MD5SUM%\n{MD5SUM}\n"),
    )
});

fn assert_query<'a>(querier: &impl QueryDesc<'a>) {
    let compressed_size = querier.compressed_size().unwrap();
    assert_eq!(compressed_size.as_str(), "1827050");
    assert_eq!(compressed_size.parse(), Ok(1827050));

    let installed_size = querier.installed_size().unwrap();
    assert_eq!(installed_size.as_str(), "14190669");
    assert_eq!(installed_size.parse(), Ok(14190669));

    assert_eq!(
        querier.md5_checksum().map(|value| value.as_str()),
        Some(MD5SUM),
    );
    assert_eq!(
        querier.sha256_checksum().map(|value| value.as_str()),
        Some(SHA256SUM),
    );
    assert_eq!(
        querier.pgp_signature().map(|value| value.as_str()),
        Some(PGPSIG),
    );
}

#[test]
fn parsed_desc() {
    let querier = ParsedDesc::parse(&TEXT).unwrap();
    dbg!(&querier);
    assert_query(&querier);
}

#[test]
fn forgetful_querier() {
    let querier = ForgetfulQuerier::new(&TEXT);
    assert_query(&querier);
}

#[test]
fn memo_querier() {
    let mut querier = MemoQuerier::new(&TEXT);

    let compressed_size = querier.compressed_size_mut().unwrap();
    assert_eq!(compressed_size.as_str(), "1827050");
    assert_eq!(compressed_size.parse(), Ok(1827050));

    let installed_size = querier.installed_size_mut().unwrap();
    assert_eq!(installed_size.as_str(), "14190669");
    assert_eq!(installed_size.parse(), Ok(14190669));

    assert_eq!(
        querier.md5_checksum_mut().map(|value| value.as_str()),
        Some(MD5SUM),
    );
    assert_eq!(
        querier.sha256_checksum_mut().map(|value| value.as_str()),
        Some(SHA256SUM),
    );
    assert_eq!(
        querier.pgp_signature_mut().map(|value| value.as_str()),
        Some(PGPSIG),
    );
}
