use super::{GetEntry, OrInsertWith};
use core::hash::{BuildHasher, Hash};
use std::collections::{btree_map, hash_map, BTreeMap, HashMap};

impl<'a, Key, Value, State> GetEntry<Key> for &'a mut HashMap<Key, Value, State>
where
    Key: Eq + Hash,
    State: BuildHasher,
{
    type Entry = hash_map::Entry<'a, Key, Value>;
    fn get_entry(self, key: Key) -> Self::Entry {
        self.entry(key)
    }
}

#[deny(unconditional_recursion)]
impl<'a, Key, Value> OrInsertWith<'a> for hash_map::Entry<'a, Key, Value> {
    type Item = Value;
    fn or_insert_with<Default>(self, default: Default) -> &'a mut Self::Item
    where
        Default: FnOnce() -> Value,
    {
        self.or_insert_with(default)
    }
}

impl<'a, Key, Value> GetEntry<Key> for &'a mut BTreeMap<Key, Value>
where
    Key: Ord,
{
    type Entry = btree_map::Entry<'a, Key, Value>;
    fn get_entry(self, key: Key) -> Self::Entry {
        self.entry(key)
    }
}

#[deny(unconditional_recursion)]
impl<'a, Key, Value> OrInsertWith<'a> for btree_map::Entry<'a, Key, Value>
where
    Key: Ord,
{
    type Item = Value;
    fn or_insert_with<Default>(self, default: Default) -> &'a mut Self::Item
    where
        Default: FnOnce() -> Self::Item,
    {
        self.or_insert_with(default)
    }
}
