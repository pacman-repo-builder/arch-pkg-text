use crate::{srcinfo::field::FieldName, value};
use pipe_trait::Pipe;

/// Location of a given [`QueryItem`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Section<'a> {
    /// The item belongs to a section under `pkgbase`.
    Base,
    /// The item belongs to a section under `pkgname`.
    Derivative(value::Name<'a>),
}

/// Return type of methods in [`Query`] and [`QueryMut`].
#[derive(Debug, Clone, Copy)]
pub struct QueryItem<'a, Value, Architecture> {
    /// Value of the item.
    pub value: Value,
    /// Location of the item.
    pub section: Section<'a>,
    /// Architecture suffix of the corresponding field.
    pub architecture: Architecture,
}

impl<'a, Value, Architecture> QueryItem<'a, Value, Architecture> {
    /// Construct an item from a tuple of `value`, `section`, and `architecture`.
    pub fn from_tuple3((value, section, architecture): (Value, Section<'a>, Architecture)) -> Self {
        QueryItem {
            section,
            architecture,
            value,
        }
    }

    /// Dissolve the item into a tuple of `value`, `section`, and `architecture`.
    pub fn into_tuple3(self) -> (Value, Section<'a>, Architecture) {
        (self.value, self.section, self.architecture)
    }

    /// Discard [`Self::architecture`].
    fn without_architecture(self) -> QueryItem<'a, Value, ()> {
        let (value, section, _) = self.into_tuple3();
        QueryItem::from_tuple3((value, section, ()))
    }

    /// Transform `value`.
    fn map<NewValue>(
        self,
        f: impl Fn(Value) -> NewValue + 'static,
    ) -> QueryItem<'a, NewValue, Architecture> {
        let (value, section, architecture) = self.into_tuple3();
        QueryItem::from_tuple3((f(value), section, architecture))
    }
}

impl<'a, Value> QueryItem<'a, Value, ()> {
    /// Construct an item from a tuple of `value` and `section`.
    pub fn from_tuple2((value, section): (Value, Section<'a>)) -> Self {
        QueryItem::from_tuple3((value, section, ()))
    }

    /// Dissolve the item into a tuple of `value` and `section`.
    pub fn into_tuple2(self) -> (Value, Section<'a>) {
        let (value, section, ()) = self.into_tuple3();
        (value, section)
    }
}

/// Return type of [`Query::query_raw_text`] and [`QueryMut::query_raw_text_mut`].
pub type QueryRawTextItem<'a> = QueryItem<'a, &'a str, Option<value::Architecture<'a>>>;

impl<'a> QueryRawTextItem<'a> {
    /// Get a single value from the `pkgbase` section.
    fn single_base_value(query_iter: impl Iterator<Item = Self>) -> Option<&'a str> {
        Self::multi_base_values(query_iter).next()
    }

    /// Get all values from the `pkgbase` section.
    fn multi_base_values(query_iter: impl Iterator<Item = Self>) -> impl Iterator<Item = &'a str> {
        query_iter
            .take_while(|item| item.section == Section::Base)
            .filter(|item| item.architecture.is_none())
            .map(|item| item.value)
    }

    /// Get a single value from each section.
    fn shared_single_values(
        query_iter: impl Iterator<Item = Self>,
    ) -> impl Iterator<Item = QueryItem<'a, &'a str, ()>> {
        query_iter
            .filter(|item| item.architecture.is_none())
            .scan(None, move |state, item| {
                if *state == Some(item.section) {
                    None
                } else {
                    *state = Some(item.section);
                    Some(item.without_architecture())
                }
            })
    }

    /// Get all values without architecture from all sections.
    fn shared_multi_no_arch_values(
        query_iter: impl Iterator<Item = Self>,
    ) -> impl Iterator<Item = QueryItem<'a, &'a str, ()>> {
        query_iter
            .filter(|item| item.architecture.is_none())
            .map(QueryItem::without_architecture)
    }
}

