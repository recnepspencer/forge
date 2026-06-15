#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanCommonPlaneLocalFrameSelectionDenialKind {
    MissingSharedPlaneIdentity,
    MissingSharedPlaneReceiptIdentity,
    MissingPlaneAgreementIdentity,
    MissingLocalFrameIdentity,
    MissingTopologyBasisIdentity,
    MissingMovementRotationPostureIdentity,
    FrameIdentityMismatch,
    TopologyBasisIdentityMismatch,
    PrecisionFactDigestMismatch,
    MovementRotationPostureIdentityMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanCommonPlaneLocalFrameSelectionDenial {
    kind: PlanarBooleanCommonPlaneLocalFrameSelectionDenialKind,
    human_reason: &'static str,
}

impl PlanarBooleanCommonPlaneLocalFrameSelectionDenial {
    pub(crate) fn new(
        kind: PlanarBooleanCommonPlaneLocalFrameSelectionDenialKind,
        human_reason: &'static str,
    ) -> Self {
        Self { kind, human_reason }
    }

    pub fn kind(&self) -> PlanarBooleanCommonPlaneLocalFrameSelectionDenialKind {
        self.kind
    }

    pub fn human_reason(&self) -> &'static str {
        self.human_reason
    }
}
