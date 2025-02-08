use parse_arch_pkg_desc::{
    db::{ForgetfulQuerier, Query},
    value::{Architecture, Dependency, Description, FileName, Name},
};
use pretty_assertions::assert_eq;

const TEXT: &str = include_str!("fixtures/gnome-shell.desc");

#[test]
fn query() {
    let querier = ForgetfulQuerier::new(TEXT);

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
