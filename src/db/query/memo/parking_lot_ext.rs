use super::MemoQuerier;
use crate::db::{
    field::ParsedField,
    query::{Query, QueryMut},
};
use parking_lot::{FairMutex, Mutex};

macro_rules! impl_query {
    ($mutex:ident) => {
        impl<'a> Query<'a> for $mutex<MemoQuerier<'a>> {
            fn query_raw_text(&self, field: ParsedField) -> Option<&'a str> {
                self.lock().query_raw_text_mut(field)
            }
        }

        impl<'a> QueryMut<'a> for $mutex<MemoQuerier<'a>> {
            fn query_raw_text_mut(&mut self, field: ParsedField) -> Option<&'a str> {
                self.query_raw_text(field)
            }
        }
    };
}

impl_query!(Mutex);
impl_query!(FairMutex);
