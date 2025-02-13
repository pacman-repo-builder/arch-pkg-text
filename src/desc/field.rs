use derive_more::{AsRef, Deref};
use strum::{AsRefStr, Display, EnumString, IntoStaticStr};

/// Field of a `desc` file.
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

/// Raw string field of a `desc` file.
pub type RawField<'a> = Field<&'a str>;

impl<'a> RawField<'a> {
    /// Get the name of the field as a string slice.
    pub const fn name_str(&self) -> &'a str {
        self.name()
    }
}

/// Parsed field of a `desc` file.
pub type ParsedField = Field<FieldName>;

impl ParsedField {
    /// Create a new [`ParsedField`].
    pub const fn new(name: FieldName) -> Self {
        Field(name)
    }

    /// Get the name of the field as a string slice.
    pub fn name_str(&self) -> &'static str {
        self.name().into()
    }
}

impl From<FieldName> for ParsedField {
    fn from(value: FieldName) -> Self {
        ParsedField::new(value)
    }
}

/// Field name of a `desc` file.
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
    Md5Checksum,
    #[strum(serialize = "SHA256SUM")]
    Sha256Checksum,
    #[strum(serialize = "PGPSIG")]
    PgpSignature,
    #[strum(serialize = "URL")]
    Url,
    #[strum(serialize = "LICENSE")]
    License,
    #[strum(serialize = "ARCH")]
    Architecture,
    #[strum(serialize = "BUILDDATE")]
    BuildDate,
    #[strum(serialize = "PACKAGER")]
    Packager,
    #[strum(serialize = "DEPENDS")]
    Dependencies,
    #[strum(serialize = "MAKEDEPENDS")]
    MakeDependencies,
    #[strum(serialize = "CHECKDEPENDS")]
    CheckDependencies,
    #[strum(serialize = "OPTDEPENDS")]
    OptionalDependencies,
    #[strum(serialize = "PROVIDES")]
    Provides,
    #[strum(serialize = "CONFLICTS")]
    Conflicts,
    #[strum(serialize = "REPLACES")]
    Replaces,
}

mod parse;
pub use parse::*;
