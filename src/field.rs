use derive_more::{AsRef, Deref};
use strum::{AsRefStr, Display, EnumString, IntoStaticStr};

/// Field of a package description.
#[derive(Debug, Clone, Copy, Eq, PartialEq)] // core traits
#[derive(AsRef, Deref, derive_more::Display)] // derive_more traits
#[display("%{_0}%")]
pub struct Field<Name>(Name);

impl<Name> Field<Name> {
    /// Get an immutable reference to the name of the field.
    pub const fn name(&self) -> &'_ Name {
        &self.0
    }

    /// Convert into the name of the field.
    pub fn into_name(self) -> Name {
        self.0
    }
}

/// Raw string field of a package description.
pub type RawField<'a> = Field<&'a str>;

impl<'a> RawField<'a> {
    /// Get the name of the field as a string slice.
    pub const fn name_str(&self) -> &'_ str {
        self.name()
    }
}

/// Field name of a package description.
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // core traits
#[derive(AsRefStr, Display, EnumString, IntoStaticStr)] // strum traits
#[strum(use_phf)]
pub enum FieldName {
    #[strum(serialize = "FILENAME")]
    FileName,
    #[strum(serialize = "NAME")]
    Name,
    #[strum(serialize = "BASE")]
    Base,
    #[strum(serialize = "VERSION")]
    Version,
    #[strum(serialize = "DESC")]
    Description,
    #[strum(serialize = "GROUPS")]
    Groups,
    #[strum(serialize = "CSIZE")]
    CompressedSize,
    #[strum(serialize = "ISIZE")]
    InstalledSize,
    #[strum(serialize = "MD5SUM")]
    Md5Sum,
    #[strum(serialize = "SHA256SUM")]
    Sha256Sum,
    #[strum(serialize = "PGPSIG")]
    PgpSignature,
    #[strum(serialize = "URL")]
    Url,
    #[strum(serialize = "LICENSE")]
    License,
    #[strum(serialize = "ARCH")]
    Arch,
    #[strum(serialize = "BUILDDATE")]
    BuildDate,
    #[strum(serialize = "PACKAGER")]
    Packager,
    #[strum(serialize = "DEPENDS")]
    Depends,
    #[strum(serialize = "MAKEDEPENDS")]
    MakeDepends,
    #[strum(serialize = "CHECKDEPENDS")]
    CheckDepends,
    #[strum(serialize = "OPTDEPENDS")]
    OptDepends,
    #[strum(serialize = "PROVIDES")]
    Provides,
    #[strum(serialize = "CONFLICTS")]
    Conflicts,
    #[strum(serialize = "REPLACES")]
    Replaces,
}

mod parse;
pub use parse::*;
