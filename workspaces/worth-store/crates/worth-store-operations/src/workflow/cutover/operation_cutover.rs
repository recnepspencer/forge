use worth_store_authority::{
    CurrentAuthorityReadmissionReceipt, RecoveryWriteFencePort, RecoveryWriteFenceReleaseReceipt,
    StoreCurrentAuthorityWitness,
};
use worth_store_physical_isolation::AtomicRecoveryPublicationReceipt;

use crate::authorization::AuthorizationReplayPolicy;
use crate::owner_plan_dag::DestructiveOperationKind;
use crate::{
    AuthorizationDenial, AuthorizationRevocationObservation, ExternalOperatorAssertion,
    OperationalAuthorizationPort, OperationalControlStore, OperationalTransitionId,
};

use super::protocol::{
    authorize, lower, publish, ready, AuthorizedCutoverCore, FencedCutoverCore, LoweredCutoverCore,
    PublishedCutoverCore, ReadmittedCutoverCore,
};
use super::publication_disposition::{
    abandon, attempt_readmission, release_terminal_source_lease, retain_for_forensics,
    CoreReadmissionOutcome, PublishedAbandonedCore, PublishedRejectedCore,
    PublishedRetainedForForensicsCore,
};
use super::{
    AuthorityAffectingRepairCutoverOperation, BackupRestoreCutoverOperation,
    PointInTimeRecoveryCutoverOperation, RecoveryCutoverDenial, RecoveryCutoverExecutionDenial,
    RollbackCutoverOperation,
};

pub struct LoweredBackupRestoreCutoverPlanDag(LoweredCutoverCore<BackupRestoreCutoverOperation>);
pub struct LoweredPointInTimeRecoveryCutoverPlanDag(
    LoweredCutoverCore<PointInTimeRecoveryCutoverOperation>,
);
pub struct LoweredRollbackCutoverPlanDag(LoweredCutoverCore<RollbackCutoverOperation>);
pub struct AuthorizedBackupRestoreCutover(AuthorizedCutoverCore<BackupRestoreCutoverOperation>);
pub struct AuthorizedPointInTimeRecoveryCutover(
    AuthorizedCutoverCore<PointInTimeRecoveryCutoverOperation>,
);
pub struct AuthorizedRollbackCutover(AuthorizedCutoverCore<RollbackCutoverOperation>);
pub struct FencedBackupRestoreCutover(FencedCutoverCore<BackupRestoreCutoverOperation>);
pub struct FencedPointInTimeRecoveryCutover(FencedCutoverCore<PointInTimeRecoveryCutoverOperation>);
pub struct FencedRollbackCutover(FencedCutoverCore<RollbackCutoverOperation>);
pub struct PublishedBackupRestorePendingReadmission(
    PublishedCutoverCore<BackupRestoreCutoverOperation>,
);
pub struct PublishedPointInTimeRecoveryPendingReadmission(
    PublishedCutoverCore<PointInTimeRecoveryCutoverOperation>,
);
pub struct PublishedRollbackPendingReadmission(PublishedCutoverCore<RollbackCutoverOperation>);
pub struct ReadmittedBackupRestoreCurrent(Box<ReadmittedCutoverCore>);
pub struct ReadmittedPointInTimeRecoveryCurrent(pub(super) Box<ReadmittedCutoverCore>);
pub struct ReadmittedRollbackCurrent(pub(super) Box<ReadmittedCutoverCore>);
pub struct LoweredAuthorityAffectingRepairCutoverPlanDag(
    LoweredCutoverCore<AuthorityAffectingRepairCutoverOperation>,
);
pub struct AuthorizedAuthorityAffectingRepairCutover(
    AuthorizedCutoverCore<AuthorityAffectingRepairCutoverOperation>,
);
pub struct FencedAuthorityAffectingRepairCutover(
    FencedCutoverCore<AuthorityAffectingRepairCutoverOperation>,
);
pub struct PublishedAuthorityAffectingRepairPendingReadmission(
    PublishedCutoverCore<AuthorityAffectingRepairCutoverOperation>,
);
pub struct ReadmittedAuthorityAffectingRepairCurrent(Box<ReadmittedCutoverCore>);

macro_rules! publication_terminal_types {
    ($outcome:ident, $rejected:ident, $abandoned:ident, $retained:ident, $readmitted:ident) => {
        pub enum $outcome {
            Readmitted($readmitted),
            RejectedByAuthority($rejected),
        }
        pub struct $rejected(pub(super) Box<PublishedRejectedCore>);
        pub struct $abandoned(pub(super) PublishedAbandonedCore);
        pub struct $retained(pub(super) PublishedRetainedForForensicsCore);
    };
}

