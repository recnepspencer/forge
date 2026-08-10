use serde::Serialize;

use super::super::{
    AdaptivePlacementDebtMarker, PlacementExecutionOrigin, PlacementObservationScopeClass,
    RecallAmplificationBudget, RecallCostClass,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RecallPreparationPlan {
    artifact_key: String,
    recall_cost_class: RecallCostClass,
    amplification_budget: RecallAmplificationBudget,
    execution_origin: PlacementExecutionOrigin,
}

impl RecallPreparationPlan {
    pub(crate) fn new(
        artifact_key: impl Into<String>,
        recall_cost_class: RecallCostClass,
        amplification_budget: RecallAmplificationBudget,
        execution_origin: PlacementExecutionOrigin,
    ) -> Self {
        Self {
            artifact_key: artifact_key.into(),
            recall_cost_class,
            amplification_budget,
            execution_origin,
        }
    }

    pub fn artifact_key(&self) -> &str {
        &self.artifact_key
    }

    pub fn recall_cost_class(&self) -> RecallCostClass {
        self.recall_cost_class
    }

    pub fn amplification_budget(&self) -> RecallAmplificationBudget {
        self.amplification_budget
    }

    pub fn execution_origin(&self) -> PlacementExecutionOrigin {
        self.execution_origin
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ColdRecallPlan {
    artifact_key: String,
    recall_cost_class: RecallCostClass,
    amplification_budget: RecallAmplificationBudget,
    execution_origin: PlacementExecutionOrigin,
}

impl ColdRecallPlan {
    pub(crate) fn new(
        artifact_key: impl Into<String>,
        recall_cost_class: RecallCostClass,
        amplification_budget: RecallAmplificationBudget,
        execution_origin: PlacementExecutionOrigin,
    ) -> Self {
        Self {
            artifact_key: artifact_key.into(),
            recall_cost_class,
            amplification_budget,
            execution_origin,
        }
    }

    pub fn artifact_key(&self) -> &str {
        &self.artifact_key
    }

    pub fn recall_cost_class(&self) -> RecallCostClass {
        self.recall_cost_class
    }

    pub fn amplification_budget(&self) -> RecallAmplificationBudget {
        self.amplification_budget
    }

    pub fn execution_origin(&self) -> PlacementExecutionOrigin {
        self.execution_origin
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BroadenedRecallPlan {
    scope_class: PlacementObservationScopeClass,
    scope_key: String,
    widened_artifact_keys: Vec<String>,
    execution_origin: PlacementExecutionOrigin,
}

impl BroadenedRecallPlan {
    pub(crate) fn new(
        scope_class: PlacementObservationScopeClass,
        scope_key: impl Into<String>,
        mut widened_artifact_keys: Vec<String>,
        execution_origin: PlacementExecutionOrigin,
    ) -> Self {
        widened_artifact_keys.sort();
        widened_artifact_keys.dedup();
        Self {
            scope_class,
            scope_key: scope_key.into(),
            widened_artifact_keys,
            execution_origin,
        }
    }

    pub fn scope_class(&self) -> PlacementObservationScopeClass {
        self.scope_class
    }

    pub fn scope_key(&self) -> &str {
        &self.scope_key
    }

    pub fn widened_artifact_keys(&self) -> &[String] {
        &self.widened_artifact_keys
    }

    pub fn execution_origin(&self) -> PlacementExecutionOrigin {
        self.execution_origin
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RecallDebtSummary {
    reason: String,
}

impl RecallDebtSummary {
    pub(crate) fn new(reason: impl Into<String>) -> Self {
        Self {
            reason: reason.into(),
        }
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RecallBreadthSummary {
    family_local_unit_count: u64,
    widened_artifact_count: u64,
}

impl RecallBreadthSummary {
    pub(crate) fn new(family_local_unit_count: u64, widened_artifact_count: u64) -> Self {
        Self {
            family_local_unit_count,
            widened_artifact_count,
        }
    }

    pub fn family_local_unit_count(&self) -> u64 {
        self.family_local_unit_count
    }

    pub fn widened_artifact_count(&self) -> u64 {
        self.widened_artifact_count
    }
}
