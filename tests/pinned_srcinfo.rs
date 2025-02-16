use arch_pkg_text::{
    srcinfo::{FieldName, Query, QueryMut, QueryRawTextItem},
    value::Base,
};
use core::{iter::empty, pin::Pin};

#[test]
fn type_check_query() {
    struct _UnsizedQuerier {
        _data: str,
    }

    impl<'a> Query<'a> for _UnsizedQuerier {
        fn query_raw_text(&self, _: FieldName) -> impl Iterator<Item = QueryRawTextItem<'a>> {
            empty()
        }
    }

    fn _ref(querier: Pin<&_UnsizedQuerier>) -> Option<Base<'_>> {
        querier.base_name()
    }

    #[cfg(feature = "std")]
    fn _box(querier: Pin<Box<_UnsizedQuerier>>) -> Option<Base<'static>> {
        querier.base_name()
    }
}

#[test]
fn type_check_query_mut() {
    struct _UnsizedQuerier {
        _data: str,
    }

    impl<'a> QueryMut<'a> for _UnsizedQuerier {
        fn query_raw_text_mut(
            &mut self,
            _: FieldName,
        ) -> impl Iterator<Item = QueryRawTextItem<'a>> {
            empty()
        }
    }

    fn _ref(mut querier: Pin<&mut _UnsizedQuerier>) -> Option<Base<'_>> {
        querier.base_name_mut()
    }

    #[cfg(feature = "std")]
    fn _box(mut querier: Pin<Box<_UnsizedQuerier>>) -> Option<Base<'static>> {
        querier.base_name_mut()
    }
}
