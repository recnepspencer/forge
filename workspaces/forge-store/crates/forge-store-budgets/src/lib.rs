#![forbid(unsafe_code)]

mod allocation_counters;
mod allocation_denial;
mod allocation_envelope;

#[cfg(test)]
mod allocation_envelope_tests;

pub use allocation_counters::{
    AllocationCounterSnapshot, AllocationCounters, ScopeAllocationCounters,
};
pub use allocation_denial::AllocationBudgetDenial;
pub use allocation_envelope::{
    AllocationByteBudget, AllocationEnvelopeDeclaration, AllocationEnvelopeDeclarationBuilder,
    AllocationEnvelopeSet, AllocationScope, FixedMetadataReservation,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetAdmissionDecision {
    Admit,
    Defer,
    Deny,
    AdmitDegraded,
}
