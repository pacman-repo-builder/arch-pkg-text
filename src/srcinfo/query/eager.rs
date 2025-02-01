use super::{
    utils::{non_blank_trimmed_lines, parse_line},
    Query, QueryItem, QueryMut, QueryRawTextItem, Section,
};
use crate::{
    srcinfo::field::{FieldName, ParsedField},
    value,
};
use derive_more::{Display, Error};
use pipe_trait::Pipe;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct EagerQuerier<'a> {
    pub base: EagerBaseSection<'a>,
    pub derivatives: HashMap<value::Name<'a>, EagerDerivativeSection<'a>>,
}

enum EagerSectionMut<'a, 'r> {
    Base(&'r mut EagerBaseSection<'a>),
    Derivative(EagerDerivativeSectionEntry<'a, 'r>),
}

impl<'a> EagerQuerier<'a> {
    fn get_or_insert(&mut self, section: Section<'a>) -> EagerSectionMut<'a, '_> {
        match section {
            Section::Base => self.base.pipe_mut(EagerSectionMut::Base),
            Section::Derivative(name) => self
                .derivatives
                .entry(name)
                .or_default()
                .pipe(|data| EagerDerivativeSectionEntry::new(name, data))
                .pipe(EagerSectionMut::Derivative),
        }
    }
}

/// Private error type for control flow.
enum AddFailure<'a> {
    MeetHeader(value::Name<'a>),
    Error(EagerQuerierParseError<'a>),
}

impl<'a, 'r> EagerSectionMut<'a, 'r> {
    fn add(&mut self, field: ParsedField<&'a str>, value: &'a str) -> Result<(), AddFailure<'a>> {
        match self {
            EagerSectionMut::Base(section) => section.add(field, value),
            EagerSectionMut::Derivative(section) => section.add(field, value),
        }
    }

    fn shrink_to_fit(&mut self) {
        match self {
            EagerSectionMut::Base(section) => section.shrink_to_fit(),
            EagerSectionMut::Derivative(section) => section.shrink_to_fit(),
        }
    }
}

