use crate::courtroom::replay::s8_layout::S8LayoutReplayBundle;
use forge_store_physical_certification::layout_harness::closeout::S8LayoutCloseoutEvidenceLane;
use forge_store_physical_certification::layout_harness::coverage::S8LayoutCoverageRowKind;
use forge_store_physical_certification::layout_harness::scenario::layout_scenario;
use forge_store_physical_certification::layout_harness::transcripts::S8LayoutTranscriptKind;

#[derive(Debug, PartialEq, Eq)]
pub struct S8LayoutCloseoutSources {
    scenario: S8LayoutReplayBundle,
    transcript: S8LayoutTranscriptKind,
    coverage: &'static [S8LayoutCoverageRowKind],
    closeout_lane: S8LayoutCloseoutEvidenceLane,
}

pub fn s8_layout_closeout_sources(replay: S8LayoutReplayBundle) -> S8LayoutCloseoutSources {
    let canonical = layout_scenario(replay.scenario().kind());
    let closeout_lane = canonical.closeout();
    let transcript = replay.transcript();
    let coverage = canonical.coverage();
    S8LayoutCloseoutSources {
        scenario: replay,
        transcript,
        coverage,
        closeout_lane,
    }
}

impl S8LayoutCloseoutSources {
    pub const fn scenario(&self) -> &S8LayoutReplayBundle {
        &self.scenario
    }
    pub const fn transcript(&self) -> S8LayoutTranscriptKind {
        self.transcript
    }
    pub const fn coverage(&self) -> &'static [S8LayoutCoverageRowKind] {
        self.coverage
    }
    pub const fn closeout_lane(&self) -> S8LayoutCloseoutEvidenceLane {
        self.closeout_lane
    }
}
