use forge_store_recovery_physics::{
    LogSequenceNumber, PersistedRecoveryArtifactDigest, RecoveryLayoutReadmissionIdentity,
};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutReadmissionIdentity([u8; 32]);

impl LayoutReadmissionIdentity {
    pub(crate) fn from_recovery_identity(identity: &RecoveryLayoutReadmissionIdentity) -> Self {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutReadmissionWitness {
    family: crate::PhysicalArtifactFamily,
    source: super::classification::LayoutReadmissionSource,
    identity: LayoutReadmissionIdentity,
    replay_frontier: Option<LogSequenceNumber>,
}

impl LayoutReadmissionWitness {
    pub(crate) fn quarantine_recovery(
        family: crate::PhysicalArtifactFamily,
        identity: &RecoveryLayoutReadmissionIdentity,
    ) -> Self {
        Self {
            family,
            source: super::classification::LayoutReadmissionSource::QuarantineRecovery,
            identity: LayoutReadmissionIdentity::from_recovery_identity(identity),
            replay_frontier: None,
        }
    }

    pub(crate) fn offline_evidence(
        family: crate::PhysicalArtifactFamily,
        identity: &RecoveryLayoutReadmissionIdentity,
        replay_frontier: LogSequenceNumber,
    ) -> Self {
        Self {
            family,
            source: super::classification::LayoutReadmissionSource::OfflineRecoveryEvidence,
            identity: LayoutReadmissionIdentity::from_recovery_identity(identity),
            replay_frontier: Some(replay_frontier),
        }
    }

    pub(crate) fn terminal_import(
        family: crate::PhysicalArtifactFamily,
        identity: &RecoveryLayoutReadmissionIdentity,
    ) -> Self {
        Self {
            family,
            source: super::classification::LayoutReadmissionSource::TerminalImport,
            identity: LayoutReadmissionIdentity::from_recovery_identity(identity),
            replay_frontier: None,
        }
    }

    pub const fn family(self) -> crate::PhysicalArtifactFamily {
        self.family
    }

    pub const fn source(self) -> super::classification::LayoutReadmissionSource {
        self.source
    }

    pub const fn identity(self) -> LayoutReadmissionIdentity {
        self.identity
    }

    pub const fn replay_frontier(self) -> Option<LogSequenceNumber> {
        self.replay_frontier
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
