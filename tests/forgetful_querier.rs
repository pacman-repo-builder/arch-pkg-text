use inspect_pacman_db::query::{ForgetfulQuerier, Query};
use pretty_assertions::assert_eq;

const TEXT: &str = include_str!("fixtures/gnome-shell.desc");

#[test]
fn query() {
    let querier = ForgetfulQuerier::new(TEXT);

    let name = querier.name().unwrap();
    assert_eq!(name.as_str(), "gnome-shell");

    let file_name = querier.file_name().unwrap();
    assert_eq!(
        file_name.as_str(),
        "gnome-shell-1:46.2-1-x86_64.pkg.tar.zst",
    );

    let mut architecture = querier.architecture().unwrap().into_iter();
    assert_eq!(architecture.next().map(|x| x.as_str()), Some("x86_64"));
    assert_eq!(architecture.next().map(|x| x.as_str()), None);

    let description = querier.description().unwrap();
    assert_eq!(description.as_str(), "Next generation desktop shell");
}
