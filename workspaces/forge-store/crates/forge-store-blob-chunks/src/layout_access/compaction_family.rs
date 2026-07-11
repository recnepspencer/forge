use forge_store_contracts::{DurableArtifactFamilyId, DurableArtifactRebuildPosture};
use forge_store_layout_indexes::access_planning::S8AccessShape;
use forge_store_layout_indexes::layout_strategy_admission::{
    phase25_compaction_rule, AdmittedCompactionLayoutRule,
};

use super::behavior::{
    corruption_behavior_for, declared_rebuild_posture, BlobLayoutCorruptionBehavior,
    BlobLayoutScopeSafeAbsenceBehavior,
};
use super::{BlobLayoutAccessDenial, BlobLayoutAccessDenialKind, BlobLayoutAccessPathEvidence};
use crate::{
    BlobAuthorityClassification, BlobChunkSecurityMetadataWitness, BlobCompactionEquivalence,
    BlobCompactionRewritePlan, LogicalContentDigest,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CompactionLayoutFamilyHome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CompactionLayoutAdmission {
    _rule: AdmittedCompactionLayoutRule,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AdmittedCompactionLayoutFamily {
    _admission: CompactionLayoutAdmission,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactionLayoutReport {
    family_id: DurableArtifactFamilyId,
    access_shape: S8AccessShape,
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

impl CompactionLayoutFamilyHome {
    const fn s8() -> Self {
        Self
    }

    fn admit(self, rule: AdmittedCompactionLayoutRule) -> CompactionLayoutAdmission {
        let _ = self;
        CompactionLayoutAdmission { _rule: rule }
    }
}

fn compaction_layout() -> AdmittedCompactionLayoutFamily {
    AdmittedCompactionLayoutFamily {
        _admission: CompactionLayoutFamilyHome::s8()
            .admit(phase25_compaction_rule().expect("phase 25 compaction rule must stay admitted")),
    }
}

impl AdmittedCompactionLayoutFamily {
    fn admit_compaction(
        &self,
        plan: &BlobCompactionRewritePlan,
        equivalence: &BlobCompactionEquivalence,
    ) -> Result<CompactionLayoutReport, BlobLayoutAccessDenial> {
        let _ = self;
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
            access_shape: S8AccessShape::CompactionRead,
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

    pub const fn access_shape(&self) -> S8AccessShape {
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
    pub fn admit_compaction_layout(
        &self,
        equivalence: &BlobCompactionEquivalence,
    ) -> Result<CompactionLayoutReport, BlobLayoutAccessDenial> {
        compaction_layout().admit_compaction(self, equivalence)
    }
}
