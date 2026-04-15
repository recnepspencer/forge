mod authority;
mod backend;
mod evidence;
mod facade;
mod failure;
mod media;
mod modes;
mod publication;
mod recovery;
mod snapshot;
mod wal;

pub use authority::{
    AuthoritativeBranchHeadRecord, AuthoritativeExportBundle, AuthoritativeExportRestoreRequest,
    CanonicalizedCommitEnvelope, FetchedAuthoritativeCommit, PersistedAuthoritativeCommit,
    RawRuntimeCommitEnvelope, VerifiedAuthoritativeAppend,
};
pub use evidence::{
    AbsentModeLaneEvidence, CanonicalizationMetrics, CheckpointAuthorityReport,
    Milestone1CertificationBundle, Milestone1SemanticCertificationEvidence,
    Milestone2CertificationBundle, Milestone35CertificationBundle, Milestone36CertificationBundle,
    Milestone3CertificationBundle, Milestone4CertificationBundle, ObservedModeFailure,
    ObservedPublicationFailure, ObservedRecoveryFailure, ObservedRecoveryFailure356,
    OperatingModeContractMatrix, OperatingModeCounterSnapshot, OperatingModeLane,
    PersistedModeLaneEvidence, StoreCounterSnapshot,
};
pub use facade::{ForgeStore, ForgeStoreBuilder};
pub use failure::{StoreError, StoreErrorKind};
pub use media::{DurabilityBarrierClass, DurableBackendFamily, DurableMediaReport};
pub use modes::{
    AbsentModeSemanticEvidence, AbsentRuntimeWitness, AcknowledgedDurableCommit,
    DurableModeBuilder, DurableMutationRequest, DurableRecoveryHandle, DurableStoreHandle,
    EmbeddedCheckpointClassification, EmbeddedCheckpointPersistenceReceipt, EmbeddedModeBuilder,
    EmbeddedStoreHandle, ExternalRuntimeCheckpointEnvelope, ExternalRuntimeCommitEnvelope,
};
pub use publication::{
    ObservedPublicationFamilyState, PublicationBarrierContract, PublicationClassification,
    PublicationFamily, PublicationState, PublicationStrategy, PublicationWriteOutcome,
};
pub use recovery::{
    BackupRestoreCompatibilityReport, BackupRestoreIncompatibility,
    BackupRestoreIncompatibilityKind, DegradedStateReport, DurableDegradedRecovery,
    DurableRecoveryDegradedKind, DurableRecoveryOutcome, DurableRecoverySourceSummary,
    DurableRetryResolution, MaintenanceArtifactFamily, MaintenanceRecoveryDisposition,
    MaintenanceRecoveryEntry, MaintenanceRecoveryReport, ObservedSnapshotVersionTuple,
    RecoveryOperatorAction, RecoveryOperatorActionKind, RecoveryOperatorDisposition,
    RecoveryQuarantineScope, RecoverySourceKind, RecoverySourceReport, RecoveryStatusReport,
    SnapshotMaintenanceRecoveryAction, SnapshotMaintenanceRecoveryReport,
};
pub use snapshot::{
    PublishedSnapshotHandle, SnapshotCaptureRequest, SnapshotId, SnapshotImageBundle,
    SnapshotReadMode, SnapshotReadRequest, SnapshotReadResult, SnapshotRestoreOutcome,
    SnapshotRestorePlan, SnapshotRestoreRequest,
};
pub use wal::{DurableMutationId, DurablePublicationPhase, RecoveryDecisionClass};

#[cfg(test)]
mod tests;
