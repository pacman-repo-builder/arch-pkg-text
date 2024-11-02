use core::ops::Not;
use parse_arch_pkg_desc::{
    field::FieldName,
    query::{MemoQuerier, QueryMut},
    value::{Architecture, Dependency},
};
use pretty_assertions::assert_eq;

const TEXT: &str = include_str!("fixtures/gnome-shell.desc");

#[test]
fn query() {
    // first querier
    let mut querier = MemoQuerier::new(TEXT);
    assert!(querier.__has_cache(FieldName::FileName).not());
    assert!(querier.__has_cache(FieldName::Name).not());
    assert!(querier.__has_cache(FieldName::Description).not());
    assert!(querier.__has_cache(FieldName::Architecture).not());
    assert!(querier.__has_cache(FieldName::MakeDependencies).not());

    // load a fresh item
    let name = querier.name_mut().unwrap();
    assert_eq!(name.as_str(), "gnome-shell");
    assert!(querier.__has_cache(FieldName::FileName));
    assert!(querier.__has_cache(FieldName::Name));
    assert!(querier.__has_cache(FieldName::Description).not());
    assert!(querier.__has_cache(FieldName::Architecture).not());
    assert!(querier.__has_cache(FieldName::MakeDependencies).not());

    // load a cached item
    let file_name = querier.file_name_mut().unwrap();
    assert_eq!(
        file_name.as_str(),
        "gnome-shell-1:46.2-1-x86_64.pkg.tar.zst",
    );

    // load another fresh item
    let mut architecture = querier.architecture_mut().unwrap().into_iter();
    assert_eq!(architecture.next(), Some(Architecture("x86_64")));
    assert_eq!(architecture.next(), None);
    assert!(querier.__has_cache(FieldName::FileName));
    assert!(querier.__has_cache(FieldName::Name));
    assert!(querier.__has_cache(FieldName::Description));
    assert!(querier.__has_cache(FieldName::Architecture));
    assert!(querier.__has_cache(FieldName::MakeDependencies).not());

    // load another cache item
    let description = querier.description_mut().unwrap();
    assert_eq!(description.as_str(), "Next generation desktop shell");

    // load the very last item
    let mut make_dependencies = querier.make_dependencies_mut().unwrap().into_iter();
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
    assert!(querier.__has_cache(FieldName::FileName));
    assert!(querier.__has_cache(FieldName::Name));
    assert!(querier.__has_cache(FieldName::Description));
    assert!(querier.__has_cache(FieldName::Architecture));
    assert!(querier.__has_cache(FieldName::MakeDependencies));

    // load items that don't exist
    assert!(querier.conflicts_mut().is_none());
    assert!(querier.provides_mut().is_none());

    // retry the cache
    let name = querier.name_mut().unwrap();
    assert_eq!(name.as_str(), "gnome-shell");
    let mut architecture = querier.architecture_mut().unwrap().into_iter();
    assert_eq!(architecture.next(), Some(Architecture("x86_64")));
    assert_eq!(architecture.next(), None);

    // second querier
    let mut querier = MemoQuerier::new(TEXT);
    assert!(querier.__has_cache(FieldName::FileName).not());
    assert!(querier.__has_cache(FieldName::Name).not());
    assert!(querier.__has_cache(FieldName::Description).not());
    assert!(querier.__has_cache(FieldName::Architecture).not());
    assert!(querier.__has_cache(FieldName::MakeDependencies).not());

    // load items that don't exist, which should fill the cache
    assert!(querier.conflicts_mut().is_none());
    assert!(querier.provides_mut().is_none());
    assert!(querier.__has_cache(FieldName::FileName));
    assert!(querier.__has_cache(FieldName::Name));
    assert!(querier.__has_cache(FieldName::Description));
    assert!(querier.__has_cache(FieldName::Architecture));
    assert!(querier.__has_cache(FieldName::MakeDependencies));

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

#[cfg(feature = "std")]
#[test]
fn query_std_mutex() {
    use parse_arch_pkg_desc::query::Query;
    use pipe_trait::Pipe;
    use std::sync::Mutex;

    fn has_cache(querier: &Mutex<MemoQuerier>, field: FieldName) -> bool {
        querier.lock().unwrap().__has_cache(field)
    }

    let querier = TEXT.pipe(MemoQuerier::new).pipe(Mutex::new);
    assert!(has_cache(&querier, FieldName::FileName).not());
    assert!(has_cache(&querier, FieldName::Name).not());
    assert!(has_cache(&querier, FieldName::Description).not());
    assert!(has_cache(&querier, FieldName::Architecture).not());
    assert!(has_cache(&querier, FieldName::MakeDependencies).not());

    // load a fresh item
    let name = querier.name().unwrap();
    assert_eq!(name.as_str(), "gnome-shell");
    assert!(has_cache(&querier, FieldName::FileName));
    assert!(has_cache(&querier, FieldName::Name));
    assert!(has_cache(&querier, FieldName::Description).not());
    assert!(has_cache(&querier, FieldName::Architecture).not());
    assert!(has_cache(&querier, FieldName::MakeDependencies).not());

    // load a cached item
    let file_name = querier.file_name().unwrap();
    assert_eq!(
        file_name.as_str(),
        "gnome-shell-1:46.2-1-x86_64.pkg.tar.zst",
    );

    // load another fresh item
    let mut architecture = querier.architecture().unwrap().into_iter();
    assert_eq!(architecture.next(), Some(Architecture("x86_64")));
    assert_eq!(architecture.next(), None);
    assert!(has_cache(&querier, FieldName::FileName));
    assert!(has_cache(&querier, FieldName::Name));
    assert!(has_cache(&querier, FieldName::Description));
    assert!(has_cache(&querier, FieldName::Architecture));
    assert!(has_cache(&querier, FieldName::MakeDependencies).not());

    // load another cache item
    let description = querier.description().unwrap();
    assert_eq!(description.as_str(), "Next generation desktop shell");

    // load the very last item
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
    assert!(has_cache(&querier, FieldName::FileName));
    assert!(has_cache(&querier, FieldName::Name));
    assert!(has_cache(&querier, FieldName::Description));
    assert!(has_cache(&querier, FieldName::Architecture));
    assert!(has_cache(&querier, FieldName::MakeDependencies));

    // load items that don't exist
    assert!(querier.conflicts().is_none());
    assert!(querier.provides().is_none());

    // retry the cache
    let name = querier.name().unwrap();
    assert_eq!(name.as_str(), "gnome-shell");
    let mut architecture = querier.architecture().unwrap().into_iter();
    assert_eq!(architecture.next().map(|x| x.as_str()), Some("x86_64"));
    assert_eq!(architecture.next().map(|x| x.as_str()), None);
}

#[cfg(feature = "parking_lot")]
#[test]
fn query_parking_lot_mutex() {
    use parking_lot::Mutex;
    use parse_arch_pkg_desc::query::Query;
    use pipe_trait::Pipe;

    fn has_cache(querier: &Mutex<MemoQuerier>, field: FieldName) -> bool {
        querier.lock().__has_cache(field)
    }

    let querier = TEXT.pipe(MemoQuerier::new).pipe(Mutex::new);
    assert!(has_cache(&querier, FieldName::FileName).not());
    assert!(has_cache(&querier, FieldName::Name).not());
    assert!(has_cache(&querier, FieldName::Description).not());
    assert!(has_cache(&querier, FieldName::Architecture).not());
    assert!(has_cache(&querier, FieldName::MakeDependencies).not());

    // load a fresh item
    let name = querier.name().unwrap();
    assert_eq!(name.as_str(), "gnome-shell");
    assert!(has_cache(&querier, FieldName::FileName));
    assert!(has_cache(&querier, FieldName::Name));
    assert!(has_cache(&querier, FieldName::Description).not());
    assert!(has_cache(&querier, FieldName::Architecture).not());
    assert!(has_cache(&querier, FieldName::MakeDependencies).not());

    // load a cached item
    let file_name = querier.file_name().unwrap();
    assert_eq!(
        file_name.as_str(),
        "gnome-shell-1:46.2-1-x86_64.pkg.tar.zst",
    );

    // load another fresh item
    let mut architecture = querier.architecture().unwrap().into_iter();
    assert_eq!(architecture.next().map(|x| x.as_str()), Some("x86_64"));
    assert_eq!(architecture.next().map(|x| x.as_str()), None);
    assert!(has_cache(&querier, FieldName::FileName));
    assert!(has_cache(&querier, FieldName::Name));
    assert!(has_cache(&querier, FieldName::Description));
    assert!(has_cache(&querier, FieldName::Architecture));
    assert!(has_cache(&querier, FieldName::MakeDependencies).not());

    // load another cache item
    let description = querier.description().unwrap();
    assert_eq!(description.as_str(), "Next generation desktop shell");

    // load the very last item
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
    assert!(has_cache(&querier, FieldName::FileName));
    assert!(has_cache(&querier, FieldName::Name));
    assert!(has_cache(&querier, FieldName::Description));
    assert!(has_cache(&querier, FieldName::Architecture));
    assert!(has_cache(&querier, FieldName::MakeDependencies));

    // load items that don't exist
    assert!(querier.conflicts().is_none());
    assert!(querier.provides().is_none());

    // retry the cache
    let name = querier.name().unwrap();
    assert_eq!(name.as_str(), "gnome-shell");
    let mut architecture = querier.architecture().unwrap().into_iter();
    assert_eq!(architecture.next().map(|x| x.as_str()), Some("x86_64"));
    assert_eq!(architecture.next().map(|x| x.as_str()), None);
}
