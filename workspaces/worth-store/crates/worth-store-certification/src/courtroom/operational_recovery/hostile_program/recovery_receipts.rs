use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PublishedReadmissionRecoveryReceipt {
    rejected_publication_identity: [u8; 32],
    readmitted_publication_identity: [u8; 32],
    evidence_identity: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RevokedAuthorizationRecoveryReceipt {
    promoted_receipt_identity: [u8; 32],
    evidence_identity: [u8; 32],
}

impl RevokedAuthorizationRecoveryReceipt {
    pub fn from_revoked_attempt_and_fresh_promotion(
        denial: &worth_store_operations::AuthorizationDenial,
        current: &worth_store_operations::CurrentReplicaPromotion,
    ) -> Option<Self> {
        let promoted_receipt_identity = current.promotion_receipt().receipt_identity();
        if *denial != worth_store_operations::AuthorizationDenial::AuthorizationRevoked
            || promoted_receipt_identity == [0; 32]
        {
            return None;
        }
        let mut digest = Sha256::new();
        digest.update(b"worth-store-s10-revoked-authorization-recovery-v1");
        digest.update(promoted_receipt_identity);
        digest.update(current.publication().publication_identity());
        digest.update(current.serve_lease().lease_identity());
        Some(Self {
            promoted_receipt_identity,
            evidence_identity: digest.finalize().into(),
        })
    }

    pub const fn promoted_receipt_identity(self) -> [u8; 32] {
        self.promoted_receipt_identity
    }
    pub const fn evidence_identity(self) -> [u8; 32] {
        self.evidence_identity
    }
}

impl PublishedReadmissionRecoveryReceipt {
    pub fn from_owner_outcomes(
        rejected: &worth_store_operations::PublishedBackupRestoreRejectedByAuthority,
        readmitted: &worth_store_operations::RecoveredReadmittedBackupRestoreCurrent,
    ) -> Option<Self> {
        let rejected_publication_identity = rejected.publication().publication_identity();
        let readmitted_publication_identity = readmitted.publication().publication_identity();
        if rejected.denial()
            != worth_store_authority::RecoveryAuthorityReadmissionDenial::StaleCurrentAuthority
            || rejected.fence_release().disposition()
                != worth_store_authority::RecoveryWriteFenceDisposition::RejectedByAuthority
            || readmitted.fence_release().disposition()
                != worth_store_authority::RecoveryWriteFenceDisposition::Readmitted
            || readmitted.readmission().publication_identity() != readmitted_publication_identity
            || readmitted.publication().posture()
                != worth_store_physical_isolation::RecoveryPublicationPosture::RecoveredAlreadyPublished
            || rejected_publication_identity == [0; 32]
            || readmitted_publication_identity == [0; 32]
            || rejected_publication_identity == readmitted_publication_identity
        {
            return None;
        }
        let mut digest = Sha256::new();
        digest.update(b"worth-store-s10-published-readmission-recovery-v1");
        digest.update(rejected_publication_identity);
        digest.update(readmitted_publication_identity);
        digest.update(rejected.fence_release().fence_identity());
        digest.update(readmitted.fence_release().fence_identity());
        Some(Self {
            rejected_publication_identity,
            readmitted_publication_identity,
            evidence_identity: digest.finalize().into(),
        })
    }

    pub const fn rejected_publication_identity(self) -> [u8; 32] {
        self.rejected_publication_identity
    }
    pub const fn readmitted_publication_identity(self) -> [u8; 32] {
        self.readmitted_publication_identity
    }
    pub const fn evidence_identity(self) -> [u8; 32] {
        self.evidence_identity
    }
}
