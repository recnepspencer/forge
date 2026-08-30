mod artifact_scope;
mod rejection;
mod untrusted_artifact;
pub(crate) mod validated;
mod validation_record;
mod version_window;

pub use artifact_scope::{
    CheckpointStreamHeaderScopeIdentity, PhysicalArtifactScope, PhysicalArtifactScopeDenial,
};
pub use rejection::{
    IndeterminatePhysicalIntegrityCause, IndeterminatePhysicalIntegrityPosture,
    PhysicalIntegrityRejection, PhysicalIntegrityVersionAxis, UnknownPhysicalIntegrityCause,
    UnknownPhysicalIntegrityPosture, UnsupportedPhysicalIntegrityVersion,
};
pub use untrusted_artifact::UntrustedPhysicalArtifact;
pub use validated::{
    IntegrityValidatedCheckpointBinding, IntegrityValidatedCheckpointBindingCompaction,
    IntegrityValidatedCheckpointDirtyBasis, IntegrityValidatedCheckpointFooter,
    IntegrityValidatedCheckpointStreamHeader, IntegrityValidatedCurrentRootSelector,
    IntegrityValidatedExtentChunkFrame, IntegrityValidatedExtentManifest,
    IntegrityValidatedFreeSpaceHeader, IntegrityValidatedFreeSpaceMembershipBlock,
    IntegrityValidatedPageFrame, IntegrityValidatedPhysicalWorkObligation,
    IntegrityValidatedPreviousRootSelector, IntegrityValidatedRootManifest,
    IntegrityValidatedWalFrame,
};
pub use validation_record::{
    PhysicalIntegrityValidationDigest, PhysicalIntegrityValidationMechanism,
    PhysicalIntegrityValidationRecord,
};
pub use version_window::{
    PhysicalIntegrityArtifactVersionAdapter, PhysicalIntegrityEnvelopeVersionAdapter,
    PhysicalIntegritySupportedVersion, PhysicalIntegrityVersionWindowOutcome,
};
