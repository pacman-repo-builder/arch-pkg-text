use super::{
    utils::{parse_line, trimmed_line_is_blank},
    QueryBaseFieldMut, QueryDerivativeFieldMut, QueryFieldMut, QueryRawTextItem, QuerySectionAssoc,
    QuerySectionMut,
};
use crate::{
    srcinfo::field::{FieldName, ParsedField},
    value::{Base, Name},
};
use core::{ops::ControlFlow, str::Lines};
use pipe_trait::Pipe;
use std::{collections::HashMap, sync::Mutex};

/// [Query the sections](QuerySectionMut) of a `.SRCINFO` text with a cache.
#[derive(Debug, Clone)]
pub struct MemoSectionQuerier<'a> {
    base_name: Base<'a>,
    base_querier: SectionContentQuerier<'a, BaseCache<'a>>,
    complete_derivatives:
        HashMap<Name<'a>, SectionContentQuerier<'a, DerivativeExclusiveCache<'a>>>,
    state: SectionQuerierState<'a>,
}

/// Construction state of the sections in [`MemoSectionQuerier`].
#[derive(Debug, Clone, Copy)]
enum SectionQuerierState<'a> {
    ConstructingBase,
    ConstructingSingleDerivativeExclusive(Name<'a>),
    AllCompleted,
}

impl<'a> MemoSectionQuerier<'a> {
    /// Create a querier for the sections of `.SRCINFO`.
    pub fn new(srcinfo: &'a str) -> Option<Self> {
        let (base_name, base_querier) = MemoBaseSection::new_pair(srcinfo)?;
        Some(MemoSectionQuerier {
            base_name,
            base_querier,
            complete_derivatives: HashMap::new(),
            state: SectionQuerierState::ConstructingBase,
        })
    }

    /// Access the a derivative exclusive querier.
    fn access_derivative(
        &mut self,
        name: Name<'a>,
    ) -> Option<&'_ mut SectionContentQuerier<'a, DerivativeExclusiveCache<'a>>> {
        match &self.state {
            SectionQuerierState::ConstructingBase => None,
            SectionQuerierState::ConstructingSingleDerivativeExclusive(_)
            | SectionQuerierState::AllCompleted => self.complete_derivatives.get_mut(&name),
        }
    }

    fn complete_till_derivative(
        &mut self,
        name: Name<'a>,
    ) -> Option<&'_ mut SectionContentQuerier<'a, DerivativeExclusiveCache<'a>>> {
        loop {
            self.state = match self.state {
                SectionQuerierState::ConstructingBase => match self.base_querier.load_all() {
                    Some(name) => SectionQuerierState::ConstructingSingleDerivativeExclusive(name),
                    None => SectionQuerierState::AllCompleted,
                },
                SectionQuerierState::ConstructingSingleDerivativeExclusive(constructing_name) => {
                    // let querier = self.complete_derivatives.get_mut(&name);
                    if let Some(querier) = self.complete_derivatives.get_mut(&name) {
                        return Some(querier);
                    }
                    // if let Some(querier) = self.complete_derivatives.get_mut(&constructing_name) {
                    //     querier.loa
                    // }
                    if name == constructing_name {
                        return None;
                    }
                    // let a = self
                    //     .complete_derivatives
                    //     .get_mut(&constructing_name)
                    //     .map(|querier| querier.load_all());
                    todo!()
                    // match self.complete_derivatives.get_mut(&constructing_name) {
                    //     Some(querier) => todo!(),
                    //     None => SectionQuerierState::AllCompleted,
                    // }
                }
                SectionQuerierState::AllCompleted => {
                    return self.complete_derivatives.get_mut(&name)
                }
            }
        }
    }

    // /// Construct previous queriers before access the desired derivative exclusive querier.
    // fn complete_derivative(
    //     &mut self,
    //     name: Name<'a>,
    // ) -> Option<&'_ mut SectionContentQuerier<'a, DerivativeExclusiveCache<'a>>> {
    //     loop {
    //         let (next_state, flow) = match &mut self.state {
    //             SectionQuerierState::ConstructingBase => {
    //                 if let Some(next_name) = self.base_querier.load_all() {
    //                     let next_state = self
    //                         .base_querier
    //                         .lines
    //                         .clone()
    //                         .pipe(SectionContentQuerier::from_lines)
    //                         .pipe(move |querier| (next_name, querier))
    //                         .pipe(Box::new)
    //                         .pipe(SectionQuerierState::ConstructingSingleDerivativeExclusive);
    //                     (next_state, ControlFlow::Continue(()))
    //                 } else {
    //                     (SectionQuerierState::AllCompleted, ControlFlow::Break(None))
    //                 }
    //             }
    //             SectionQuerierState::ConstructingSingleDerivativeExclusive(name) => {
    //                 if pair.0 == name {
    //                     return Some(&mut pair.1);
    //                 }
    //                 todo!("Something with {pair:?}") // TODO
    //             }
    //             SectionQuerierState::AllCompleted => return None,
    //         };
    //         self.state = next_state;
    //         if let ControlFlow::Break(querier) = flow {
    //             return querier;
    //         }
    //     }
    // }
}

