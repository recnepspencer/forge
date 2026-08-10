use serde::Serialize;
use worth_relational::facade::history::BranchId;

use super::super::{PlacementObservationScopeClass, TierResidenceClass};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TierLocalityFootprint {
    scope_class: PlacementObservationScopeClass,
    scope_key: String,
    artifact_keys: Vec<String>,
}

impl TierLocalityFootprint {
    pub(crate) fn new(
        scope_class: PlacementObservationScopeClass,
        scope_key: impl Into<String>,
        mut artifact_keys: Vec<String>,
    ) -> Self {
        artifact_keys.sort();
        artifact_keys.dedup();
        Self {
            scope_class,
            scope_key: scope_key.into(),
            artifact_keys,
        }
    }

    pub fn scope_class(&self) -> PlacementObservationScopeClass {
        self.scope_class
    }

    pub fn scope_key(&self) -> &str {
        &self.scope_key
    }

    pub fn artifact_keys(&self) -> &[String] {
        &self.artifact_keys
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FamilyLocalPlacementPlan {
    locality_footprint: TierLocalityFootprint,
    target_residence: TierResidenceClass,
}

impl FamilyLocalPlacementPlan {
    pub(crate) fn new(
        locality_footprint: TierLocalityFootprint,
        target_residence: TierResidenceClass,
    ) -> Self {
        Self {
            locality_footprint,
            target_residence,
        }
    }

    pub fn locality_footprint(&self) -> &TierLocalityFootprint {
        &self.locality_footprint
    }

    pub fn target_residence(&self) -> TierResidenceClass {
        self.target_residence
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RetainedRangePlacementPlan {
    branch_id: BranchId,
    retained_basis_label: String,
    target_residence: TierResidenceClass,
}

impl RetainedRangePlacementPlan {
    pub(crate) fn new(
        branch_id: BranchId,
        retained_basis_label: impl Into<String>,
        target_residence: TierResidenceClass,
    ) -> Self {
        Self {
            branch_id,
            retained_basis_label: retained_basis_label.into(),
            target_residence,
        }
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn retained_basis_label(&self) -> &str {
        &self.retained_basis_label
    }

    pub fn target_residence(&self) -> TierResidenceClass {
        self.target_residence
    }
}
