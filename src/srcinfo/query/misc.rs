//! Miscellaneous items.

use super::{EncourageReuse, Query, QueryMut, QueryRawTextItem};
use crate::srcinfo::FieldName;

/// Wrapper struct to permit [`QueryMut`] on a struct that only implements [`Query`].
#[derive(Debug, Default, Clone, Copy)]
pub struct DeriveQueryMut<Querier: ?Sized>(pub Querier);

impl<'a, Querier: Query<'a> + ?Sized> Query<'a> for DeriveQueryMut<Querier> {
    fn query_raw_text(&self, field_name: FieldName) -> impl Iterator<Item = QueryRawTextItem<'a>> {
        self.0.query_raw_text(field_name)
    }
}

impl<'a, Querier: Query<'a> + ?Sized> QueryMut<'a> for DeriveQueryMut<Querier> {
    fn query_raw_text_mut(
        &mut self,
        field_name: FieldName,
    ) -> impl Iterator<Item = QueryRawTextItem<'a>> {
        self.query_raw_text(field_name)
    }
}

impl<'a, Querier: Query<'a> + EncourageReuse + ?Sized> EncourageReuse for DeriveQueryMut<Querier> {
    const ENCOURAGE_REUSE: bool = Querier::ENCOURAGE_REUSE;
}
