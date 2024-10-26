use inspect_pacman_db::query::{MemoQuerier, QueryMut};
use pretty_assertions::assert_eq;

const TEXT: &str = include_str!("fixtures/gnome-shell.desc");

#[test]
fn query() {
    let mut querier = MemoQuerier::new(TEXT);
    let name = querier.name_mut().unwrap();
    assert_eq!(name.as_str(), "gnome-shell");
    let file_name = querier.file_name_mut().unwrap();
    assert_eq!(file_name.as_str(), "");
}
