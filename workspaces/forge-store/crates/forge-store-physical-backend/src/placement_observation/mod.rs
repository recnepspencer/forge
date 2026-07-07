mod chunk_write;
mod cleanup_execution;
mod external_recovery;
#[cfg(test)]
mod external_recovery_tests;
mod manifest_traversal;
mod recovery_probe;
mod residue;
mod residue_scan;

pub use chunk_write::{
    BlobBackendChunkWriteObservation, BlobBackendChunkWriteObservationKind,
    BlobBackendChunkWriteSession,
};
pub use cleanup_execution::{
    ExternalPlacementCleanupExecutionError, ExternalPlacementCleanupObservation,
    ExternalPlacementCleanupRequest, ExternalPlacementCleanupSession,
    PhysicalStoreExternalPlacementCleanupExecutor, StoreOwnedExternalPlacementCleanup,
};
pub use external_recovery::{
    ExternalPlacementCleanupReceipt, ExternalPlacementMissingDenial,
    ExternalPlacementOrphanScanReceipt, ExternalPlacementRecoverabilityDenial,
    ExternalPlacementRecoveryProbe, StoreExternalPlacementRecoverabilityEvidence,
};
pub use manifest_traversal::{
    BlobPhysicalManifestTraversalObservation, BlobPhysicalManifestTraversalRequest,
    BlobPhysicalManifestTraversalSession, PhysicalStoreBlobManifestTraverser,
    StoreOwnedBlobPhysicalManifestTraversal,
};
pub use recovery_probe::{
    ExternalPlacementRecoveryProbeExecutionError, ExternalPlacementRecoveryProbeObservation,
    ExternalPlacementRecoveryProbeRequest, ExternalPlacementRecoveryProbeSession,
    PhysicalStoreExternalPlacementRecoveryProber, StoreOwnedExternalPlacementRecoveryProbe,
};
pub use residue::{
    BlobBackendResidueObservation, BlobBackendResidueObservationKind,
    BlobPhysicalManifestObservation, BlobPhysicalManifestObservationDenial,
    BlobPhysicalManifestValidation,
};
pub use residue_scan::{
    BlobBackendResidueScanObservation, BlobBackendResidueScanRequest,
    BlobBackendResidueScanSession, PhysicalStoreBlobResidueScanner,
    StoreOwnedBlobBackendResidueScan,
};
