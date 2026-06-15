use crate::scenario::HarnessScenarioId;

use super::HarnessEvidenceFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarnessFailureLocation {
    scenario_id: HarnessScenarioId,
    step_index: usize,
    step_label: String,
    evidence_family: Option<HarnessEvidenceFamily>,
}

impl HarnessFailureLocation {
    pub(crate) fn new(
        scenario_id: HarnessScenarioId,
        step_index: usize,
        step_label: impl Into<String>,
        evidence_family: Option<HarnessEvidenceFamily>,
    ) -> Self {
        Self {
            scenario_id,
            step_index,
            step_label: step_label.into(),
            evidence_family,
        }
    }

    pub fn scenario_id(&self) -> &HarnessScenarioId {
        &self.scenario_id
    }

    pub fn step_index(&self) -> usize {
        self.step_index
    }

    pub fn step_label(&self) -> &str {
        &self.step_label
    }

    pub fn evidence_family(&self) -> Option<HarnessEvidenceFamily> {
        self.evidence_family
    }
}