macro_rules! def_struct {
    (
        base single {$(
            $base_single_name:ident, $base_single_name_mut:ident = $base_single_field:ident -> $base_single_type:ident;
        )*}
        base multi {$(
            $base_multi_name:ident, $base_multi_name_mut:ident = $base_multi_field:ident -> $base_multi_type:ident;
        )*}
        shared single {$(
            $shared_single_name:ident, $shared_single_name_mut:ident = $shared_single_field:ident -> $shared_single_type:ident;
        )*}
        shared multi no_arch {$(
            $shared_multi_no_arch_name:ident, $shared_multi_no_arch_name_mut:ident = $shared_multi_no_arch_field:ident -> $shared_multi_no_arch_type:ident;
        )*}
        shared multi arch {$(
            $shared_multi_arch_name:ident, $shared_multi_arch_name_mut:ident = $shared_multi_arch_field:ident -> $shared_multi_arch_type:ident;
        )*}
    ) => {
        enum QueryIter<
            Name,
            $($base_single_field,)*
            $($base_multi_field,)*
            $($shared_single_field,)*
            $($shared_multi_no_arch_field,)*
            $($shared_multi_arch_field,)*
        > {
            Name(Name),
            $($base_single_field($base_single_field),)*
            $($base_multi_field($base_multi_field),)*
            $($shared_single_field($shared_single_field),)*
            $($shared_multi_no_arch_field($shared_multi_no_arch_field),)*
            $($shared_multi_arch_field($shared_multi_arch_field),)*
        }

        impl<
            'a,
            Name,
            $($base_single_field,)*
            $($base_multi_field,)*
            $($shared_single_field,)*
            $($shared_multi_no_arch_field,)*
            $($shared_multi_arch_field,)*
        > Iterator for QueryIter<
            Name,
            $($base_single_field,)*
            $($base_multi_field,)*
            $($shared_single_field,)*
            $($shared_multi_no_arch_field,)*
            $($shared_multi_arch_field,)*
        > where
            Name: Iterator<Item = QueryRawTextItem<'a>>,
            $($base_single_field: Iterator<Item = QueryRawTextItem<'a>>,)*
            $($base_multi_field: Iterator<Item = QueryRawTextItem<'a>>,)*
            $($shared_single_field: Iterator<Item = QueryRawTextItem<'a>>,)*
            $($shared_multi_no_arch_field: Iterator<Item = QueryRawTextItem<'a>>,)*
            $($shared_multi_arch_field: Iterator<Item = QueryRawTextItem<'a>>,)*
        {
            type Item = QueryRawTextItem<'a>;
            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    QueryIter::Name(iter) => iter.next(),
                    $(QueryIter::$base_single_field(iter) => iter.next(),)*
                    $(QueryIter::$base_multi_field(iter) => iter.next(),)*
                    $(QueryIter::$shared_single_field(iter) => iter.next(),)*
                    $(QueryIter::$shared_multi_no_arch_field(iter) => iter.next(),)*
                    $(QueryIter::$shared_multi_arch_field(iter) => iter.next(),)*
                }
            }
        }

        impl<'a> Query<'a> for EagerQuerier<'a> {
            fn query_raw_text(&self, field_name: FieldName) -> impl Iterator<Item = QueryRawTextItem<'a>> {
                match field_name {
                    FieldName::Name => {
                        self.derivatives
                            .keys()
                            .map(|name| (name.as_str(), Section::Derivative(*name), None))
                            .map(QueryRawTextItem::from_tuple3)
                            .pipe(QueryIter::Name)
                    }
                    $(FieldName::$base_single_field => {
                        self.$base_single_name()
                            .into_iter()
                            .map(|value| (value.as_str(), Section::Base, None))
                            .map(QueryRawTextItem::from_tuple3)
                            .pipe(QueryIter::$base_single_field)
                    })*
                    $(FieldName::$base_multi_field => {
                        self.$base_multi_name()
                            .map(|value| (value.as_str(), Section::Base, None))
                            .map(QueryRawTextItem::from_tuple3)
                            .pipe(QueryIter::$base_multi_field)
                    })*
                    $(FieldName::$shared_single_field => {
                        self.$shared_single_name()
                            .map(QueryItem::into_tuple2)
                            .map(|(value, section)| (value.as_str(), section, None))
                            .map(QueryRawTextItem::from_tuple3)
                            .pipe(QueryIter::$shared_single_field)
                    })*
                    $(FieldName::$shared_multi_no_arch_field => {
                        self.$shared_multi_no_arch_name()
                            .map(QueryItem::into_tuple2)
                            .map(|(value, section)| (value.as_str(), section, None))
                            .map(QueryRawTextItem::from_tuple3)
                            .pipe(QueryIter::$shared_multi_no_arch_field)
                    })*
                    $(FieldName::$shared_multi_arch_field => {
                        self.$shared_multi_arch_name()
                            .map(QueryItem::into_tuple3)
                            .map(|(value, section, architecture)| (value.as_str(), section, architecture))
                            .map(QueryRawTextItem::from_tuple3)
                            .pipe(QueryIter::$shared_multi_arch_field)
                    })*
                }
            }
            $(fn $base_single_name(&self) -> Option<value::$base_single_type<'a>> {
                self.base.$base_single_name
            })*
            $(fn $base_multi_name(&self) -> impl Iterator<Item = value::$base_multi_type<'a>> {
                self.base.$base_multi_name.iter().copied()
            })*
            $(fn $shared_single_name(&self) -> impl Iterator<Item = QueryItem<'a, value::$shared_single_type<'a>, ()>> {
                let tail = self
                    .derivatives
                    .iter()
                    .map(|(name, derivative)| (name, derivative.$shared_single_name))
                    .flat_map(|(name, value)| value.map(|value| (*name, value)))
                    .map(|(name, value)| (value, Section::Derivative(name)))
                    .map(QueryItem::from_tuple2);
                self.base
                    .$shared_single_name
                    .into_iter()
                    .map(|value| (value, Section::Base))
                    .map(QueryItem::from_tuple2)
                    .chain(tail)
            })*
            $(fn $shared_multi_no_arch_name(&self) -> impl Iterator<Item = QueryItem<'a, value::$shared_multi_no_arch_type<'a>, ()>> {
                let tail = self
                    .derivatives
                    .iter()
                    .map(|(name, derivative)| (name, &derivative.$shared_multi_no_arch_name))
                    .flat_map(|(name, values)| values.iter().map(|value| (*name, *value)))
                    .map(|(name, value)| (value, Section::Derivative(name)))
                    .map(QueryItem::from_tuple2);
                self.base
                    .$shared_multi_no_arch_name
                    .iter()
                    .copied()
                    .map(|value| (value, Section::Base))
                    .map(QueryItem::from_tuple2)
                    .chain(tail)
            })*
            $(fn $shared_multi_arch_name(&self) -> impl Iterator<
                Item = QueryItem<'a, value::$shared_multi_arch_type<'a>, Option<value::Architecture<'a>>>,
            > {
                let tail = self
                    .derivatives
                    .iter()
                    .map(|(name, derivative)| (name, &derivative.$shared_multi_arch_name))
                    .flat_map(|(name, values)| {
                        values.iter().map(|(value, architecture)| (*name, *value, *architecture))
                    })
                    .map(|(name, value, architecture)| (value, Section::Derivative(name), architecture))
                    .map(QueryItem::from_tuple3);
                self.base
                    .$shared_multi_arch_name
                    .iter()
                    .copied()
                    .map(|(value, architecture)| (value, Section::Base, architecture))
                    .map(QueryItem::from_tuple3)
                    .chain(tail)
            })*
        }

        impl<'a> QueryMut<'a> for EagerQuerier<'a> {
            fn query_raw_text_mut(&mut self, field_name: FieldName) -> impl Iterator<Item = QueryRawTextItem<'a>> {
                self.query_raw_text(field_name)
            }
            $(fn $base_single_name_mut(&mut self) -> Option<value::$base_single_type<'a>> { self.$base_single_name() })*
            $(fn $base_multi_name_mut(&mut self) -> impl Iterator<Item = value::$base_multi_type<'a>> { self.$base_multi_name() })*
            $(fn $shared_single_name_mut(&mut self) -> impl Iterator<Item = QueryItem<'a, value::$shared_single_type<'a>, ()>> { self.$shared_single_name() })*
            $(fn $shared_multi_no_arch_name_mut(&mut self) -> impl Iterator<Item = QueryItem<'a, value::$shared_multi_no_arch_type<'a>, ()>> {
                self.$shared_multi_no_arch_name()
            })*
            $(fn $shared_multi_arch_name_mut(&mut self) -> impl Iterator<
                Item = QueryItem<'a, value::$shared_multi_arch_type<'a>, Option<value::Architecture<'a>>>,
            > { self.$shared_multi_arch_name() })*
        }

        #[derive(Debug, Default, Clone)]
        pub struct EagerBaseSection<'a> {
            $($base_single_name: Option<value::$base_single_type<'a>>,)*
            $($base_multi_name: Vec<value::$base_multi_type<'a>>,)*
            $($shared_single_name: Option<value::$shared_single_type<'a>>,)*
            $($shared_multi_no_arch_name: Vec<value::$shared_multi_no_arch_type<'a>>,)*
            $($shared_multi_arch_name: Vec<(value::$shared_multi_arch_type<'a>, Option<value::Architecture<'a>>)>,)*
        }

        #[derive(Debug, Display, Error, Clone, Copy)]
        pub enum EagerBaseAlreadySetError<'a> {
            $(
                #[display("Field {} is already set", FieldName::$base_single_field)]
                $base_single_field(#[error(not(source))] value::$base_single_type<'a>),
            )*
            $(
                #[display("Field {} is already set", FieldName::$shared_single_field)]
                $shared_single_field(#[error(not(source))] value::$shared_single_type<'a>),
            )*
        }

        impl<'a> EagerBaseSection<'a> {
            fn add(
                &mut self,
                field: ParsedField<&'a str>,
                value: &'a str,
            ) -> Result<(), AddFailure<'a>> {
                match (field.name(), field.architecture()) {
                    (FieldName::Name, None) => {
                        return value
                            .pipe(value::Name)
                            .pipe(AddFailure::MeetHeader)
                            .pipe(Err);
                    }
                    (FieldName::Name, Some(_)) => return Ok(()), // TODO: callback fn to record warnings?
                    $((FieldName::$base_single_field, None) => {
                        return EagerBaseSection::add_value_to_option(
                            &mut self.$base_single_name,
                            value,
                            value::$base_single_type::new,
                            EagerBaseAlreadySetError::$base_single_field,
                        );
                    })*
                    $((FieldName::$base_single_field, Some(_)) => return Ok(()),)* // TODO: callback fn to record warnings?
                    $((FieldName::$base_multi_field, None) => {
                        self.$base_multi_name.push(value::$base_multi_type::new(value));
                        return Ok(());
                    })*
                    $((FieldName::$base_multi_field, Some(_)) => return Ok(()),)* // TODO: callback fn to record warnings?
                    $((FieldName::$shared_single_field, None) => {
                        return EagerBaseSection::add_value_to_option(
                            &mut self.$shared_single_name,
                            value,
                            value::$shared_single_type::new,
                            EagerBaseAlreadySetError::$shared_single_field,
                        );
                    })*
                    $((FieldName::$shared_single_field, Some(_)) => return Ok(()),)* // TODO: callback fn to record warnings?
                    $((FieldName::$shared_multi_no_arch_field, None) => {
                        self.$shared_multi_no_arch_name.push(value::$shared_multi_no_arch_type::new(value));
                        return Ok(());
                    })*
                    $((FieldName::$shared_multi_no_arch_field, Some(_)) => return Ok(()),)* // TODO: callback fn to record warnings?
                    $((FieldName::$shared_multi_arch_field, architecture) => {
                        self.$shared_multi_arch_name.push((
                            value::$shared_multi_arch_type::new(value),
                            architecture.copied().map(value::Architecture),
                        ));
                        return Ok(());
                    })*
                }
            }

            fn add_value_to_option<Value: Copy>(
                target: &mut Option<Value>,
                value: &'a str,
                make_value: impl FnOnce(&'a str) -> Value,
                make_error: impl FnOnce(Value) -> EagerBaseAlreadySetError<'a>,
            ) -> Result<(), AddFailure<'a>> {
                let Some(old_value) = target else {
                    *target = Some(make_value(value));
                    return Ok(());
                };
                (*old_value)
                    .pipe(make_error)
                    .pipe(EagerQuerierParseError::BaseFieldAlreadySet)
                    .pipe(AddFailure::Error)
                    .pipe(Err)
            }

            pub fn shrink_to_fit(&mut self) {
                $(self.$base_multi_name.shrink_to_fit();)*
                $(self.$shared_multi_no_arch_name.shrink_to_fit();)*
                $(self.$shared_multi_arch_name.shrink_to_fit();)*
            }

            $(pub fn $base_single_name(&self) -> Option<value::$base_single_type<'a>> { self.$base_single_name })*
            $(pub fn $base_multi_name(&self) -> &'_ [value::$base_multi_type<'a>] { &self.$base_multi_name })*
            $(pub fn $shared_single_name(&self) -> Option<value::$shared_single_type<'a>> { self.$shared_single_name })*
            $(pub fn $shared_multi_no_arch_name(&self) -> &'_ [value::$shared_multi_no_arch_type<'a>] { &self.$shared_multi_no_arch_name })*
            $(pub fn $shared_multi_arch_name(&self) -> &'_ [(
                value::$shared_multi_arch_type<'a>,
                Option<value::Architecture<'a>>,
            )] { &self.$shared_multi_arch_name })*
        }

        #[derive(Debug, Default, Clone)]
        pub struct EagerDerivativeSection<'a> {
            $($shared_single_name: Option<value::$shared_single_type<'a>>,)*
            $($shared_multi_no_arch_name: Vec<value::$shared_multi_no_arch_type<'a>>,)*
            $($shared_multi_arch_name: Vec<(value::$shared_multi_arch_type<'a>, Option<value::Architecture<'a>>)>,)*
        }

        struct EagerDerivativeSectionEntry<'a, 'r> {
            name: value::Name<'a>,
            data: &'r mut EagerDerivativeSection<'a>,
        }

        #[derive(Debug, Display, Error, Clone, Copy)]
        pub enum EagerDerivativeAlreadySetError<'a> {$(
            #[display("Field {} is already set", FieldName::$shared_single_field)]
            $shared_single_field(#[error(not(source))] value::$shared_single_type<'a>),
        )*}

        impl<'a> EagerDerivativeSection<'a> {
            pub fn shrink_to_fit(&mut self) {
                $(self.$shared_multi_no_arch_name.shrink_to_fit();)*
                $(self.$shared_multi_arch_name.shrink_to_fit();)*
            }

            $(pub fn $shared_single_name(&self) -> Option<value::$shared_single_type<'a>> { self.$shared_single_name })*
            $(pub fn $shared_multi_no_arch_name(&self) -> &'_ [value::$shared_multi_no_arch_type<'a>] { &self.$shared_multi_no_arch_name })*
            $(pub fn $shared_multi_arch_name(&self) -> &'_ [(
                value::$shared_multi_arch_type<'a>,
                Option<value::Architecture<'a>>,
            )] { &self.$shared_multi_arch_name })*
        }

        impl<'a, 'r> EagerDerivativeSectionEntry<'a, 'r> {
            fn new(name: value::Name<'a>, data: &'r mut EagerDerivativeSection<'a>) -> Self {
                EagerDerivativeSectionEntry { name, data }
            }

            fn add(
                &mut self,
                field: ParsedField<&'a str>,
                value: &'a str,
            ) -> Result<(), AddFailure<'a>> {
                match (field.name(), field.architecture()) {
                    (FieldName::Name, None) => {
                        return value
                            .pipe(value::Name)
                            .pipe(AddFailure::MeetHeader)
                            .pipe(Err);
                    }
                    (FieldName::Name, Some(_)) => return Ok(()), // TODO: callback fn to record warnings?
                    $((FieldName::$base_single_field, _) => return Ok(()),)* // TODO: callback fn to record warnings?
                    $((FieldName::$base_multi_field, _) => return Ok(()),)* // TODO: callback fn to record warnings?
                    $((FieldName::$shared_single_field, None) => {
                        return EagerDerivativeSectionEntry::add_value_to_option(
                            self.name,
                            &mut self.data.$shared_single_name,
                            value,
                            value::$shared_single_type::new,
                            EagerDerivativeAlreadySetError::$shared_single_field,
                        );
                    })*
                    $((FieldName::$shared_single_field, Some(_)) => return Ok(()),)* // TODO: callback fn to record warnings?
                    $((FieldName::$shared_multi_no_arch_field, None) => {
                        self.data.$shared_multi_no_arch_name.push(value::$shared_multi_no_arch_type::new(value));
                        return Ok(());
                    })*
                    $((FieldName::$shared_multi_no_arch_field, Some(_)) => return Ok(()),)* // TODO: callback fn to record warnings?
                    $((FieldName::$shared_multi_arch_field, architecture) => {
                        self.data.$shared_multi_arch_name.push((
                            value::$shared_multi_arch_type::new(value),
                            architecture.copied().map(value::Architecture),
                        ));
                        return Ok(());
                    })*
                }
            }

            fn add_value_to_option<Value: Copy>(
                name: value::Name<'a>,
                target: &mut Option<Value>,
                value: &'a str,
                make_value: impl FnOnce(&'a str) -> Value,
                make_error: impl FnOnce(Value) -> EagerDerivativeAlreadySetError<'a>,
            ) -> Result<(), AddFailure<'a>> {
                let Some(old_value) = target else {
                    *target = Some(make_value(value));
                    return Ok(());
                };
                (*old_value)
                    .pipe(make_error)
                    .pipe(move |error| EagerQuerierParseError::DerivativeFieldAlreadySet(name, error))
                    .pipe(AddFailure::Error)
                    .pipe(Err)
            }

            fn shrink_to_fit(&mut self) {
                self.data.shrink_to_fit()
            }
        }
    };
}

