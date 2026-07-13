use forge_store_contracts::{DurableArtifactFamilyId, DurableArtifactRebuildPosture};

use super::behavior::{
    corruption_behavior_for, declared_rebuild_posture, BlobLayoutCorruptionBehavior,
    BlobLayoutScopeSafeAbsenceBehavior,
};
use super::{
    BlobLayoutAccessDenial, BlobLayoutAccessDenialKind, BlobLayoutAccessPathEvidence,
    BlobLayoutAccessShape,
};
use crate::{
    BlobAuthorityClassification, BlobChunkSecurityMetadataWitness, BlobCompactionEquivalence,
    BlobCompactionRewritePlan, LogicalContentDigest,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactionLayoutReport {
    family_id: DurableArtifactFamilyId,
    access_shape: BlobLayoutAccessShape,
    rebuild_posture: DurableArtifactRebuildPosture,
    absence_behavior: BlobLayoutScopeSafeAbsenceBehavior,
    corruption_behavior: BlobLayoutCorruptionBehavior,
    old_root: crate::ChunkTreeRoot,
    new_root: crate::ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    security_metadata: BlobChunkSecurityMetadataWitness,
    authority_classification: BlobAuthorityClassification,
    reachable_chunks: u64,
    reference_edges: u64,
    counter_evidence: BlobLayoutAccessPathEvidence,
}

impl CompactionLayoutReport {
    fn project_compaction(
        plan: &BlobCompactionRewritePlan,
        equivalence: &BlobCompactionEquivalence,
    ) -> Result<CompactionLayoutReport, BlobLayoutAccessDenial> {
        if !equivalence.matches_plan_basis(plan) {
            return Err(BlobLayoutAccessDenial::new(
                BlobLayoutAccessDenialKind::CompactionLayoutRequiresPlanBoundEquivalence,
            ));
        }
        Ok(CompactionLayoutReport::from_plan(plan, equivalence))
    }
}

impl CompactionLayoutReport {
    fn from_plan(
        plan: &BlobCompactionRewritePlan,
        equivalence: &BlobCompactionEquivalence,
    ) -> Self {
        let family_id = DurableArtifactFamilyId::MaintenanceCompaction;
        let rebuild_posture = declared_rebuild_posture(family_id);
        Self {
            family_id,
            access_shape: BlobLayoutAccessShape::CompactionRead,
            rebuild_posture,
            absence_behavior: BlobLayoutScopeSafeAbsenceBehavior::ScopedMaintenanceScan,
            corruption_behavior: corruption_behavior_for(rebuild_posture),
            old_root: equivalence.old_root().clone(),
            new_root: equivalence.new_root().clone(),
            logical_content_digest: equivalence.logical_content_digest().clone(),
            security_metadata: equivalence.security_metadata(),
            authority_classification: equivalence.authority_classification(),
            reachable_chunks: equivalence.reachable_chunks(),
            reference_edges: equivalence.reference_edges(),
            counter_evidence: BlobLayoutAccessPathEvidence::from_compaction(
                family_id,
                plan.counters(),
            ),
        }
    }

    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn access_shape(&self) -> BlobLayoutAccessShape {
        self.access_shape
    }

    pub const fn rebuild_posture(&self) -> DurableArtifactRebuildPosture {
        self.rebuild_posture
    }

    pub const fn absence_behavior(&self) -> BlobLayoutScopeSafeAbsenceBehavior {
        self.absence_behavior
    }

    pub const fn corruption_behavior(&self) -> BlobLayoutCorruptionBehavior {
        self.corruption_behavior
    }

    pub const fn old_root(&self) -> &crate::ChunkTreeRoot {
        &self.old_root
    }

    pub const fn new_root(&self) -> &crate::ChunkTreeRoot {
        &self.new_root
    }

    pub const fn logical_content_digest(&self) -> &LogicalContentDigest {
        &self.logical_content_digest
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub const fn authority_classification(&self) -> BlobAuthorityClassification {
        self.authority_classification
    }

    pub const fn reachable_chunks(&self) -> u64 {
        self.reachable_chunks
    }

    pub const fn reference_edges(&self) -> u64 {
        self.reference_edges
    }

    pub const fn counter_evidence(&self) -> BlobLayoutAccessPathEvidence {
        self.counter_evidence
    }

    pub const fn requires_rebuild_parity(&self) -> bool {
        !matches!(
            self.rebuild_posture,
            DurableArtifactRebuildPosture::NoRebuild
        )
    }
}

impl BlobCompactionRewritePlan {
    pub fn project_compaction_layout(
        &self,
        equivalence: &BlobCompactionEquivalence,
    ) -> Result<CompactionLayoutReport, BlobLayoutAccessDenial> {
        CompactionLayoutReport::project_compaction(self, equivalence)
    }
}
