use forge_store_layout_indexes::access_execution::S8ExecutedAccessReceipt;
use forge_store_physical_certification::layout_harness::coverage::S8LayoutCoverageRowKind;
use forge_store_physical_certification::layout_harness::shortcut_denials::S8LayoutShortcutDenialKind;
use forge_store_physical_certification::layout_harness::transcripts::{
    S8LayoutExecutedScenarioWitness, S8LayoutTranscriptKind,
};

#[derive(Debug, PartialEq, Eq)]
pub struct S8LayoutReplayBundle {
    scenario: S8LayoutExecutedScenarioWitness,
}

pub fn assemble_s8_layout_replay_bundle(
    scenario: S8LayoutExecutedScenarioWitness,
) -> S8LayoutReplayBundle {
    S8LayoutReplayBundle { scenario }
}

impl S8LayoutReplayBundle {
    pub const fn scenario(&self) -> &S8LayoutExecutedScenarioWitness {
        &self.scenario
    }
    pub const fn transcript(&self) -> S8LayoutTranscriptKind {
        self.scenario.transcript()
    }
    pub const fn coverage(&self) -> &'static [S8LayoutCoverageRowKind] {
        self.scenario.coverage()
    }
    pub const fn shortcut_denials(&self) -> &'static [S8LayoutShortcutDenialKind] {
        self.scenario.shortcut_denials()
    }
    pub const fn executed_access(&self) -> &S8ExecutedAccessReceipt {
        self.scenario.executed_access()
    }
}