def_struct! {
    base single {
        base_name, base_name_mut = Base -> Base; // TODO: make this always exist
        epoch, epoch_mut = Epoch -> Epoch;
        release, release_mut = Release -> Release;
        version, version_mut = Version -> UpstreamVersion;
    }
    base multi {
        valid_pgp_keys, valid_pgp_keys_mut = ValidPgpKeys -> PgpKey;
    }
    shared single {
        description, description_mut = Description -> Description;
        change_log, change_log_mut = ChangeLog -> ChangeLog;
        install_script, install_script_mut = InstallScript -> FileName;
        url, url_mut = Url -> Url;
    }
    shared multi no_arch {
        architecture, architecture_mut = Architecture -> Architecture;
        backup, backup_mut = Backup -> FilePath;
        groups, groups_mut = Groups -> Group;
        license, license_mut = License -> License;
        no_extract, no_extract_mut = NoExtract -> FileName;
        options, options_mut = Options -> BuildOption;
    }
    shared multi arch {
        /* MISC */
        source, source_mut = Source -> Source;

        /* DEPENDENCIES */
        dependencies, dependencies_mut = Dependencies -> Dependency;
        make_dependencies, make_dependencies_mut = MakeDependencies -> Dependency;
        check_dependencies, check_dependencies_mut = CheckDependencies -> Dependency;
        opt_dependencies, opt_dependencies_mut = OptionalDependencies -> DependencyAndReason;
        provides, provides_mut = Provides -> Dependency;
        conflicts, conflicts_mut = Conflicts -> Dependency;
        replaces, replaces_mut = Replaces -> Dependency;

        /* CHECKSUMS */
        md5_checksums, md5_checksums_mut = Md5Checksums -> SkipOrHex128;
        sha1_checksums, sha1_checksums_mut = Sha1Checksums -> SkipOrHex160;
        sha224_checksums, sha224_checksums_mut = Sha224Checksums -> SkipOrHex224;
        sha256_checksums, sha256_checksums_mut = Sha256Checksums -> SkipOrHex256;
        sha284_checksums, sha384_checksums_mut = Sha384Checksums -> SkipOrHex384;
        sha512_checksums, sha512_checksums_mut = Sha512Checksums -> SkipOrHex512;
        blake2b_checksums, blake2b_checksums_mut = Blake2bChecksums -> SkipOrHex512;
    }
}

