use crate::{PhysicalOrderingContractDenial, PhysicalReadProtectedFootprintBasis};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalPublicationDenial {
    InPlaceReachableOverwrite,
    MissingOldReachability,
    MissingReachabilityEvidence,
    StaleRootPublicationEpoch,
    StaleManifestPublicationEpoch,
    WeakOrdering(PhysicalOrderingContractDenial),
    RootPublicationValidationRootMismatch,
    NewRootPublicationProofMismatch,
    CheckpointReceiptIsNotCopyOnWritePublicationAuthority,
    PublicationWithoutReadiness,
    MixedTreeAfterCrash,
    UnexpectedRecoveredRoot,
    IdentityReuseWithoutCrashStableFence,
    IdentityReuseWithoutGenerationAdvance,
    IdentityReuseOwnerMismatch,
    ReclaimBeforeReadPlanRelease {
        old_reachability: PhysicalReadProtectedFootprintBasis,
    },
}

impl From<PhysicalOrderingContractDenial> for PhysicalPublicationDenial {
    fn from(value: PhysicalOrderingContractDenial) -> Self {
        Self::WeakOrdering(value)
    }
}
