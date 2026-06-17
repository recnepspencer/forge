#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanPointSplitPostureDenialKind {
    EmptyPointEventGroup,
    MixedPointEventKind,
    SharedEndpointExactEndpointMismatch,
    SharedEndpointInteriorParticipant,
    SharedEndpointMissingParticipant,
    SharedEndpointMissingProvenance,
    SharedEndpointProvenanceMismatch,
    TJunctionMissingEndpointParticipant,
    TJunctionMissingInteriorParticipant,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanPointSplitPostureDenial {
    kind: PlanarBooleanPointSplitPostureDenialKind,
    evidence_identity: String,
    human_reason: String,
}

impl PlanarBooleanPointSplitPostureDenial {
    pub(crate) fn new(
        kind: PlanarBooleanPointSplitPostureDenialKind,
        evidence_identity: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            evidence_identity: evidence_identity.into(),
            human_reason: human_reason.into(),
        }
    }

    pub fn kind(&self) -> PlanarBooleanPointSplitPostureDenialKind {
        self.kind
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
