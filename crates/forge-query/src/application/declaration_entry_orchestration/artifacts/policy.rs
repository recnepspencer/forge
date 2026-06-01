#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationEntryOrchestrationArtifactPolicy {
    OrdinaryEnvelopeOnly,
    CheckedOutcomeOnly,
    ProofVisibleTranscript,
}

impl ForgeQueryDeclarationEntryOrchestrationArtifactPolicy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OrdinaryEnvelopeOnly => "ordinary_envelope_only",
            Self::CheckedOutcomeOnly => "checked_outcome_only",
            Self::ProofVisibleTranscript => "proof_visible_transcript",
        }
    }
}
