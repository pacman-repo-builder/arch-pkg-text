use crate::{
    srcinfo::{
        field::FieldName,
        query::{QueryItem, QueryRawTextItem, Section},
    },
    value::Name,
};
use pipe_trait::Pipe;

macro_rules! def_cache {
    (
        base single ($($base_single_field:ident)*)
        base multi ($($base_multi_field:ident)*)
        shared single ($($shared_single_field:ident)*)
        shared multi no_arch ($($shared_multi_no_arch_field:ident)*)
        shared multi arch ($($shared_multi_arch_field:ident)*)
    ) => {
        #[derive(Debug, Default, Clone)]
        struct SingleValueCache<'a>(Option<&'a str>);

        impl<'a> SingleValueCache<'a> {
            fn get(&self, index: usize) -> Option<&'a str> {
                (index == 0).then_some(()).and(self.0)
            }

            fn add(&mut self, value: &'a str) {
                self.0 = Some(value);
            }

            fn add_opt(&mut self, value: Option<&'a str>) {
                if let Some(value) = value {
                    self.add(value);
                }
            }

            fn shrink_to_fit(&mut self) {}
        }

        #[derive(Debug, Clone)]
        struct MultiValueCache<Value>(Vec<Value>);

        impl<Value: Copy> MultiValueCache<Value> {
            fn get(&self, index: usize) -> Option<Value> {
                self.0.get(index).copied()
            }

            fn add(&mut self, value: Value) {
                self.0.push(value);
            }

            fn add_opt(&mut self, value: Option<Value>) {
                if let Some(value) = value {
                    self.add(value);
                }
            }

            fn shrink_to_fit(&mut self) {
                self.0.shrink_to_fit();
            }
        }

        impl<Value> Default for MultiValueCache<Value> {
            fn default() -> Self {
                MultiValueCache(Vec::new())
            }
        }

        #[derive(Debug, Default, Clone)]
        #[allow(non_snake_case, reason = "We don't access the field names directly, keep it simple.")]
        pub struct Cache<'a> {
            $($base_single_field: SingleValueCache<'a>,)*
            $($base_multi_field: MultiValueCache<&'a str>,)*
            $($shared_single_field: MultiValueCache<QueryItem<'a, &'a str, ()>>,)*
            $($shared_multi_no_arch_field: MultiValueCache<QueryItem<'a, &'a str, ()>>,)*
            $($shared_multi_arch_field: MultiValueCache<QueryRawTextItem<'a>>,)*
            derivative_names: MultiValueCache<&'a str>,
        }

        impl<'a> Cache<'a> {
            pub fn get(&self, field_name: FieldName, index: usize) -> Option<QueryRawTextItem<'a>> {
                match field_name {
                    $(FieldName::$base_single_field => self.$base_single_field.get(index).map(Cache::create_base_value),)*
                    $(FieldName::$base_multi_field => self.$base_multi_field.get(index).map(Cache::create_base_value),)*
                    $(FieldName::$shared_single_field => self.$shared_single_field.get(index).map(Cache::create_shared_value_no_arch),)*
                    $(FieldName::$shared_multi_no_arch_field => self.$shared_multi_no_arch_field.get(index).map(Cache::create_shared_value_no_arch),)*
                    $(FieldName::$shared_multi_arch_field => self.$shared_multi_arch_field.get(index),)*
                    FieldName::Name => self.derivative_names.get(index).map(Cache::create_name_value),
                }
            }

            fn create_base_value(value: &str) -> QueryRawTextItem<'_> {
                QueryRawTextItem::from_tuple3((value, Section::Base, None))
            }

            fn create_shared_value_no_arch(value: QueryItem<'a, &'a str, ()>) -> QueryRawTextItem<'a> {
                let (value, section) = value.into_tuple2();
                QueryRawTextItem::from_tuple3((value, section, None))
            }

            fn create_name_value(value: &str) -> QueryRawTextItem<'_> {
                let section = value.pipe(Name).pipe(Section::Derivative);
                QueryRawTextItem::from_tuple3((value, section, None))
            }

            pub fn add(&mut self, field_name: FieldName, item: QueryRawTextItem<'a>) {
                match field_name {
                    $(FieldName::$base_single_field => self.$base_single_field.add_opt(Cache::extract_base_value(item)),)*
                    $(FieldName::$base_multi_field => self.$base_multi_field.add_opt(Cache::extract_base_value(item)),)*
                    $(FieldName::$shared_single_field => self.$shared_single_field.add_opt(Cache::extract_shared_value_no_arch(item)),)*
                    $(FieldName::$shared_multi_no_arch_field => self.$shared_multi_no_arch_field.add_opt(Cache::extract_shared_value_no_arch(item)),)*
                    $(FieldName::$shared_multi_arch_field => self.$shared_multi_arch_field.add(item),)*
                    FieldName::Name => self.derivative_names.add_opt(Cache::extract_name_value(item)),
                }
            }

            fn extract_base_value(item: QueryRawTextItem) -> Option<&'_ str> {
                let (value, section, architecture) = item.into_tuple3();
                debug_assert_eq!(section, Section::Base);
                architecture.is_none().then_some(value)
            }

            fn extract_shared_value_no_arch(item: QueryRawTextItem) -> Option<QueryItem<'_, &'_ str, ()>> {
                let (value, section, architecture) = item.into_tuple3();
                architecture
                    .is_none()
                    .then_some(QueryItem::from_tuple2((value, section)))
            }

            fn extract_name_value(item: QueryRawTextItem) -> Option<&'_ str> {
                let (value, section, architecture) = item.into_tuple3();
                architecture.is_none().then_some(())?;
                debug_assert_eq!(section, value.pipe(Name).pipe(Section::Derivative));
                Some(value)
            }

            pub fn shrink_to_fit(&mut self) {
                $(self.$base_single_field.shrink_to_fit();)*
                $(self.$base_multi_field.shrink_to_fit();)*
                $(self.$shared_single_field.shrink_to_fit();)*
                $(self.$shared_multi_no_arch_field.shrink_to_fit();)*
                $(self.$shared_multi_arch_field.shrink_to_fit();)*
                self.derivative_names.shrink_to_fit();
            }
        }
    };
}

def_cache! {
    base single (Base Epoch Release Version)
    base multi (ValidPgpKeys)
    shared single (Description ChangeLog InstallScript Url)
    shared multi no_arch (Architecture Backup Groups License NoExtract Options)
    shared multi arch (
        Source Dependencies MakeDependencies CheckDependencies OptionalDependencies Provides Conflicts Replaces
        Md5Checksums Sha1Checksums Sha224Checksums Sha256Checksums Sha384Checksums Sha512Checksums Blake2bChecksums
    )
}
