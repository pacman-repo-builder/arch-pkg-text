use crate::{
    field::{FieldName, ParsedField},
    value,
};

macro_rules! def_scalar_mtd {
    ($(
        $(#[$attrs:meta])*
        $name:ident = $field_name:ident -> $value_type:ident;
    )*) => {$(
        $(#[$attrs])*
        fn $name(self) -> Option<value::$value_type<'a>> {
            self.query_raw_text(ParsedField::new(FieldName::$field_name))
                .map(value::$value_type::new)
        }
    )*};
}

macro_rules! def_list_mtd {
    ($(
        $(#[$attrs:meta])*
        $name:ident = $field_name:ident -> $value_type:ident;
    )*) => {$(
        $(#[$attrs])*
        fn $name(self) -> Option<value::$value_type<&'a str>> {
            self.query_raw_text(ParsedField::new(FieldName::$field_name))
                .map(value::$value_type::new)
        }
    )*};
}

pub trait Query<'a>: Sized {
    fn query_raw_text(self, field: ParsedField) -> Option<&'a str>;

    def_scalar_mtd! {
        file_name = FileName -> FileName;
        name = Name -> Name;
        base = Base -> Base;
        version = Version -> Version;
        description = Description -> Description;
        compressed_size = CompressedSize -> Size;
        installed_size = InstalledSize -> Size;
        md5sum = Md5Sum -> Md5Sum;
        sha256sum = Sha256Sum -> Sha256Sum;
        pgp_signature = PgpSignature -> PgpSignature;
        url = Url -> Url;
        build_date = BuildDate -> Timestamp;
        packager = Packager -> Packager;
    }

    def_list_mtd! {
        groups = Groups -> GroupList;
        license = License -> LicenseList;
        arch = Arch -> ArchList;
        depends = Depends -> DependList;
        make_depends = MakeDepends -> DependList;
        check_depends = CheckDepends -> DependList;
        opt_depends = OptDepends -> DependAndReasonList;
        provides = Provides -> DependList;
        conflicts = Conflicts -> DependList;
        replaces = Replaces -> DependList;
    }
}

mod forgetful;
mod memo;

pub use forgetful::ForgetfulQuerier;
pub use memo::MemoQuerier;
