#![doc = include_str!("courtroom/cross_cutting/certification_compile_fail_proofs.md")]
#![forbid(unsafe_code)]

//! Store certification courtroom: evidence, replay, scenario, and closeout surfaces.
//!
//! Public API follows lifecycle order: authority, evidence, scenario, replay, and closeout.

pub mod authority;
pub mod courtroom;
pub mod evidence;
mod physical_fixture_encoding;
mod scenario;

include!("internal_modules.rs");
mod public_api;

pub use courtroom::blobs::capsule_readiness_provenance::{
    certify_blob_capsule_readiness, BlobCapsuleReadinessCertificationReport,
};
pub use courtroom::operational_recovery::*;
pub use public_api::*;
