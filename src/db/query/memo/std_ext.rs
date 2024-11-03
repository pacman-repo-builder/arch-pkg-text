use super::MemoQuerier;
use crate::db::{
    field::ParsedField,
    query::{Query, QueryMut},
};
use std::sync::Mutex;

impl<'a> Query<'a> for Mutex<MemoQuerier<'a>> {
    fn query_raw_text(&self, field: ParsedField) -> Option<&'a str> {
        self.lock()
            .expect("lock must be acquired successfully")
            .query_raw_text_mut(field)
    }
}

impl<'a> QueryMut<'a> for Mutex<MemoQuerier<'a>> {
    fn query_raw_text_mut(&mut self, field: ParsedField) -> Option<&'a str> {
        self.query_raw_text(field)
    }
}
