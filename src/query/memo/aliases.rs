use super::MemoQuerier;
use crate::field::ParsedField;
use std::collections::{hash_map::RandomState, BTreeMap, HashMap};

/// [Query](crate::query::Query) with a [hash map](HashMap) cache.
pub type HashMemoQuerier<'a, State = RandomState> =
    MemoQuerier<'a, HashMap<ParsedField, Option<&'a str>, State>>;

/// [Query](crate::query::Query) with a [btree map](BTreeMap) cache.
pub type BTreeMemoQuerier<'a> = MemoQuerier<'a, BTreeMap<ParsedField, Option<&'a str>>>;
