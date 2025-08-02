use super::{Query, QueryMut, QueryRawTextItem};
use crate::srcinfo::{FieldName, misc::ReuseAdvice};
use core::{
    ops::{Deref, DerefMut},
    pin::Pin,
};

impl<'a, Querier: Query<'a> + ?Sized> Query<'a> for &Querier {
    fn query_raw_text(&self, field_name: FieldName) -> impl Iterator<Item = QueryRawTextItem<'a>> {
        Querier::query_raw_text(*self, field_name)
    }
}

impl<'a, Querier: Query<'a> + ?Sized> QueryMut<'a> for &Querier {
    fn query_raw_text_mut(
        &mut self,
        field_name: FieldName,
    ) -> impl Iterator<Item = QueryRawTextItem<'a>> {
        self.query_raw_text(field_name)
    }
}

impl<'a, Querier: QueryMut<'a> + ?Sized> QueryMut<'a> for &mut Querier {
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

impl<Querier: ReuseAdvice + ?Sized> ReuseAdvice for &Querier {
    type ShouldReuse = Querier::ShouldReuse;
}

impl<Querier: ReuseAdvice + ?Sized> ReuseAdvice for &mut Querier {
    type ShouldReuse = Querier::ShouldReuse;
}

impl<Ptr: Deref<Target: ReuseAdvice>> ReuseAdvice for Pin<Ptr> {
    type ShouldReuse = <Ptr::Target as ReuseAdvice>::ShouldReuse;
}

#[cfg(feature = "std")]
mod std_ext;
