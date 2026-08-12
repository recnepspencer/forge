use sha2::{Digest, Sha256};

use super::artifact_walk::ObservedRecoveryArtifact;
use super::conclusion::RecoveryObserverConclusion;

pub(super) fn conclude(artifacts: &[ObservedRecoveryArtifact]) -> RecoveryObserverConclusion {
    let mut digest = Sha256::new();
    digest.update(b"worth.store.recovery-observer.artifact-set.v1");
    digest.update((artifacts.len() as u64).to_le_bytes());
    for artifact in artifacts {
        digest.update((artifact.path().len() as u64).to_le_bytes());
        digest.update(artifact.path().as_bytes());
        digest.update(artifact.byte_length().to_le_bytes());
        digest.update(artifact.digest());
    }
    RecoveryObserverConclusion::new(digest.finalize().into())
}
