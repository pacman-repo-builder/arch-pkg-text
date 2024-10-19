use super::{Depend, DependAndReason, DependReason};

impl<'a> DependAndReason<'a> {
    /// Extract [`Depend`] and [`DependReason`].
    pub fn components(&self) -> (Depend<'a>, Option<DependReason<'a>>) {
        // split with ": " instead of ':' because of epoch in version
        match self.split_once(": ") {
            Some((depend, reason)) => (Depend(depend.trim()), Some(DependReason(reason.trim()))),
            None => (Depend(self.trim()), None),
        }
    }
}
