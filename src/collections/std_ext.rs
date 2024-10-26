use super::GetMutOrInsertWith;
use core::hash::{BuildHasher, Hash};
use std::collections::{BTreeMap, HashMap};

impl<'a, Key, Value, State> GetMutOrInsertWith<'a, Key> for HashMap<Key, Value, State>
where
    Key: Eq + Hash,
    State: BuildHasher,
{
    type Item = Value;
    fn get_mut_or_insert_with<Default>(&mut self, key: Key, default: Default) -> &mut Self::Item
    where
        Default: FnOnce() -> Self::Item,
    {
        self.entry(key).or_insert_with(default)
    }
}

impl<'a, Key, Value> GetMutOrInsertWith<'a, Key> for BTreeMap<Key, Value>
where
    Key: Ord,
{
    type Item = Value;
    fn get_mut_or_insert_with<Default>(&mut self, key: Key, default: Default) -> &mut Self::Item
    where
        Default: FnOnce() -> Self::Item,
    {
        self.entry(key).or_insert_with(default)
    }
}
