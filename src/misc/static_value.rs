/// [Static boolean](StaticBool) which corresponds to `false`.
pub struct False;
impl sealed::Sealed for False {}
impl StaticValue for False {
    type CorrespondingRuntimeType = bool;
    const VALUE: Self::CorrespondingRuntimeType = false;
}

/// [Static boolean](StaticBool) which corresponds to `true`.
pub struct True;
impl sealed::Sealed for True {}
impl StaticValue for True {
    type CorrespondingRuntimeType = bool;
    const VALUE: Self::CorrespondingRuntimeType = true;
}

/// To be used as an associated type which is analogous to a `bool`.
pub trait StaticBool: StaticValue<CorrespondingRuntimeType = bool> {}
impl<X: StaticValue<CorrespondingRuntimeType = bool> + ?Sized> StaticBool for X {}

pub trait StaticValue: sealed::Sealed {
    type CorrespondingRuntimeType: ?Sized;
    const VALUE: Self::CorrespondingRuntimeType;
}

mod sealed {
    pub trait Sealed {}
}
