use crate::s8_runtime_matrix::S8RuntimeMatrixDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8LayoutCloseoutDenial {
    RuntimeMatrixIncomplete(S8RuntimeMatrixDenial),
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
