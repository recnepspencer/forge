use crate::evidence::{HarnessEvidenceBundle, HarnessEvidenceLedger};
use crate::scenario::HarnessScenarioId;

use super::{HarnessReplayRecord, HarnessScenarioResultLedger};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarnessRunReceipt {
    scenario_id: HarnessScenarioId,
    evidence: HarnessEvidenceBundle,
    evidence_ledger: HarnessEvidenceLedger,
    replay_record: HarnessReplayRecord,
    completed_steps: usize,
}

impl HarnessRunReceipt {
    pub(crate) fn new(
        scenario_id: HarnessScenarioId,
        evidence_ledger: HarnessEvidenceLedger,
        operation_identities: Vec<String>,
        completed_steps: usize,
    ) -> Self {
        let evidence = evidence_ledger.aggregate_evidence();
        let replay_record = HarnessReplayRecord::new(
            scenario_id.clone(),
            operation_identities,
            evidence_ledger.clone(),
        );
        Self {
            scenario_id,
            evidence,
            evidence_ledger,
            replay_record,
            completed_steps,
        }
    }

    pub fn scenario_id(&self) -> &HarnessScenarioId {
        &self.scenario_id
    }

    pub fn evidence(&self) -> &HarnessEvidenceBundle {
        &self.evidence
    }

    pub fn evidence_ledger(&self) -> &HarnessEvidenceLedger {
        &self.evidence_ledger
    }

    pub fn replay_record(&self) -> &HarnessReplayRecord {
        &self.replay_record
    }

    pub fn result_ledger(&self) -> HarnessScenarioResultLedger {
        HarnessScenarioResultLedger::new(self.scenario_id.clone(), self.evidence_ledger.clone())
    }

    pub fn completed_steps(&self) -> usize {
        self.completed_steps
    }

    pub fn assert_complete(&self) {
        assert!(self.completed_steps > 0);
    }
}
