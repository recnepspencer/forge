use serde::Serialize;

use super::super::manifests::{ArtifactFamilyId, ArtifactSemanticVersion};

pub struct RetainedAuthorityCompatibilityWitness {
    family_id: ArtifactFamilyId,
}

impl RetainedAuthorityCompatibilityWitness {
    pub(crate) fn new(family_id: ArtifactFamilyId) -> Self {
        Self { family_id }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityMaintenanceAdmissionWitness {
    family_id: ArtifactFamilyId,
    pub(super) compatibility_lane_id: String,
    maintenance_lane_id: String,
    pub(super) maintenance_work_class_label: String,
}

impl CompatibilityMaintenanceAdmissionWitness {
    pub(crate) fn new(family_id: ArtifactFamilyId, maintenance_lane_id: impl Into<String>) -> Self {
        Self {
            family_id,
            compatibility_lane_id: "compatibility.derived.legacy".to_string(),
            maintenance_lane_id: maintenance_lane_id.into(),
            maintenance_work_class_label: "DerivedFamilyRebuild".to_string(),
        }
    }

    pub(crate) fn for_lane(
        family_id: ArtifactFamilyId,
        compatibility_lane_id: impl Into<String>,
        maintenance_lane_id: impl Into<String>,
        maintenance_work_class_label: impl Into<String>,
    ) -> Self {
        Self {
            family_id,
            compatibility_lane_id: compatibility_lane_id.into(),
            maintenance_lane_id: maintenance_lane_id.into(),
            maintenance_work_class_label: maintenance_work_class_label.into(),
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn maintenance_lane_id(&self) -> &str {
        &self.maintenance_lane_id
    }

    pub fn compatibility_lane_id(&self) -> &str {
        &self.compatibility_lane_id
    }

    pub fn maintenance_work_class_label(&self) -> &str {
        &self.maintenance_work_class_label
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityMaintenanceLaneRequirement {
    pub(super) family_id: ArtifactFamilyId,
    pub(super) compatibility_lane_id: String,
    pub(super) maintenance_work_class_label: String,
}

impl CompatibilityMaintenanceLaneRequirement {
    pub fn new(
        family_id: ArtifactFamilyId,
        compatibility_lane_id: impl Into<String>,
        maintenance_work_class_label: impl Into<String>,
    ) -> Self {
        Self {
            family_id,
            compatibility_lane_id: compatibility_lane_id.into(),
            maintenance_work_class_label: maintenance_work_class_label.into(),
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityMaintenanceLaneAdmission {
    witness: CompatibilityMaintenanceAdmissionWitness,
}

impl CompatibilityMaintenanceLaneAdmission {
    pub(crate) fn new(witness: CompatibilityMaintenanceAdmissionWitness) -> Self {
        Self { witness }
    }

    pub fn witness(&self) -> &CompatibilityMaintenanceAdmissionWitness {
        &self.witness
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityMaintenanceLaneRejection {
    family_id: ArtifactFamilyId,
    reason: String,
}

impl CompatibilityMaintenanceLaneRejection {
    pub(crate) fn new(family_id: ArtifactFamilyId, reason: impl Into<String>) -> Self {
        Self {
            family_id,
            reason: reason.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedRebuildCompatibilityPlan {
    family_id: ArtifactFamilyId,
    source_semantic_version: ArtifactSemanticVersion,
    target_semantic_version: ArtifactSemanticVersion,
    maintenance_lane_id: String,
}

impl DerivedRebuildCompatibilityPlan {
    pub(crate) fn new(
        family_id: ArtifactFamilyId,
        source_semantic_version: ArtifactSemanticVersion,
        target_semantic_version: ArtifactSemanticVersion,
        maintenance_lane_id: impl Into<String>,
    ) -> Self {
        Self {
            family_id,
            source_semantic_version,
            target_semantic_version,
            maintenance_lane_id: maintenance_lane_id.into(),
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn target_semantic_version(&self) -> ArtifactSemanticVersion {
        self.target_semantic_version
    }

    pub fn maintenance_lane_id(&self) -> &str {
        &self.maintenance_lane_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityRebuildDebt {
    family_id: ArtifactFamilyId,
    debt_record_count: u64,
}

impl CompatibilityRebuildDebt {
    pub(crate) fn new(family_id: ArtifactFamilyId, debt_record_count: u64) -> Self {
        Self {
            family_id,
            debt_record_count,
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn debt_record_count(&self) -> u64 {
        self.debt_record_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StaleDerivedVersionRejection {
    family_id: ArtifactFamilyId,
    observed_semantic_version: ArtifactSemanticVersion,
}

impl StaleDerivedVersionRejection {
    pub(crate) fn new(
        family_id: ArtifactFamilyId,
        observed_semantic_version: ArtifactSemanticVersion,
    ) -> Self {
        Self {
            family_id,
            observed_semantic_version,
        }
    }
}
