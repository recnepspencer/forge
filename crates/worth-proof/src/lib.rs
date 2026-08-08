//! Compile-time proof-bearing progression substrate for WORTH.

mod artifact;
mod assumption;
mod band;
mod binding;
mod brand;
mod collections;
mod composition;
pub mod contracts;
mod dx;
mod effect;
mod facade;
mod linear;
mod phase;
pub mod prelude;
mod proof;
pub mod raw;
mod recipe;
mod transition;
mod type_level;

pub(crate) use type_level::type_level_traits;

pub use facade::*;

#[doc(hidden)]
pub use band::__band_guard_package_matches_any_prefix;
