use strum::{AsRefStr, Display, EnumString, IntoStaticStr};

/// Operator at the start of a [`DependSpec`](super::DependSpec).
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // core traits
#[derive(AsRefStr, Display, EnumString, IntoStaticStr)] // strum traits
pub enum DependSpecOperator {
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

impl DependSpecOperator {
    /// Parse a dependency spec operator from an input string.
    pub fn parse(input: &str) -> Option<(Self, &'_ str)> {
        use DependSpecOperator::*;
        [LessOrEqual, GreaterOrEqual, Less, Equal, Greater] // XOrEqual must place before X
            .into_iter()
            .find_map(|candidate| {
                input
                    .strip_prefix(candidate.as_ref())
                    .map(|rest| (candidate, rest))
            })
    }
}
