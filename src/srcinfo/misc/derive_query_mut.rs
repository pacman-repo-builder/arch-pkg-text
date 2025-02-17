use super::ReuseAdvice;
use crate::srcinfo::{FieldName, Query, QueryMut, QueryRawTextItem};

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

impl<'a, Querier: Query<'a> + ReuseAdvice + ?Sized> ReuseAdvice for DeriveQueryMut<Querier> {
    type ShouldReuse = Querier::ShouldReuse;
}
