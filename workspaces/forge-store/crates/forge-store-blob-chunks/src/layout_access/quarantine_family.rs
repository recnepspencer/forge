use forge_store_contracts::{DurableArtifactFamilyId, DurableArtifactRebuildPosture};
use forge_store_layout_indexes::access_planning::S8AccessShape;
use forge_store_layout_indexes::layout_strategy_admission::{
    phase25_quarantine_rule, AdmittedQuarantineLayoutRule,
};

use super::behavior::{
    corruption_behavior_for, declared_rebuild_posture, BlobLayoutCorruptionBehavior,
    BlobLayoutScopeSafeAbsenceBehavior,
};
use super::{BlobLayoutAccessDenial, BlobLayoutAccessPathEvidence};
use crate::{
    BlobChunkOrdinal, BlobChunkQuarantine, BlobCorruptionDetectionSource,
    BlobCorruptionPlacementClass, BlobGeneration, BlobObjectId, BlobQuarantineLifecycleState,
    BlobQuarantineRepairCapability, StoredChunkDigest,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct QuarantineLayoutFamilyHome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct QuarantineLayoutAdmission {
    _rule: AdmittedQuarantineLayoutRule,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AdmittedQuarantineLayoutFamily {
    _admission: QuarantineLayoutAdmission,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuarantineLayoutReport {
    family_id: DurableArtifactFamilyId,
    access_shape: S8AccessShape,
    rebuild_posture: DurableArtifactRebuildPosture,
    absence_behavior: BlobLayoutScopeSafeAbsenceBehavior,
    corruption_behavior: BlobLayoutCorruptionBehavior,
    source: BlobCorruptionDetectionSource,
    object_id: BlobObjectId,
    generation: BlobGeneration,
    ordinal: BlobChunkOrdinal,
    stored_digest: StoredChunkDigest,
    placement_class: BlobCorruptionPlacementClass,
    lifecycle_state: BlobQuarantineLifecycleState,
    repair_capability: BlobQuarantineRepairCapability,
    reference_edges: u64,
    counter_evidence: BlobLayoutAccessPathEvidence,
}

impl QuarantineLayoutFamilyHome {
    const fn s8() -> Self {
        Self
    }

    fn admit(self, rule: AdmittedQuarantineLayoutRule) -> QuarantineLayoutAdmission {
        let _ = self;
        QuarantineLayoutAdmission { _rule: rule }
    }
}

fn quarantine_layout() -> AdmittedQuarantineLayoutFamily {
    AdmittedQuarantineLayoutFamily {
        _admission: QuarantineLayoutFamilyHome::s8()
            .admit(phase25_quarantine_rule().expect("phase 25 quarantine rule must stay admitted")),
    }
}

impl AdmittedQuarantineLayoutFamily {
    fn admit_quarantine(
        &self,
        quarantine: &BlobChunkQuarantine,
    ) -> Result<QuarantineLayoutReport, BlobLayoutAccessDenial> {
        let _ = self;
        Ok(QuarantineLayoutReport::from_quarantine(quarantine))
    }
}

impl QuarantineLayoutReport {
    fn from_quarantine(quarantine: &BlobChunkQuarantine) -> Self {
        let family_id = DurableArtifactFamilyId::QuarantineRecord;
        let rebuild_posture = declared_rebuild_posture(family_id);
        let localized = quarantine.localization();
        Self {
            family_id,
            access_shape: S8AccessShape::QuarantineRead,
            rebuild_posture,
            absence_behavior: BlobLayoutScopeSafeAbsenceBehavior::ScopedVerifierScan,
            corruption_behavior: corruption_behavior_for(rebuild_posture),
            source: quarantine.source(),
            object_id: quarantine.object_id().clone(),
            generation: quarantine.generation(),
            ordinal: quarantine.ordinal(),
            stored_digest: quarantine.stored_digest().clone(),
            placement_class: quarantine.placement_class(),
            lifecycle_state: quarantine.state(),
            repair_capability: quarantine.repair_capability(),
            reference_edges: localized.reference_edges().edge_count(),
            counter_evidence: BlobLayoutAccessPathEvidence::from_corruption(
                family_id,
                quarantine.counters(),
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

    pub const fn source(&self) -> BlobCorruptionDetectionSource {
        self.source
    }

    pub const fn object_id(&self) -> &BlobObjectId {
        &self.object_id
    }

    pub const fn generation(&self) -> BlobGeneration {
        self.generation
    }

    pub const fn ordinal(&self) -> BlobChunkOrdinal {
        self.ordinal
    }

    pub const fn stored_digest(&self) -> &StoredChunkDigest {
        &self.stored_digest
    }

    pub const fn placement_class(&self) -> BlobCorruptionPlacementClass {
        self.placement_class
    }

    pub const fn lifecycle_state(&self) -> BlobQuarantineLifecycleState {
        self.lifecycle_state
    }

    pub const fn repair_capability(&self) -> BlobQuarantineRepairCapability {
        self.repair_capability
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

impl BlobChunkQuarantine {
    pub fn admit_quarantine_layout(
        &self,
    ) -> Result<QuarantineLayoutReport, BlobLayoutAccessDenial> {
        quarantine_layout().admit_quarantine(self)
    }
}
