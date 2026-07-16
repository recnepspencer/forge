use std::path::Path;

use sha2::{Digest, Sha256};

use worth_store_authority::{
    CurrentAuthorityReadmissionReceipt, RecoveryCutoverAuthorityOwner, RecoveryWriteFencePort,
    RecoveryWriteFenceReceipt, RecoveryWriteFenceReleaseReceipt, StoreCurrentAuthorityWitness,
};
use worth_store_physical_isolation::{
    AtomicRecoveryPublicationReceipt, CurrentPhysicalRoot, RecoveryPublicationOwner,
    ReopenRecoveryPublicationRequest,
};

use crate::{
    OperationalControlStore, OperationalOperationId, OperationalTransitionId,
    PendingRecoveryPublicationHandle, RecoveryPublicationOperationKind,
};

use super::RecoveryCutoverExecutionDenial;

pub(super) struct RecoveredPublishedCore {
    pub(super) operation_id: OperationalOperationId,
    pub(super) fence: RecoveryWriteFenceReceipt,
    pub(super) publication: AtomicRecoveryPublicationReceipt,
    pub(super) source_lease: Option<super::post_verification::RecoveryCutoverSourceLease>,
    pub(super) authority_posture: worth_store_authority::RecoveryAuthorityAdmissionPosture,
    pub(super) admission_policy: worth_store_authority::RecoveryAuthorityAdmissionPolicy,
}

pub(super) struct RecoveredReadmittedCore {
    pub(super) operation_id: OperationalOperationId,
    pub(super) publication: AtomicRecoveryPublicationReceipt,
    pub(super) readmission: CurrentAuthorityReadmissionReceipt,
    pub(super) fence_release: RecoveryWriteFenceReleaseReceipt,
    pub(super) source_lease: Option<super::post_verification::RecoveryCutoverSourceLease>,
}

pub enum RecoveredPendingRecoveryPublication {
    BackupRestore(RecoveredBackupRestorePendingReadmission),
    PointInTimeRecovery(RecoveredPointInTimeRecoveryPendingReadmission),
    Rollback(RecoveredRollbackPendingReadmission),
    AuthorityAffectingRepair(RecoveredAuthorityAffectingRepairPendingReadmission),
}

pub struct RecoveredBackupRestorePendingReadmission(RecoveredPublishedCore);
pub struct RecoveredPointInTimeRecoveryPendingReadmission(RecoveredPublishedCore);
pub struct RecoveredRollbackPendingReadmission(RecoveredPublishedCore);
pub struct RecoveredAuthorityAffectingRepairPendingReadmission(RecoveredPublishedCore);
pub struct RecoveredReadmittedBackupRestoreCurrent(pub(super) Box<RecoveredReadmittedCore>);
pub struct RecoveredReadmittedPointInTimeRecoveryCurrent(Box<RecoveredReadmittedCore>);
pub struct RecoveredReadmittedRollbackCurrent(Box<RecoveredReadmittedCore>);
pub struct RecoveredReadmittedAuthorityAffectingRepairCurrent(Box<RecoveredReadmittedCore>);

macro_rules! recovered_readmission_outcome {
    ($outcome:ident, $readmitted:ident, $rejected:ident) => {
        pub enum $outcome {
            Readmitted($readmitted),
            RejectedByAuthority(super::operation_cutover::$rejected),
        }
    };
}

recovered_readmission_outcome!(
    RecoveredBackupRestoreReadmissionOutcome,
    RecoveredReadmittedBackupRestoreCurrent,
    PublishedBackupRestoreRejectedByAuthority
);
recovered_readmission_outcome!(
    RecoveredPointInTimeRecoveryReadmissionOutcome,
    RecoveredReadmittedPointInTimeRecoveryCurrent,
    PublishedPointInTimeRecoveryRejectedByAuthority
);
recovered_readmission_outcome!(
    RecoveredRollbackReadmissionOutcome,
    RecoveredReadmittedRollbackCurrent,
    PublishedRollbackRejectedByAuthority
);
recovered_readmission_outcome!(
    RecoveredAuthorityAffectingRepairReadmissionOutcome,
    RecoveredReadmittedAuthorityAffectingRepairCurrent,
    PublishedAuthorityAffectingRepairRejectedByAuthority
);

