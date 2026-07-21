//! Activation lane — staging, frame gate, atomic swap.

pub mod activation_staging;
pub mod frame_activation_gate;

mod application_publication;
mod catalog_activation_denial;
mod catalog_activation_input;
pub(crate) mod committed_allocation_attempt;
#[cfg(test)]
pub use committed_allocation_attempt::UiCommittedAllocationActivationInspectionOutcome;
pub use committed_allocation_attempt::{
    UiCommittedAllocationActivationCounters, UiCommittedAllocationActivationDenial,
    UiCommittedAllocationActivationDenialReason, WorthUiPlanSwapReceipt,
    WorthUiPriorValidPlanObservation,
};
mod gate;
mod query_aware_plan_outcome;
mod semantic_no_op;
#[cfg(test)]
mod test_frame_boundary;

pub(crate) use application_publication::WorthUiPreparedApplicationPublication;
pub use catalog_activation_denial::{
    WorthUiAllocationCatalogActivationDenial, WorthUiAllocationCatalogPreparationStage,
};
pub(crate) use catalog_activation_input::UiAllocationCatalogDeltaActivationInput;
pub(crate) use query_aware_plan_outcome::WorthUiQueryAwarePlanOutcome;
pub use semantic_no_op::{
    WorthUiNoOpProvenancePosture, WorthUiNoOpQueryPosture, WorthUiSemanticNoOpReceipt,
    WorthUiSemanticNoOpWork,
};

pub(crate) fn certification_precommit_interruption(label: &'static str) -> bool {
    #[cfg(any(test, feature = "certification-support"))]
    {
        crate::certification_support::interrupt_if_armed(label)
    }
    #[cfg(not(any(test, feature = "certification-support")))]
    {
        let _ = label;
        false
    }
}
