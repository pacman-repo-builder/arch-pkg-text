use super::{Dependency, DependencyName, DependencySpecification};

impl<'a> Dependency<'a> {
    /// Extract [`DependencyName`] and [`DependencySpecification`].
    pub fn components(&self) -> (DependencyName<'a>, DependencySpecification<'a>) {
        let (name, spec) = DependencyName::parse(self);
        let spec = DependencySpecification(spec);
        (name, spec)
    }
}
