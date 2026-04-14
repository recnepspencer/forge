mod authority;
mod backend;
mod evidence;
mod facade;
mod failure;
mod modes;
mod publication;
mod recovery;
mod snapshot;
mod wal;

pub use authority::{
    AuthoritativeBranchHeadRecord, AuthoritativeExportBundle, CanonicalizedCommitEnvelope,
    FetchedAuthoritativeCommit, PersistedAuthoritativeCommit, RawRuntimeCommitEnvelope,
    VerifiedAuthoritativeAppend,
};
pub use evidence::{
    AbsentModeLaneEvidence, CanonicalizationMetrics, CheckpointAuthorityReport,
    Milestone1CertificationBundle, Milestone1SemanticCertificationEvidence,
    Milestone2CertificationBundle, Milestone3CertificationBundle, Milestone4CertificationBundle,
    ObservedModeFailure, ObservedRecoveryFailure, OperatingModeContractMatrix,
    OperatingModeCounterSnapshot, OperatingModeLane, PersistedModeLaneEvidence,
    StoreCounterSnapshot,
};
pub use facade::{ForgeStore, ForgeStoreBuilder};
pub use failure::{StoreError, StoreErrorKind};
pub use modes::{
    AbsentModeSemanticEvidence, AbsentRuntimeWitness, AcknowledgedDurableCommit,
    DurableModeBuilder, DurableMutationRequest, DurableRecoveryHandle, DurableStoreHandle,
    EmbeddedCheckpointClassification, EmbeddedCheckpointPersistenceReceipt, EmbeddedModeBuilder,
    EmbeddedStoreHandle, ExternalRuntimeCheckpointEnvelope, ExternalRuntimeCommitEnvelope,
};
pub use recovery::{DurableRecoveryOutcome, DurableRetryResolution};
pub use snapshot::{
    PublishedSnapshotHandle, SnapshotCaptureRequest, SnapshotId, SnapshotImageBundle,
    SnapshotReadMode, SnapshotReadRequest, SnapshotReadResult, SnapshotRestoreOutcome,
};
pub use wal::{DurableMutationId, DurablePublicationPhase, RecoveryDecisionClass};

#[cfg(test)]
mod tests;
