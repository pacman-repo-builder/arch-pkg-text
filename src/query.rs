use crate::field::ParsedField;

pub trait Query<'a>: Sized {
    fn query_raw_text(self, field: ParsedField) -> Option<&'a str>;
}

mod forgetful;

pub use forgetful::ForgetfulQuerier;

// TODO: MemoQuerier?
