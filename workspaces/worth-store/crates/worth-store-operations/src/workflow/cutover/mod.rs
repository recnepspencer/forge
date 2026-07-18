mod frontier;
mod operation_cutover;
mod post_verification;
mod protocol;
mod publication_disposition;
mod publication_terminal;
mod readmitted_source_lease;
mod recovered_publication;
mod recovered_publication_disposition;
mod restore_drill_certification;

pub use frontier::{
    CurrentRecoveryAuthoritySnapshot, RecoveryAuthorityDelta, RecoveryAuthorityFrontier,
    RecoveryCutoverDenial,
};
pub use operation_cutover::{
    AuthorityAffectingRepairReadmissionOutcome, AuthorizedAuthorityAffectingRepairCutover,
    AuthorizedBackupRestoreCutover, AuthorizedPointInTimeRecoveryCutover,
    AuthorizedRollbackCutover, BackupRestoreReadmissionOutcome,
    FencedAuthorityAffectingRepairCutover, FencedBackupRestoreCutover,
    FencedPointInTimeRecoveryCutover, FencedRollbackCutover,
    LoweredAuthorityAffectingRepairCutoverPlanDag, LoweredBackupRestoreCutoverPlanDag,
    LoweredPointInTimeRecoveryCutoverPlanDag, LoweredRollbackCutoverPlanDag,
    PointInTimeRecoveryReadmissionOutcome, PublishedAuthorityAffectingRepairAbandoned,
    PublishedAuthorityAffectingRepairPendingReadmission,
    PublishedAuthorityAffectingRepairRejectedByAuthority,
    PublishedAuthorityAffectingRepairRetainedForForensics, PublishedBackupRestoreAbandoned,
    PublishedBackupRestorePendingReadmission, PublishedBackupRestoreRejectedByAuthority,
    PublishedBackupRestoreRetainedForForensics, PublishedPointInTimeRecoveryAbandoned,
    PublishedPointInTimeRecoveryPendingReadmission,
    PublishedPointInTimeRecoveryRejectedByAuthority,
    PublishedPointInTimeRecoveryRetainedForForensics, PublishedRollbackAbandoned,
    PublishedRollbackPendingReadmission, PublishedRollbackRejectedByAuthority,
    PublishedRollbackRetainedForForensics, ReadmittedAuthorityAffectingRepairCurrent,
    ReadmittedBackupRestoreCurrent, ReadmittedPointInTimeRecoveryCurrent,
    ReadmittedRollbackCurrent, RecoverySourceLeaseFinalizationDenial, RollbackReadmissionOutcome,
};
pub use post_verification::{
    PostVerifiedAuthorityAffectingRepair, PostVerifiedBackupRestore,
    PostVerifiedPointInTimeRecovery, PostVerifiedRollback,
    ResolvedAuthorityAffectingRepairCutoverCandidate, ResolvedBackupRestoreCutoverCandidate,
    ResolvedPointInTimeRecoveryCutoverCandidate, ResolvedRollbackCutoverCandidate,
};
pub use protocol::RecoveryCutoverExecutionDenial;
pub use readmitted_source_lease::CompletedRetainedAuthorityRollback;
pub(crate) use recovered_publication::recover_pending;
pub use recovered_publication::{
    RecoveredAuthorityAffectingRepairPendingReadmission,
    RecoveredAuthorityAffectingRepairReadmissionOutcome, RecoveredBackupRestorePendingReadmission,
    RecoveredBackupRestoreReadmissionOutcome, RecoveredPendingRecoveryPublication,
    RecoveredPointInTimeRecoveryPendingReadmission, RecoveredPointInTimeRecoveryReadmissionOutcome,
    RecoveredReadmittedAuthorityAffectingRepairCurrent, RecoveredReadmittedBackupRestoreCurrent,
    RecoveredReadmittedPointInTimeRecoveryCurrent, RecoveredReadmittedRollbackCurrent,
    RecoveredRollbackPendingReadmission, RecoveredRollbackReadmissionOutcome,
};
pub use restore_drill_certification::{
    RestoreDrillCertification, RestoreDrillCertificationDenial, RestoreDrillExpectation,
};

#[derive(Debug, Clone, Copy)]
pub(crate) struct BackupRestoreCutoverOperation;
#[derive(Debug, Clone, Copy)]
pub(crate) struct PointInTimeRecoveryCutoverOperation;
#[derive(Debug, Clone, Copy)]
pub(crate) struct RollbackCutoverOperation;
#[derive(Debug, Clone, Copy)]
pub(crate) struct AuthorityAffectingRepairCutoverOperation;
