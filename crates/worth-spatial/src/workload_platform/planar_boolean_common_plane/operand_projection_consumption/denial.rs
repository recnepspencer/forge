#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanCommonPlaneOperandProjectionConsumptionDenialKind {
    MissingLocalFrameSelectionIdentity,
    MissingSharedPlaneReceiptIdentity,
    MissingSharedPlaneIdentity,
    MissingPlaneAgreementIdentity,
    MissingProjectionStageIdentity,
    MissingUpstreamSurfaceSupportIdentity,
    MissingCertifiedPlaneSupportIdentity,
    MissingProjectionLocalBasisIdentity,
    ProjectionLocalBasisSelectionMismatch,
    MissingProjectedEntityCount,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanCommonPlaneOperandProjectionConsumptionDenial {
    kind: PlanarBooleanCommonPlaneOperandProjectionConsumptionDenialKind,
    human_reason: &'static str,
}

impl PlanarBooleanCommonPlaneOperandProjectionConsumptionDenial {
    pub(crate) fn new(
        kind: PlanarBooleanCommonPlaneOperandProjectionConsumptionDenialKind,
        human_reason: &'static str,
    ) -> Self {
        Self { kind, human_reason }
    }

    pub fn kind(&self) -> PlanarBooleanCommonPlaneOperandProjectionConsumptionDenialKind {
        self.kind
    }

    pub fn human_reason(&self) -> &'static str {
        self.human_reason
    }
}
