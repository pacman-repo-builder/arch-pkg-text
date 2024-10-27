use super::{DependencySpecification, DependencySpecificationOperator, Version};

impl<'a> DependencySpecification<'a> {
    /// Extract [`DependencySpecificationOperator`] and [`Version`].
    pub fn components(&self) -> Option<(DependencySpecificationOperator, Version<'a>)> {
        DependencySpecificationOperator::parse(self)
            .map(|(depend_spec_operator, version)| (depend_spec_operator, Version(version)))
    }
}
