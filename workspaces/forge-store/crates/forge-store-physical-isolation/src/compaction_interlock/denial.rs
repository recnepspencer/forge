use crate::{
    LatchAcquisitionDenial, ManifestEpoch, PhysicalReadProtectedFootprintBasis, RootEpoch,
};
use forge_store_recovery_physics::CompactionArtifactResidueReason;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompactionReadInterlockDenial {
    EmptyCandidateRangeSet,
    QuarantinedCandidateRange,
    SourceEvidenceMismatch,
    StaleCompactionSourceEpoch {
        expected: RootEpoch,
        observed: RootEpoch,
    },
    InPlaceOverwriteOfProtectedStructure,
    MissingOldRootPreservation,
    BackendResidueCandidateSelection(CompactionArtifactResidueReason),
    EarlyReclaimBeforeReadRelease {
        protected: PhysicalReadProtectedFootprintBasis,
    },
    ExpectedMutationLaneDenialNotProduced,
    StaleEpochReuse {
        source_epoch: RootEpoch,
        reused_epoch: RootEpoch,
    },
    StaleManifestEpochReuse {
        source_epoch: ManifestEpoch,
        reused_epoch: ManifestEpoch,
    },
    PublicationRootMismatch,
    PublicationReachabilityFootprintMismatch {
        protected: PhysicalReadProtectedFootprintBasis,
        preserved: PhysicalReadProtectedFootprintBasis,
    },
    LatchAcquisition(LatchAcquisitionDenial),
    MixedRootDuringCompaction,
    PreCutoverReadReceiptMismatch,
    PostCutoverReadReceiptMismatch,
    LsmTombstoneRetentionMissing,
    LsmPublicationBindingMissing,
    LsmCounterBindingMismatch,
    LsmPhysicalTargetMismatch,
}
