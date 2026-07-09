use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum PreparationFailureClass {
    PlanningProofInsufficient,
    PacketOverlapDetected,
    ReductionIdentityConflict,
    SerialStrategySelected,
    WorkerEvaluationFailure,
    FragmentCanonicalizationFailure,
    PublicationIsolationViolation,
    ConsumerFailureNonAuthoritative,
}

impl PreparationFailureClass {
    pub(crate) const fn diagnostic_label(self) -> &'static str {
        match self {
            Self::PlanningProofInsufficient => "planning_proof_insufficient",
            Self::PacketOverlapDetected => "packet_overlap_detected",
            Self::ReductionIdentityConflict => "reduction_identity_conflict",
            Self::SerialStrategySelected => "serial_strategy_selected",
            Self::WorkerEvaluationFailure => "worker_evaluation_failure",
            Self::FragmentCanonicalizationFailure => "fragment_canonicalization_failure",
            Self::PublicationIsolationViolation => "publication_isolation_violation",
            Self::ConsumerFailureNonAuthoritative => "consumer_failure_non_authoritative",
        }
    }
}
