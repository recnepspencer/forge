use worth_store::physical_runtime::{
    PhysicalMutationAcknowledgment, PhysicalMutationExecutedBoundaryEvidence,
    PhysicalMutationPerformanceEvidence,
};
use worth_store_budgets::CounterEvidenceStrength;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct S6FlushDurabilityEvidenceRow {
    executed: PhysicalMutationExecutedBoundaryEvidence,
    performance: PhysicalMutationPerformanceEvidence,
}

impl S6FlushDurabilityEvidenceRow {
    pub fn from_physical_acknowledgment(acknowledgment: &PhysicalMutationAcknowledgment) -> Self {
        Self {
            executed: acknowledgment.executed_boundary_evidence(),
            performance: acknowledgment.performance_evidence(),
        }
    }

    pub const fn executed_boundary(&self) -> PhysicalMutationExecutedBoundaryEvidence {
        self.executed
    }

    pub const fn performance(&self) -> PhysicalMutationPerformanceEvidence {
        self.performance
    }

    pub const fn counter_strength(&self) -> CounterEvidenceStrength {
        CounterEvidenceStrength::Exact
    }
}
