#![doc = include_str!("certification_compile_fail_proofs.md")]
#![doc = include_str!("receipt_authority_compile_fail_proofs.md")]
#![forbid(unsafe_code)]

//! Store certification courtroom — evidence, replay, scenario, and closeout surfaces.
//!
//! Public API follows lifecycle order: authority → evidence → scenario → replay → closeout.

pub mod authority;
mod capsule_readiness_provenance;
pub mod courtroom;
pub mod evidence;
pub mod s8_layout_closeout;

include!("internal_modules.rs");
mod public_api;

pub use capsule_readiness_provenance::{
    certify_s7_capsule_readiness, S7CapsuleReadinessCertificationReport,
};
pub use public_api::*;
