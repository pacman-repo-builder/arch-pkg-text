pub mod _utils;
pub use _utils::*;

use arch_pkg_text::{
    ParsedDesc, QueryDesc,
    value::{Architecture, Dependency, Description, FileName, Name},
};
use std::sync::LazyLock;

static TEXT: LazyLock<String> = LazyLock::new(|| {
    include_str!("fixtures/gnome-shell.desc").insert_above_line(
        |line| line.contains("%DESC%"),
        "%THISFIELDISUNKNOWN%\nFoo\nBar\nBaz",
    )
});

fn assert_query(querier: &ParsedDesc) {
    assert_eq!(querier.name(), Some(Name("gnome-shell")));

    assert_eq!(
        querier.file_name(),
        Some(FileName("gnome-shell-1:46.2-1-x86_64.pkg.tar.zst")),
    );

    let mut architecture = querier.architecture().unwrap().into_iter();
    assert_eq!(architecture.next(), Some(Architecture("x86_64")));
    assert_eq!(architecture.next(), None);

    assert_eq!(
        querier.description(),
        Some(Description("Next generation desktop shell")),
    );

    let mut make_dependencies = querier.make_dependencies().unwrap().into_iter();
    assert_eq!(make_dependencies.next(), Some(Dependency("asciidoc")));
    assert_eq!(
        make_dependencies.next(),
        Some(Dependency("bash-completion")),
    );
    assert_eq!(
        make_dependencies.next(),
        Some(Dependency("evolution-data-server")),
    );
    assert_eq!(make_dependencies.next(), Some(Dependency("gi-docgen")));
    assert_eq!(make_dependencies.next().map(|x| x.as_str()), Some("git"));
    assert_eq!(
        make_dependencies.next(),
        Some(Dependency("gnome-keybindings")),
    );
    assert_eq!(
        make_dependencies.next(),
        Some(Dependency("gobject-introspection")),
    );
    assert_eq!(make_dependencies.next(), Some(Dependency("meson")));
    assert_eq!(make_dependencies.next(), Some(Dependency("sassc")));
    assert_eq!(make_dependencies.next(), None);
}

/// Issue: Infinite loop calling `ParsedDesc::parse` on package description text with unknown fields.
///
/// <https://github.com/pacman-repo-builder/arch-pkg-text/issues/30>
#[test]
fn issue_30() {
    let querier = ParsedDesc::parse(&TEXT).unwrap();
    assert_query(&querier);
}
