#![cfg_attr(not(feature = "std"), no_std)]
pub mod desc;
pub mod misc;
pub mod parse;
pub mod srcinfo;
pub mod value;

pub use desc::{Query as QueryDesc, QueryMut as QueryDescMut};
pub use parse::ParsedDesc;
#[cfg(feature = "std")]
pub use parse::ParsedSrcinfo;
pub use srcinfo::{Query as QuerySrcinfo, QueryMut as QuerySrcinfoMut};

#[cfg(feature = "std")]
pub use indexmap;
#[cfg(feature = "parking_lot")]
pub use parking_lot;
