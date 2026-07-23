//! Transitional managed-live replacement compatibility.
//!
//! Query 9.14 phases 17, 19, 23, and 24 are this module's exit trigger.

mod basis_digest;
mod decision;
mod denial;
mod entry;
mod plan;
mod planner;
mod preservation;
mod rebind;
mod retirement;

pub use denial::{
    WorthUiQueryBindingDriftDenial, WorthUiQueryBindingDriftDenialKind,
    WorthUiQueryLiveRebindPlanDenial,
};
pub use entry::{WorthUiQueryLiveRebindEntry, WorthUiQueryLiveRebindOutcome};
pub use plan::{WorthUiQueryLiveRebindCounters, WorthUiQueryLiveRebindPlan};
pub(crate) use planner::WorthUiQueryLiveRebindPlanner;
pub use preservation::{WorthUiQueryBindingPreservation, WorthUiQueryBindingPreservationReceipt};
pub use rebind::{
    WorthUiQueryBindingRebind, WorthUiQueryBindingRebindReason, WorthUiQueryRebindRequiredSurface,
};
pub use retirement::{WorthUiQueryBindingRetirement, WorthUiQueryBindingRetirementReason};
