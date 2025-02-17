//! Miscellaneous items.

use super::{EncourageReuse, Query, QueryMut};
use crate::desc::ParsedField;

/// Wrapper struct to permit [`QueryMut`] on a struct that only implements [`Query`].
#[derive(Debug, Default, Clone, Copy)]
pub struct DeriveQueryMut<Querier: ?Sized>(pub Querier);

impl<'a, Querier: Query<'a> + ?Sized> Query<'a> for DeriveQueryMut<Querier> {
    fn query_raw_text(&self, field: ParsedField) -> Option<&'a str> {
        self.0.query_raw_text(field)
    }
}

impl<'a, Querier: Query<'a> + ?Sized> QueryMut<'a> for DeriveQueryMut<Querier> {
    fn query_raw_text_mut(&mut self, field: ParsedField) -> Option<&'a str> {
        self.query_raw_text(field)
    }
}

impl<'a, Querier: Query<'a> + EncourageReuse + ?Sized> EncourageReuse for DeriveQueryMut<Querier> {
    const ENCOURAGE_REUSE: bool = Querier::ENCOURAGE_REUSE;
}
