use crate::budget::S8PlannedCounterEnvelope;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8PlanningCapabilityGrant {
    PointLookup,
    OrderedRange,
    PrefixTraversal,
    BlobStreaming,
    ExactScan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8SelectionCandidateEligibility {
    RegistryAdmitted {
        granted_capability: S8PlanningCapabilityGrant,
        planned_counter_envelope: S8PlannedCounterEnvelope,
    },
    ExplicitDegradedExactScan {
        planned_counter_envelope: S8PlannedCounterEnvelope,
        budget_rows: u64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8DeterministicSelectionRule {
    SoleEligibleCandidate,
    OrderedIndexReadsPreferBTree,
    BufferedOrTraversalReadsPreferLsm,
    ExplicitDegradedExactScan,
}
