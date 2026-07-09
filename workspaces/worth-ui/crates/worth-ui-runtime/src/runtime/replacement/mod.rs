//! Replacement lane — admit → compare → classify → narrow → match → reconcile → rebind → lowering input.

#[path = "../admission/mod.rs"]
pub mod admission;
#[path = "../candidate/mod.rs"]
pub mod candidate;
#[path = "../equivalence/mod.rs"]
pub mod equivalence;
#[path = "../file_rust_replacement_parity/mod.rs"]
pub mod file_rust_replacement_parity;
#[path = "../impact/mod.rs"]
pub mod impact;
#[path = "../matching/mod.rs"]
pub mod matching;
#[path = "../narrowing/mod.rs"]
pub mod narrowing;
#[path = "../query_binding/mod.rs"]
pub mod query_binding;
#[path = "../query_live_rebind/mod.rs"]
pub mod query_live_rebind;
#[path = "../reconciliation/mod.rs"]
pub mod reconciliation;
#[path = "../state_inventory/mod.rs"]
pub mod state_inventory;

pub mod node_classification;
mod orchestrator;
mod transitions;

pub use node_classification::{
    WorthUiAmbiguousReplacementDenial, WorthUiNodeLifecycleTransition,
    WorthUiNodeReplacementClassification, WorthUiNodeReplacementCounters,
    WorthUiNodeReplacementPlan,
};
pub use orchestrator::WorthUiReplacementLoweringDenial;
pub use transitions::{
    WorthUiReplacementAdmissionBasis, WorthUiReplacementComparisonReady,
    WorthUiReplacementIdentityReady, WorthUiReplacementImpactReady,
    WorthUiReplacementLoweringReady, WorthUiReplacementNarrowingReady,
    WorthUiReplacementNodePlanReady, WorthUiReplacementQueryComparisonReady,
    WorthUiReplacementReconciliationReady,
};
