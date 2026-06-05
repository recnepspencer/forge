//! `worth-kernel` owns primitive-construction semantics and the Query-facing
//! authoring boundary over Worth topology and spatial authority surfaces.
//!
//! The public surface is intentionally narrow:
//!
//! - [`facade::authoring`] for authored entry
//! - [`facade::outcome`] for prepared result and accepted/rejected outcome truth
//! - [`facade::diagnostics`] for the intentionally public family, witness,
//!   preview, arbitration, policy, continuity, motion, and rejection
//!   diagnostics
//!
//! It does not export a public certification bucket, a public query-proof /
//! realization-proof report warehouse, public replay / branch-preview-runtime /
//! hostility-suite proof products, or a second local runtime.

#![forbid(unsafe_code)]

mod binding;
mod construction;
mod spatial_intent;
mod test_support;

pub mod facade;
