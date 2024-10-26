pub trait GetMutOrInsertWith<'a, Key> {
    type Item;
    fn get_mut_or_insert_with<Default>(
        &'a mut self,
        key: Key,
        default: Default,
    ) -> &'a mut Self::Item
    where
        Default: FnOnce() -> Self::Item;
}

#[cfg(feature = "std")]
mod std_ext;
