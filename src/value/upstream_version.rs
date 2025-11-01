use super::UpstreamVersion;
use core::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    iter::FusedIterator,
    str::Split,
};
use derive_more::{AsRef, Display, Error};
use pipe_trait::Pipe;

/// Component of [`UpstreamVersion`], it includes a numeric prefix and a non-numeric suffix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UpstreamVersionComponent<'a> {
    prefix: Option<u64>,
    suffix: &'a str,
}

impl<'a> UpstreamVersionComponent<'a> {
    /// Construct a new component.
    pub fn new(prefix: Option<u64>, suffix: &'a str) -> Self {
        UpstreamVersionComponent { prefix, suffix }
    }

    /// Parse a component from segment text.
    ///
    /// ```
    /// # use arch_pkg_text::value::UpstreamVersionComponent;
    /// # use pretty_assertions::assert_eq;
    /// assert_eq!(UpstreamVersionComponent::parse("").components(), (None, ""));
    /// assert_eq!(
    ///     UpstreamVersionComponent::parse("alpha").components(),
    ///     (None, "alpha"),
    /// );
    /// assert_eq!(
    ///     UpstreamVersionComponent::parse("123").components(),
    ///     (Some(123), ""),
    /// );
    /// assert_eq!(
    ///     UpstreamVersionComponent::parse("123alpha").components(),
    ///     (Some(123), "alpha"),
    /// );
    /// assert_eq!(
    ///     UpstreamVersionComponent::parse("0alpha").components(),
    ///     (Some(0), "alpha"),
    /// );
    /// assert_eq!(
    ///     UpstreamVersionComponent::parse("00alpha").components(),
    ///     (Some(0), "alpha"),
    /// );
    /// ```
    pub fn parse(segment: &'a str) -> Self {
        if segment.is_empty() {
            return UpstreamVersionComponent::new(None, "");
        }
        let mut prefix = 0;
        let mut boundary = segment.len();
        for (idx, char) in segment.char_indices() {
            if char.is_ascii_digit() {
                prefix *= 10;
                prefix += char as u64 - b'0' as u64;
            } else {
                boundary = idx;
                break;
            }
        }
        let prefix = (boundary != 0).then_some(prefix);
        let suffix = &segment[boundary..];
        UpstreamVersionComponent::new(prefix, suffix)
    }

    /// Exact the numeric prefix and non-numeric suffix.
    pub fn components(&self) -> (Option<u64>, &'a str) {
        let UpstreamVersionComponent { prefix, suffix } = *self;
        (prefix, suffix)
    }
}

/// Iterator over [`UpstreamVersionComponent`].
///
/// This struct is created by calling [`ValidUpstreamVersion::components`].
#[derive(Debug, Clone)]
pub struct UpstreamVersionComponentIter<'a> {
    segments: Split<'a, [char; 4]>,
}

impl<'a> Iterator for UpstreamVersionComponentIter<'a> {
    type Item = UpstreamVersionComponent<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.segments.next().map(UpstreamVersionComponent::parse)
    }
}

impl DoubleEndedIterator for UpstreamVersionComponentIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.segments
            .next_back()
            .map(UpstreamVersionComponent::parse)
    }
}

impl FusedIterator for UpstreamVersionComponentIter<'_> {}

/// Upstream version which has been [validated](UpstreamVersion::validate).
#[derive(Debug, Display, Clone, Copy, AsRef)]
pub struct ValidUpstreamVersion<'a>(&'a str);

impl<'a> ValidUpstreamVersion<'a> {
    /// Get an immutable reference to the raw string underneath.
    pub fn as_str(&self) -> &'a str {
        self.0
    }

    /// Iterate over all components of the version.
    ///
    /// Components are separated by dots (`.`), underscores (`_`), plus signs
    /// (`+`), or at signs (`@`).
    /// All separators are treated the same way because they are treated
    /// the same by [`vercmp`](https://man.archlinux.org/man/vercmp.8.en).
    pub fn components(&self) -> UpstreamVersionComponentIter<'a> {
        UpstreamVersionComponentIter {
            segments: self.as_str().split(['.', '_', '+', '@']),
        }
    }
}

