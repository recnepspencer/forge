#![doc = include_str!("test_support_compile_fail_proofs.md")]
#![forbid(unsafe_code)]

//! Store test support — production-facade harness helpers plus named synthetic
//! courtroom authority for certification replay.
//!
//! Production admission authority lives in production crates. This crate falsifies
//! production topology through named harness modules rather than bypassing it:
//! `harness::production_facade` assembles legal production flows, while
//! `harness::test_authority` exposes courtroom-only synthetic evidence.

mod allocation_sentinels;
mod hostile_readmission_json_fixtures;
mod json_fixture_boundary;
mod large_record_streams;
mod memory_pressure;
mod native_aspect_fixture_authoring;
mod native_aspect_fixtures;
mod resident_pressure_fixtures;
mod terminal_projection_json_fixtures;

pub mod harness;

pub use harness::production_facade::*;
pub use harness::{production_facade, test_authority};

#[deprecated(
    since = "0.0.0",
    note = "use harness_physical_reference — test support must not imply production authority"
)]
pub use harness::physical_reference::test_physical_reference;
