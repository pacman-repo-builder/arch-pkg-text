use crate::{
    srcinfo::field::FieldName,
    value::{self, Base, Name},
};
use pipe_trait::Pipe;

/// Associated types for [`QuerySection`] and [`QuerySectionMut`].
pub trait QuerySectionAssoc {
    type BaseSection;
    type DerivativeExclusiveSection;
}

/// Query a section from a `.SRCINFO` file.
pub trait QuerySection<'a>: QuerySectionMut<'a>
where
    Self::BaseSection: QueryBaseField<'a>,
    Self::DerivativeExclusiveSection: QueryDerivativeField<'a>,
{
    /// Get the section under `pkgbase`.
    fn base(&self) -> Self::BaseSection;

    /// Get an exclusive section whose `pkgname` matches `name`.
    fn derivative_exclusive(&self, name: Name) -> Option<Self::DerivativeExclusiveSection>;

    /// Get all exclusive sections under `pkgname`.
    fn all_derivative_exclusives(
        &self,
    ) -> impl IntoIterator<Item = Self::DerivativeExclusiveSection>;

    /// Get a inheriting derivative section whose `pkgname` matches `name`.
    fn derivative(
        &self,
        name: Name,
    ) -> Option<JoinedSection<Self::BaseSection, Self::DerivativeExclusiveSection>> {
        let base = self.base();
        self.derivative_exclusive(name)
            .map(|derivative_exclusive| JoinedSection::new(base, derivative_exclusive))
    }
}

/// Query a section from a `.SRCINFO` file.
pub trait QuerySectionMut<'a>: QuerySectionAssoc
where
    Self::BaseSection: QueryBaseFieldMut<'a>,
    Self::DerivativeExclusiveSection: QueryDerivativeFieldMut<'a>,
{
    /// Get the section under `pkgbase`.
    fn base_mut(&mut self) -> Self::BaseSection;

    /// Get an exclusive section whose `pkgname` matches `name`.
    fn derivative_exclusive_mut(&mut self, name: Name) -> Option<Self::DerivativeExclusiveSection>;

    /// Get all exclusive sections under `pkgname`.
    fn all_derivative_exclusives_mut(
        &mut self,
    ) -> impl IntoIterator<Item = Self::DerivativeExclusiveSection>;

    /// Get a inheriting derivative section whose `pkgname` matches `name`.
    fn derivative_mut(
        &mut self,
        name: Name,
    ) -> Option<JoinedMutSection<Self::BaseSection, Self::DerivativeExclusiveSection>> {
        let base = self.base_mut();
        self.derivative_exclusive_mut(name)
            .map(|derivative_exclusive| JoinedMutSection::new(base, derivative_exclusive))
    }
}

fn query_single_no_arch<'a>(
    iter: impl IntoIterator<Item = QueryRawTextItem<'a>>,
) -> Option<&'a str> {
    let QueryRawTextItem {
        architecture,
        value,
    } = iter.into_iter().next()?;
    if architecture.is_some() {
        return None;
    }
    Some(value)
}

macro_rules! def_query_single_no_arch {
    ($(
        $(#[$attrs:meta])*
        $name:ident = $field_name:ident -> $value_type:ident;
    )*) => {$(
        $(#[$attrs])*
        fn $name(&self) -> Option<value::$value_type<'a>> {
            self.query_raw_text(FieldName::$field_name)
                .pipe(query_single_no_arch)
                .map(value::$value_type::new)
        }
    )*};
}

macro_rules! def_query_single_no_arch_mut {
    ($(
        $(#[$attrs:meta])*
        $name:ident = $field_name:ident -> $value_type:ident;
    )*) => {$(
        $(#[$attrs])*
        fn $name(&mut self) -> Option<value::$value_type<'a>> {
            self.query_raw_text_mut(FieldName::$field_name)
                .pipe(query_single_no_arch)
                .map(value::$value_type::new)
        }
    )*};
}

fn query_multi_no_arch<'a>(
    iter: impl IntoIterator<Item = QueryRawTextItem<'a>>,
) -> impl Iterator<Item = &'a str> {
    iter.into_iter()
        .filter(|item| item.architecture.is_none())
        .map(|item| item.value)
}

