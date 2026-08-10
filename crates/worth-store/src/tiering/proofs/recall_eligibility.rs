use serde::Serialize;

use super::classification::{RecallAmplificationBudget, RecallCostClass};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RecallEligibilityWitness {
    artifact_key: String,
    recall_cost_class: RecallCostClass,
    amplification_budget: RecallAmplificationBudget,
}

impl RecallEligibilityWitness {
    pub(crate) fn new(
        artifact_key: impl Into<String>,
        recall_cost_class: RecallCostClass,
        amplification_budget: RecallAmplificationBudget,
    ) -> Self {
        Self {
            artifact_key: artifact_key.into(),
            recall_cost_class,
            amplification_budget,
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
}
