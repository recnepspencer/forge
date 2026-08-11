use serde::Serialize;

use super::super::{PlacementBudgetClass, PlacementExecutionOrigin, TierResidenceClass};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuthoritativeTierMovePlan {
    artifact_key: String,
    target_residence: TierResidenceClass,
    budget_class: PlacementBudgetClass,
    execution_origin: PlacementExecutionOrigin,
}

impl AuthoritativeTierMovePlan {
    pub(crate) fn new(
        artifact_key: impl Into<String>,
        target_residence: TierResidenceClass,
        budget_class: PlacementBudgetClass,
        execution_origin: PlacementExecutionOrigin,
    ) -> Self {
        Self {
            artifact_key: artifact_key.into(),
            target_residence,
            budget_class,
            execution_origin,
        }
    }

    pub fn artifact_key(&self) -> &str {
        &self.artifact_key
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
