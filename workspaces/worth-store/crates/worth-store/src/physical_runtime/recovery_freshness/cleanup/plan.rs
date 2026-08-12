use std::collections::BTreeMap;

use sha2::{Digest, Sha256};
use worth_store_physical_backend::{
    AdmittedRecoveryFilesystemMedia, PhysicalRecoveryMediaGeneration,
};
use worth_store_physical_format::{
    store_namespace::StableStoreIdentity, PhysicalCheckpointIdentity, VerifiedCheckpointStream,
};
use worth_store_wal::{
    LogSequenceNumber, VerifiedWalArtifact, WalLsnRange, WalSegmentArtifactIdentity,
};

use crate::physical_runtime::{
    CompletedPhysicalRecoveryFreshReopen, PhysicalRecoveryCoordination,
};

use super::{
    StoreRecoveryCleanupEligibility, StoreRecoveryCleanupFreshnessDenial,
    StoreRecoveryCleanupFreshnessFailure, StoreRecoveryCleanupRemovalBasis,
};

/// Store-owned, consuming removal plan for one exact bounded candidate set.
///
/// The plan is derived from fresh-reopen and verified checkpoint/WAL facts.
/// Callers may request admission, but cannot add a candidate after admission,
/// substitute its bytes, or mint per-artifact removal eligibility directly.
pub struct StoreRecoveryCleanupPlan<'e> {
    identity: [u8; 32],
    store: StableStoreIdentity,
    media_generation: PhysicalRecoveryMediaGeneration,
    session: [u8; 16],
    policy_identity: [u8; 32],
    candidates: BTreeMap<WalSegmentArtifactIdentity, StoreRecoveryCleanupEligibility<'e>>,
}

struct CommonBasis {
    store: StableStoreIdentity,
    media_generation: PhysicalRecoveryMediaGeneration,
    session: [u8; 16],
    published_generation: u64,
    sealed_publication_basis: [u8; 32],
    checkpoint: PhysicalCheckpointIdentity,
    compaction_generation: u64,
    compaction_digest: [u8; 32],
    retained_boundary: LogSequenceNumber,
}

struct PendingCandidate {
    wal: VerifiedWalArtifact,
    artifact: WalSegmentArtifactIdentity,
    lsn_range: WalLsnRange,
    byte_count: u64,
    artifact_digest: [u8; 32],
}

pub(in crate::physical_runtime) fn admit<'e>(
    coordination: &PhysicalRecoveryCoordination,
    media: &AdmittedRecoveryFilesystemMedia,
    reopened: &CompletedPhysicalRecoveryFreshReopen,
    checkpoint: &'e VerifiedCheckpointStream,
    wal: impl IntoIterator<Item = VerifiedWalArtifact>,
) -> Result<StoreRecoveryCleanupPlan<'e>, StoreRecoveryCleanupFreshnessFailure> {
    let common = common_basis(coordination, media, reopened, checkpoint)?;
    let capacity = coordination.cleanup_capacity();
    let mut pending = BTreeMap::new();
    let mut admitted_bytes = 0_u64;
    for wal in wal {
        if pending.len() == capacity.cleanup_candidates() {
            return Err(invalid());
        }
        let inspection = wal.inspection();
        let next_bytes = admitted_bytes
            .checked_add(inspection.byte_count())
            .filter(|bytes| *bytes <= capacity.cleanup_bytes())
            .ok_or_else(invalid)?;
        if inspection.byte_count() == 0
            || inspection.lsn_range().end_exclusive() > common.retained_boundary
        {
            return Err(invalid());
        }
        let candidate = PendingCandidate {
            artifact: inspection.identity(),
            lsn_range: inspection.lsn_range(),
            byte_count: inspection.byte_count(),
            artifact_digest: inspection.artifact_digest(),
            wal,
        };
        if pending.insert(candidate.artifact, candidate).is_some() {
            return Err(invalid());
        }
        admitted_bytes = next_bytes;
    }
    let policy_identity = policy_identity(&common, capacity);
    let identity = plan_identity(&common, policy_identity, &pending);
    let candidates = pending
        .into_iter()
        .map(|(artifact, pending)| {
            (
                artifact,
                StoreRecoveryCleanupEligibility {
                    checkpoint,
                    wal: pending.wal,
                    removal: StoreRecoveryCleanupRemovalBasis {
                        store: common.store,
                        media_generation: common.media_generation,
                        session: common.session,
                        plan: identity,
                        published_generation: common.published_generation,
                        sealed_publication_basis: common.sealed_publication_basis,
                        checkpoint: common.checkpoint,
                        compaction_generation: common.compaction_generation,
                        compaction_digest: common.compaction_digest,
                        retained_boundary: common.retained_boundary,
                        artifact,
                        lsn_range: pending.lsn_range,
                        byte_count: pending.byte_count,
                        artifact_digest: pending.artifact_digest,
                    },
                },
            )
        })
        .collect();
    Ok(StoreRecoveryCleanupPlan {
        identity,
        store: common.store,
        media_generation: common.media_generation,
        session: common.session,
        policy_identity,
        candidates,
    })
}