impl<'a> QuerySectionAssoc for MemoSectionQuerier<'a> {
    type BaseSection = MemoBaseSection<'a, 'a>; // TODO: improve lifetime ergonomic, maybe remove the second one
    type DerivativeExclusiveSection = MemoDerivativeExclusiveSection<'a, 'a>; // TODO: improve lifetime ergonomic, maybe remove the second one
}

#[derive(Debug)]
pub struct MemoBaseSection<'a, 'r> {
    name: Base<'a>,
    querier: &'r mut SectionContentQuerier<'a, BaseCache<'a>>,
}

impl<'a, 'r> MemoBaseSection<'a, 'r> {
    fn new_pair(srcinfo: &'a str) -> Option<(Base<'a>, SectionContentQuerier<'a, BaseCache<'a>>)> {
        let (trimmed_line, line) = srcinfo
            .lines()
            .map(|line| (line.trim(), line))
            .find(|(trimmed_line, _)| !trimmed_line_is_blank(trimmed_line))?;

        let (raw_field, value) = parse_line(trimmed_line)?;
        let field = raw_field.to_parsed::<FieldName, &str>().ok()?;
        (*field.name() == FieldName::Base && field.architecture().is_none()).then_some(())?;
        let name = Base(value);

        let start_offset = line.as_ptr() as usize + line.len() - srcinfo.as_ptr() as usize;
        let under_base_header = &srcinfo[start_offset..];
        let querier = SectionContentQuerier::from_text(under_base_header);

        Some((name, querier))
    }
}

impl<'a, 'r> QueryBaseFieldMut<'a> for MemoBaseSection<'a, 'r> {
    fn name_mut(&mut self) -> Base<'a> {
        self.name
    }
}

impl<'a, 'r> QueryFieldMut<'a> for MemoBaseSection<'a, 'r> {
    fn query_raw_text_mut(
        &mut self,
        field: FieldName,
    ) -> impl IntoIterator<Item = QueryRawTextItem<'a>> {
        QueryFieldIterator {
            section: self.querier,
            field,
            index: 0,
        }
    }
}

#[derive(Debug)]
pub struct MemoDerivativeExclusiveSection<'a, 'r> {
    name: Name<'a>,
    querier: &'r mut SectionContentQuerier<'a, DerivativeExclusiveCache<'a>>,
}

impl<'a, 'r> QueryDerivativeFieldMut<'a> for MemoDerivativeExclusiveSection<'a, 'r> {
    fn name_mut(&mut self) -> Name<'a> {
        self.name
    }
}

impl<'a, 'r> QueryFieldMut<'a> for MemoDerivativeExclusiveSection<'a, 'r> {
    fn query_raw_text_mut(
        &mut self,
        field: FieldName,
    ) -> impl IntoIterator<Item = QueryRawTextItem<'a>> {
        QueryFieldIterator {
            section: self.querier,
            field,
            index: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Entry<'a> {
    field: ParsedField<&'a str>,
    value: &'a str,
}

#[derive(Debug, Clone)]
struct SectionContentQuerier<'a, Cache> {
    lines: Lines<'a>,
    cache: Cache,
    next_name: Option<Name<'a>>,
}

impl<'a, Cache: Default> SectionContentQuerier<'a, Cache> {
    /// Create an instance of [`SectionContentQuerier`] from a string.
    fn from_text(text: &'a str) -> Self {
        Self::from_lines(text.lines())
    }