macro_rules! def_query_multi_no_arch {
    ($(
        $(#[$attrs:meta])*
        $name:ident = $field_name:ident -> $item_type:ident;
    )*) => {$(
        fn $name(&self) -> impl Iterator<Item = value::$item_type<'a>> {
            self.query_raw_text(FieldName::$field_name)
                .pipe(query_multi_no_arch)
                .map(value::$item_type::new)
        }
    )*};
}

macro_rules! def_query_multi_no_arch_mut {
    ($(
        $(#[$attrs:meta])*
        $name:ident = $field_name:ident -> $item_type:ident;
    )*) => {$(
        fn $name(&mut self) -> impl Iterator<Item = value::$item_type<'a>> {
            self.query_raw_text_mut(FieldName::$field_name)
                .pipe(query_multi_no_arch)
                .map(value::$item_type::new)
        }
    )*};
}

fn query_multi_arch_some<'a>(
    iter: impl IntoIterator<Item = QueryRawTextItem<'a>>,
    architecture: Option<&'a str>,
) -> impl Iterator<Item = &'a str> {
    iter.into_iter()
        .filter(move |item| item.architecture == architecture)
        .map(|item| item.value)
}

macro_rules! def_query_multi_arch {
    ($(
        $(#[$all_attrs:meta])* $name_all:ident, $(#[$some_attrs:meta])* $name_some:ident =
            $field_name:ident -> $item_type:ident;
    )*) => {$(
        $(#[$all_attrs])*
        fn $name_all(&self) -> impl Iterator<Item = QueryArchitectureItem<'a, value::$item_type<'a>>> {
            self.query_raw_text(FieldName::$field_name)
                .into_iter()
                .map(move |item| item.into_query_architecture_item(value::$item_type::new))
        }

        $(#[$some_attrs])*
        fn $name_some(&self, architecture: Option<&'a str>) -> impl Iterator<Item = value::$item_type<'a>> {
            query_multi_arch_some(self.query_raw_text(FieldName::$field_name), architecture)
                .map(value::$item_type::new)

        }
    )*};
}

macro_rules! def_query_multi_arch_mut {
    ($(
        $(#[$all_attrs:meta])* $name_all:ident, $(#[$some_attrs:meta])* $name_some:ident =
            $field_name:ident -> $item_type:ident;
    )*) => {$(
        $(#[$all_attrs])*
        fn $name_all(&mut self) -> impl Iterator<Item = QueryArchitectureItem<'a, value::$item_type<'a>>> {
            self.query_raw_text_mut(FieldName::$field_name)
                .into_iter()
                .map(|item| item.into_query_architecture_item(value::$item_type::new))
        }

        $(#[$some_attrs])*
        fn $name_some(&mut self, architecture: Option<&'a str>) -> impl Iterator<Item = value::$item_type<'a>> {
            query_multi_arch_some(self.query_raw_text_mut(FieldName::$field_name), architecture)
                .map(value::$item_type::new)
        }
    )*};
}

macro_rules! def_traits {
    (
        base single {$(
            $(#[$base_single_attrs:meta])*
            $base_single_name:ident, $base_single_name_mut:ident = $base_single_field_name:ident -> $base_single_value_type:ident;
        )*}
        base multi {$(
            $(#[$base_multi_attrs:meta])*
            $base_multi_name:ident, $base_multi_name_mut:ident = $base_multi_field_name:ident -> $base_multi_value_type:ident;
        )*}
        shared single {$(
            $(#[$shared_single_attrs:meta])*
            $shared_single_name:ident, $shared_single_name_mut:ident = $shared_single_field_name:ident -> $shared_single_value_type:ident;
        )*}
        shared multi no_arch {$(
            $(#[$shared_multi_no_arch_attrs:meta])*
            $shared_multi_no_arch_name:ident, $shared_multi_no_arch_name_mut:ident =
                $shared_multi_no_arch_field_name:ident -> $shared_multi_no_arch_value_type:ident;
        )*}
        shared multi arch {$(
            $(#[$shared_multi_arch_all_attrs:meta])*
            $shared_multi_arch_name_all:ident, $shared_multi_arch_name_some:ident,
            $(#[$shared_multi_arch_some_attrs:meta])*
            $shared_multi_arch_name_all_mut:ident, $shared_multi_arch_name_some_mut:ident =
                $shared_multi_arch_field_name:ident -> $shared_multi_arch_value_type:ident;
        )*}
    ) => {
        /// Query a field of the `pkgbase` section of a `.SRCINFO` file.
        pub trait QueryBaseField<'a>: QueryField<'a> + QueryBaseFieldMut<'a> {
            /// Get the value of the `pkgbase` field of the `.SRCINFO` file.
            fn name(&self) -> Base<'a>;

            def_query_single_no_arch! {$(
                $(#[$base_single_attrs])*
                $base_single_name = $base_single_field_name -> $base_single_value_type;
            )*}

            def_query_multi_no_arch! {$(
                $(#[$base_multi_attrs])*
                $base_multi_name = $base_multi_field_name -> $base_multi_value_type;
            )*}
        }

        /// Query a field of a `pkgname` section of a `.SRCINFO` file.
        pub trait QueryDerivativeField<'a>: QueryField<'a> + QueryDerivativeFieldMut<'a>
        {
            /// Get the value of the `pkgname` field of the section.
            fn name(&self) -> Name<'a>;
        }

        /// Query a field of the `pkgbase` section of a `.SRCINFO` file.
        pub trait QueryBaseFieldMut<'a>: QueryFieldMut<'a> {
            /// Get the value of the `pkgbase` field of the `.SRCINFO` file.
            fn name_mut(&mut self) -> Base<'a>;

            def_query_single_no_arch_mut! {$(
                $(#[$base_single_attrs])*
                $base_single_name_mut = $base_single_field_name -> $base_single_value_type;
            )*}

            def_query_multi_no_arch_mut! {$(
                $(#[$base_multi_attrs])*
                $base_multi_name_mut = $base_multi_field_name -> $base_multi_value_type;
            )*}
        }

        /// Query a field of a `pkgname` section of a `.SRCINFO` file.
        pub trait QueryDerivativeFieldMut<'a>: QueryFieldMut<'a> {
            /// Get the value of the `pkgname` field of the section.
            fn name_mut(&mut self) -> Name<'a>;
        }

        /// Query a field of either a `pkgbase` or `pkgname` section of a `.SRCINFO` file.
        pub trait QueryField<'a>: QueryFieldMut<'a> {
            fn query_raw_text(&self, field_name: FieldName) -> impl IntoIterator<Item = QueryRawTextItem<'a>>;

            def_query_single_no_arch! {$(
                $(#[$shared_single_attrs])*
                $shared_single_name = $shared_single_field_name -> $shared_single_value_type;
            )*}

            def_query_multi_no_arch! {$(
                $(#[$shared_multi_no_arch_attrs])*
                $shared_multi_no_arch_name = $shared_multi_no_arch_field_name -> $shared_multi_no_arch_value_type;
            )*}

            def_query_multi_arch! {$(
                $(#[$shared_multi_arch_all_attrs])* $shared_multi_arch_name_all,
                $(#[$shared_multi_arch_some_attrs])* $shared_multi_arch_name_some =
                    $shared_multi_arch_field_name -> $shared_multi_arch_value_type;
            )*}
        }

        /// Query a field of either a `pkgbase` or `pkgname` section of a `.SRCINFO` file.
        pub trait QueryFieldMut<'a> {
            fn query_raw_text_mut(&mut self, field_name: FieldName) -> impl IntoIterator<Item = QueryRawTextItem<'a>>;

            def_query_single_no_arch_mut! {$(
                $(#[$shared_single_attrs])*
                $shared_single_name_mut = $shared_single_field_name -> $shared_single_value_type;
            )*}

            def_query_multi_no_arch_mut! {$(
                $(#[$shared_multi_no_arch_attrs])*
                $shared_multi_no_arch_name_mut = $shared_multi_no_arch_field_name -> $shared_multi_no_arch_value_type;
            )*}

            def_query_multi_arch_mut! {$(
                $(#[$shared_multi_arch_all_attrs])* $shared_multi_arch_name_all_mut,
                $(#[$shared_multi_arch_some_attrs])* $shared_multi_arch_name_some_mut =
                    $shared_multi_arch_field_name -> $shared_multi_arch_value_type;
            )*}
        }
    };
}

def_traits! {
    base single {
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
        source, source_architecture, source_mut, source_architecture_mut = Source -> Source;

        /* DEPENDENCIES */
        dependencies, dependencies_architecture, dependencies_mut, dependencies_architecture_mut = Dependencies -> Dependency;
        make_dependencies, make_dependencies_architecture, make_dependencies_mut, make_dependencies_architecture_mut = MakeDependencies -> Dependency;
        check_dependencies, check_dependencies_architecture, check_dependencies_mut, check_dependencies_architecture_mut = CheckDependencies -> Dependency;
        opt_dependencies, opt_dependencies_architecture, opt_dependencies_mut, opt_dependencies_architecture_mut = OptionalDependencies -> DependencyAndReason;
        provides, provides_architecture, provides_mut, provides_architecture_mut = Provides -> Dependency;
        conflicts, conflicts_architecture, conflicts_mut, conflicts_architecture_mut = Conflicts -> Dependency;
        replaces, replaces_architecture, replaces_mut, replaces_architecture_mut = Replaces -> Dependency;

        /* CHECKSUMS */
        md5_checksums, md5_checksums_architecture, md5_checksums_mut, md5_checksums_architecture_mut = Md5Checksums -> Hex128;
        sha1_checksums, sha1_checksums_architecture, sha1_checksums_mut, sha1_checksums_architecture_mut = Sha1Checksums -> Hex160;
        sha224_checksums, sha224_checksums_architecture, sha224_checksums_mut, sha224_checksums_architecture_mut = Sha224Checksums -> Hex224;
        sha256_checksums, sha256_checksums_architecture, sha256_checksums_mut, sha256_checksums_architecture_mut = Sha256Checksums -> Hex256;
        sha512_checksums, sha512_checksums_architecture, sha512_checksums_mut, sha512_checksums_architecture_mut = Sha512Checksums -> Hex512;
    }
}

/// [Iterator item](Iterator::Item) of [`query_raw_text`](QueryField::query_raw_text)
/// and [`query_raw_text_mut`](QueryFieldMut::query_raw_text_mut).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryRawTextItem<'a> {
    /// Architecture of the field.
    pub architecture: Option<&'a str>,
    /// Value of the field.
    pub value: &'a str,
}

impl<'a> QueryRawTextItem<'a> {
    fn into_query_architecture_item<Value, MakeValue>(
        self,
        make_value: MakeValue,
    ) -> QueryArchitectureItem<'a, Value>
    where
        MakeValue: FnOnce(&'a str) -> Value,
    {
        let QueryRawTextItem {
            architecture,
            value,
        } = self;
        let value = make_value(value);
        QueryArchitectureItem {
            architecture,
            value,
        }
    }

    fn from_tuple((architecture, value): (Option<&'a str>, &'a str)) -> Self {
        QueryRawTextItem {
            architecture,
            value,
        }
    }
}

/// [Iterator item](Iterator::Item) of query functions that return architectures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryArchitectureItem<'a, Value> {
    /// Architecture of the field.
    pub architecture: Option<&'a str>,
    /// Value of the field.
    pub value: Value,
}

mod derivative;
pub use derivative::{JoinedMutSection, JoinedSection};

mod forgetful;
pub use forgetful::{
    ForgetfulBaseSection, ForgetfulDerivativeExclusiveSection, ForgetfulSectionQuerier,
};

#[cfg(feature = "std")]
mod memo;

mod utils;
