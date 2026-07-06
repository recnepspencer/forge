//! Lifecycle-ordered public facade for blob chunk proof flow.
//!
//! Each submodule groups exports by authority class:
//! capabilities, outcomes, denials, and counter witnesses.
//! Hostile-lane `reject_*` constructors live in [`hostile_lane`], separate from
//! production capability admission.

mod compaction;
mod corruption;
mod dedupe;
mod handoffs;
mod harness;
mod hostile_lane;
mod identity;
mod integrity;
mod lifecycle;
mod placement;
mod publication;
mod reachability;
mod recovery;
mod retention_reclaim;
mod streaming;

// Lifecycle-ordered public re-exports.
pub use identity::*;
pub use integrity::*;
pub use dedupe::*;
pub use streaming::*;
pub use lifecycle::*;
pub use publication::*;
pub use reachability::*;
pub use placement::*;
pub use compaction::*;
pub use corruption::*;
pub use recovery::*;
pub use retention_reclaim::*;
pub use handoffs::*;
pub use hostile_lane::*;
pub use harness::*;