fn common_basis(
    coordination: &PhysicalRecoveryCoordination,
    media: &AdmittedRecoveryFilesystemMedia,
    reopened: &CompletedPhysicalRecoveryFreshReopen,
    checkpoint: &VerifiedCheckpointStream,
) -> Result<CommonBasis, StoreRecoveryCleanupFreshnessFailure> {
    let occurrence = reopened.fresh_reopen_occurrence();
    let root = reopened.root();
    let source = checkpoint.source();
    let checkpoint_root = source.root();
    let store = source.identity().store_identity();
    if store != media.store_identity()
        || occurrence.session() != coordination.session_identity()
        || checkpoint_root.generation() > root.generation()
        || checkpoint_root.tree_identity() != root.tree_identity()
    {
        return Err(invalid());
    }
    let compaction = checkpoint.compaction_cutover();
    Ok(CommonBasis {
        store,
        media_generation: media.media_generation(),
        session: occurrence.session(),
        published_generation: occurrence.generation(),
        sealed_publication_basis: occurrence.plan(),
        checkpoint: source.identity(),
        compaction_generation: compaction.product_generation(),
        compaction_digest: checkpoint.footer().binding_records_digest(),
        retained_boundary: LogSequenceNumber::new(source.wal().covered_end_lsn_exclusive()),
    })
}

fn policy_identity(
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

fn plan_identity(
    common: &CommonBasis,
    policy_identity: [u8; 32],
    candidates: &BTreeMap<WalSegmentArtifactIdentity, PendingCandidate>,
) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth.store.recovery.cleanup-plan.v2");
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

fn invalid() -> StoreRecoveryCleanupFreshnessFailure {
    StoreRecoveryCleanupFreshnessFailure {
        denial: StoreRecoveryCleanupFreshnessDenial::InvalidCleanupEligibility,
        sample: None,
        read: None,
        binding: None,
    }
}

impl StoreRecoveryCleanupPlan<'_> {
    pub const fn identity(&self) -> [u8; 32] {
        self.identity
    }

    pub(super) const fn policy_identity(&self) -> [u8; 32] {
        self.policy_identity
    }

    pub(super) fn bindings_match(
        &self,
        coordination: &PhysicalRecoveryCoordination,
        media: &AdmittedRecoveryFilesystemMedia,
    ) -> bool {
        self.store == media.store_identity()
            && self.media_generation == media.media_generation()
            && self.session == coordination.session_identity()
    }

    pub(super) fn take(
        &mut self,
        artifact: WalSegmentArtifactIdentity,
    ) -> Option<StoreRecoveryCleanupEligibility<'_>> {
        self.candidates.remove(&artifact)
    }
}