pub(crate) fn recover_pending(
    handle: &PendingRecoveryPublicationHandle,
    publication_directory: &Path,
    current_root: CurrentPhysicalRoot,
    current: &StoreCurrentAuthorityWitness,
    fence_port: &impl RecoveryWriteFencePort,
    source_leases: Option<&worth_store_physical_isolation::RecoverySourceLeaseRegistry>,
) -> Result<RecoveredPendingRecoveryPublication, RecoveryCutoverExecutionDenial> {
    if handle.authority_identity() != current.authority_identity() {
        return Err(RecoveryCutoverExecutionDenial::StaleAuthority);
    }
    let publication =
        RecoveryPublicationOwner::reopen_published(ReopenRecoveryPublicationRequest::new(
            publication_directory,
            current_root,
            handle.publication_identity(),
            handle.publication_plan_fingerprint(),
            handle.candidate_media_identity(),
        ))
        .map_err(RecoveryCutoverExecutionDenial::Publication)?;
    let fence = RecoveryCutoverAuthorityOwner::recover_active_write_fence(
        current,
        handle.fence_identity(),
        handle.fence_plan_fingerprint(),
        handle.cutover_plan_fingerprint(),
        handle.candidate_media_identity(),
        fence_port,
    )
    .map_err(RecoveryCutoverExecutionDenial::Fence)?;
    let source_lease = recover_source_lease(handle, source_leases)?;
    let core = RecoveredPublishedCore {
        operation_id: handle.operation_id().clone(),
        fence,
        publication,
        source_lease,
        authority_posture: handle.authority_posture(),
        admission_policy: handle.admission_policy(),
    };
    Ok(match handle.operation_kind() {
        RecoveryPublicationOperationKind::BackupRestore => {
            RecoveredPendingRecoveryPublication::BackupRestore(
                RecoveredBackupRestorePendingReadmission(core),
            )
        }
        RecoveryPublicationOperationKind::PointInTimeRecovery => {
            RecoveredPendingRecoveryPublication::PointInTimeRecovery(
                RecoveredPointInTimeRecoveryPendingReadmission(core),
            )
        }
        RecoveryPublicationOperationKind::Rollback => {
            RecoveredPendingRecoveryPublication::Rollback(RecoveredRollbackPendingReadmission(core))
        }
        RecoveryPublicationOperationKind::AuthorityAffectingRepair => {
            RecoveredPendingRecoveryPublication::AuthorityAffectingRepair(
                RecoveredAuthorityAffectingRepairPendingReadmission(core),
            )
        }
    })
}

fn recover_source_lease(
    handle: &PendingRecoveryPublicationHandle,
    registry: Option<&worth_store_physical_isolation::RecoverySourceLeaseRegistry>,
) -> Result<
    Option<super::post_verification::RecoveryCutoverSourceLease>,
    RecoveryCutoverExecutionDenial,
> {
    let operation_identity: [u8; 32] =
        Sha256::digest(handle.operation_id().as_str().as_bytes()).into();
    match handle.operation_kind() {
        RecoveryPublicationOperationKind::PointInTimeRecovery => {
            let registry =
                registry.ok_or(RecoveryCutoverExecutionDenial::MissingSourceLeaseRegistry)?;
            match registry.recover_bound(
                operation_identity,
                worth_store_physical_isolation::RecoverySourceLeaseKind::PointInTimeRecovery,
            ).map_err(RecoveryCutoverExecutionDenial::SourceLease)? {
                worth_store_physical_isolation::RecoveredRecoverySourceLease::PointInTimeRecovery(lease) =>
                    Ok(Some(super::post_verification::RecoveryCutoverSourceLease::PointInTime(lease))),
                _ => Err(RecoveryCutoverExecutionDenial::MissingSourceLeaseRegistry),
            }
        }
        RecoveryPublicationOperationKind::Rollback => {
            let registry =
                registry.ok_or(RecoveryCutoverExecutionDenial::MissingSourceLeaseRegistry)?;
            match registry
                .recover_bound(
                    operation_identity,
                    worth_store_physical_isolation::RecoverySourceLeaseKind::Rollback,
                )
                .map_err(RecoveryCutoverExecutionDenial::SourceLease)?
            {
                worth_store_physical_isolation::RecoveredRecoverySourceLease::Rollback(lease) => {
                    Ok(Some(
                        super::post_verification::RecoveryCutoverSourceLease::Rollback(lease),
                    ))
                }
                _ => Err(RecoveryCutoverExecutionDenial::MissingSourceLeaseRegistry),
            }
        }
        RecoveryPublicationOperationKind::BackupRestore
        | RecoveryPublicationOperationKind::AuthorityAffectingRepair => Ok(None),
    }
}

