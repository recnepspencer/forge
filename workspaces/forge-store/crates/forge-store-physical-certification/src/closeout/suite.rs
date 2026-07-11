use crate::HarnessCoverageStage;

use super::{
    ExecutedSimulationHarnessAcceptanceSuiteEvidence,
    ExecutedSimulationHarnessAcceptanceSuiteEvidenceSet, PhysicalSimulationHarnessCloseoutDenial,
    SimulationHarnessAcceptanceSuiteEvidence, SimulationHarnessAcceptanceSuiteReceipt,
    SimulationHarnessAcceptanceSuiteReceiptSet,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalSimulationHarnessCloseoutSuite {
    sequence: HarnessCoverageStage,
}

impl PhysicalSimulationHarnessCloseoutSuite {
    pub const fn simulation_admission() -> Self {
        Self {
            sequence: HarnessCoverageStage::SimulationAdmission,
        }
    }

    pub const fn sequence(&self) -> HarnessCoverageStage {
        self.sequence
    }

    pub fn execute_required_acceptance_suites(
        &self,
        executed_suites: ExecutedSimulationHarnessAcceptanceSuiteEvidenceSet,
    ) -> Result<SimulationHarnessAcceptanceSuiteReceiptSet, PhysicalSimulationHarnessCloseoutDenial>
    {
        let receipts = executed_suites
            .into_executed()
            .into_iter()
            .map(executed_acceptance_suite_receipt)
            .collect::<Result<Vec<_>, _>>()?;
        SimulationHarnessAcceptanceSuiteReceiptSet::from_receipts(receipts)
    }
}

fn executed_acceptance_suite_receipt(
    executed: ExecutedSimulationHarnessAcceptanceSuiteEvidence,
) -> Result<SimulationHarnessAcceptanceSuiteReceipt, PhysicalSimulationHarnessCloseoutDenial> {
    let suite_evidence = SimulationHarnessAcceptanceSuiteEvidence::from_executed_suite(executed);
    SimulationHarnessAcceptanceSuiteReceipt::from_suite_evidence(suite_evidence)
}
