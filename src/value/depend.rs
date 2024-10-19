use super::{Depend, DependName, DependSpec};

impl<'a> Depend<'a> {
    /// Extract [`DependName`] and [`DependSpec`].
    pub fn components(&self) -> (DependName<'a>, DependSpec<'a>) {
        let (name, spec) = DependName::parse(self);
        let spec = DependSpec(spec);
        (name, spec)
    }
}
