//! Activation lane — staging, frame gate, atomic swap.

#[path = "../activation_staging/mod.rs"]
pub mod activation_staging;
#[path = "../frame_activation_gate/mod.rs"]
pub mod frame_activation_gate;

pub(crate) mod committed_allocation_attempt;
pub use committed_allocation_attempt::{
    UiCommittedAllocationActivationCounterExhaustion, UiCommittedAllocationActivationCounters,
    UiCommittedAllocationActivationDenial, UiCommittedAllocationActivationDenialEvidence,
    UiCommittedAllocationActivationDenialReason, UiCommittedAllocationActivationInspection,
    UiCommittedAllocationActivationInspectionDenialKind,
    UiCommittedAllocationActivationInspectionOutcome, WorthUiPlanSwapReceipt,
    WorthUiPriorValidPlanObservation,
};
mod gate;

pub use gate::{
    WorthUiAllocationCatalogActivationDenial, WorthUiAllocationCatalogPreparationStage,
};
