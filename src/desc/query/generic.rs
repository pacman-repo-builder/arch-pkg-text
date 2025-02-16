use super::{Query, QueryMut};
use crate::desc::ParsedField;

impl<'a, Querier: Query<'a> + ?Sized> Query<'a> for &'a Querier {
    fn query_raw_text(&self, field: ParsedField) -> Option<&'a str> {
        Querier::query_raw_text(*self, field)
    }
}

impl<'a, Querier: Query<'a> + ?Sized> QueryMut<'a> for &'a Querier {
    fn query_raw_text_mut(&mut self, field: ParsedField) -> Option<&'a str> {
        self.query_raw_text(field)
    }
}

impl<'a, Querier: QueryMut<'a> + ?Sized> QueryMut<'a> for &'a mut Querier {
    fn query_raw_text_mut(&mut self, field: ParsedField) -> Option<&'a str> {
        Querier::query_raw_text_mut(*self, field)
    }
}

#[cfg(feature = "parking_lot")]
mod parking_lot_ext;
#[cfg(feature = "std")]
mod std_ext;
