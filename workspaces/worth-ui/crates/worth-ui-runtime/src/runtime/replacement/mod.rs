//! Replacement lane — admit → compare → classify → narrow → match → reconcile → rebind → lowering input.

pub mod admission;
pub mod candidate;
pub mod compatibility;
pub mod equivalence;
#[cfg(test)]
pub mod file_rust_replacement_parity;
pub mod impact;
pub mod matching;
pub mod narrowing;
pub mod query_binding;
pub mod reconciliation;
pub mod state_inventory;

pub mod node_classification;
mod orchestrator;
mod platform_state_inventory;
mod query_orchestration;
mod transitions;

pub use node_classification::{
    WorthUiAmbiguousReplacementDenial, WorthUiNodeLifecycleTransition,
    WorthUiNodeReplacementClassification, WorthUiNodeReplacementCounters,
    WorthUiNodeReplacementPlan,
};
pub use orchestrator::WorthUiReplacementLoweringDenial;
pub use transitions::WorthUiReplacementLoweringReady;