#[derive(Debug, Display, Error, Clone, Copy)]
pub enum EagerQuerierParseError<'a> {
    #[display("Failed to insert value to the pkgbase section: {_0}")]
    BaseFieldAlreadySet(#[error(not(source))] EagerBaseAlreadySetError<'a>),
    #[display("Failed to insert value to the pkgname section named {_0}: {_1}")]
    DerivativeFieldAlreadySet(value::Name<'a>, EagerDerivativeAlreadySetError<'a>),
    #[display("Invalid line: {_0:?}")]
    InvalidLine(#[error(not(source))] &'a str),
}

/// Return type of [`EagerQuerier::parse`].
#[derive(Debug, Clone)]
pub struct EagerQuerierParseReturn<'a> {
    /// The result of the parsing process, either partial or complete.
    querier: EagerQuerier<'a>,
    /// Possible error encountered during parsing.
    error: Option<EagerQuerierParseError<'a>>,
}

impl<'a> EagerQuerierParseReturn<'a> {
    /// Return an `Ok` of [`EagerQuerier`] if there was no error.
    ///
    /// Otherwise, return an `Err` of [`EagerQuerierParseError`].
    pub fn into_complete(self) -> Result<EagerQuerier<'a>, EagerQuerierParseError<'a>> {
        match self.error {
            Some(error) => Err(error),
            None => Ok(self.querier),
        }
    }

