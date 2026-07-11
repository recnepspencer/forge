use super::{S8LayoutCloseoutDenial, S8LayoutCloseoutSources};
use forge_store_physical_certification::layout_harness::scenario::{
    layout_scenario, S8LayoutTransitionState,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LayoutCloseoutVerifier {
    covered_rows: usize,
    denied_shortcuts: usize,
}

pub fn verify_s8_layout_closeout(
    sources: &S8LayoutCloseoutSources,
) -> Result<S8LayoutCloseoutVerifier, S8LayoutCloseoutDenial> {
    let canonical = layout_scenario(sources.scenario().scenario().kind());
    if canonical.production_apis().is_empty()
        || canonical.actors().is_empty()
        || canonical.faults().is_empty()
        || canonical.observers().is_empty()
        || canonical.oracles().is_empty()
        || canonical.transitions().is_empty()
    {
        return Err(S8LayoutCloseoutDenial::IncompleteCanonicalScenarioVocabulary);
    }
    if !canonical
        .transitions()
        .contains(&S8LayoutTransitionState::Executed)
    {
        return Err(S8LayoutCloseoutDenial::ScenarioDoesNotReachExecutedEvidence);
    }
    if sources.transcript() != canonical.transcript() {
        return Err(S8LayoutCloseoutDenial::CanonicalTranscriptMismatch);
    }
    if sources.closeout_lane() != canonical.closeout() {
        return Err(S8LayoutCloseoutDenial::CanonicalCloseoutLaneMismatch);
    }
    if sources.coverage() != canonical.coverage() || sources.coverage().is_empty() {
        return Err(S8LayoutCloseoutDenial::CanonicalCoverageMismatch);
    }
    if sources.scenario().shortcut_denials() != canonical.shortcut_denials() {
        return Err(S8LayoutCloseoutDenial::CanonicalShortcutDenialMismatch);
    }
    let denied_shortcuts = canonical.shortcut_denials().len();
    if denied_shortcuts == 0 {
        return Err(S8LayoutCloseoutDenial::ShortcutDenialsRequired);
    }
    Ok(S8LayoutCloseoutVerifier {
        covered_rows: canonical.coverage().len(),
        denied_shortcuts,
    })
}

impl S8LayoutCloseoutVerifier {
    pub const fn covered_rows(&self) -> usize {
        self.covered_rows
    }
    pub const fn denied_shortcuts(&self) -> usize {
        self.denied_shortcuts
    }
}
