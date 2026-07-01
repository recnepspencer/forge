#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SimulationEvidencePolicy {
    MinimalReplayable,
    DiagnosticReplayable,
    FutureExtensionSlot,
}

impl SimulationEvidencePolicy {
    pub const fn minimal_replayable() -> Self {
        Self::MinimalReplayable
    }

    pub const fn diagnostic_replayable() -> Self {
        Self::DiagnosticReplayable
    }
}

pub(crate) fn evidence_policy_token(policy: SimulationEvidencePolicy) -> &'static str {
    match policy {
        SimulationEvidencePolicy::MinimalReplayable => "minimal-replayable",
        SimulationEvidencePolicy::DiagnosticReplayable => "diagnostic-replayable",
        SimulationEvidencePolicy::FutureExtensionSlot => "future-extension-slot",
    }
}
