use super::{EncourageReuse, Query, QueryMut, QueryRawTextItem};
use crate::srcinfo::FieldName;
use core::{
    ops::{Deref, DerefMut},
    pin::Pin,
};

impl<'a, Querier: Query<'a> + ?Sized> Query<'a> for &'a Querier {
    fn query_raw_text(&self, field_name: FieldName) -> impl Iterator<Item = QueryRawTextItem<'a>> {
        Querier::query_raw_text(*self, field_name)
    }
}

impl<'a, Querier: Query<'a> + ?Sized> QueryMut<'a> for &'a Querier {
    fn query_raw_text_mut(
        &mut self,
        field_name: FieldName,
    ) -> impl Iterator<Item = QueryRawTextItem<'a>> {
        self.query_raw_text(field_name)
    }
}

impl<'a, Querier: QueryMut<'a> + ?Sized> QueryMut<'a> for &'a mut Querier {
    fn query_raw_text_mut(
        &mut self,
        field_name: FieldName,
    ) -> impl Iterator<Item = QueryRawTextItem<'a>> {
        Querier::query_raw_text_mut(*self, field_name)
    }
}

impl<'a, Ptr: Deref<Target: Query<'a>>> Query<'a> for Pin<Ptr> {
    fn query_raw_text(&self, field_name: FieldName) -> impl Iterator<Item = QueryRawTextItem<'a>> {
        self.deref().query_raw_text(field_name)
    }
}

impl<'a, Ptr: DerefMut<Target: QueryMut<'a> + Unpin>> QueryMut<'a> for Pin<Ptr> {
    fn query_raw_text_mut(
        &mut self,
        field_name: FieldName,
    ) -> impl Iterator<Item = QueryRawTextItem<'a>> {
        self.deref_mut().query_raw_text_mut(field_name)
    }
}

impl<Querier: EncourageReuse + ?Sized> EncourageReuse for &Querier {
    const ENCOURAGE_REUSE: bool = Querier::ENCOURAGE_REUSE;
}

impl<Querier: EncourageReuse + ?Sized> EncourageReuse for &mut Querier {
    const ENCOURAGE_REUSE: bool = Querier::ENCOURAGE_REUSE;
}

impl<Ptr: Deref<Target: EncourageReuse + ?Sized>> EncourageReuse for Pin<Ptr> {
    const ENCOURAGE_REUSE: bool = <Ptr::Target as EncourageReuse>::ENCOURAGE_REUSE;
}

#[cfg(feature = "std")]
mod std_ext;
