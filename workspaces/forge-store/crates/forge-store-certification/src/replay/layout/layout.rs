use forge_store_layout_indexes::ExecutedLayoutOperation;
use forge_store_physical_certification::layout_harness::coverage::LayoutCoverageRowKind;
use forge_store_physical_certification::layout_harness::shortcut_denials::LayoutShortcutDenialKind;
use forge_store_physical_certification::layout_harness::transcripts::{
    LayoutExecutedScenarioWitness, LayoutTranscriptKind,
};

#[derive(Debug, PartialEq, Eq)]
pub struct LayoutReplayBundle {
    scenario: LayoutExecutedScenarioWitness,
}

pub fn assemble_layout_index_layout_replay_bundle(
    scenario: LayoutExecutedScenarioWitness,
) -> LayoutReplayBundle {
    LayoutReplayBundle { scenario }
}

impl LayoutReplayBundle {
    pub const fn scenario(&self) -> &LayoutExecutedScenarioWitness {
        &self.scenario
    }
    pub const fn transcript(&self) -> LayoutTranscriptKind {
        self.scenario.transcript()
    }
    pub const fn coverage(&self) -> &'static [LayoutCoverageRowKind] {
        self.scenario.coverage()
    }
    pub const fn shortcut_denials(&self) -> &'static [LayoutShortcutDenialKind] {
        self.scenario.shortcut_denials()
    }
    pub const fn executed_access(&self) -> &ExecutedLayoutOperation {
        self.scenario.executed_access()
    }
}
