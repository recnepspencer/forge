use forge_store_layout_indexes::ExecutedLayoutOperation;
use forge_store_physical_certification::layout_harness::{
    scenario::LayoutScenarioKind, transcripts::LayoutExecutedScenarioWitness,
};

#[derive(Debug, PartialEq, Eq)]
pub struct LayoutScenarioCertificate {
    executed: LayoutExecutedScenarioWitness,
}

pub fn certify_layout_index_layout_scenario(
    executed: LayoutExecutedScenarioWitness,
) -> LayoutScenarioCertificate {
    LayoutScenarioCertificate { executed }
}

impl LayoutScenarioCertificate {
    pub const fn kind(&self) -> LayoutScenarioKind {
        self.executed.kind()
    }
    pub const fn executed_access(&self) -> &ExecutedLayoutOperation {
        self.executed.executed_access()
    }
}
