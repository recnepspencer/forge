use serde::Serialize;

use super::classification::{PlacementBudgetClass, PlacementExecutionOrigin, TierResidenceClass};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TierPlacementEvidence {
    residence_class: TierResidenceClass,
    budget_class: PlacementBudgetClass,
    execution_origin: PlacementExecutionOrigin,
}

impl TierPlacementEvidence {
    pub(crate) fn new(
        residence_class: TierResidenceClass,
        budget_class: PlacementBudgetClass,
        execution_origin: PlacementExecutionOrigin,
    ) -> Self {
        Self {
            residence_class,
            budget_class,
            execution_origin,
        }
    }

    pub fn residence_class(&self) -> TierResidenceClass {
        self.residence_class
    }

    pub fn budget_class(&self) -> PlacementBudgetClass {
        self.budget_class
    }

    pub fn execution_origin(&self) -> PlacementExecutionOrigin {
        self.execution_origin
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlacementNonAuthorityWitness {
    artifact_key: String,
}

impl PlacementNonAuthorityWitness {
    pub(crate) fn new(artifact_key: impl Into<String>) -> Self {
        Self {
            artifact_key: artifact_key.into(),
        }
    }

    pub fn artifact_key(&self) -> &str {
        &self.artifact_key
    }
}
