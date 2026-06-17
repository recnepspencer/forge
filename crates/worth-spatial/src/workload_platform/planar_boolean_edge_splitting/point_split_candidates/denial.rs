#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanPointSplitCandidateDenialKind {
    ConflictingCarrierParameterFacts,
    MissingCarrierParameter,
    MissingParticipationRow,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanPointSplitCandidateDenial {
    kind: PlanarBooleanPointSplitCandidateDenialKind,
    evidence_identity: String,
    human_reason: String,
    rejected_missing_parameter_facts: usize,
    rejected_conflicting_parameter_facts: usize,
}

impl PlanarBooleanPointSplitCandidateDenial {
    pub(crate) fn new(
        kind: PlanarBooleanPointSplitCandidateDenialKind,
        evidence_identity: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            evidence_identity: evidence_identity.into(),
            human_reason: human_reason.into(),
            rejected_missing_parameter_facts: 0,
            rejected_conflicting_parameter_facts: 0,
        }
    }

    pub(crate) fn with_rejected_missing_parameter_fact(
        kind: PlanarBooleanPointSplitCandidateDenialKind,
        evidence_identity: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            evidence_identity: evidence_identity.into(),
            human_reason: human_reason.into(),
            rejected_missing_parameter_facts: 1,
            rejected_conflicting_parameter_facts: 0,
        }
    }

    pub(crate) fn with_rejected_conflicting_parameter_fact(
        kind: PlanarBooleanPointSplitCandidateDenialKind,
        evidence_identity: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            evidence_identity: evidence_identity.into(),
            human_reason: human_reason.into(),
            rejected_missing_parameter_facts: 0,
            rejected_conflicting_parameter_facts: 1,
        }
    }

    pub fn kind(&self) -> PlanarBooleanPointSplitCandidateDenialKind {
        self.kind
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }

    pub fn rejected_missing_parameter_facts(&self) -> usize {
        self.rejected_missing_parameter_facts
    }

    pub fn rejected_conflicting_parameter_facts(&self) -> usize {
        self.rejected_conflicting_parameter_facts
    }
}
