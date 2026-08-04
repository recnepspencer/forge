use serde::Deserialize;
use serde::Serialize;

pub const REQUIRED_RESOURCE_CERTIFICATION_FAMILIES: [ResourceCertificationFamily; 5] = [
    ResourceCertificationFamily::AsyncResourceLifecycleParity,
    ResourceCertificationFamily::OutOfOrderCompletionSupersession,
    ResourceCertificationFamily::AsyncRollbackObservationEquivalence,
    ResourceCertificationFamily::AsyncBranchRestoreReplayEquivalence,
    ResourceCertificationFamily::AsyncInflightBoundedness,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResourceCertificationFamily {
    AsyncResourceLifecycleParity,
    OutOfOrderCompletionSupersession,
    AsyncRollbackObservationEquivalence,
    AsyncBranchRestoreReplayEquivalence,
    AsyncInflightBoundedness,
}
