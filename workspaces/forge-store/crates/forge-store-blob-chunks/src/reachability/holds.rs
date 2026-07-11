use forge_store_contracts::StableDigest;
use forge_store_operations_vocabulary::{
    BackupExportCustodyMode, BackupExportCustodyReadiness, S10BackupExportCustodyHandoff,
};
use forge_store_physical_isolation::{ReadDuringCheckpointVerdict, StablePhysicalReadPlan};

use crate::{
    reachability::edges::BlobReachabilityAuthorityKey, BlobCorruptionGuard,
    BlobGenerationPublished, BlobReachabilityCounterSnapshot, BlobReachabilityDenial,
    BlobReachabilityEdgeKind, BlobResumeCheckpoint, BlobResumeCheckpointStateKind,
    BlobRetentionHold, BlobRetentionHoldKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobReachabilityProtectedHold {
    identity: StableDigest,
    authority: BlobReachabilityAuthorityKey,
    kind: BlobReachabilityEdgeKind,
    protected_references: u64,
}

impl BlobReachabilityProtectedHold {
    pub(crate) fn from_stable_read_plan(
        plan: &StablePhysicalReadPlan,
        authority: BlobReachabilityAuthorityKey,
    ) -> Self {
        Self::from_authority_parts(
            BlobReachabilityEdgeKind::ReadPlanHoldReference,
            authority,
            plan.reachability_barrier().protected_references(),
            stable_read_plan_hold_basis(plan),
        )
    }

    pub(crate) fn from_checkpoint_verdict(
        verdict: &ReadDuringCheckpointVerdict,
        authority: BlobReachabilityAuthorityKey,
    ) -> Self {
        Self::from_authority_parts(
            BlobReachabilityEdgeKind::CheckpointHoldReference,
            authority,
            1,
            checkpoint_hold_basis(verdict),
        )
    }

    pub fn from_corruption_guard(
        guard: &BlobCorruptionGuard,
        published: &BlobGenerationPublished,
    ) -> Result<Self, BlobReachabilityDenial> {
        let quarantine = guard.quarantine();
        reject_quarantine_authority_mismatch(quarantine, published)?;
        Ok(Self::from_parts(
            BlobReachabilityEdgeKind::QuarantineHoldReference,
            published,
            1,
            format!(
                "quarantine:{}:{}:{}",
                quarantine.object_id().digest().as_str(),
                quarantine.generation().sequence(),
                quarantine.ordinal().get()
            ),
        ))
    }

    pub(crate) fn from_export_readiness(
        readiness: &BackupExportCustodyReadiness,
        authority: BlobReachabilityAuthorityKey,
    ) -> Result<Self, BlobReachabilityDenial> {
        if !matches!(
            readiness.mode(),
            Some(BackupExportCustodyMode::Export) | None
        ) {
            return Err(invalid_hold_denial());
        }
        Ok(Self::from_authority_parts(
            BlobReachabilityEdgeKind::ExportHoldReference,
            authority,
            1,
            format!(
                "export:{}:{}",
                readiness.mode_label(),
                readiness
                    .receipt()
                    .receipt_id()
                    .security_scope_fingerprint()
            ),
        ))
    }

    pub(crate) fn from_backup_repair_backup_handoff(
        handoff: &S10BackupExportCustodyHandoff,
        authority: BlobReachabilityAuthorityKey,
    ) -> Result<Self, BlobReachabilityDenial> {
        let readiness = handoff.readiness();
        if !matches!(
            readiness.mode(),
            Some(BackupExportCustodyMode::Backup)
                | Some(BackupExportCustodyMode::PointInTimeRecovery)
        ) {
            return Err(invalid_hold_denial());
        }
        Ok(Self::from_authority_parts(
            BlobReachabilityEdgeKind::BackupHoldReference,
            authority,
            1,
            format!(
                "backup:{}:{}",
                readiness.mode_label(),
                handoff.receipt().receipt_id().security_scope_fingerprint()
            ),
        ))
    }

    pub(crate) fn from_retention_hold(
        hold: &BlobRetentionHold,
        authority: BlobReachabilityAuthorityKey,
    ) -> Self {
        let kind = match hold.kind() {
            BlobRetentionHoldKind::Generation => BlobReachabilityEdgeKind::GenerationHoldReference,
            BlobRetentionHoldKind::TimeWindow => BlobReachabilityEdgeKind::TimeWindowHoldReference,
            BlobRetentionHoldKind::Export => BlobReachabilityEdgeKind::ExportHoldReference,
            BlobRetentionHoldKind::Capsule => BlobReachabilityEdgeKind::ReplicationCapsuleReference,
            BlobRetentionHoldKind::Quarantine => BlobReachabilityEdgeKind::QuarantineHoldReference,
            BlobRetentionHoldKind::ReadPlan => BlobReachabilityEdgeKind::ReadPlanHoldReference,
            BlobRetentionHoldKind::Checkpoint => BlobReachabilityEdgeKind::CheckpointHoldReference,
            BlobRetentionHoldKind::TenantCustody => {
                BlobReachabilityEdgeKind::TenantCustodyHoldReference
            }
            BlobRetentionHoldKind::ResumeSession => {
                BlobReachabilityEdgeKind::ResumeSessionReference
            }
            BlobRetentionHoldKind::PlacementMove => {
                BlobReachabilityEdgeKind::PlacementMoveReference
            }
            BlobRetentionHoldKind::Backup => BlobReachabilityEdgeKind::BackupHoldReference,
        };
        Self::from_authority_parts(
            kind,
            authority,
            1,
            format!("retention:{}", hold.identity().as_str()),
        )
    }

    pub fn from_unfinished_resume_checkpoint(
        checkpoint: &BlobResumeCheckpoint,
        published: &BlobGenerationPublished,
    ) -> Result<Self, BlobReachabilityDenial> {
        if matches!(
            checkpoint.state(),
            BlobResumeCheckpointStateKind::SessionClosed
                | BlobResumeCheckpointStateKind::SessionReclaimed
        ) {
            return Err(invalid_hold_denial());
        }
        if checkpoint.root_candidate().is_none() {
            return Err(invalid_hold_denial());
        }
        reject_resume_authority_mismatch(checkpoint, published)?;
        Ok(Self::from_parts(
            BlobReachabilityEdgeKind::ResumeSessionReference,
            published,
            1,
            format!("resume:{}", checkpoint.checkpoint_identity().as_str()),
        ))
    }

    fn from_parts(
        kind: BlobReachabilityEdgeKind,
        published: &BlobGenerationPublished,
        protected_references: u64,
        local_basis: String,
    ) -> Self {
        Self::from_authority_parts(
            kind,
            BlobReachabilityAuthorityKey::from_published(published),
            protected_references,
            local_basis,
        )
    }

    fn from_authority_parts(
        kind: BlobReachabilityEdgeKind,
        authority: BlobReachabilityAuthorityKey,
        protected_references: u64,
        local_basis: String,
    ) -> Self {
        Self {
            identity: authority.hold_identity(&local_basis),
            authority,
            kind,
            protected_references,
        }
    }

    pub const fn identity(&self) -> &StableDigest {
        &self.identity
    }

    pub const fn kind(&self) -> BlobReachabilityEdgeKind {
        self.kind
    }

    pub(crate) fn authority_key(&self) -> BlobReachabilityAuthorityKey {
        self.authority.clone()
    }

    pub(crate) const fn can_seed_registry_authority(&self) -> bool {
        !matches!(
            self.kind,
            BlobReachabilityEdgeKind::ReadPlanHoldReference
                | BlobReachabilityEdgeKind::CheckpointHoldReference
                | BlobReachabilityEdgeKind::BackupHoldReference
                | BlobReachabilityEdgeKind::ExportHoldReference
        )
    }

    pub const fn protected_references(&self) -> u64 {
        self.protected_references
    }
}

fn reject_quarantine_authority_mismatch(
    quarantine: &crate::BlobChunkQuarantine,
    published: &BlobGenerationPublished,
) -> Result<(), BlobReachabilityDenial> {
    if quarantine.object_id() != published.object_id()
        || quarantine.generation() != published.generation()
        || quarantine
            .localization()
            .reference_edges()
            .validated_edge_count_for_generation(
                published.object_id(),
                published.generation(),
                published.chunk_tree_root(),
                published.logical_content_digest(),
            )
            .is_err()
    {
        return Err(wrong_authority_denial());
    }
    Ok(())
}

fn reject_resume_authority_mismatch(
    checkpoint: &BlobResumeCheckpoint,
    published: &BlobGenerationPublished,
) -> Result<(), BlobReachabilityDenial> {
    if checkpoint.security_metadata() != published.security_metadata() {
        return Err(wrong_authority_denial());
    }
    if let Some(root_candidate) = checkpoint.root_candidate() {
        let intent = root_candidate.intent();
        if intent.object_id() != published.object_id()
            || intent.generation() != published.generation()
            || intent.chunk_tree_root() != published.chunk_tree_root()
            || intent.logical_content_digest() != published.logical_content_digest()
            || intent.security_metadata() != published.security_metadata()
        {
            return Err(wrong_authority_denial());
        }
    }
    Ok(())
}

fn wrong_authority_denial() -> BlobReachabilityDenial {
    BlobReachabilityDenial::WrongBlobAuthority {
        counters: BlobReachabilityCounterSnapshot::start().record_wrong_authority_denial(),
    }
}

fn invalid_hold_denial() -> BlobReachabilityDenial {
    BlobReachabilityDenial::InvalidProtectedHold {
        counters: BlobReachabilityCounterSnapshot::start().record_wrong_authority_denial(),
    }
}

fn stable_read_plan_hold_basis(plan: &StablePhysicalReadPlan) -> String {
    format!(
        "read:{}:{}",
        plan.root_epoch().get(),
        plan.reachability_barrier()
            .footprint_basis()
            .canonical_digest()
    )
}

fn checkpoint_hold_basis(verdict: &ReadDuringCheckpointVerdict) -> String {
    format!(
        "checkpoint:{}:{}",
        verdict.proof().checkpoint_publication_root().epoch().get(),
        verdict
            .post_publication_read()
            .read_plan_release()
            .root()
            .epoch()
            .get()
    )
}
