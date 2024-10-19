use super::{DependSpec, DependSpecOperator, Version};

impl<'a> DependSpec<'a> {
    /// Extract [`DependSpecOperator`] and [`Version`].
    pub fn components(&self) -> Option<(DependSpecOperator, Version<'a>)> {
        DependSpecOperator::parse(self)
            .map(|(depend_spec_operator, version)| (depend_spec_operator, Version(version)))
    }
}
