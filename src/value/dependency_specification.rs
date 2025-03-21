use super::{DependencySpecification, DependencySpecificationOperator, Version};

impl<'a> DependencySpecification<'a> {
    /// Extract [`DependencySpecificationOperator`] and [`Version`].
    ///
    /// ```
    /// # use arch_pkg_text::value::{DependencySpecification, DependencySpecificationOperator, Version};
    /// assert!(matches!(
    ///     DependencySpecification(">=1.27.0-1").components(),
    ///     Some((
    ///         DependencySpecificationOperator::GreaterOrEqual,
    ///         Version("1.27.0-1"),
    ///     )),
    /// ));
    /// assert!(DependencySpecification("").components().is_none());
    /// ```
    pub fn components(&self) -> Option<(DependencySpecificationOperator, Version<'a>)> {
        DependencySpecificationOperator::parse(self)
            .map(|(depend_spec_operator, version)| (depend_spec_operator, Version(version)))
    }
}
