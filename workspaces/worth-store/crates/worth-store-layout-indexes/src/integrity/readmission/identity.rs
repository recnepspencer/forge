use sha2::{Digest, Sha256};
use worth_store_recovery_physics::{
    PersistedRecoveryArtifactDigest, RecoveryLayoutReadmissionIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutReadmissionIdentity([u8; 32]);

impl LayoutReadmissionIdentity {
    pub(super) fn from_recovery(identity: &RecoveryLayoutReadmissionIdentity) -> Self {
        let mut digest = Sha256::new();
        match identity {
            RecoveryLayoutReadmissionIdentity::QuarantineReceipt(receipt) => {
                digest.update(b"quarantine-receipt");
                update_field(&mut digest, receipt.as_str());
            }
            RecoveryLayoutReadmissionIdentity::OfflineArtifactDigest(artifact) => {
                digest.update(b"offline-artifact");
                update_artifact_digest(&mut digest, artifact);
            }
        }
        Self(digest.finalize().into())
    }

    pub const fn fingerprint(self) -> [u8; 32] {
        self.0
    }
}

fn update_artifact_digest(digest: &mut Sha256, artifact: &PersistedRecoveryArtifactDigest) {
    update_field(digest, artifact.value());
    update_field(digest, artifact.format_version());
    update_field(digest, artifact.backend_profile());
    update_field(digest, artifact.recovery_profile());
    digest.update((artifact.record_count() as u64).to_be_bytes());
    digest.update((artifact.byte_count() as u64).to_be_bytes());
}

fn update_field(digest: &mut Sha256, value: &str) {
    digest.update((value.len() as u64).to_be_bytes());
    digest.update(value.as_bytes());
}
