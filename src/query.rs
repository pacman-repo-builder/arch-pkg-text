use crate::{
    field::{FieldName, ParsedField},
    value,
};

macro_rules! def_traits {
    ($(
        $(#[$attrs:meta])*
        $name:ident, $name_mut:ident = $field_name:ident -> $value_type:ident;
    )*) => {
        pub trait Query<'a>: QueryMut<'a> {
            fn query_raw_text(&self, field: ParsedField) -> Option<&'a str>;
            $(
                $(#[$attrs])*
                fn $name(&self) -> Option<value::$value_type<&'a str>> {
                    self.query_raw_text(ParsedField::new(FieldName::$field_name))
                        .map(value::$value_type::new)
                }
            )*
        }

        pub trait QueryMut<'a> {
            fn query_raw_text_mut(&mut self, field: ParsedField) -> Option<&'a str>;
            $(
                $(#[$attrs])*
                fn $name_mut(&mut self) -> Option<value::$value_type<&'a str>> {
                    self.query_raw_text_mut(ParsedField::new(FieldName::$field_name))
                        .map(value::$value_type::new)
                }
            )*
        }
    };
}

def_traits! {
    file_name, file_name_mut  = FileName -> FileName;
    name, name_mut  = Name -> Name;
    base, base_mut  = Base -> Base;
    version, version_mut  = Version -> Version;
    description, description_mut  = Description -> Description;
    groups, groups_mut = Groups -> GroupList;
    compressed_size, compressed_size_mut  = CompressedSize -> Size;
    installed_size, installed_size_mut  = InstalledSize -> Size;
    md5_checksum, md5_checksum_mut  = Md5Checksum -> Hex128;
    sha256_checksum, sha256_checksum_mut  = Sha256Checksum -> Hex256;
    pgp_signature, pgp_signature_mut  = PgpSignature -> PgpSignature;
    url, url_mut  = Url -> Url;
    license, license_mut = License -> LicenseList;
    architecture, architecture_mut = Architecture -> ArchitectureList;
    build_date, build_date_mut  = BuildDate -> Timestamp;
    packager, packager_mut  = Packager -> Packager;
    dependencies, dependencies_mut = Dependencies -> DependencyList;
    make_dependencies, make_dependencies_mut = MakeDependencies -> DependencyList;
    check_dependencies, check_dependencies_mut = CheckDependencies -> DependencyList;
    opt_dependencies, opt_dependencies_mut = OptionalDependencies -> DependencyAndReasonList;
    provides, provides_mut = Provides -> DependencyList;
    conflicts, conflicts_mut = Conflicts -> DependencyList;
    replaces, replaces_mut = Replaces -> DependencyList;
}

mod forgetful;
mod memo;

pub use forgetful::ForgetfulQuerier;
pub use memo::MemoQuerier;