publication_terminal_types!(
    BackupRestoreReadmissionOutcome,
    PublishedBackupRestoreRejectedByAuthority,
    PublishedBackupRestoreAbandoned,
    PublishedBackupRestoreRetainedForForensics,
    ReadmittedBackupRestoreCurrent
);
publication_terminal_types!(
    PointInTimeRecoveryReadmissionOutcome,
    PublishedPointInTimeRecoveryRejectedByAuthority,
    PublishedPointInTimeRecoveryAbandoned,
    PublishedPointInTimeRecoveryRetainedForForensics,
    ReadmittedPointInTimeRecoveryCurrent
);
publication_terminal_types!(
    RollbackReadmissionOutcome,
    PublishedRollbackRejectedByAuthority,
    PublishedRollbackAbandoned,
    PublishedRollbackRetainedForForensics,
    ReadmittedRollbackCurrent
);
publication_terminal_types!(
    AuthorityAffectingRepairReadmissionOutcome,
    PublishedAuthorityAffectingRepairRejectedByAuthority,
    PublishedAuthorityAffectingRepairAbandoned,
    PublishedAuthorityAffectingRepairRetainedForForensics,
    ReadmittedAuthorityAffectingRepairCurrent
);

#[derive(Debug)]
pub enum RecoverySourceLeaseFinalizationDenial {
    MissingOrWrongLease,
    Isolation(worth_store_physical_isolation::RecoverySourceLeaseDenial),
}

macro_rules! operation_cutover {
    ($resolved:ty, $lowered:ident, $authorized:ident, $fenced:ident, $published:ident,
        $outcome:ident, $rejected:ident, $abandoned:ident, $retained:ident,
        $readmitted:ident, $marker:ty, $operation:expr) => {
        impl $resolved {
            pub fn lower_cutover(
                self,
                current: &StoreCurrentAuthorityWitness,
            ) -> Result<$lowered, RecoveryCutoverDenial> {
                lower::<$marker>(self.0, current, $operation).map($lowered)
            }
        }
        impl $lowered {
            pub const fn explanation(&self) -> &crate::CanonicalOwnerPlanDagExplanation {
                &self.0.explanation
            }
            #[allow(clippy::too_many_arguments)]
            pub fn authorize(
                self,
                port: &impl OperationalAuthorizationPort,
                assertion: &ExternalOperatorAssertion,
                requested_at: u64,
                expires_at: u64,
                replay_policy: AuthorizationReplayPolicy,
                revocation: AuthorizationRevocationObservation,
            ) -> Result<$authorized, AuthorizationDenial> {
                authorize(
                    self.0,
                    port,
                    assertion,
                    requested_at,
                    expires_at,
                    replay_policy,
                    revocation,
                )
                .map($authorized)
            }
        }
        impl $authorized {
            pub fn establish_write_fence(
                self,
                control: &OperationalControlStore,
                transition: OperationalTransitionId,
                current: &StoreCurrentAuthorityWitness,
                fence_port: &impl RecoveryWriteFencePort,
                observed_at: u64,
                revocation: AuthorizationRevocationObservation,
            ) -> Result<$fenced, RecoveryCutoverExecutionDenial> {
                ready(
                    self.0,
                    control,
                    transition,
                    current,
                    fence_port,
                    observed_at,
                    revocation,
                )
                .map($fenced)
            }
        }
        impl $fenced {
            pub fn publish(
                self,
                control: &impl crate::OperationalControlStorePort,
                transition: OperationalTransitionId,
            ) -> Result<$published, RecoveryCutoverExecutionDenial> {
                publish(self.0, control, transition).map($published)
            }
        }
        impl $published {
            pub fn attempt_readmission(
                self,
                control: &OperationalControlStore,
                transition: OperationalTransitionId,
                current: &StoreCurrentAuthorityWitness,
                fence_port: &impl RecoveryWriteFencePort,
            ) -> Result<$outcome, RecoveryCutoverExecutionDenial> {
                Ok(
                    match attempt_readmission(self.0, control, transition, current, fence_port)? {
                        CoreReadmissionOutcome::Readmitted(value) => {
                            $outcome::Readmitted($readmitted(value))
                        }
                        CoreReadmissionOutcome::Rejected(value) => {
                            $outcome::RejectedByAuthority($rejected(value))
                        }
                    },
                )
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
                    $outcome::RejectedByAuthority(value) => {
                        Err(RecoveryCutoverExecutionDenial::Readmission(value.0.denial))
                    }
                }
            }
            pub fn abandon(
                self,
                reason_identity: [u8; 32],
                control: &OperationalControlStore,
                transition: OperationalTransitionId,
                fence_port: &impl RecoveryWriteFencePort,
            ) -> Result<$abandoned, RecoveryCutoverExecutionDenial> {
                abandon(self.0, reason_identity, control, transition, fence_port).map($abandoned)
            }
            pub fn retain_for_forensics(
                self,
                retention_plan_identity: [u8; 32],
                control: &OperationalControlStore,
                transition: OperationalTransitionId,
                fence_port: &impl RecoveryWriteFencePort,
            ) -> Result<$retained, RecoveryCutoverExecutionDenial> {
                retain_for_forensics(
                    self.0,
                    retention_plan_identity,
                    control,
                    transition,
                    fence_port,
                )
                .map($retained)
            }
            pub const fn publication(&self) -> &AtomicRecoveryPublicationReceipt {
                &self.0.publication
            }
        }
        impl $rejected {
            pub const fn operation_id(&self) -> &crate::OperationalOperationId {
                &self.0.operation_id
            }
            pub const fn denial(
                &self,
            ) -> worth_store_authority::RecoveryAuthorityReadmissionDenial {
                self.0.denial
            }
            pub const fn publication(&self) -> &AtomicRecoveryPublicationReceipt {
                &self.0.publication
            }
            pub const fn observed_authority(
                &self,
            ) -> worth_store_authority::StoreCurrentAuthorityIdentity {
                self.0.observed_authority
            }
            pub const fn fence_release(&self) -> RecoveryWriteFenceReleaseReceipt {
                self.0.fence_release
            }
            pub fn release_source_lease(
                mut self,
            ) -> Result<
                worth_store_physical_isolation::RecoverySourceLeaseReleaseReceipt,
                RecoverySourceLeaseFinalizationDenial,
            > {
                release_terminal_source_lease(&mut self.0.source_lease)
            }
        }
        impl $abandoned {
            pub const fn publication(&self) -> &AtomicRecoveryPublicationReceipt {
                &self.0.publication
            }
            pub const fn reason_identity(&self) -> [u8; 32] {
                self.0.reason_identity
            }
            pub const fn fence_release(&self) -> RecoveryWriteFenceReleaseReceipt {
                self.0.fence_release
            }
            pub fn release_source_lease(
                mut self,
            ) -> Result<
                worth_store_physical_isolation::RecoverySourceLeaseReleaseReceipt,
                RecoverySourceLeaseFinalizationDenial,
            > {
                release_terminal_source_lease(&mut self.0.source_lease)
            }
        }
        impl $retained {
            pub const fn publication(&self) -> &AtomicRecoveryPublicationReceipt {
                &self.0.publication
            }
            pub const fn retention_plan_identity(&self) -> [u8; 32] {
                self.0.retention_plan_identity
            }
            pub const fn fence_release(&self) -> RecoveryWriteFenceReleaseReceipt {
                self.0.fence_release
            }
            pub fn release_source_lease(
                mut self,
            ) -> Result<
                worth_store_physical_isolation::RecoverySourceLeaseReleaseReceipt,
                RecoverySourceLeaseFinalizationDenial,
            > {
                release_terminal_source_lease(&mut self.0.source_lease)
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
        }
    };
}

