#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanEndpointBoundaryNormalizationDenialKind {
    MissingEndpointBoundaryAuthority,
    EndpointSplitWouldCreateZeroLengthFragment,
    ContradictoryBoundaryAction,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanEndpointBoundaryNormalizationDenial {
    kind: PlanarBooleanEndpointBoundaryNormalizationDenialKind,
    evidence_identity: String,
    human_reason: String,
}

impl PlanarBooleanEndpointBoundaryNormalizationDenial {
    pub(crate) fn new(
        kind: PlanarBooleanEndpointBoundaryNormalizationDenialKind,
        evidence_identity: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            evidence_identity: evidence_identity.into(),
            human_reason: human_reason.into(),
        }
    }

    pub fn kind(&self) -> PlanarBooleanEndpointBoundaryNormalizationDenialKind {
        self.kind
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
