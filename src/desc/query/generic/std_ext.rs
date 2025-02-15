use crate::desc::{ParsedField, Query, QueryMut};
use std::{
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
};

macro_rules! impl_pointer {
    ($wrapper:ident) => {
        impl<'a, Querier: Query<'a>> Query<'a> for $wrapper<Querier> {
            fn query_raw_text(&self, field: ParsedField) -> Option<&'a str> {
                Querier::query_raw_text(self, field)
            }
        }

        impl<'a, Querier: Query<'a>> QueryMut<'a> for $wrapper<Querier> {
            fn query_raw_text_mut(&mut self, field: ParsedField) -> Option<&'a str> {
                self.query_raw_text(field)
            }
        }
    };
}

impl_pointer!(Box);
impl_pointer!(Rc);
impl_pointer!(Arc);

macro_rules! impl_lock {
    ($wrapper:ident, $lock:ident) => {
        impl<'a, Querier: QueryMut<'a>> Query<'a> for $wrapper<Querier> {
            fn query_raw_text(&self, field: ParsedField) -> Option<&'a str> {
                self.$lock()
                    .expect("lock must be acquired successfully")
                    .query_raw_text_mut(field)
            }
        }

        impl<'a, Querier: QueryMut<'a>> QueryMut<'a> for $wrapper<Querier> {
            fn query_raw_text_mut(&mut self, field: ParsedField) -> Option<&'a str> {
                self.query_raw_text(field)
            }
        }
    };
}

impl_lock!(Mutex, lock);
impl_lock!(RwLock, write);
