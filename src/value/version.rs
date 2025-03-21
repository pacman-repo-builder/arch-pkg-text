use super::{
    Epoch, Release, UpstreamVersion, ValidUpstreamVersion, ValidateUpstreamVersionError, Version,
};
use core::num::ParseIntError;
use derive_more::{Display, Error};

/// Result of [`Version::parse`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParsedVersion<'a> {
    epoch: Option<u64>,
    upstream: ValidUpstreamVersion<'a>,
    release: u64,
}

impl<'a> ParsedVersion<'a> {
    /// Construct a parsed version.
    pub fn new(epoch: Option<u64>, upstream: ValidUpstreamVersion<'a>, release: u64) -> Self {
        ParsedVersion {
            epoch,
            upstream,
            release,
        }
    }

    /// Extract the epoch, upstream version, and release respectively.
    pub fn components(&self) -> (Option<u64>, ValidUpstreamVersion<'a>, u64) {
        let ParsedVersion {
            epoch,
            upstream,
            release,
        } = *self;
        (epoch, upstream, release)
    }
}

/// Error type of [`Version::components`].
#[derive(Debug, Display, Clone, Copy, Error)]
pub enum SplitVersionError {
    #[display("Release suffix not found")]
    MissingRelease,
}

/// Error type of [`Version::parse`].
#[derive(Debug, Display, Clone, Error)]
pub enum ParseVersionError<'a> {
    #[display("Failed to split components: {_0}")]
    InvalidComponents(SplitVersionError),
    #[display("Invalid epoch: {_0}")]
    InvalidEpoch(ParseIntError),
    #[display("Invalid upstream version: {_0}")]
    InvalidUpstream(#[error(not(source))] ValidateUpstreamVersionError<'a>),
    #[display("Invalid release: {_0}")]
    InvalidRelease(ParseIntError),
}

/// Result of [`Version::components`].
type Components<'a> = (Option<Epoch<'a>>, UpstreamVersion<'a>, Release<'a>);

impl<'a> Version<'a> {
    /// Exact the epoch, upstream version, and release respectively as containers of raw strings.
    ///
    /// This function only splits the internal string into its components, it does not validate the components.
    /// Use [`Version::parse`] to actually validate them.
    ///
    /// A valid version usually contains an epoch, an upstream version, and a release suffix:
    ///
    /// ```
    /// # use arch_pkg_text::value::{Epoch, Version};
    /// # use pretty_assertions::assert_eq;
    /// let (epoch, upstream, release) = Version("2:0.1.2_rc.1-1").components().unwrap();
    /// let epoch = epoch.as_ref().map(Epoch::as_str);
    /// let upstream = upstream.as_str();
    /// let release = release.as_str();
    /// assert_eq!((epoch, upstream, release), (Some("2"), "0.1.2_rc.1", "1"));
    /// ```
    ///
    /// Epoch is optional:
    ///
    /// ```
    /// # use arch_pkg_text::value::{Epoch, Version};
    /// # use pretty_assertions::assert_eq;
    /// let (epoch, upstream, release) = Version("0.1.2_rc.1-1").components().unwrap();
    /// let epoch = epoch.as_ref().map(Epoch::as_str);
    /// let upstream = upstream.as_str();
    /// let release = release.as_str();
    /// assert_eq!((epoch, upstream, release), (None, "0.1.2_rc.1", "1"));
    /// ```
    ///
    /// Release is mandatory:
    ///
    /// ```
    /// # use arch_pkg_text::value::{SplitVersionError, Version};
    /// let result = Version("2:0.1.2_rc.1").components();
    /// # dbg!(&result);
    /// assert!(matches!(result, Err(SplitVersionError::MissingRelease)));
    /// ```
    pub fn components(&self) -> Result<Components<'a>, SplitVersionError> {
        let (epoch, rest) = match self.split_once(':') {
            Some((epoch, rest)) => (Some(epoch), rest),
            None => (None, self.as_str()),
        };
        let (upstream, release) = rest
            .rsplit_once('-')
            .ok_or(SplitVersionError::MissingRelease)?;
        Ok((
            epoch.map(Epoch),
            UpstreamVersion(upstream),
            Release(release),
        ))
    }

    /// Parse and validate the version.
    ///
    /// A valid version usually contains an epoch, an upstream version, and a release suffix:
    ///
    /// ```
    /// # use arch_pkg_text::value::Version;
    /// # use pretty_assertions::assert_eq;
    /// let (epoch, upstream, release) = Version("2:0.1.2_rc.1-1").parse().unwrap().components();
    /// assert_eq!(
    ///     (epoch, upstream.as_str(), release),
    ///     (Some(2), "0.1.2_rc.1", 1)
    /// );
    /// ```
    ///
    /// Epoch is optional:
    ///
    /// ```
    /// # use arch_pkg_text::value::Version;
    /// # use pretty_assertions::assert_eq;
    /// let (epoch, upstream, release) = Version("0.1.2_rc.1-1").parse().unwrap().components();
    /// assert_eq!(
    ///     (epoch, upstream.as_str(), release),
    ///     (None, "0.1.2_rc.1", 1)
    /// );
    /// ```
    ///
    /// Release is mandatory:
    ///
    /// ```
    /// # use arch_pkg_text::value::{ParseVersionError, SplitVersionError, Version};
    /// let result = Version("2:0.1.2_rc.1").parse();
    /// # dbg!(&result);
    /// assert!(matches!(
    ///     result,
    ///     Err(ParseVersionError::InvalidComponents(SplitVersionError::MissingRelease))),
    /// );
    /// ```
    ///
    /// Epoch must be a valid integer:
    ///
    /// ```
    /// # use arch_pkg_text::value::{ParseVersionError, Version};
    /// let result = Version("2.1:0.1.2_rc.1-1").parse();
    /// # dbg!(&result);
    /// assert!(matches!(
    ///     result,
    ///     Err(ParseVersionError::InvalidEpoch(_))),
    /// );
    /// ```
    ///
    /// Upstream version must be valid:
    ///
    /// ```
    /// # use arch_pkg_text::value::{ParseVersionError, Version};
    /// let result = Version("2:0.1.2-rc.1-1").parse();
    /// # dbg!(&result);
    /// assert!(matches!(
    ///     result,
    ///     Err(ParseVersionError::InvalidUpstream(_))),
    /// );
    /// ```
    ///
    /// Release must be a valid integer:
    ///
    /// ```
    /// # use arch_pkg_text::value::{ParseVersionError, Version};
    /// let result = Version("2:0.1.2_rc.1-a").parse();
    /// # dbg!(&result);
    /// assert!(matches!(
    ///     result,
    ///     Err(ParseVersionError::InvalidRelease(_))),
    /// );
    /// ```
    pub fn parse(&self) -> Result<ParsedVersion<'a>, ParseVersionError<'a>> {
        let (epoch, upstream, release) = self
            .components()
            .map_err(ParseVersionError::InvalidComponents)?;
        let epoch = epoch
            .as_ref()
            .map(Epoch::parse)
            .transpose()
            .map_err(ParseVersionError::InvalidEpoch)?;
        let upstream = upstream
            .validate()
            .map_err(ParseVersionError::InvalidUpstream)?;
        let release = release.parse().map_err(ParseVersionError::InvalidRelease)?;
        Ok(ParsedVersion::new(epoch, upstream, release))
    }
}

impl<'a> TryFrom<Version<'a>> for ParsedVersion<'a> {
    type Error = ParseVersionError<'a>;
    fn try_from(version: Version<'a>) -> Result<Self, Self::Error> {
        version.parse()
    }
}
