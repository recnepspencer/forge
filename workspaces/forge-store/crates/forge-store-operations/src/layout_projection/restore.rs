use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_layout_indexes::observation::AccessShape;
use forge_store_security::StoreTrustBoundaryReadmissionTrigger;

use crate::BackupImportCustodyReadmission;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreLayoutEvidenceReport {
    family_id: DurableArtifactFamilyId,
    access_shape: AccessShape,
    trigger: StoreTrustBoundaryReadmissionTrigger,
}

impl RestoreLayoutEvidenceReport {
    fn from_readmission(readmission: &BackupImportCustodyReadmission) -> Self {
        Self {
            family_id: DurableArtifactFamilyId::ImportBundle,
            access_shape: AccessShape::PointLookup,
            trigger: readmission.observation().readmission_trigger(),
        }
    }

    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }
    pub const fn declared_access_shape(&self) -> AccessShape {
        self.access_shape
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
    pub fn project_restore_evidence_layout(&self) -> RestoreLayoutEvidenceReport {
        RestoreLayoutEvidenceReport::from_readmission(self)
    }
}
