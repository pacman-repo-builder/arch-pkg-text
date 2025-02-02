use super::{AddFailure, ParsedSrcinfo, ParsedSrcinfoDerivativeSectionEntryMut};
use crate::{
    srcinfo::{
        field::{FieldName, ParsedField},
        query::{Query, QueryItem, QueryMut, QueryRawTextItem, Section},
    },
    value,
};
use derive_more::{Display, Error};
use pipe_trait::Pipe;

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
        /// Private [iterator](Iterator) type to be used as the underlying return types in [`Query`] and [`QueryMut`].
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

        impl<'a> Query<'a> for ParsedSrcinfo<'a> {
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

        impl<'a> QueryMut<'a> for ParsedSrcinfo<'a> {
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

        /// Parsed information of a `pkgbase` section.
        #[derive(Debug, Default, Clone)]
        pub struct ParsedSrcinfoBaseSection<'a> {
            $($base_single_name: Option<value::$base_single_type<'a>>,)*
            $($base_multi_name: Vec<value::$base_multi_type<'a>>,)*
            $($shared_single_name: Option<value::$shared_single_type<'a>>,)*
            $($shared_multi_no_arch_name: Vec<value::$shared_multi_no_arch_type<'a>>,)*
            $($shared_multi_arch_name: Vec<(value::$shared_multi_arch_type<'a>, Option<value::Architecture<'a>>)>,)*
        }

        /// Error that occurs when `.SRCINFO` defines unique field twice.
        #[derive(Debug, Display, Error, Clone, Copy)]
        pub enum ParsedSrcinfoAlreadySetError<'a> {
            $(
                #[display("Field {} is already set", FieldName::$base_single_field)]
                $base_single_field(#[error(not(source))] value::$base_single_type<'a>),
            )*
            $(
                #[display("Field {} is already set", FieldName::$shared_single_field)]
                $shared_single_field(#[error(not(source))] value::$shared_single_type<'a>),
            )*
        }

        impl<'a> ParsedSrcinfoBaseSection<'a> {
            /// Add an entry to the section.
            pub(super) fn add(
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
                        return ParsedSrcinfoBaseSection::add_value_to_option(
                            &mut self.$base_single_name,
                            value,
                            value::$base_single_type::new,
                            ParsedSrcinfoAlreadySetError::$base_single_field,
                        );
                    })*
                    $((FieldName::$base_single_field, Some(_)) => return Ok(()),)* // TODO: callback fn to record warnings?
                    $((FieldName::$base_multi_field, None) => {
                        self.$base_multi_name.push(value::$base_multi_type::new(value));
                        return Ok(());
                    })*
                    $((FieldName::$base_multi_field, Some(_)) => return Ok(()),)* // TODO: callback fn to record warnings?
                    $((FieldName::$shared_single_field, None) => {
                        return ParsedSrcinfoBaseSection::add_value_to_option(
                            &mut self.$shared_single_name,
                            value,
                            value::$shared_single_type::new,
                            ParsedSrcinfoAlreadySetError::$shared_single_field,
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

            /// Shrink all internal containers' capacities to fit.
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

        /// Parsed information of a `pkgname` section.
        #[derive(Debug, Default, Clone)]
        pub struct ParsedSrcinfoDerivativeSection<'a> {
            $($shared_single_name: Option<value::$shared_single_type<'a>>,)*
            $($shared_multi_no_arch_name: Vec<value::$shared_multi_no_arch_type<'a>>,)*
            $($shared_multi_arch_name: Vec<(value::$shared_multi_arch_type<'a>, Option<value::Architecture<'a>>)>,)*
        }

        /// Error that occurs when `.SRCINFO` defines unique field twice.
        #[derive(Debug, Display, Error, Clone, Copy)]
        pub enum ParsedDerivativeAlreadySetError<'a> {$(
            #[display("Field {} is already set", FieldName::$shared_single_field)]
            $shared_single_field(#[error(not(source))] value::$shared_single_type<'a>),
        )*}

        impl<'a> ParsedSrcinfoDerivativeSection<'a> {
            /// Shrink all internal containers' capacities to fit.
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

        impl<'a, 'r> ParsedSrcinfoDerivativeSectionEntryMut<'a, 'r> {
            /// Add an entry to the section.
            pub(super) fn add(
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
                        return ParsedSrcinfoDerivativeSectionEntryMut::add_value_to_option(
                            self.name,
                            &mut self.data.$shared_single_name,
                            value,
                            value::$shared_single_type::new,
                            ParsedDerivativeAlreadySetError::$shared_single_field,
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
        }
    };
}

def_struct! {
    base single {
        base_name, base_name_mut = Base -> Base;
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
        sha384_checksums, sha384_checksums_mut = Sha384Checksums -> SkipOrHex384;
        sha512_checksums, sha512_checksums_mut = Sha512Checksums -> SkipOrHex512;
        blake2b_checksums, blake2b_checksums_mut = Blake2bChecksums -> SkipOrHex512;
    }
}
