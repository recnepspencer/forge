//! Lifecycle-ordered public facade for blob chunk proof flow.
//!
//! Each submodule groups exports by authority class:
//! capabilities, outcomes, denials, and counter witnesses.
//! Hostile-lane `reject_*` constructors live in [`hostile_lane`], separate from
//! production capability admission.

mod capsule_readiness;
#[cfg(feature = "certification-test-authority")]
pub mod certification_test_authority;
mod compaction;
mod corruption;
mod dedupe;
mod export_bundle;
mod handoffs;
pub mod hostile_lane;
mod identity;
mod import_readmission;
mod integrity;
mod lifecycle;
mod placement;
mod publication;
mod reachability;
mod recovery;
mod retention_reclaim;
mod streaming;

// Lifecycle-ordered public re-exports.
pub use capsule_readiness::*;
pub use compaction::*;
pub use corruption::*;
pub use dedupe::*;
pub use export_bundle::*;
pub use handoffs::*;
pub use identity::*;
pub use import_readmission::*;
pub use integrity::*;
pub use lifecycle::*;
pub use placement::*;
pub use publication::*;
pub use reachability::*;
pub use recovery::*;
pub use retention_reclaim::*;
pub use streaming::*;
