use crate::desc::{EncourageReuse, ParsedField, Query, QueryMut};
use parking_lot::{FairMutex, Mutex, RwLock};

macro_rules! impl_lock {
    ($wrapper:ident, $lock:ident) => {
        impl<'a, Querier: QueryMut<'a> + ?Sized> Query<'a> for $wrapper<Querier> {
            fn query_raw_text(&self, field: ParsedField) -> Option<&'a str> {
                self.$lock().query_raw_text_mut(field)
            }
        }

        impl<'a, Querier: QueryMut<'a> + ?Sized> QueryMut<'a> for $wrapper<Querier> {
            fn query_raw_text_mut(&mut self, field: ParsedField) -> Option<&'a str> {
                self.query_raw_text(field)
            }
        }

        impl<Querier: EncourageReuse + ?Sized> EncourageReuse for $wrapper<Querier> {
            const ENCOURAGE_REUSE: bool = Querier::ENCOURAGE_REUSE;
        }
    };
}

impl_lock!(Mutex, lock);
impl_lock!(FairMutex, lock);
impl_lock!(RwLock, write);
