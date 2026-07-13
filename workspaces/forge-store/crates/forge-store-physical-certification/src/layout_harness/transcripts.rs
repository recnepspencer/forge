use super::closeout::LayoutCloseoutEvidenceLane;
use super::coverage::LayoutCoverageRowKind;
use super::scenario::{
    layout_scenario, LayoutScenarioDefinition, LayoutScenarioKind, LayoutTransitionState,
};
use super::shortcut_denials::LayoutShortcutDenialKind;
use forge_store_layout_indexes::ExecutedLayoutOperation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutTranscriptKind {
    ScenarioTranscript,
    ReplayBundle,
    ShortcutDenialTrace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutExecutionAdmissionDenial {
    UnsupportedScenario,
    ScenarioExecutionDeferred,
    MissingProductionApiCoverage,
    MissingTransitionCoverage,
    ScenarioDoesNotReachExecutedEvidence,
    MissingCoverageRows,
    MissingShortcutDenials,
}

#[derive(Debug, PartialEq, Eq)]
pub struct LayoutExecutedScenarioWitness {
    kind: LayoutScenarioKind,
    transcript: LayoutTranscriptKind,
    coverage: &'static [LayoutCoverageRowKind],
    shortcut_denials: &'static [LayoutShortcutDenialKind],
    closeout: LayoutCloseoutEvidenceLane,
    executed_access: ExecutedLayoutOperation,
}

pub fn admit_layout_index_layout_exact_counter_execution(
    executed_access: ExecutedLayoutOperation,
) -> Result<LayoutExecutedScenarioWitness, LayoutExecutionAdmissionDenial> {
    admit_layout_index_layout_executed_scenario(
        layout_scenario(LayoutScenarioKind::ExactCounter),
        executed_access,
    )
}

impl LayoutExecutedScenarioWitness {
    pub const fn kind(&self) -> LayoutScenarioKind {
        self.kind
    }
    pub const fn transcript(&self) -> LayoutTranscriptKind {
        self.transcript
    }
    pub const fn coverage(&self) -> &'static [LayoutCoverageRowKind] {
        self.coverage
    }
    pub const fn shortcut_denials(&self) -> &'static [LayoutShortcutDenialKind] {
        self.shortcut_denials
    }
    pub const fn closeout(&self) -> LayoutCloseoutEvidenceLane {
        self.closeout
    }
    pub const fn executed_access(&self) -> &ExecutedLayoutOperation {
        &self.executed_access
    }
}

fn admit_layout_index_layout_executed_scenario(
    definition: LayoutScenarioDefinition,
    executed_access: ExecutedLayoutOperation,
) -> Result<LayoutExecutedScenarioWitness, LayoutExecutionAdmissionDenial> {
    if definition.kind() != LayoutScenarioKind::ExactCounter {
        return Err(LayoutExecutionAdmissionDenial::ScenarioExecutionDeferred);
    }
    if !definition
        .transitions()
        .contains(&LayoutTransitionState::Executed)
    {
        return Err(LayoutExecutionAdmissionDenial::ScenarioDoesNotReachExecutedEvidence);
    }
    if definition.coverage().is_empty() {
        return Err(LayoutExecutionAdmissionDenial::MissingCoverageRows);
    }
    if definition.shortcut_denials().is_empty() {
        return Err(LayoutExecutionAdmissionDenial::MissingShortcutDenials);
    }
    Ok(LayoutExecutedScenarioWitness {
        kind: definition.kind(),
        transcript: definition.transcript(),
        coverage: definition.coverage(),
        shortcut_denials: definition.shortcut_denials(),
        closeout: definition.closeout(),
        executed_access,
    })
}
