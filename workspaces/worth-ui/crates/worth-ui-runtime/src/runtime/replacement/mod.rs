//! Replacement lane — admit → compare → classify → narrow → match → reconcile → rebind → lowering input.

#[path = "../candidate/mod.rs"]
pub mod candidate;
#[path = "../admission/mod.rs"]
pub mod admission;
#[path = "../equivalence/mod.rs"]
pub mod equivalence;
#[path = "../impact/mod.rs"]
pub mod impact;
#[path = "../narrowing/mod.rs"]
pub mod narrowing;
#[path = "../matching/mod.rs"]
pub mod matching;
#[path = "../reconciliation/mod.rs"]
pub mod reconciliation;
#[path = "../query_binding/mod.rs"]
pub mod query_binding;
#[path = "../query_live_rebind/mod.rs"]
pub mod query_live_rebind;
#[path = "../state_inventory/mod.rs"]
pub mod state_inventory;
#[path = "../file_rust_replacement_parity/mod.rs"]
pub mod file_rust_replacement_parity;

pub mod node_classification;
mod orchestrator;
mod transitions;

pub use node_classification::{
    WorthUiAmbiguousReplacementDenial, WorthUiNodeLifecycleTransition,
    WorthUiNodeReplacementClassification, WorthUiNodeReplacementCounters, WorthUiNodeReplacementPlan,
};
pub(crate) use node_classification::WorthUiNodeReplacementClassifier;
pub use orchestrator::WorthUiReplacementLoweringDenial;
pub use transitions::{
    WorthUiReplacementAdmissionBasis, WorthUiReplacementComparisonReady,
    WorthUiReplacementIdentityReady, WorthUiReplacementImpactReady, WorthUiReplacementLoweringReady,
    WorthUiReplacementNarrowingReady, WorthUiReplacementNodePlanReady,
    WorthUiReplacementQueryComparisonReady, WorthUiReplacementReconciliationReady,
};