macro_rules! recovered_operation {
    ($pending:ident, $outcome:ident, $readmitted:ident, $rejected:ident,
        $abandoned:ident, $retained:ident) => {
        impl $pending {
            pub fn attempt_readmission(
                self,
                control: &OperationalControlStore,
                transition: OperationalTransitionId,
                current: &StoreCurrentAuthorityWitness,
                fence_port: &impl RecoveryWriteFencePort,
            ) -> Result<$outcome, RecoveryCutoverExecutionDenial> {
                Ok(match super::recovered_publication_disposition::attempt_recovered_readmission(
                    self.0, control, transition, current, fence_port,
                )? {
                    super::recovered_publication_disposition::RecoveredCoreReadmissionOutcome::Readmitted(value) =>
                        $outcome::Readmitted($readmitted(value)),
                    super::recovered_publication_disposition::RecoveredCoreReadmissionOutcome::Rejected(value) =>
                        $outcome::RejectedByAuthority(super::operation_cutover::$rejected(value)),
                })
            }
            pub fn readmit(
                self,
                control: &OperationalControlStore,
                transition: OperationalTransitionId,
                current: &StoreCurrentAuthorityWitness,
                fence_port: &impl RecoveryWriteFencePort,
            ) -> Result<$readmitted, RecoveryCutoverExecutionDenial> {
                match self.attempt_readmission(control, transition, current, fence_port)? {
                    $outcome::Readmitted(value) => Ok(value),
                    $outcome::RejectedByAuthority(value) =>
                        Err(RecoveryCutoverExecutionDenial::Readmission(value.0.denial)),
                }
            }
            pub fn abandon(
                self,
                reason_identity: [u8; 32],
                control: &OperationalControlStore,
                transition: OperationalTransitionId,
                fence_port: &impl RecoveryWriteFencePort,
            ) -> Result<super::operation_cutover::$abandoned, RecoveryCutoverExecutionDenial> {
                super::recovered_publication_disposition::abandon_recovered(
                    self.0, reason_identity, control, transition, fence_port,
                ).map(super::operation_cutover::$abandoned)
            }
            pub fn retain_for_forensics(
                self,
                retention_plan_identity: [u8; 32],
                control: &OperationalControlStore,
                transition: OperationalTransitionId,
                fence_port: &impl RecoveryWriteFencePort,
            ) -> Result<super::operation_cutover::$retained, RecoveryCutoverExecutionDenial> {
                super::recovered_publication_disposition::retain_recovered_for_forensics(
                    self.0, retention_plan_identity, control, transition, fence_port,
                ).map(super::operation_cutover::$retained)
            }
            pub const fn publication(&self) -> &AtomicRecoveryPublicationReceipt {
                &self.0.publication
            }
        }
        impl $readmitted {
            pub const fn publication(&self) -> &AtomicRecoveryPublicationReceipt {
                &self.0.publication
            }
            pub const fn readmission(&self) -> CurrentAuthorityReadmissionReceipt {
                self.0.readmission
            }
            pub const fn fence_release(&self) -> RecoveryWriteFenceReleaseReceipt {
                self.0.fence_release
            }
            pub fn release_source_lease(mut self) -> Result<
                worth_store_physical_isolation::RecoverySourceLeaseReleaseReceipt,
                super::operation_cutover::RecoverySourceLeaseFinalizationDenial,
            > {
                super::publication_disposition::release_terminal_source_lease(
                    &mut self.0.source_lease,
                )
            }
        }
    };
}

recovered_operation!(
    RecoveredBackupRestorePendingReadmission,
    RecoveredBackupRestoreReadmissionOutcome,
    RecoveredReadmittedBackupRestoreCurrent,
    PublishedBackupRestoreRejectedByAuthority,
    PublishedBackupRestoreAbandoned,
    PublishedBackupRestoreRetainedForForensics
);
recovered_operation!(
    RecoveredPointInTimeRecoveryPendingReadmission,
    RecoveredPointInTimeRecoveryReadmissionOutcome,
    RecoveredReadmittedPointInTimeRecoveryCurrent,
    PublishedPointInTimeRecoveryRejectedByAuthority,
    PublishedPointInTimeRecoveryAbandoned,
    PublishedPointInTimeRecoveryRetainedForForensics
);
recovered_operation!(
    RecoveredRollbackPendingReadmission,
    RecoveredRollbackReadmissionOutcome,
    RecoveredReadmittedRollbackCurrent,
    PublishedRollbackRejectedByAuthority,
    PublishedRollbackAbandoned,
    PublishedRollbackRetainedForForensics
);
recovered_operation!(
    RecoveredAuthorityAffectingRepairPendingReadmission,
    RecoveredAuthorityAffectingRepairReadmissionOutcome,
    RecoveredReadmittedAuthorityAffectingRepairCurrent,
    PublishedAuthorityAffectingRepairRejectedByAuthority,
    PublishedAuthorityAffectingRepairAbandoned,
    PublishedAuthorityAffectingRepairRetainedForForensics
);
