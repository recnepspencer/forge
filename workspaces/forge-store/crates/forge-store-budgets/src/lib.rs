#![forbid(unsafe_code)]

mod allocation_counters;
mod allocation_denial;
mod allocation_envelope;
mod blob_harness_envelope;
mod counter_strength;

#[cfg(test)]
mod allocation_envelope_tests;
#[cfg(test)]
mod counter_strength_tests;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetAdmissionDecision {
    Admit,
    Defer,
    Deny,
    AdmitDegraded,
}
