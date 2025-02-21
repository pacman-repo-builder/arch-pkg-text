use crate::desc::{ParsedField, Query, QueryMut, misc::ReuseAdvice};
use std::{
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
};

macro_rules! impl_reuse {
    ($wrapper:ident) => {
        impl<Querier: ReuseAdvice + ?Sized> ReuseAdvice for $wrapper<Querier> {
            type ShouldReuse = Querier::ShouldReuse;
        }
    };
}

macro_rules! impl_pointer {
    ($wrapper:ident) => {
        impl<'a, Querier: Query<'a> + ?Sized> Query<'a> for $wrapper<Querier> {
            fn query_raw_text(&self, field: ParsedField) -> Option<&'a str> {
                Querier::query_raw_text(self, field)
            }
        }

        impl<'a, Querier: Query<'a> + ?Sized> QueryMut<'a> for $wrapper<Querier> {
            fn query_raw_text_mut(&mut self, field: ParsedField) -> Option<&'a str> {
                self.query_raw_text(field)
            }
        }

        impl_reuse!($wrapper);
    };
}

impl_pointer!(Box);
impl_pointer!(Rc);
impl_pointer!(Arc);

macro_rules! impl_lock {
    ($wrapper:ident, $lock:ident) => {
        impl<'a, Querier: QueryMut<'a> + ?Sized> Query<'a> for $wrapper<Querier> {
            fn query_raw_text(&self, field: ParsedField) -> Option<&'a str> {
                self.$lock()
                    .expect("lock must be acquired successfully")
                    .query_raw_text_mut(field)
            }
        }

        impl<'a, Querier: QueryMut<'a> + ?Sized> QueryMut<'a> for $wrapper<Querier> {
            fn query_raw_text_mut(&mut self, field: ParsedField) -> Option<&'a str> {
                self.query_raw_text(field)
            }
        }

        impl_reuse!($wrapper);
    };
}

impl_lock!(Mutex, lock);
impl_lock!(RwLock, write);
