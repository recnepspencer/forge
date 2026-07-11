use crate::courtroom::layout::runtime_matrix::LayoutRuntimeCompletenessDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8LayoutCloseoutDenial {
    RuntimeMatrixIncomplete(LayoutRuntimeCompletenessDenial),
    MissingCoverageRows,
    ShortcutDenialsRequired,
    CanonicalTranscriptMismatch,
    CanonicalCoverageMismatch,
    CanonicalShortcutDenialMismatch,
    CanonicalCloseoutLaneMismatch,
    IncompleteCanonicalScenarioVocabulary,
    CanonicalScenarioInventoryMismatch,
    ScenarioDoesNotReachExecutedEvidence,
    HandoffPlanBindingMismatch,
}
