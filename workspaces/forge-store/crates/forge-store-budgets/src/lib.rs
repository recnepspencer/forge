#![forbid(unsafe_code)]

mod allocation_counters;
mod allocation_denial;
mod allocation_envelope;
mod blob_harness_envelope;
mod counter_strength;
mod layout_plan;
mod pre_execution;

#[cfg(test)]
mod allocation_envelope_tests;
#[cfg(test)]
mod counter_strength_tests;
#[cfg(test)]
mod pre_execution_tests;

pub use allocation_counters::{
    AllocationCounterSnapshot, AllocationCounters, ScopeAllocationCounters,
};
pub use allocation_denial::AllocationBudgetDenial;
pub use allocation_envelope::{
    AllocationByteBudget, AllocationEnvelopeDeclaration, AllocationEnvelopeDeclarationBuilder,
    AllocationEnvelopeSet, AllocationScope, FixedMetadataReservation,
};
pub use blob_harness_envelope::{BlobHarnessEnvelopeDeclaration, BlobHarnessEnvelopeProfile};
pub use counter_strength::CounterEvidenceStrength;
pub use layout_plan::{
    layout_plan_budget_admission, AdmittedLayoutPlanBudget, LayoutPlanBudget,
    LayoutPlanBudgetAdmission, LayoutPlanBudgetDenial, LayoutPlanBudgetOutcome,
    LayoutPlanBudgetScope, LayoutPlanWork,
};
pub use pre_execution::{
    pre_execution_budget_admission, PreExecutionBudgetAdmission,
    PreExecutionBudgetAdmissionOutcome, PreExecutionBudgetAdmissionReceipt,
    PreExecutionBudgetDenial, PreExecutionBudgetEnvelope, PreExecutionBudgetRequest,
    PreExecutionBudgetScope,
};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetAdmissionDecision {
    Admit,
    Defer,
    Deny,
    AdmitDegraded,
}
