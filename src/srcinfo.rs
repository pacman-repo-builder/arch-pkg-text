//! Fields and queriers of the text format of `.SRCINFO` files.

pub mod misc;

mod field;
pub use field::*;

mod query;
pub use query::*;