impl Ord for ValidUpstreamVersion<'_> {
    /// Comparing two validated upstream versions.
    ///
    /// This comparison aims to emulate [`vercmp`](https://man.archlinux.org/man/vercmp.8.en)'s
    /// algorithm on validated upstream versions.
    ///
    /// ```
    /// # use arch_pkg_text::value::UpstreamVersion;
    /// let validate = |raw| UpstreamVersion(raw).validate().unwrap();
    ///
    /// // Two versions are considered equal if their internal strings are equal
    /// assert!(validate("1.2.3") == validate("1.2.3"));
    /// assert!(validate("1.2_3") == validate("1.2_3"));
    /// assert!(validate("1_2_3") == validate("1_2_3"));
    ///
    /// // Each component pair of two versions are compared until an unequal pair is found
    /// assert!(validate("1.2.0") < validate("1.2.3"));
    /// assert!(validate("1.3.2") > validate("1.2.3"));
    /// assert!(validate("1.2.3.0.5.6") < validate("1.2.3.4.5.6"));
    /// assert!(validate("1.2.3.4.5") > validate("1.2.3.2.1"));
    /// assert!(validate("2.1.4") > validate("2.1.0.5"));
    /// assert!(validate("1.1.0") < validate("1.2"));
    ///
    /// // If one version is the leading part of another, the latter is considered greater
    /// assert!(validate("1.1.0") > validate("1.1"));
    /// assert!(validate("1.1.0") < validate("1.1.0.0"));
    ///
    /// // The difference between dots, underscores, plus signs, and at signs are ignored
    /// assert!(validate("1.2.3") == validate("1.2_3"));
    /// assert!(validate("1.2.3") == validate("1_2_3"));
    /// assert!(validate("1.2.0") < validate("1.2_3"));
    /// assert!(validate("1_1.0") > validate("1.1"));
    /// assert!(validate("1+2.3") == validate("1@2_3"));
    /// assert!(validate("1@2@3") == validate("1+2+3"));
    /// assert!(validate("1@2.0") < validate("1.2_3"));
    /// assert!(validate("1_1.0") > validate("1+1"));
    ///
    /// // Leading zeros are ignored
    /// assert!(validate("01.02.3") == validate("1.2.03"));
    /// assert!(validate("1.02.0") < validate("1.2.3"));
    /// assert!(validate("1.01.0") > validate("1.1"));
    /// assert!(validate("1.1.0") > validate("1.001"));
    /// ```
    ///
    /// **NOTE:** For licensing reason, this trait method was implemented from scratch by testing
    /// case-by-case without looking at the source code of `vercmp` so there might be edge-cases and
    /// subtle differences.
    /// Contributors are welcomed to propose PRs to fix these edge-cases as long as they don't look
    /// at the source code of `vercmp`.
    fn cmp(&self, other: &Self) -> Ordering {
        self.components().cmp(other.components())
    }
}

impl PartialOrd for ValidUpstreamVersion<'_> {
    /// Return a `Some(ordering)` with `ordering` being the result of [`ValidUpstreamVersion::cmp`].
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for ValidUpstreamVersion<'_> {}

impl PartialEq for ValidUpstreamVersion<'_> {
    /// Return `true` if [`ValidUpstreamVersion::cmp`] returns [`Ordering::Equal`].
    /// Otherwise, return `false`.
    ///
    /// **NOTE:** Two versions being equal doesn't necessarily means that their internal
    /// strings are equal. This is because dots (`.`), underscores (`_`), plus signs (`+`),
    /// and at signs (`@`) were ignored during parsing.
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Hash for ValidUpstreamVersion<'_> {
    /// This custom hash algorithm was implemented in such a way to be consistent with [`ValidUpstreamVersion::cmp`]
    /// and [`ValidUpstreamVersion::eq`].
    fn hash<State: Hasher>(&self, state: &mut State) {
        for component in self.components() {
            component.hash(state);
        }
    }
}

/// Error of [`UpstreamVersion::validate`].
#[derive(Debug, Display, Clone, Copy, Error)]
#[display("{input:?} is not a valid version because {character:?} is not a valid character")]
pub struct ValidateUpstreamVersionError<'a> {
    character: char,
    input: UpstreamVersion<'a>,
}

impl<'a> UpstreamVersion<'a> {
    /// Validate the version, return a [`ValidUpstreamVersion`] on success.
    ///
    /// > Package release tags follow the same naming restrictions as version tags.
    /// > -- from <https://wiki.archlinux.org/title/Arch_package_guidelines#Package_versioning>
    ///
    /// > Package names can contain only alphanumeric characters and any of `@`, `.`, `_`, `+`, `-`.
    /// > Names are not allowed to start with hyphens or dots. All letters should be lowercase.
    /// > -- from <https://wiki.archlinux.org/title/Arch_package_guidelines#Package_naming>
    ///
    /// Since a dash (`-`) signifies a `pkgrel` which is not part of upstream version, it is not
    /// considered a valid character.
    ///
    /// ```
    /// # use arch_pkg_text::value::UpstreamVersion;
    /// # use pretty_assertions::assert_eq;
    /// assert_eq!(
    ///     UpstreamVersion("12.34_56a").validate().unwrap().as_str(),
    ///     "12.34_56a",
    /// );
    /// assert!(UpstreamVersion("2:12.34_56a-1").validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<ValidUpstreamVersion<'a>, ValidateUpstreamVersionError<'a>> {
        let invalid_char = self.chars().find(
            |char| !matches!(char, '0'..='9' | '.' | '_' | '+' | '@' | 'a'..='z' | 'A'..='Z' ),
        );
        if let Some(character) = invalid_char {
            Err(ValidateUpstreamVersionError {
                character,
                input: *self,
            })
        } else {
            self.as_str().pipe(ValidUpstreamVersion).pipe(Ok)
        }
    }
}

impl<'a> TryFrom<UpstreamVersion<'a>> for ValidUpstreamVersion<'a> {
    type Error = ValidateUpstreamVersionError<'a>;
    fn try_from(value: UpstreamVersion<'a>) -> Result<Self, Self::Error> {
        value.validate()
    }
}

impl<'a> From<ValidUpstreamVersion<'a>> for UpstreamVersion<'a> {
    fn from(value: ValidUpstreamVersion<'a>) -> Self {
        value.as_str().pipe(UpstreamVersion)
    }
}
