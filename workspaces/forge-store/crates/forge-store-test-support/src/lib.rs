#![doc = include_str!("test_support_compile_fail_proofs.md")]
#![forbid(unsafe_code)]

//! Store test support — harness fixtures and synthetic authority for certification replay.
//!
//! Production admission authority lives in production crates. This crate falsifies
//! production topology through named harness modules rather than bypassing it.

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

pub use harness::*;

#[deprecated(
    since = "0.0.0",
    note = "use harness_physical_reference — test support must not imply production authority"
)]
pub use harness::physical_reference::test_physical_reference;