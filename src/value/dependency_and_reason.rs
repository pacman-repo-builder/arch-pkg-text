use super::{Dependency, DependencyAndReason, DependencyReason};

impl<'a> DependencyAndReason<'a> {
    /// Extract [`Dependency`] and [`DependencyReason`].
    pub fn components(&self) -> (Dependency<'a>, Option<DependencyReason<'a>>) {
        // split with ": " instead of ':' because of epoch in version
        match self.split_once(": ") {
            Some((depend, reason)) => (
                Dependency(depend.trim()),
                Some(DependencyReason(reason.trim())),
            ),
            None => (Dependency(self.trim()), None),
        }
    }
}
