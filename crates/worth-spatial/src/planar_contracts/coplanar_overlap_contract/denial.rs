#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoplanarOverlapDenialKind {
    MissingPlanarNeighborhood,
    MissingFaceIdentity,
    MismatchedPlanarNeighborhood,
    MismatchedFrameIdentity,
    MismatchedMovementRotationPosture,
    MismatchedTolerancePolicy,
    AreaPolicyRequired,
    AmbiguousContactRequiresPolicy,
}

impl CoplanarOverlapDenialKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingPlanarNeighborhood => "missing-planar-neighborhood",
            Self::MissingFaceIdentity => "missing-face-identity",
            Self::MismatchedPlanarNeighborhood => "mismatched-planar-neighborhood",
            Self::MismatchedFrameIdentity => "mismatched-frame-identity",
            Self::MismatchedMovementRotationPosture => "mismatched-movement-rotation-posture",
            Self::MismatchedTolerancePolicy => "mismatched-tolerance-policy",
            Self::AreaPolicyRequired => "area-policy-required",
            Self::AmbiguousContactRequiresPolicy => "ambiguous-contact-requires-policy",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoplanarOverlapDenialBasisLocus {
    PlanarNeighborhood,
    FaceIdentity,
    FrameIdentity,
    MovementRotation,
    TolerancePolicy,
    SignedArea,
    SegmentContact,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoplanarOverlapDenial {
    kind: CoplanarOverlapDenialKind,
    locus: CoplanarOverlapDenialBasisLocus,
    reason: String,
}

impl CoplanarOverlapDenial {
    pub(crate) fn new(
        kind: CoplanarOverlapDenialKind,
        locus: CoplanarOverlapDenialBasisLocus,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            locus,
            reason: reason.into(),
        }
    }

    pub fn kind(&self) -> CoplanarOverlapDenialKind {
        self.kind
    }

    pub fn locus(&self) -> CoplanarOverlapDenialBasisLocus {
        self.locus
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}
