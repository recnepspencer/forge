use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum PreparationFailureClass {
    PlanningProofInsufficient,
    PacketOverlapDetected,
    ReductionIdentityConflict,
    FallbackToSerial,
    WorkerEvaluationFailure,
    FragmentCanonicalizationFailure,
    PublicationIsolationViolation,
    ConsumerFailureNonAuthoritative,
}
