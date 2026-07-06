#![doc = include_str!("certification_compile_fail_proofs.md")]
#![doc = include_str!("receipt_authority_compile_fail_proofs.md")]
#![forbid(unsafe_code)]

//! Store certification courtroom — evidence, replay, scenario, and closeout surfaces.
//!
//! Public API follows lifecycle order: authority → evidence → scenario → replay → closeout.

pub mod authority;
pub mod courtroom;
pub mod evidence;

include!("internal_modules.rs");
mod public_api;

pub use public_api::*;