macro_rules! def_traits {
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
        /// Get information from a querier of `.SRCINFO`.
        pub trait Query<'a>: QueryMut<'a> {
            fn query_raw_text(&self, field_name: FieldName) -> impl Iterator<Item = QueryRawTextItem<'a>>;

            $(fn $base_single_name(&self) -> Option<value::$base_single_type<'a>> {
                self.query_raw_text(FieldName::$base_single_field)
                    .pipe(QueryRawTextItem::single_base_value)
                    .map(value::$base_single_type::new)
            })*

            $(fn $base_multi_name(&self) -> impl Iterator<Item = value::$base_multi_type<'a>> {
                self.query_raw_text(FieldName::$base_multi_field)
                    .pipe(QueryRawTextItem::multi_base_values)
                    .map(value::$base_multi_type::new)
            })*

            fn derivative_names(&self) -> impl Iterator<Item = value::Name<'a>> {
                self.query_raw_text(FieldName::Name)
                    .filter(|item| item.architecture.is_none())
                    .map(|item| item.value)
                    .map(value::Name::new)
            }

            $(fn $shared_single_name(&self) -> impl Iterator<Item = QueryItem<'a, value::$shared_single_type<'a>, ()>> {
                self.query_raw_text(FieldName::$shared_single_field)
                    .pipe(QueryRawTextItem::shared_single_values)
                    .map(|item| item.map(value::$shared_single_type::new))
            })*

            $(fn $shared_multi_no_arch_name(&self) -> impl Iterator<Item = QueryItem<'a, value::$shared_multi_no_arch_type<'a>, ()>> {
                self.query_raw_text(FieldName::$shared_multi_no_arch_field)
                    .pipe(QueryRawTextItem::shared_multi_no_arch_values)
                    .map(|item| item.map(value::$shared_multi_no_arch_type::new))
            })*

            $(fn $shared_multi_arch_name(
                &self,
            ) -> impl Iterator<Item = QueryItem<'a, value::$shared_multi_arch_type<'a>, Option<value::Architecture<'a>>>> {
                self.query_raw_text(FieldName::$shared_multi_arch_field)
                    .map(|item| item.map(value::$shared_multi_arch_type::new))
            })*
        }

        /// Get information from a querier of `.SRCINFO`, mutability required.
        pub trait QueryMut<'a> {
            fn query_raw_text_mut(&mut self, field_name: FieldName) -> impl Iterator<Item = QueryRawTextItem<'a>>;

            $(fn $base_single_name_mut(&mut self) -> Option<value::$base_single_type<'a>> {
                self.query_raw_text_mut(FieldName::$base_single_field)
                    .pipe(QueryRawTextItem::single_base_value)
                    .map(value::$base_single_type::new)
            })*

            $(fn $base_multi_name_mut(&mut self) -> impl Iterator<Item = value::$base_multi_type<'a>> {
                self.query_raw_text_mut(FieldName::$base_multi_field)
                    .pipe(QueryRawTextItem::multi_base_values)
                    .map(value::$base_multi_type::new)
            })*

            fn derivative_names_mut(&mut self) -> impl Iterator<Item = value::Name<'a>> {
                self.query_raw_text_mut(FieldName::Name)
                    .filter(|item| item.architecture.is_none())
                    .map(|item| item.value)
                    .map(value::Name::new)
            }

            $(fn $shared_single_name_mut(&mut self) -> impl Iterator<Item = QueryItem<'a, value::$shared_single_type<'a>, ()>> {
                self.query_raw_text_mut(FieldName::$shared_single_field)
                    .pipe(QueryRawTextItem::shared_single_values)
                    .map(|item| item.map(value::$shared_single_type::new))
            })*

            $(fn $shared_multi_no_arch_name_mut(&mut self) -> impl Iterator<Item = QueryItem<'a, value::$shared_multi_no_arch_type<'a>,()>> {
                self.query_raw_text_mut(FieldName::$shared_multi_no_arch_field)
                    .pipe(QueryRawTextItem::shared_multi_no_arch_values)
                    .map(|item| item.map(value::$shared_multi_no_arch_type::new))
            })*

            $(fn $shared_multi_arch_name_mut(
                &mut self,
            ) -> impl Iterator<Item = QueryItem<'a, value::$shared_multi_arch_type<'a>, Option<value::Architecture<'a>>>> {
                self.query_raw_text_mut(FieldName::$shared_multi_arch_field)
                    .map(|item| item.map(value::$shared_multi_arch_type::new))
            })*
        }
    };
}

def_traits! {
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
        md5_checksums, md5_checksums_mut = Md5Checksums -> Hex128;
        sha1_checksums, sha1_checksums_mut = Sha1Checksums -> Hex160;
        sha224_checksums, sha224_checksums_mut = Sha224Checksums -> Hex224;
        sha256_checksums, sha256_checksums_mut = Sha256Checksums -> Hex256;
        sha512_checksums, sha512_checksums_mut = Sha512Checksums -> Hex512;
    }
}

mod utils; // TODO: remove this if it turns out not to be reused

mod forgetful;
pub use forgetful::*;