operation_cutover!(
    super::ResolvedBackupRestoreCutoverCandidate,
    LoweredBackupRestoreCutoverPlanDag,
    AuthorizedBackupRestoreCutover,
    FencedBackupRestoreCutover,
    PublishedBackupRestorePendingReadmission,
    BackupRestoreReadmissionOutcome,
    PublishedBackupRestoreRejectedByAuthority,
    PublishedBackupRestoreAbandoned,
    PublishedBackupRestoreRetainedForForensics,
    ReadmittedBackupRestoreCurrent,
    BackupRestoreCutoverOperation,
    DestructiveOperationKind::BackupRestoreCutover
);
operation_cutover!(
    super::ResolvedPointInTimeRecoveryCutoverCandidate,
    LoweredPointInTimeRecoveryCutoverPlanDag,
    AuthorizedPointInTimeRecoveryCutover,
    FencedPointInTimeRecoveryCutover,
    PublishedPointInTimeRecoveryPendingReadmission,
    PointInTimeRecoveryReadmissionOutcome,
    PublishedPointInTimeRecoveryRejectedByAuthority,
    PublishedPointInTimeRecoveryAbandoned,
    PublishedPointInTimeRecoveryRetainedForForensics,
    ReadmittedPointInTimeRecoveryCurrent,
    PointInTimeRecoveryCutoverOperation,
    DestructiveOperationKind::PointInTimeRecoveryCutover
);
operation_cutover!(
    super::ResolvedRollbackCutoverCandidate,
    LoweredRollbackCutoverPlanDag,
    AuthorizedRollbackCutover,
    FencedRollbackCutover,
    PublishedRollbackPendingReadmission,
    RollbackReadmissionOutcome,
    PublishedRollbackRejectedByAuthority,
    PublishedRollbackAbandoned,
    PublishedRollbackRetainedForForensics,
    ReadmittedRollbackCurrent,
    RollbackCutoverOperation,
    DestructiveOperationKind::RollbackCutover
);
operation_cutover!(
    super::ResolvedAuthorityAffectingRepairCutoverCandidate,
    LoweredAuthorityAffectingRepairCutoverPlanDag,
    AuthorizedAuthorityAffectingRepairCutover,
    FencedAuthorityAffectingRepairCutover,
    PublishedAuthorityAffectingRepairPendingReadmission,
    AuthorityAffectingRepairReadmissionOutcome,
    PublishedAuthorityAffectingRepairRejectedByAuthority,
    PublishedAuthorityAffectingRepairAbandoned,
    PublishedAuthorityAffectingRepairRetainedForForensics,
    ReadmittedAuthorityAffectingRepairCurrent,
    AuthorityAffectingRepairCutoverOperation,
    DestructiveOperationKind::AuthorityAffectingRepairCutover
);
