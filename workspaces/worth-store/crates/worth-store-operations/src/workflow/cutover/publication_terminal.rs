use worth_store_authority::{
    CurrentAuthorityReadmissionReceipt, RecoveryWriteFencePort, RecoveryWriteFenceReleaseReceipt,
    StoreCurrentAuthorityWitness,
};
use worth_store_physical_isolation::AtomicRecoveryPublicationReceipt;

use crate::{OperationalControlStore, OperationalTransitionId};

use super::operation_cutover::*;
use super::publication_disposition::{
    abandon, attempt_readmission, release_terminal_source_lease, retain_for_forensics,
    CoreReadmissionOutcome,
};
use super::RecoveryCutoverExecutionDenial;

macro_rules! publication_terminal {
    ($published:ident, $outcome:ident, $rejected:ident, $abandoned:ident, $retained:ident,
        $readmitted:ident) => {
        impl $published {
            pub fn attempt_readmission(
                self,
                control: &OperationalControlStore,
                transition: OperationalTransitionId,
                current: &StoreCurrentAuthorityWitness,
                fence_port: &impl RecoveryWriteFencePort,
            ) -> Result<$outcome, RecoveryCutoverExecutionDenial> {
                self.attempt_readmission_through(control, transition, current, fence_port)
            }

            #[cfg(feature = "certification-test-authority")]
            pub fn attempt_readmission_with_certification_control_store(
                self,
                control: &dyn crate::OperationalControlStorePort,
                transition: OperationalTransitionId,
                current: &StoreCurrentAuthorityWitness,
                fence_port: &impl RecoveryWriteFencePort,
            ) -> Result<$outcome, RecoveryCutoverExecutionDenial> {
                self.attempt_readmission_through(control, transition, current, fence_port)
            }

            fn attempt_readmission_through(
                self,
                control: &dyn crate::OperationalControlStorePort,
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
                self.readmit_through(control, transition, current, fence_port)
            }

            #[cfg(feature = "certification-test-authority")]
            pub fn readmit_with_certification_control_store(
                self,
                control: &dyn crate::OperationalControlStorePort,
                transition: OperationalTransitionId,
                current: &StoreCurrentAuthorityWitness,
                fence_port: &impl RecoveryWriteFencePort,
            ) -> Result<$readmitted, RecoveryCutoverExecutionDenial> {
                self.readmit_through(control, transition, current, fence_port)
            }

            fn readmit_through(
                self,
                control: &dyn crate::OperationalControlStorePort,
                transition: OperationalTransitionId,
                current: &StoreCurrentAuthorityWitness,
                fence_port: &impl RecoveryWriteFencePort,
            ) -> Result<$readmitted, RecoveryCutoverExecutionDenial> {
                match self.attempt_readmission_through(control, transition, current, fence_port)? {
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
                self.abandon_through(reason_identity, control, transition, fence_port)
            }

            #[cfg(feature = "certification-test-authority")]
            pub fn abandon_with_certification_control_store(
                self,
                reason_identity: [u8; 32],
                control: &dyn crate::OperationalControlStorePort,
                transition: OperationalTransitionId,
                fence_port: &impl RecoveryWriteFencePort,
            ) -> Result<$abandoned, RecoveryCutoverExecutionDenial> {
                self.abandon_through(reason_identity, control, transition, fence_port)
            }

            fn abandon_through(
                self,
                reason_identity: [u8; 32],
                control: &dyn crate::OperationalControlStorePort,
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
                self.retain_for_forensics_through(
                    retention_plan_identity,
                    control,
                    transition,
                    fence_port,
                )
            }

            #[cfg(feature = "certification-test-authority")]
            pub fn retain_for_forensics_with_certification_control_store(
                self,
                retention_plan_identity: [u8; 32],
                control: &dyn crate::OperationalControlStorePort,
                transition: OperationalTransitionId,
                fence_port: &impl RecoveryWriteFencePort,
            ) -> Result<$retained, RecoveryCutoverExecutionDenial> {
                self.retain_for_forensics_through(
                    retention_plan_identity,
                    control,
                    transition,
                    fence_port,
                )
            }

            fn retain_for_forensics_through(
                self,
                retention_plan_identity: [u8; 32],
                control: &dyn crate::OperationalControlStorePort,
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

publication_terminal!(
    PublishedBackupRestorePendingReadmission,
    BackupRestoreReadmissionOutcome,
    PublishedBackupRestoreRejectedByAuthority,
    PublishedBackupRestoreAbandoned,
    PublishedBackupRestoreRetainedForForensics,
    ReadmittedBackupRestoreCurrent
);
publication_terminal!(
    PublishedPointInTimeRecoveryPendingReadmission,
    PointInTimeRecoveryReadmissionOutcome,
    PublishedPointInTimeRecoveryRejectedByAuthority,
    PublishedPointInTimeRecoveryAbandoned,
    PublishedPointInTimeRecoveryRetainedForForensics,
    ReadmittedPointInTimeRecoveryCurrent
);
publication_terminal!(
    PublishedRollbackPendingReadmission,
    RollbackReadmissionOutcome,
    PublishedRollbackRejectedByAuthority,
    PublishedRollbackAbandoned,
    PublishedRollbackRetainedForForensics,
    ReadmittedRollbackCurrent
);
publication_terminal!(
    PublishedAuthorityAffectingRepairPendingReadmission,
    AuthorityAffectingRepairReadmissionOutcome,
    PublishedAuthorityAffectingRepairRejectedByAuthority,
    PublishedAuthorityAffectingRepairAbandoned,
    PublishedAuthorityAffectingRepairRetainedForForensics,
    ReadmittedAuthorityAffectingRepairCurrent
);
