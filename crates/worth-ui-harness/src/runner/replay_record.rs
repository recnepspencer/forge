use crate::evidence::HarnessEvidenceLedger;
use crate::scenario::HarnessScenarioId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarnessReplayRecord {
    scenario_id: HarnessScenarioId,
    operation_identities: Vec<String>,
    evidence_ledger: HarnessEvidenceLedger,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HarnessReplayDenial {
    ScenarioIdentityChanged {
        expected: HarnessScenarioId,
        provided: HarnessScenarioId,
    },
    OperationIdentityChanged {
        step_index: usize,
        expected: String,
        provided: String,
    },
    OperationCountChanged {
        expected: usize,
        provided: usize,
    },
    EvidenceLedgerChanged,
}

impl HarnessReplayRecord {
    pub(crate) fn new(
        scenario_id: HarnessScenarioId,
        operation_identities: Vec<String>,
        evidence_ledger: HarnessEvidenceLedger,
    ) -> Self {
        Self {
            scenario_id,
            operation_identities,
            evidence_ledger,
        }
    }

    pub fn scenario_id(&self) -> &HarnessScenarioId {
        &self.scenario_id
    }

    pub fn operation_identities(&self) -> &[String] {
        &self.operation_identities
    }

    pub fn evidence_ledger(&self) -> &HarnessEvidenceLedger {
        &self.evidence_ledger
    }

    pub(crate) fn validate_replay(&self, replay: &Self) -> Result<(), HarnessReplayDenial> {
        if self.scenario_id != replay.scenario_id {
            return Err(HarnessReplayDenial::ScenarioIdentityChanged {
                expected: self.scenario_id.clone(),
                provided: replay.scenario_id.clone(),
            });
        }
        if self.operation_identities.len() != replay.operation_identities.len() {
            return Err(HarnessReplayDenial::OperationCountChanged {
                expected: self.operation_identities.len(),
                provided: replay.operation_identities.len(),
            });
        }
        for (step_index, expected_identity) in self.operation_identities.iter().enumerate() {
            match replay.operation_identities.get(step_index) {
                Some(provided_identity) if provided_identity == expected_identity => {}
                Some(provided_identity) => {
                    return Err(HarnessReplayDenial::OperationIdentityChanged {
                        step_index,
                        expected: expected_identity.clone(),
                        provided: provided_identity.clone(),
                    });
                }
                None => unreachable!("operation count was validated before identity comparison"),
            }
        }
        if self.evidence_ledger != replay.evidence_ledger {
            return Err(HarnessReplayDenial::EvidenceLedgerChanged);
        }
        Ok(())
    }
}
