use strum::{AsRefStr, Display, EnumString, IntoStaticStr};

/// Field of a `.SRCINFO` file.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Field<Name, Architecture> {
    name: Name,
    architecture: Option<Architecture>,
}

impl Field<(), ()> {
    /// Create a blank [`Field`].
    ///
    /// This function when combined with [`with_name`](Field::with_name) and [`with_architecture`](Field::with_architecture)
    /// would produce a usable [`Field`].
    pub fn blank() -> Self {
        Field {
            name: (),
            architecture: None,
        }
    }
}

impl<Name, Architecture> Field<Name, Architecture> {
    /// Replace the name of the field.
    pub fn with_name<NewName>(self, name: NewName) -> Field<NewName, Architecture> {
        Field {
            name,
            architecture: self.architecture,
        }
    }

    /// Replace the architecture suffix of the field.
    pub fn with_architecture<NewArchitecture>(
        self,
        architecture: Option<NewArchitecture>,
    ) -> Field<Name, NewArchitecture> {
        Field {
            name: self.name,
            architecture,
        }
    }

    /// Get an immutable reference to the name of the field.
    pub const fn name(&self) -> &'_ Name {
        &self.name
    }

    /// Get an immutable reference to the architecture suffix of the field.
    pub const fn architecture(&self) -> Option<&'_ Architecture> {
        self.architecture.as_ref()
    }

    /// Destructure into a tuple of field name and architecture.
    pub fn into_components(self) -> (Name, Option<Architecture>) {
        (self.name, self.architecture)
    }
}

impl<'a, Architecture> Field<&'a str, Architecture> {
    /// Get the name of the field as a string slice.
    pub const fn name_str(&self) -> &'_ str {
        self.name()
    }
}

impl<'a, Name> Field<Name, &'a str> {
    /// Get the name of the field as a string slice.
    pub fn architecture_str(&self) -> Option<&'_ str> {
        self.architecture().copied()
    }
}

/// Raw string field of a `.SRCINFO` file.
pub type RawField<'a> = Field<&'a str, &'a str>;

/// Parsed field of a `.SRCINFO` file.
pub type ParsedField<Architecture> = Field<FieldName, Architecture>;

impl<Architecture> ParsedField<Architecture> {
    /// Get the name of the field as a string slice.
    pub fn name_str(&self) -> &'static str {
        self.name().into()
    }
}

/// Convert a [`FieldName`] into a [`ParsedField`] without an architecture.
impl<Architecture> From<FieldName> for ParsedField<Architecture> {
    fn from(value: FieldName) -> Self {
        Field::blank().with_name(value).with_architecture(None)
    }
}

/// Field name of a `.SRCINFO` file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // core traits
#[derive(AsRefStr, Display, EnumString, IntoStaticStr)] // strum traits
#[strum(use_phf)]
pub enum FieldName {
    /* SECTION HEADERS */
    #[strum(serialize = "pkgbase")]
    Base,
    #[strum(serialize = "pkgname")]
    Name,

    /* BASE SECTION ONLY */
    #[strum(serialize = "epoch")]
    Epoch,
    #[strum(serialize = "pkgrel")]
    Release,
    #[strum(serialize = "validpgpkeys")]
    ValidPgpKeys,
    #[strum(serialize = "pkgver")]
    Version,

    /* ANY SECTION: MISC */
    #[strum(serialize = "arch")]
    Architecture,
    #[strum(serialize = "backup")]
    Backup,
    #[strum(serialize = "changelog")]
    ChangeLog,
    #[strum(serialize = "pkgdesc")]
    Description,
    #[strum(serialize = "groups")]
    Groups,
    #[strum(serialize = "install")]
    InstallScriptName,
    #[strum(serialize = "license")]
    License,
    #[strum(serialize = "noextract")]
    NoExtract,
    #[strum(serialize = "options")]
    Options,
    #[strum(serialize = "source")]
    Source,
    #[strum(serialize = "url")]
    Url,

    /* ANY SECTION: DEPENDENCIES */
    #[strum(serialize = "depends")]
    Dependencies,
    #[strum(serialize = "checkdepends")]
    CheckDependencies,
    #[strum(serialize = "makedepends")]
    MakeDependencies,
    #[strum(serialize = "optdepends")]
    OptionalDependencies,
    #[strum(serialize = "provides")]
    Provides,
    #[strum(serialize = "conflicts")]
    Conflicts,
    #[strum(serialize = "replaces")]
    Replaces,

    /* ANY SECTION: CHECKSUMS */
    #[strum(serialize = "md5sums")]
    Md5Checksums,
    #[strum(serialize = "sha1sums")]
    Sha1Checksums,
    #[strum(serialize = "sha224sums")]
    Sha224Checksums,
    #[strum(serialize = "sha256sums")]
    Sha256Checksums,
    #[strum(serialize = "sha384sums")]
    Sha384Checksums,
    #[strum(serialize = "sha512sums")]
    Sha512Checksums,
}

mod parse;
pub use parse::*;
