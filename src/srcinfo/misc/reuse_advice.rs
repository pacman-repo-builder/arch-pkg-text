pub use crate::misc::{False, StaticBool, StaticValue, True};

/// Denote whether a certain querier should be reused.
///
/// "Reuse" means to call methods of [`Query`](crate::srcinfo::Query) and/or [`QueryMut`](crate::srcinfo::QueryMut) more than once.
pub trait ReuseAdvice {
    /// Whether the querier should be reused.
    type ShouldReuse: StaticBool + ?Sized;
}

/// Utility to lookup the `bool` value of [`ReuseAdvice`].
pub trait ReuseAdviceBool: ReuseAdvice {
    /// The value of [`ReuseAdvice::ShouldReuse`] as a bool.
    const SHOULD_REUSE: bool = <Self::ShouldReuse>::VALUE;
}
impl<Querier: ReuseAdvice + ?Sized> ReuseAdviceBool for Querier {}

/// Utility to lookup the `bool` value of `self` whose type implements [`ReuseAdvice`].
///
/// This trait is the dyn-friendly version of [`ReuseAdviceBool`].
pub trait ReuseAdviceSelf: ReuseAdvice {
    /// Determine wether `self` [should be reused](ReuseAdvice).
    fn should_reuse(&self) -> bool {
        Self::SHOULD_REUSE
    }
}
impl<Querier: ReuseAdvice + ?Sized> ReuseAdviceSelf for Querier {}
