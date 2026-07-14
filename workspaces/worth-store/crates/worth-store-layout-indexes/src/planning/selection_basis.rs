use crate::access::budget::PlannedCounterEnvelope;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanningCapabilityGrant {
    PointLookup,
    OrderedRange,
    PrefixTraversal,
    BlobStreaming,
    ExactScan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionCandidateEligibility {
    RegistryAdmitted {
        granted_capability: PlanningCapabilityGrant,
        planned_counter_envelope: PlannedCounterEnvelope,
    },
    ExplicitDegradedExactScan {
        planned_counter_envelope: PlannedCounterEnvelope,
        budget_rows: u64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeterministicSelectionRule {
    SoleEligibleCandidate,
    ExplicitDegradedExactScan,
}
