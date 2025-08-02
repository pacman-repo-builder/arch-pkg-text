use super::{Query, QueryMut};
use crate::desc::{ParsedField, misc::ReuseAdvice};
use core::{
    ops::{Deref, DerefMut},
    pin::Pin,
};

impl<'a, Querier: Query<'a> + ?Sized> Query<'a> for &Querier {
    fn query_raw_text(&self, field: ParsedField) -> Option<&'a str> {
        Querier::query_raw_text(*self, field)
    }
}

impl<'a, Querier: Query<'a> + ?Sized> QueryMut<'a> for &Querier {
    fn query_raw_text_mut(&mut self, field: ParsedField) -> Option<&'a str> {
        self.query_raw_text(field)
    }
}

impl<'a, Querier: QueryMut<'a> + ?Sized> QueryMut<'a> for &mut Querier {
    fn query_raw_text_mut(&mut self, field: ParsedField) -> Option<&'a str> {
        Querier::query_raw_text_mut(*self, field)
    }
}

impl<'a, Ptr: Deref<Target: Query<'a>>> Query<'a> for Pin<Ptr> {
    fn query_raw_text(&self, field: ParsedField) -> Option<&'a str> {
        self.deref().query_raw_text(field)
    }
}

impl<'a, Ptr: DerefMut<Target: QueryMut<'a> + Unpin>> QueryMut<'a> for Pin<Ptr> {
    fn query_raw_text_mut(&mut self, field: ParsedField) -> Option<&'a str> {
        self.deref_mut().query_raw_text_mut(field)
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

#[cfg(feature = "parking_lot")]
mod parking_lot_ext;
#[cfg(feature = "std")]
mod std_ext;
