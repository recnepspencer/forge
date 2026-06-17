#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanSegmentCarrierSetDenialKind {
    OperandSlotSideMismatch,
    ProjectionOperandSideMismatch,
    OperandSourceContextMismatch,
    ProjectionStageIdentityMismatch,
    ProjectionLocalBasisIdentityMismatch,
    PrecisionBasisIdentityMismatch,
    MissingPrecisionBasisIdentity,
    MissingBoundaryLoop,
    MissingBoundarySegments,
    MissingSourceFaceIdentity,
    MissingSourceLoopIdentity,
    MissingSourceEdgeIdentity,
    MissingProjectedEndpointProof,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSegmentCarrierSetDenial {
    kind: PlanarBooleanSegmentCarrierSetDenialKind,
    human_reason: String,
}

impl PlanarBooleanSegmentCarrierSetDenial {
    pub(crate) fn new(
        kind: PlanarBooleanSegmentCarrierSetDenialKind,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            human_reason: human_reason.into(),
        }
    }

    pub fn kind(&self) -> PlanarBooleanSegmentCarrierSetDenialKind {
        self.kind
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
