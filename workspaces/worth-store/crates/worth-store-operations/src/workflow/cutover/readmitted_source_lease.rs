use super::operation_cutover::{
    ReadmittedPointInTimeRecoveryCurrent, ReadmittedRollbackCurrent,
    RecoverySourceLeaseFinalizationDenial,
};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletedRetainedAuthorityRollback {
    publication: worth_store_physical_isolation::AtomicRecoveryPublicationReceipt,
    readmission: worth_store_authority::CurrentAuthorityReadmissionReceipt,
    fence_release: worth_store_authority::RecoveryWriteFenceReleaseReceipt,
    source_lease_release: worth_store_physical_isolation::RecoverySourceLeaseReleaseReceipt,
    evidence_identity: [u8; 32],
}

impl ReadmittedPointInTimeRecoveryCurrent {
    pub fn release_source_lease(
        mut self,
    ) -> Result<
        worth_store_physical_isolation::RecoverySourceLeaseReleaseReceipt,
        RecoverySourceLeaseFinalizationDenial,
    > {
        match self.0.source_lease.take() {
            Some(super::post_verification::RecoveryCutoverSourceLease::PointInTime(lease)) => lease
                .release()
                .map_err(RecoverySourceLeaseFinalizationDenial::Isolation),
            _ => Err(RecoverySourceLeaseFinalizationDenial::MissingOrWrongLease),
        }
    }
}

impl ReadmittedRollbackCurrent {
    pub fn release_source_lease(
        mut self,
    ) -> Result<
        worth_store_physical_isolation::RecoverySourceLeaseReleaseReceipt,
        RecoverySourceLeaseFinalizationDenial,
    > {
        match self.0.source_lease.take() {
            Some(super::post_verification::RecoveryCutoverSourceLease::Rollback(lease)) => lease
                .release()
                .map_err(RecoverySourceLeaseFinalizationDenial::Isolation),
            _ => Err(RecoverySourceLeaseFinalizationDenial::MissingOrWrongLease),
        }
    }

    pub fn complete_retained_authority_rollback(
        mut self,
    ) -> Result<CompletedRetainedAuthorityRollback, RecoverySourceLeaseFinalizationDenial> {
        let publication = self.0.publication.clone();
        let readmission = self.0.readmission;
        let fence_release = self.0.fence_release;
        let source_lease_release = match self.0.source_lease.take() {
            Some(super::post_verification::RecoveryCutoverSourceLease::Rollback(lease)) => lease
                .release()
                .map_err(RecoverySourceLeaseFinalizationDenial::Isolation)?,
            _ => return Err(RecoverySourceLeaseFinalizationDenial::MissingOrWrongLease),
        };
        let mut digest = Sha256::new();
        digest.update(b"worth-store-completed-retained-authority-rollback-v1");
        digest.update(publication.publication_identity());
        digest.update(readmission.publication_identity());
        digest.update(fence_release.fence_identity());
        digest.update(source_lease_release.lease_identity());
        digest.update(source_lease_release.source_identity());
        Ok(CompletedRetainedAuthorityRollback {
            publication,
            readmission,
            fence_release,
            source_lease_release,
            evidence_identity: digest.finalize().into(),
        })
    }
}

impl CompletedRetainedAuthorityRollback {
    pub const fn publication(
        &self,
    ) -> &worth_store_physical_isolation::AtomicRecoveryPublicationReceipt {
        &self.publication
    }
    pub const fn readmission(&self) -> worth_store_authority::CurrentAuthorityReadmissionReceipt {
        self.readmission
    }
    pub const fn fence_release(&self) -> worth_store_authority::RecoveryWriteFenceReleaseReceipt {
        self.fence_release
    }
    pub const fn source_lease_release(
        &self,
    ) -> worth_store_physical_isolation::RecoverySourceLeaseReleaseReceipt {
        self.source_lease_release
    }
    pub const fn evidence_identity(&self) -> [u8; 32] {
        self.evidence_identity
    }
}
