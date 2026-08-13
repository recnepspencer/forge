use std::collections::BTreeMap;

use sha2::{Digest, Sha256};
use worth_store_wal::WalSegmentArtifactIdentity;

use super::admission::CommonBasis;
use super::candidates::PendingCandidate;

pub(super) fn policy_identity(
    common: &CommonBasis,
    capacity: crate::physical_runtime::PhysicalRecoveryCoordinationCapacity,
) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth.store.recovery.cleanup-policy.v2");
    digest.update(common.store.bytes());
    digest.update((capacity.cleanup_candidates() as u64).to_le_bytes());
    digest.update(capacity.cleanup_bytes().to_le_bytes());
    digest.finalize().into()
}

pub(super) fn plan_identity(
    common: &CommonBasis,
    descriptive_plan_identity: [u8; 32],
    policy_identity: [u8; 32],
    candidates: &BTreeMap<WalSegmentArtifactIdentity, PendingCandidate>,
) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth.store.recovery.cleanup-execution-plan.v3");
    digest.update(descriptive_plan_identity);
    digest.update(common.store.bytes());
    digest.update(common.session);
    digest.update(common.published_generation.to_le_bytes());
    digest.update(common.sealed_publication_basis);
    digest.update(common.checkpoint.store_identity().bytes());
    digest.update(common.checkpoint.sequence().get().to_le_bytes());
    digest.update(common.compaction_generation.to_le_bytes());
    digest.update(common.compaction_digest);
    digest.update(common.retained_boundary.get().to_le_bytes());
    digest.update(policy_identity);
    digest.update((candidates.len() as u64).to_le_bytes());
    for candidate in candidates.values() {
        digest.update(candidate.artifact.segment().get().to_le_bytes());
        digest.update(candidate.artifact.generation().get().to_le_bytes());
        digest.update(candidate.lsn_range.start().get().to_le_bytes());
        digest.update(candidate.lsn_range.end_exclusive().get().to_le_bytes());
        digest.update(candidate.byte_count.to_le_bytes());
        digest.update(candidate.artifact_digest);
    }
    digest.finalize().into()
}
