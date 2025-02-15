use super::{Query, QueryMut, QueryRawTextItem};
use crate::srcinfo::FieldName;

impl<'a, Querier: Query<'a>> Query<'a> for &'a Querier {
    fn query_raw_text(&self, field_name: FieldName) -> impl Iterator<Item = QueryRawTextItem<'a>> {
        Querier::query_raw_text(*self, field_name)
    }
}

impl<'a, Querier: Query<'a>> QueryMut<'a> for &'a Querier {
    fn query_raw_text_mut(
        &mut self,
        field_name: FieldName,
    ) -> impl Iterator<Item = QueryRawTextItem<'a>> {
        self.query_raw_text(field_name)
    }
}

impl<'a, Querier: QueryMut<'a>> QueryMut<'a> for &'a mut Querier {
    fn query_raw_text_mut(
        &mut self,
        field_name: FieldName,
    ) -> impl Iterator<Item = QueryRawTextItem<'a>> {
        Querier::query_raw_text_mut(*self, field_name)
    }
}
