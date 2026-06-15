use crate::evidence::HarnessEvidenceLedger;
use crate::scenario::HarnessScenarioId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarnessScenarioResultLedger {
    scenario_id: HarnessScenarioId,
    evidence: HarnessEvidenceLedger,
}

impl HarnessScenarioResultLedger {
    pub(crate) fn new(scenario_id: HarnessScenarioId, evidence: HarnessEvidenceLedger) -> Self {
        Self {
            scenario_id,
            evidence,
        }
    }

    pub fn scenario_id(&self) -> &HarnessScenarioId {
        &self.scenario_id
    }

    pub fn evidence(&self) -> &HarnessEvidenceLedger {
        &self.evidence
    }
}