    /// Return both the parsed querier and the error regardless of whether there was an error.
    pub fn into_partial(self) -> (EagerQuerier<'a>, Option<EagerQuerierParseError<'a>>) {
        (self.querier, self.error)
    }

    /// Return a reference to the stored error if there was one.
    pub fn error(&self) -> Option<&'_ EagerQuerierParseError<'a>> {
        self.error.as_ref()
    }
}

impl<'a> EagerQuerier<'a> {
    pub fn new(text: &'a str) -> Self {
        text.pipe(EagerQuerier::parse).into_partial().0
    }

    pub fn parse(text: &'a str) -> EagerQuerierParseReturn<'a> {
        let mut querier = EagerQuerier::default();
        let lines = non_blank_trimmed_lines(text);
        let mut section_mut = querier.get_or_insert(Section::Base);

        for line in lines {
            let Some((field, value)) = parse_line(line) else {
                return EagerQuerierParseReturn {
                    querier,
                    error: line.pipe(EagerQuerierParseError::InvalidLine).pipe(Some),
                };
            };
            let Ok(field) = field.to_parsed::<FieldName, &str>() else {
                continue;
            };
            if value.is_empty() {
                continue;
            }
            match section_mut.add(field, value) {
                Ok(()) => {}
                Err(AddFailure::MeetHeader(name)) => {
                    section_mut.shrink_to_fit();
                    section_mut = querier.get_or_insert(Section::Derivative(name));
                }
                Err(AddFailure::Error(error)) => {
                    return EagerQuerierParseReturn {
                        querier,
                        error: Some(error),
                    };
                }
            }
        }

        EagerQuerierParseReturn {
            querier,
            error: None,
        }
    }
}
