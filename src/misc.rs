//! Miscellaneous items.

pub use crate::{desc::misc as desc, srcinfo::misc as srcinfo};
pub use typebool::{Bool as StaticBool, False, True};

#[cfg(feature = "std")]
pub use indexmap;
#[cfg(feature = "parking_lot")]
pub use parking_lot;
pub use typebool;
