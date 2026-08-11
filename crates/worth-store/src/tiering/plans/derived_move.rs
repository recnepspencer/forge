use serde::Serialize;

use super::super::{
    PlacementArtifactFamily, PlacementBudgetClass, PlacementExecutionOrigin, TierResidenceClass,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedTierMovePlan {
    artifact_family: PlacementArtifactFamily,
    artifact_id: String,
    target_residence: TierResidenceClass,
    budget_class: PlacementBudgetClass,
    execution_origin: PlacementExecutionOrigin,
}

impl DerivedTierMovePlan {
    pub(crate) fn new(
        artifact_family: PlacementArtifactFamily,
        artifact_id: impl Into<String>,
        target_residence: TierResidenceClass,
        budget_class: PlacementBudgetClass,
        execution_origin: PlacementExecutionOrigin,
    ) -> Self {
        Self {
            artifact_family,
            artifact_id: artifact_id.into(),
            target_residence,
            budget_class,
            execution_origin,
        }
    }

    pub fn artifact_family(&self) -> PlacementArtifactFamily {
        self.artifact_family
    }

    pub fn artifact_id(&self) -> &str {
        &self.artifact_id
    }

    pub fn target_residence(&self) -> TierResidenceClass {
        self.target_residence
    }

    pub fn budget_class(&self) -> PlacementBudgetClass {
        self.budget_class
    }

    pub fn execution_origin(&self) -> PlacementExecutionOrigin {
        self.execution_origin
    }
}
