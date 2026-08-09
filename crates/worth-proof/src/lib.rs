//! Compile-time proof-bearing progression substrate for WORTH.

mod artifact;
mod assumption;
mod band;
mod collections;
mod composition;
mod dx;
mod facade;
mod phase;
pub mod prelude;
mod proof;
pub mod raw;
mod recipe;
mod transition;

#[doc(hidden)]
pub use band::__band_guard_package_matches_any_prefix;
pub use facade::*;
