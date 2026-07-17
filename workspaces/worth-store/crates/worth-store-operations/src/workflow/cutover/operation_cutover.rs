use worth_store_authority::{RecoveryWriteFencePort, StoreCurrentAuthorityWitness};

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
    PublishedAbandonedCore, PublishedRejectedCore, PublishedRetainedForForensicsCore,
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
    pub(super) PublishedCutoverCore<BackupRestoreCutoverOperation>,
);
pub struct PublishedPointInTimeRecoveryPendingReadmission(
    pub(super) PublishedCutoverCore<PointInTimeRecoveryCutoverOperation>,
);
pub struct PublishedRollbackPendingReadmission(
    pub(super) PublishedCutoverCore<RollbackCutoverOperation>,
);
pub struct ReadmittedBackupRestoreCurrent(pub(super) Box<ReadmittedCutoverCore>);
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
    pub(super) PublishedCutoverCore<AuthorityAffectingRepairCutoverOperation>,
);
pub struct ReadmittedAuthorityAffectingRepairCurrent(pub(super) Box<ReadmittedCutoverCore>);

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
        $marker:ty, $operation:expr) => {
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
                    super::protocol::CutoverReadinessInput {
                        control,
                        append: control,
                        transition,
                        current,
                        fence_port,
                        observed_at,
                        revocation,
                    },
                )
                .map($fenced)
            }

            #[cfg(feature = "certification-test-authority")]
            #[allow(clippy::too_many_arguments)]
            pub fn establish_write_fence_with_certification_control_store(
                self,
                control: &OperationalControlStore,
                append: &dyn crate::OperationalControlStorePort,
                transition: OperationalTransitionId,
                current: &StoreCurrentAuthorityWitness,
                fence_port: &impl RecoveryWriteFencePort,
                observed_at: u64,
                revocation: AuthorizationRevocationObservation,
            ) -> Result<$fenced, RecoveryCutoverExecutionDenial> {
                ready(
                    self.0,
                    super::protocol::CutoverReadinessInput {
                        control,
                        append,
                        transition,
                        current,
                        fence_port,
                        observed_at,
                        revocation,
                    },
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
    };
}

operation_cutover!(
    super::ResolvedBackupRestoreCutoverCandidate,
    LoweredBackupRestoreCutoverPlanDag,
    AuthorizedBackupRestoreCutover,
    FencedBackupRestoreCutover,
    PublishedBackupRestorePendingReadmission,
    BackupRestoreCutoverOperation,
    DestructiveOperationKind::BackupRestoreCutover
);
operation_cutover!(
    super::ResolvedPointInTimeRecoveryCutoverCandidate,
    LoweredPointInTimeRecoveryCutoverPlanDag,
    AuthorizedPointInTimeRecoveryCutover,
    FencedPointInTimeRecoveryCutover,
    PublishedPointInTimeRecoveryPendingReadmission,
    PointInTimeRecoveryCutoverOperation,
    DestructiveOperationKind::PointInTimeRecoveryCutover
);
operation_cutover!(
    super::ResolvedRollbackCutoverCandidate,
    LoweredRollbackCutoverPlanDag,
    AuthorizedRollbackCutover,
    FencedRollbackCutover,
    PublishedRollbackPendingReadmission,
    RollbackCutoverOperation,
    DestructiveOperationKind::RollbackCutover
);
operation_cutover!(
    super::ResolvedAuthorityAffectingRepairCutoverCandidate,
    LoweredAuthorityAffectingRepairCutoverPlanDag,
    AuthorizedAuthorityAffectingRepairCutover,
    FencedAuthorityAffectingRepairCutover,
    PublishedAuthorityAffectingRepairPendingReadmission,
    AuthorityAffectingRepairCutoverOperation,
    DestructiveOperationKind::AuthorityAffectingRepairCutover
);
