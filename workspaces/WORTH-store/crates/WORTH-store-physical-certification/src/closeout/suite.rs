use crate::Roadmap2HarnessSequence;

use super::{
    PhysicalSimulationHarnessCloseoutDenial, S45AcceptanceSuiteEvidence, S45AcceptanceSuiteReceipt,
    S45AcceptanceSuiteReceiptSet, S45ExecutedAcceptanceSuiteEvidence,
    S45ExecutedAcceptanceSuiteEvidenceSet,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalSimulationHarnessCloseoutSuite {
    sequence: Roadmap2HarnessSequence,
}

impl PhysicalSimulationHarnessCloseoutSuite {
    pub const fn roadmap2_s45() -> Self {
        Self {
            sequence: Roadmap2HarnessSequence::S45,
        }
    }

    pub const fn sequence(&self) -> Roadmap2HarnessSequence {
        self.sequence
    }

    pub fn execute_required_acceptance_suites(
        &self,
        executed_suites: S45ExecutedAcceptanceSuiteEvidenceSet,
    ) -> Result<S45AcceptanceSuiteReceiptSet, PhysicalSimulationHarnessCloseoutDenial> {
        let receipts = executed_suites
            .into_executed()
            .into_iter()
            .map(executed_acceptance_suite_receipt)
            .collect::<Result<Vec<_>, _>>()?;
        S45AcceptanceSuiteReceiptSet::from_receipts(receipts)
    }
}

fn executed_acceptance_suite_receipt(
    executed: S45ExecutedAcceptanceSuiteEvidence,
) -> Result<S45AcceptanceSuiteReceipt, PhysicalSimulationHarnessCloseoutDenial> {
    let suite_evidence = S45AcceptanceSuiteEvidence::from_executed_suite(executed);
    S45AcceptanceSuiteReceipt::from_suite_evidence(suite_evidence)
}
