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
        md5sum, md5sum_mut  = Md5Sum -> Md5Sum;
        sha256sum, sha256sum_mut  = Sha256Sum -> Sha256Sum;
        pgp_signature, pgp_signature_mut  = PgpSignature -> PgpSignature;
        url, url_mut  = Url -> Url;
        build_date, build_date_mut  = BuildDate -> Timestamp;
        packager, packager_mut  = Packager -> Packager;
    }
    list {
        groups, groups_mut = Groups -> GroupList;
        license, license_mut = License -> LicenseList;
        arch, arch_mut = Arch -> ArchList;
        depends, depends_mut = Depends -> DependList;
        make_depends, make_depends_mut = MakeDepends -> DependList;
        check_depends, check_depends_mut = CheckDepends -> DependList;
        opt_depends, opt_depends_mut = OptDepends -> DependAndReasonList;
        provides, provides_mut = Provides -> DependList;
        conflicts, conflicts_mut = Conflicts -> DependList;
        replaces, replaces_mut = Replaces -> DependList;
    }
}

mod forgetful;
mod memo;

pub use forgetful::ForgetfulQuerier;
pub use memo::MemoQuerier;
