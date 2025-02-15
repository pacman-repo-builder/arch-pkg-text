use crate::srcinfo::{FieldName, Query, QueryMut, QueryRawTextItem};
use std::{rc::Rc, sync::Arc};

macro_rules! impl_pointer {
    ($wrapper:ident) => {
        impl<'a, Querier: Query<'a>> Query<'a> for $wrapper<Querier> {
            fn query_raw_text(
                &self,
                field_name: FieldName,
            ) -> impl Iterator<Item = QueryRawTextItem<'a>> {
                Querier::query_raw_text(self, field_name)
            }
        }

        impl<'a, Querier: Query<'a>> QueryMut<'a> for $wrapper<Querier> {
            fn query_raw_text_mut(
                &mut self,
                field_name: FieldName,
            ) -> impl Iterator<Item = QueryRawTextItem<'a>> {
                self.query_raw_text(field_name)
            }
        }
    };
}

impl_pointer!(Box);
impl_pointer!(Rc);
impl_pointer!(Arc);