    /// Create an instance of [`SectionContentQuerier`] from [`Lines`].
    fn from_lines(lines: Lines<'a>) -> Self {
        SectionContentQuerier {
            lines,
            cache: Cache::default(),
            next_name: None,
        }
    }
}

impl<'a, Cache: MemoCache<'a>> SectionContentQuerier<'a, Cache> {
    /// Parse the next key-value pair, save it to the cache and return it.
    fn next_entry(&mut self) -> Option<Entry<'a>> {
        let run = |lines: &mut Lines<'a>| loop {
            let line = lines.next().ok_or(None)?.trim();

            if trimmed_line_is_blank(line) {
                continue;
            }

            let (raw_field, value) = parse_line(line).ok_or(None)?;

            let Ok(field) = raw_field.to_parsed::<FieldName, &str>() else {
                continue;
            };

            if *field.name() == FieldName::Name {
                return value.pipe(Name).pipe(Some).pipe(Err);
            }

            return Ok(Entry { field, value });
        };

        match run(&mut self.lines) {
            Ok(entry) => {
                self.cache.add(
                    *entry.field.name(),
                    entry.field.architecture_str(),
                    Some(entry.value),
                );
                Some(entry)
            }
            Err(next_name) => {
                self.next_name = next_name;
                None
            }
        }
    }

    /// Get a value of `field` at `index`.
    fn get(&mut self, field: FieldName, index: usize) -> Option<QueryRawTextItem<'a>> {
        loop {
            if let Some(value) = self.cache.get(field, index) {
                return value;
            }

            let discovery = self.cache.discovery(field);

            if discovery.completed || discovery.discovered <= index {
                return None;
            }

            if self.next_entry().is_none() {
                self.cache.add(field, None, None);
                return None;
            }
        }
    }

    /// Load all of this section, and return the name of the next section if it exists.
    fn load_all(&mut self) -> Option<Name<'a>> {
        while self.next_entry().is_some() {}
        self.next_name
    }
}

struct QueryFieldIterator<'r, 'a, Cache> {
    section: &'r mut SectionContentQuerier<'a, Cache>,
    field: FieldName,
    index: usize,
}

impl<'r, 'a, Cache: MemoCache<'a>> Iterator for QueryFieldIterator<'r, 'a, Cache> {
    type Item = QueryRawTextItem<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.section.get(self.field, self.index)?;
        self.index += 1;
        Some(item)
    }
}

macro_rules! debug_unreachable {
    ($default:expr $(, $($message:tt)*)?) => {
        if cfg!(debug_assertions) {
            unreachable!($($($message)*)?)
        } else {
            $default
        }
    };
}

