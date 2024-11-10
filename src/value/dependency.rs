use super::{Dependency, DependencyName, DependencySpecification};

impl<'a> Dependency<&'a str> {
    /// Extract [`DependencyName`] and [`DependencySpecification`].
    ///
    /// ```
    /// # use parse_arch_pkg_desc::value::{Dependency, DependencyName, DependencySpecification};
    /// # use pretty_assertions::assert_eq;
    /// let depend = Dependency("rustup>=1.27.0-1");
    /// let (name, spec) = depend.components();
    /// assert_eq!(name, DependencyName("rustup"));
    /// assert_eq!(spec, DependencySpecification(">=1.27.0-1"));
    /// ```
    pub fn components(&self) -> (DependencyName<&'a str>, DependencySpecification<&'a str>) {
        let (name, spec) = DependencyName::parse(self);
        let spec = DependencySpecification(spec);
        (name, spec)
    }
}
