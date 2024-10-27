use super::{DependencySpecification, DependencySpecificationOperator, Version};

impl<'a> DependencySpecification<'a> {
    /// Extract [`DependencySpecificationOperator`] and [`Version`].
    ///
    /// ```
    /// # use inspect_pacman_db::value::{DependencySpecification, DependencySpecificationOperator, Version};
    /// # use pretty_assertions::assert_eq;
    /// assert_eq!(
    ///     DependencySpecification(">=1.27.0-1").components(),
    ///     Some((
    ///         DependencySpecificationOperator::GreaterOrEqual,
    ///         Version("1.27.0-1"),
    ///     )),
    /// );
    /// assert_eq!(DependencySpecification("").components(), None);
    /// ```
    pub fn components(&self) -> Option<(DependencySpecificationOperator, Version<'a>)> {
        DependencySpecificationOperator::parse(self)
            .map(|(depend_spec_operator, version)| (depend_spec_operator, Version(version)))
    }
}
