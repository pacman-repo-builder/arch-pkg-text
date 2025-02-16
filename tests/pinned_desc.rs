use arch_pkg_text::{
    desc::{ParsedField, Query, QueryMut},
    value::Base,
};
use core::pin::Pin;

#[test]
fn type_check_query() {
    struct _UnsizedQuerier {
        _data: str,
    }

    impl<'a> Query<'a> for _UnsizedQuerier {
        fn query_raw_text(&self, _: ParsedField) -> Option<&'a str> {
            unimplemented!()
        }
    }

    fn _ref(querier: Pin<&_UnsizedQuerier>) -> Option<Base<'_>> {
        querier.base()
    }

    #[cfg(feature = "std")]
    fn _box(querier: Pin<Box<_UnsizedQuerier>>) -> Option<Base<'static>> {
        querier.base()
    }
}

#[test]
fn type_check_query_mut() {
    struct _UnsizedQuerier {
        _data: str,
    }

    impl<'a> QueryMut<'a> for _UnsizedQuerier {
        fn query_raw_text_mut(&mut self, _: ParsedField) -> Option<&'a str> {
            unimplemented!()
        }
    }

    fn _ref(mut querier: Pin<&mut _UnsizedQuerier>) -> Option<Base<'_>> {
        querier.base_mut()
    }

    #[cfg(feature = "std")]
    fn _box(mut querier: Pin<Box<_UnsizedQuerier>>) -> Option<Base<'static>> {
        querier.base_mut()
    }
}
