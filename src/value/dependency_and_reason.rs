use super::{Dependency, DependencyAndReason, DependencyReason};

impl<'a> DependencyAndReason<&'a str> {
    /// Extract [`Dependency`] and [`DependencyReason`].
    ///
    /// ```
    /// # use arch_pkg_text::value::{Dependency, DependencyAndReason, DependencyReason};
    /// # use pretty_assertions::assert_eq;
    /// let depend_and_reason = DependencyAndReason("lldb: rust-lldb script");
    /// let (depend, reason) = depend_and_reason.components();
    /// assert_eq!(depend, Dependency("lldb"));
    /// assert_eq!(reason, Some(DependencyReason("rust-lldb script")));
    /// ```
    pub fn components(&self) -> (Dependency<&'a str>, Option<DependencyReason<&'a str>>) {
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
