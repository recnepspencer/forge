#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationEntryOrchestrationArtifactPolicy {
    OrdinaryEnvelopeOnly,
    CheckedOutcomeOnly,
    ProofVisibleTranscript,
}

impl WorthQueryDeclarationEntryOrchestrationArtifactPolicy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OrdinaryEnvelopeOnly => "ordinary_envelope_only",
            Self::CheckedOutcomeOnly => "checked_outcome_only",
            Self::ProofVisibleTranscript => "proof_visible_transcript",
        }
    }
}
