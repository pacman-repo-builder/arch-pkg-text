use strum::{AsRefStr, Display, EnumString, IntoStaticStr};

/// Operator at the start of a [`DependencySpecification`](super::DependencySpecification).
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // core traits
#[derive(AsRefStr, Display, EnumString, IntoStaticStr)] // strum traits
pub enum DependencySpecificationOperator {
    #[strum(serialize = "<")]
    Less = -2,
    #[strum(serialize = "<=")]
    LessOrEqual = -1,
    #[strum(serialize = "=")]
    Equal = 0,
    #[strum(serialize = ">=")]
    GreaterOrEqual = 1,
    #[strum(serialize = ">")]
    Greater = 2,
}

impl DependencySpecificationOperator {
    /// Parse a dependency spec operator from an input string.
    ///
    /// ```
    /// # use inspect_pacman_db::value::DependencySpecificationOperator;
    /// # use pretty_assertions::assert_eq;
    /// assert_eq!(
    ///     DependencySpecificationOperator::parse("<1.27.0-1"),
    ///     Some((DependencySpecificationOperator::Less, "1.27.0-1")),
    /// );
    /// assert_eq!(
    ///     DependencySpecificationOperator::parse("<=1.27.0-1"),
    ///     Some((DependencySpecificationOperator::LessOrEqual, "1.27.0-1")),
    /// );
    /// assert_eq!(
    ///     DependencySpecificationOperator::parse("=1.27.0-1"),
    ///     Some((DependencySpecificationOperator::Equal, "1.27.0-1")),
    /// );
    /// assert_eq!(
    ///     DependencySpecificationOperator::parse(">=1.27.0-1"),
    ///     Some((DependencySpecificationOperator::GreaterOrEqual, "1.27.0-1")),
    /// );
    /// assert_eq!(
    ///     DependencySpecificationOperator::parse(">1.27.0-1"),
    ///     Some((DependencySpecificationOperator::Greater, "1.27.0-1")),
    /// );
    /// assert_eq!(DependencySpecificationOperator::parse("1.27.0-1"), None);
    ///
    /// ```
    pub fn parse(input: &str) -> Option<(Self, &'_ str)> {
        use DependencySpecificationOperator::*;
        [LessOrEqual, GreaterOrEqual, Less, Equal, Greater] // XOrEqual must place before X
            .into_iter()
            .find_map(|candidate| {
                input
                    .strip_prefix(candidate.as_ref())
                    .map(|rest| (candidate, rest))
            })
    }
}
