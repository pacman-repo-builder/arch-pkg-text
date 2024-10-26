pub trait GetEntry<Key> {
    type Entry;
    fn get_entry(self, key: Key) -> Self::Entry;
}

pub trait OrInsertWith<'a> {
    type Item;
    fn or_insert_with<F>(self, default: F) -> &'a mut Self::Item
    where
        F: FnOnce() -> Self::Item;
}

#[cfg(feature = "std")]
mod std_ext;