macro_rules! def_cache {
    (
        base single ($($base_single_field:ident)*)
        base multi ($($base_multi_field:ident)*)
        shared single ($($shared_single_field:ident)*)
        shared multi no_arch ($($shared_multi_no_arch_field:ident)*)
        shared multi arch ($($shared_multi_arch_field:ident)*)
    ) => {
        #[derive(Debug, Default, Clone, Copy)]
        struct CacheDiscovery {
            discovered: usize,
            completed: bool,
        }

        #[derive(Debug, Clone, Copy)]
        enum CacheErr {
            OccupiedWithNone,
            Unoccupied,
        }

        #[derive(Debug, Clone)]
        pub struct SingleValueCache<'a>(Result<&'a str, CacheErr>); // Result<&str, CacheErr> uses less memory than Option<Option<&str>>

        impl<'a> SingleValueCache<'a> {
            fn new() -> Self {
                SingleValueCache(Err(CacheErr::Unoccupied))
            }

            fn get_0(&self) -> Option<Option<&'a str>> {
                match self.0 {
                    Ok(value) => Some(Some(value)),
                    Err(CacheErr::OccupiedWithNone) => Some(None),
                    Err(CacheErr::Unoccupied) => None,
                }
            }

            fn get(&self, index: usize) -> Option<Option<&'a str>> {
                match index {
                    0 => self.get_0(),
                    _ => None,
                }
            }

            fn add(&mut self, value: Option<&'a str>) {
                self.0 = match value {
                    Some(value) => Ok(value),
                    None => Err(CacheErr::OccupiedWithNone),
                }
            }

            fn discovery(&self) -> CacheDiscovery {
                match self.get_0() {
                    Some(Some(_)) => CacheDiscovery { discovered: 1, completed: true },
                    Some(None) => CacheDiscovery { discovered: 0, completed: true },
                    None => CacheDiscovery { discovered: 0, completed: false },
                }
            }
        }

        #[derive(Debug, Clone)]
        struct MultiValueCache<Value> {
            discovered: Vec<Value>,
            completed: bool,
        }

        impl<Value> MultiValueCache<Value> {
            fn new() -> Self {
                MultiValueCache {
                    discovered: Vec::new(),
                    completed: false,
                }
            }

            fn get(&self, index: usize) -> Option<Option<Value>>
            where
                Value: Copy,
            {
                match (self.discovered.get(index), self.completed) {
                    (Some(value), _) => Some(Some(*value)),
                    (None, true) => Some(None),
                    (None, false) => None,
                }
            }

            fn add(&mut self, value: Option<Value>) {
                if let Some(value) = value {
                    self.discovered.push(value)
                } else {
                    self.discovered.shrink_to_fit();
                    self.completed = true;
                }
            }

            fn discovery(&self) -> CacheDiscovery {
                CacheDiscovery {
                    discovered: self.discovered.len(),
                    completed: self.completed,
                }
            }
        }

        impl<'a> MultiValueCache<QueryRawTextItem<'a>> {
            fn add_with_architecture(&mut self, architecture: Option<&'a str>, value: Option<&'a str>) {
                self.add(value.map(|value| QueryRawTextItem { architecture, value }))
            }
        }

        fn create_raw_item_nested_option(get_result: Option<Option<&str>>) -> Option<Option<QueryRawTextItem<'_>>> {
            get_result.map(|get_result| get_result.map(|value| QueryRawTextItem { architecture: None, value }))
        }

        trait MemoCache<'a> {
            fn get(&self, field: FieldName, index: usize) -> Option<Option<QueryRawTextItem<'a>>>;
            fn add(&mut self, field: FieldName, architecture: Option<&'a str>, value: Option<&'a str>);
            fn discovery(&self, field: FieldName) -> CacheDiscovery;
        }

        #[derive(Debug, Clone)]
        #[allow(non_snake_case, reason = "We don't access the field names directly, keep it simple.")]
        struct BaseCache<'a> {
            $($base_single_field: SingleValueCache<'a>,)*
            $($shared_single_field: SingleValueCache<'a>,)*
            $($base_multi_field: MultiValueCache<&'a str>,)*
            $($shared_multi_no_arch_field: MultiValueCache<&'a str>,)*
            $($shared_multi_arch_field: MultiValueCache<QueryRawTextItem<'a>>,)*
        }

        impl<'a> MemoCache<'a> for BaseCache<'a> {
            fn get(&self, field: FieldName, index: usize) -> Option<Option<QueryRawTextItem<'a>>> {
                match field {
                    $(FieldName::$base_single_field => create_raw_item_nested_option(self.$base_single_field.get(index)),)*
                    $(FieldName::$shared_single_field => create_raw_item_nested_option(self.$shared_single_field.get(index)),)*
                    $(FieldName::$base_multi_field => create_raw_item_nested_option(self.$base_multi_field.get(index)),)*
                    $(FieldName::$shared_multi_no_arch_field => create_raw_item_nested_option(self.$shared_multi_no_arch_field.get(index)),)*
                    $(FieldName::$shared_multi_arch_field => self.$shared_multi_arch_field.get(index),)*
                    _ => debug_unreachable!(None, "shouldn't query {field:?}"),
                }
            }

            fn add(&mut self, field: FieldName, architecture: Option<&'a str>, value: Option<&'a str>) {
                match (field, architecture, value) {
                    $((FieldName::$base_single_field, None, _) => self.$base_single_field.add(value),)*
                    $((FieldName::$shared_single_field, None, _) => self.$shared_single_field.add(value),)*
                    $((FieldName::$base_multi_field, None, _) => self.$base_multi_field.add(value),)*
                    $((FieldName::$shared_multi_no_arch_field, None, _) => self.$shared_multi_no_arch_field.add(value),)*
                    $((FieldName::$shared_multi_arch_field, _, _) => self.$shared_multi_arch_field.add_with_architecture(architecture, value),)*
                    _ => debug_unreachable!((), "shouldn't query {field:?}"),
                }
            }

            fn discovery(&self, field: FieldName) -> CacheDiscovery {
                match field {
                    $(FieldName::$base_single_field => self.$base_single_field.discovery(),)*
                    $(FieldName::$shared_single_field => self.$shared_single_field.discovery(),)*
                    $(FieldName::$base_multi_field => self.$base_multi_field.discovery(),)*
                    $(FieldName::$shared_multi_no_arch_field => self.$shared_multi_no_arch_field.discovery(),)*
                    $(FieldName::$shared_multi_arch_field => self.$shared_multi_arch_field.discovery(),)*
                    _ => debug_unreachable!(CacheDiscovery::default(), "shouldn't query {field:?}"),
                }
            }
        }

        impl<'a> Default for BaseCache<'a> {
            fn default() -> Self {
                BaseCache {
                    $($base_single_field: SingleValueCache::new(),)*
                    $($shared_single_field: SingleValueCache::new(),)*
                    $($base_multi_field: MultiValueCache::new(),)*
                    $($shared_multi_no_arch_field: MultiValueCache::new(),)*
                    $($shared_multi_arch_field: MultiValueCache::new(),)*
                }
            }
        }

        #[derive(Debug, Clone)]
        #[allow(non_snake_case, reason = "We don't access the field names directly, keep it simple.")]
        struct DerivativeExclusiveCache<'a> {
            $($shared_single_field: SingleValueCache<'a>,)*
            $($shared_multi_no_arch_field: MultiValueCache<&'a str>,)*
            $($shared_multi_arch_field: MultiValueCache<QueryRawTextItem<'a>>,)*
        }

        impl<'a> MemoCache<'a> for DerivativeExclusiveCache<'a> {
            fn get(&self, field: FieldName, index: usize) -> Option<Option<QueryRawTextItem<'a>>> {
                match field {
                    $(FieldName::$shared_single_field => create_raw_item_nested_option(self.$shared_single_field.get(index)),)*
                    $(FieldName::$shared_multi_no_arch_field => create_raw_item_nested_option(self.$shared_multi_no_arch_field.get(index)),)*
                    $(FieldName::$shared_multi_arch_field => self.$shared_multi_arch_field.get(index),)*
                    _ => debug_unreachable!(None, "shouldn't query {field:?}"),
                }
            }

            fn add(&mut self, field: FieldName, architecture: Option<&'a str>, value: Option<&'a str>) {
                match (field, architecture, value) {
                    $((FieldName::$shared_single_field, None, _) => self.$shared_single_field.add(value),)*
                    $((FieldName::$shared_multi_no_arch_field, None, _) => self.$shared_multi_no_arch_field.add(value),)*
                    $((FieldName::$shared_multi_arch_field, _, _) => self.$shared_multi_arch_field.add_with_architecture(architecture, value),)*
                    _ => debug_unreachable!((), "shouldn't query {field:?}"),
                }
            }

            fn discovery(&self, field: FieldName) -> CacheDiscovery {
                match field {
                    $(FieldName::$shared_single_field => self.$shared_single_field.discovery(),)*
                    $(FieldName::$shared_multi_no_arch_field => self.$shared_multi_no_arch_field.discovery(),)*
                    $(FieldName::$shared_multi_arch_field => self.$shared_multi_arch_field.discovery(),)*
                    _ => debug_unreachable!(CacheDiscovery::default(), "shouldn't query {field:?}"),
                }
            }
        }

        impl<'a> Default for DerivativeExclusiveCache<'a> {
            fn default() -> Self {
                DerivativeExclusiveCache {
                    $($shared_single_field: SingleValueCache::new(),)*
                    $($shared_multi_no_arch_field: MultiValueCache::new(),)*
                    $($shared_multi_arch_field: MultiValueCache::new(),)*
                }
            }
        }
    };
}

def_cache! {
    base single (Epoch Release Version)
    base multi (ValidPgpKeys)
    shared single (Description ChangeLog InstallScript Url)
    shared multi no_arch (Architecture Backup Groups License NoExtract Options)
    shared multi arch (
        Source Dependencies MakeDependencies CheckDependencies OptionalDependencies Provides Conflicts Replaces
        Md5Checksums Sha1Checksums Sha224Checksums Sha256Checksums Sha384Checksums Sha512Checksums
    )
}
