//! Miscellaneous items.

mod static_value;

pub use static_value::*;

pub use crate::{desc::misc as desc, srcinfo::misc as srcinfo};

#[cfg(feature = "std")]
pub use indexmap;
#[cfg(feature = "parking_lot")]
pub use parking_lot;
