use crate::{
    field::{FieldName, ParsedField},
    value,
};

macro_rules! def_traits {
    (
        scalar {$(
            $(#[$scalar_attrs:meta])*
            $scalar_name:ident, $scalar_name_mut:ident = $scalar_field_name:ident -> $scalar_value_type:ident;
        )*}
        list {$(
            $(#[$list_attrs:meta])*
            $list_name:ident, $list_name_mut:ident = $list_field_name:ident -> $list_value_type:ident;
        )*}
    ) => {
        pub trait Query<'a>: QueryMut<'a> {
            fn query_raw_text(&self, field: ParsedField) -> Option<&'a str>;
            $(
                $(#[$scalar_attrs])*
                fn $scalar_name(&self) -> Option<value::$scalar_value_type<'a>> {
                    self.query_raw_text(ParsedField::new(FieldName::$scalar_field_name))
                        .map(value::$scalar_value_type::new)
                }
            )*
            $(
                $(#[$list_attrs])*
                fn $list_name(&self) -> Option<value::$list_value_type<&'a str>> {
                    self.query_raw_text(ParsedField::new(FieldName::$list_field_name))
                        .map(value::$list_value_type::new)
                }
            )*
        }

        pub trait QueryMut<'a> {
            fn query_raw_text_mut(&mut self, field: ParsedField) -> Option<&'a str>;
            $(
                $(#[$scalar_attrs])*
                fn $scalar_name_mut(&mut self) -> Option<value::$scalar_value_type<'a>> {
                    self.query_raw_text_mut(ParsedField::new(FieldName::$scalar_field_name))
                        .map(value::$scalar_value_type::new)
                }
            )*
            $(
                $(#[$list_attrs])*
                fn $list_name_mut(&mut self) -> Option<value::$list_value_type<&'a str>> {
                    self.query_raw_text_mut(ParsedField::new(FieldName::$list_field_name))
                        .map(value::$list_value_type::new)
                }
            )*
        }
    };
}

def_traits! {
    scalar {
        file_name, file_name_mut  = FileName -> FileName;
        name, name_mut  = Name -> Name;
        base, base_mut  = Base -> Base;
        version, version_mut  = Version -> Version;
        description, description_mut  = Description -> Description;
        compressed_size, compressed_size_mut  = CompressedSize -> Size;
        installed_size, installed_size_mut  = InstalledSize -> Size;
        md5_checksum, md5_checksum_mut  = Md5Checksum -> Hex128;
        sha256_checksum, sha256_checksum_mut  = Sha256Checksum -> Hex256;
        pgp_signature, pgp_signature_mut  = PgpSignature -> PgpSignature;
        url, url_mut  = Url -> Url;
        build_date, build_date_mut  = BuildDate -> Timestamp;
        packager, packager_mut  = Packager -> Packager;
    }
    list {
        groups, groups_mut = Groups -> GroupList;
        license, license_mut = License -> LicenseList;
        architecture, architecture_mut = Architecture -> ArchitectureList;
        dependencies, dependencies_mut = Dependencies -> DependencyList;
        make_dependencies, make_dependencies_mut = MakeDependencies -> DependencyList;
        check_dependencies, check_dependencies_mut = CheckDependencies -> DependencyList;
        opt_dependencies, opt_dependencies_mut = OptionalDependencies -> DependencyAndReasonList;
        provides, provides_mut = Provides -> DependencyList;
        conflicts, conflicts_mut = Conflicts -> DependencyList;
        replaces, replaces_mut = Replaces -> DependencyList;
    }
}

mod forgetful;
mod memo;

pub use forgetful::ForgetfulQuerier;
pub use memo::MemoQuerier;
