use worth_store_authority::CurrentAuthorityReadmissionReceipt;
use worth_store_offline_verifier::{
    post_verify_closed_staged_recovery, BackupVerificationBudget,
    ClosedStagedRecoveryVerificationRequest, StagedRecoveryExpectedFrontier,
    StagedRecoveryPostVerificationDenial,
};

use crate::{ExecutedBackupRestore, OperationalOperationId};

use super::RecoveredReadmittedBackupRestoreCurrent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreDrillExpectation {
    operation_id: OperationalOperationId,
    staging_plan_fingerprint: [u8; 32],
    staged_content_fingerprint: [u8; 32],
    expected_frontier: StagedRecoveryExpectedFrontier,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreDrillCertification {
    operation_id: OperationalOperationId,
    publication_identity: [u8; 32],
    readmission: CurrentAuthorityReadmissionReceipt,
    final_verification_identity: [u8; 32],
    final_manifest_digest: [u8; 32],
}

#[derive(Debug)]
pub enum RestoreDrillCertificationDenial {
    OperationMismatch,
    PublishedMediaMismatch,
    InvalidExpectation,
    FinalOfflineVerification(StagedRecoveryPostVerificationDenial),
}

impl ExecutedBackupRestore {
    pub fn prepare_restore_drill_expectation(&self) -> RestoreDrillExpectation {
        let frontier = self.receipt().recovered_frontier();
        let expected_frontier = StagedRecoveryExpectedFrontier::exact(
            frontier.durable_checkpoint_lsn(),
            frontier.wal_end_exclusive_lsn(),
            frontier.acknowledged_frontier(),
        )
        .expect("restore receipt carries an ordered exact frontier");
        RestoreDrillExpectation {
            operation_id: self.operation_id().clone(),
            staging_plan_fingerprint: self.staged_media().plan_fingerprint(),
            staged_content_fingerprint: self.staged_media().content_fingerprint(),
            expected_frontier,
        }
    }
}

impl RecoveredReadmittedBackupRestoreCurrent {
    pub fn certify_restore_drill(
        self,
        expected: RestoreDrillExpectation,
        budget: BackupVerificationBudget,
    ) -> Result<RestoreDrillCertification, RestoreDrillCertificationDenial> {
        if self.0.operation_id != expected.operation_id {
            return Err(RestoreDrillCertificationDenial::OperationMismatch);
        }
        if self.0.publication.candidate_media_identity() != expected.staged_content_fingerprint {
            return Err(RestoreDrillCertificationDenial::PublishedMediaMismatch);
        }
        let request = ClosedStagedRecoveryVerificationRequest::from_reopened_published_media(
            self.0.publication.candidate_media_root(),
            expected.staging_plan_fingerprint,
            expected.staged_content_fingerprint,
            expected.expected_frontier,
        )
        .ok_or(RestoreDrillCertificationDenial::InvalidExpectation)?;
        let verified = post_verify_closed_staged_recovery(request, budget)
            .map_err(RestoreDrillCertificationDenial::FinalOfflineVerification)?;
        if verified.root() != self.0.publication.candidate_media_root()
            || verified.staged_content_fingerprint()
                != self.0.publication.candidate_media_identity()
        {
            return Err(RestoreDrillCertificationDenial::PublishedMediaMismatch);
        }
        Ok(RestoreDrillCertification {
            operation_id: expected.operation_id,
            publication_identity: self.0.publication.publication_identity(),
            readmission: self.0.readmission,
            final_verification_identity: verified.verification_identity(),
            final_manifest_digest: verified.manifest_digest(),
        })
    }
}

impl RestoreDrillCertification {
    pub const fn operation_id(&self) -> &OperationalOperationId {
        &self.operation_id
    }
    pub const fn publication_identity(&self) -> [u8; 32] {
        self.publication_identity
    }
    pub const fn readmission(&self) -> CurrentAuthorityReadmissionReceipt {
        self.readmission
    }
    pub const fn final_verification_identity(&self) -> [u8; 32] {
        self.final_verification_identity
    }
    pub const fn final_manifest_digest(&self) -> [u8; 32] {
        self.final_manifest_digest
    }
}
