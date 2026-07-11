use super::closeout::S8LayoutCloseoutEvidenceLane;
use super::coverage::S8LayoutCoverageRowKind;
use super::scenario::{
    layout_scenario, S8LayoutScenarioDefinition, S8LayoutScenarioKind, S8LayoutTransitionState,
};
use super::shortcut_denials::S8LayoutShortcutDenialKind;
use forge_store_layout_indexes::access_execution::S8ExecutedAccessReceipt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum S8LayoutTranscriptKind {
    ScenarioTranscript,
    ReplayBundle,
    ShortcutDenialTrace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8LayoutExecutionAdmissionDenial {
    UnsupportedScenario,
    ScenarioExecutionDeferred,
    MissingProductionApiCoverage,
    MissingTransitionCoverage,
    ScenarioDoesNotReachExecutedEvidence,
    MissingCoverageRows,
    MissingShortcutDenials,
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8LayoutExecutedScenarioWitness {
    kind: S8LayoutScenarioKind,
    transcript: S8LayoutTranscriptKind,
    coverage: &'static [S8LayoutCoverageRowKind],
    shortcut_denials: &'static [S8LayoutShortcutDenialKind],
    closeout: S8LayoutCloseoutEvidenceLane,
    executed_access: S8ExecutedAccessReceipt,
}

pub fn admit_layout_index_layout_exact_counter_execution(
    executed_access: S8ExecutedAccessReceipt,
) -> Result<S8LayoutExecutedScenarioWitness, S8LayoutExecutionAdmissionDenial> {
    admit_layout_index_layout_executed_scenario(
        layout_scenario(S8LayoutScenarioKind::ExactCounter),
        executed_access,
    )
}

impl S8LayoutExecutedScenarioWitness {
    pub const fn kind(&self) -> S8LayoutScenarioKind {
        self.kind
    }
    pub const fn transcript(&self) -> S8LayoutTranscriptKind {
        self.transcript
    }
    pub const fn coverage(&self) -> &'static [S8LayoutCoverageRowKind] {
        self.coverage
    }
    pub const fn shortcut_denials(&self) -> &'static [S8LayoutShortcutDenialKind] {
        self.shortcut_denials
    }
    pub const fn closeout(&self) -> S8LayoutCloseoutEvidenceLane {
        self.closeout
    }
    pub const fn executed_access(&self) -> &S8ExecutedAccessReceipt {
        &self.executed_access
    }
}

fn admit_layout_index_layout_executed_scenario(
    definition: S8LayoutScenarioDefinition,
    executed_access: S8ExecutedAccessReceipt,
) -> Result<S8LayoutExecutedScenarioWitness, S8LayoutExecutionAdmissionDenial> {
    if definition.kind() != S8LayoutScenarioKind::ExactCounter {
        return Err(S8LayoutExecutionAdmissionDenial::ScenarioExecutionDeferred);
    }
    if !definition
        .transitions()
        .contains(&S8LayoutTransitionState::Executed)
    {
        return Err(S8LayoutExecutionAdmissionDenial::ScenarioDoesNotReachExecutedEvidence);
    }
    if definition.coverage().is_empty() {
        return Err(S8LayoutExecutionAdmissionDenial::MissingCoverageRows);
    }
    if definition.shortcut_denials().is_empty() {
        return Err(S8LayoutExecutionAdmissionDenial::MissingShortcutDenials);
    }
    Ok(S8LayoutExecutedScenarioWitness {
        kind: definition.kind(),
        transcript: definition.transcript(),
        coverage: definition.coverage(),
        shortcut_denials: definition.shortcut_denials(),
        closeout: definition.closeout(),
        executed_access,
    })
}
