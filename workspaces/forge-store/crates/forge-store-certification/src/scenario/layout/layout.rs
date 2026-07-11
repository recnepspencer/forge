use forge_store_layout_indexes::access_execution::S8ExecutedAccessReceipt;
use forge_store_physical_certification::layout_harness::{
    scenario::S8LayoutScenarioKind, transcripts::S8LayoutExecutedScenarioWitness,
};

#[derive(Debug, PartialEq, Eq)]
pub struct S8LayoutScenarioCertificate {
    executed: S8LayoutExecutedScenarioWitness,
}

pub fn certify_s8_layout_scenario(
    executed: S8LayoutExecutedScenarioWitness,
) -> S8LayoutScenarioCertificate {
    S8LayoutScenarioCertificate { executed }
}

impl S8LayoutScenarioCertificate {
    pub const fn kind(&self) -> S8LayoutScenarioKind {
        self.executed.kind()
    }
    pub const fn executed_access(&self) -> &S8ExecutedAccessReceipt {
        self.executed.executed_access()
    }
}
