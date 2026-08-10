use serde::Serialize;
use worth_relational::facade::history::BranchId;

use super::classification::TierResidenceClass;
use worth_store_contracts::PlacementArtifactFamily;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuthoritativeTierResidency {
    branch_id: BranchId,
    residence_class: TierResidenceClass,
}

impl AuthoritativeTierResidency {
    pub fn new(branch_id: BranchId, residence_class: TierResidenceClass) -> Self {
        Self {
            branch_id,
            residence_class,
        }
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn residence_class(&self) -> TierResidenceClass {
        self.residence_class
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedTierResidency {
    artifact_family: PlacementArtifactFamily,
    artifact_id: String,
    residence_class: TierResidenceClass,
}

impl DerivedTierResidency {
    pub fn new(
        artifact_family: PlacementArtifactFamily,
        artifact_id: impl Into<String>,
        residence_class: TierResidenceClass,
    ) -> Self {
        Self {
            artifact_family,
            artifact_id: artifact_id.into(),
            residence_class,
        }
    }

    pub fn artifact_family(&self) -> PlacementArtifactFamily {
        self.artifact_family
    }

    pub fn artifact_id(&self) -> &str {
        &self.artifact_id
    }

    pub fn residence_class(&self) -> TierResidenceClass {
        self.residence_class
    }
}
