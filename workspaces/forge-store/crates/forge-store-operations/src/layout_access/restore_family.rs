use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_layout_indexes::access_planning::S8AccessShape;
use forge_store_layout_indexes::layout_strategy_admission::{
    phase28_restore_evidence_rule, AdmittedRestoreEvidenceLayoutRule, Phase28LayoutAuthorityPosture,
};
use forge_store_security::StoreTrustBoundaryReadmissionTrigger;

use crate::BackupImportCustodyReadmission;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RestoreLayoutAdmission {
    rule: AdmittedRestoreEvidenceLayoutRule,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreLayoutEvidenceReport {
    admission: RestoreLayoutAdmission,
    family_id: DurableArtifactFamilyId,
    access_shape: S8AccessShape,
    posture: Phase28LayoutAuthorityPosture,
    trigger: StoreTrustBoundaryReadmissionTrigger,
}

impl RestoreLayoutEvidenceReport {
    fn from_readmission(
        readmission: &BackupImportCustodyReadmission,
        rule: AdmittedRestoreEvidenceLayoutRule,
    ) -> Self {
        Self {
            admission: RestoreLayoutAdmission { rule },
            family_id: DurableArtifactFamilyId::ImportBundle,
            access_shape: S8AccessShape::PointLookup,
            posture: Phase28LayoutAuthorityPosture::ReadmissionRequired,
            trigger: readmission.observation().readmission_trigger(),
        }
    }

    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }
    pub const fn declared_access_shape(&self) -> S8AccessShape {
        self.access_shape
    }
    pub const fn authority_posture(&self) -> Phase28LayoutAuthorityPosture {
        self.posture
    }
    pub fn readmission_trigger(&self) -> StoreTrustBoundaryReadmissionTrigger {
        self.trigger.clone()
    }
    pub const fn requires_explicit_readmission(&self) -> bool {
        true
    }
    pub const fn cannot_be_foreground_authority(&self) -> bool {
        true
    }
}

impl BackupImportCustodyReadmission {
    pub fn admit_restore_evidence_layout(&self) -> RestoreLayoutEvidenceReport {
        RestoreLayoutEvidenceReport::from_readmission(
            self,
            phase28_restore_evidence_rule()
                .expect("phase-28 restore evidence rule must stay admitted"),
        )
    }
}
