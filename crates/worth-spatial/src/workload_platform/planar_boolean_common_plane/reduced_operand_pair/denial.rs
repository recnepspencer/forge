#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanCommonPlaneReducedOperandPairDenialKind {
    DuplicateOperandSide,
    MissingLeftOperand,
    MissingRightOperand,
    SharedPlaneReceiptIdentityMismatch,
    SharedPlaneIdentityMismatch,
    PlaneAgreementIdentityMismatch,
    LocalFrameSelectionIdentityMismatch,
    ProjectionLocalBasisIdentityMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanCommonPlaneReducedOperandPairDenial {
    kind: PlanarBooleanCommonPlaneReducedOperandPairDenialKind,
    human_reason: &'static str,
}

impl PlanarBooleanCommonPlaneReducedOperandPairDenial {
    pub(crate) fn new(
        kind: PlanarBooleanCommonPlaneReducedOperandPairDenialKind,
        human_reason: &'static str,
    ) -> Self {
        Self { kind, human_reason }
    }

    pub fn kind(&self) -> PlanarBooleanCommonPlaneReducedOperandPairDenialKind {
        self.kind
    }

    pub fn human_reason(&self) -> &'static str {
        self.human_reason
    }
}
