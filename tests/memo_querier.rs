use inspect_pacman_db::query::{MemoQuerier, QueryMut};
use pretty_assertions::assert_eq;

const TEXT: &str = include_str!("fixtures/gnome-shell.desc");

#[test]
fn query() {
    // first querier
    let mut querier = MemoQuerier::new(TEXT);

    // load a fresh item
    let name = querier.name_mut().unwrap();
    assert_eq!(name.as_str(), "gnome-shell");

    // load a cached item
    let file_name = querier.file_name_mut().unwrap();
    assert_eq!(
        file_name.as_str(),
        "gnome-shell-1:46.2-1-x86_64.pkg.tar.zst",
    );

    // load another fresh item
    let mut arch = querier.arch_mut().unwrap().into_iter();
    assert_eq!(arch.next().map(|x| x.as_str()), Some("x86_64"));
    assert_eq!(arch.next().map(|x| x.as_str()), None);

    // load another cache item
    let description = querier.description_mut().unwrap();
    assert_eq!(description.as_str(), "Next generation desktop shell");

    // load the very last item
    let mut make_depends = querier.make_depends_mut().unwrap().into_iter();
    assert_eq!(make_depends.next().map(|x| x.as_str()), Some("asciidoc"));
    assert_eq!(
        make_depends.next().map(|x| x.as_str()),
        Some("bash-completion"),
    );
    assert_eq!(
        make_depends.next().map(|x| x.as_str()),
        Some("evolution-data-server"),
    );
    assert_eq!(make_depends.next().map(|x| x.as_str()), Some("gi-docgen"));
    assert_eq!(make_depends.next().map(|x| x.as_str()), Some("git"));
    assert_eq!(
        make_depends.next().map(|x| x.as_str()),
        Some("gnome-keybindings"),
    );
    assert_eq!(
        make_depends.next().map(|x| x.as_str()),
        Some("gobject-introspection"),
    );
    assert_eq!(make_depends.next().map(|x| x.as_str()), Some("meson"));
    assert_eq!(make_depends.next().map(|x| x.as_str()), Some("sassc"));
    assert_eq!(make_depends.next().map(|x| x.as_str()), None);

    // load items that don't exist
    assert!(querier.conflicts_mut().is_none());
    assert!(querier.provides_mut().is_none());

    // retry the cache
    let name = querier.name_mut().unwrap();
    assert_eq!(name.as_str(), "gnome-shell");
    let mut arch = querier.arch_mut().unwrap().into_iter();
    assert_eq!(arch.next().map(|x| x.as_str()), Some("x86_64"));
    assert_eq!(arch.next().map(|x| x.as_str()), None);

    // second querier
    let mut querier = MemoQuerier::new(TEXT);

    // load items that don't exist, which should fill the cache
    assert!(querier.conflicts_mut().is_none());
    assert!(querier.provides_mut().is_none());

    // load the very first item
    let file_name = querier.file_name_mut().unwrap();
    assert_eq!(
        file_name.as_str(),
        "gnome-shell-1:46.2-1-x86_64.pkg.tar.zst",
    );

    // load the second item
    let name = querier.name_mut().unwrap();
    assert_eq!(name.as_str(), "gnome-shell");
}
