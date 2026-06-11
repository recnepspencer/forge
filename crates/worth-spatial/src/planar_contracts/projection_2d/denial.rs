#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectPointToCertifiedPlane2DDenialKind {
    MissingSourcePointIdentity,
    MissingSourcePointBasisDigest,
    MissingLocalFrameReceipt,
    MissingMovementRotationPostureIdentity,
    MissingTolerancePolicyIdentity,
    NonFiniteSourcePoint,
    NonFiniteLocalDelta,
    FrameBasisMismatch,
    OffPlanePoint,
    SemanticRotationInvalidatedPlanarClass,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectPointToCertifiedPlane2DDenialBasisLocus {
    SourcePointIdentity,
    SourcePointBasisDigest,
    LocalFrameReceipt,
    MovementRotationPosture,
    TolerancePolicy,
    SourcePoint,
    LocalDelta,
    FrameBasis,
    PlaneDistance,
    MovementRotationPlanarClass,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectPointToCertifiedPlane2DDenial {
    kind: ProjectPointToCertifiedPlane2DDenialKind,
    basis_locus: ProjectPointToCertifiedPlane2DDenialBasisLocus,
    reason: &'static str,
}

impl ProjectPointToCertifiedPlane2DDenial {
    pub(crate) const fn new(
        kind: ProjectPointToCertifiedPlane2DDenialKind,
        reason: &'static str,
    ) -> Self {
        Self {
            kind,
            basis_locus: denial_basis_locus(kind),
            reason,
        }
    }

    pub fn kind(&self) -> ProjectPointToCertifiedPlane2DDenialKind {
        self.kind
    }

    pub fn basis_locus(&self) -> ProjectPointToCertifiedPlane2DDenialBasisLocus {
        self.basis_locus
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

const fn denial_basis_locus(
    kind: ProjectPointToCertifiedPlane2DDenialKind,
) -> ProjectPointToCertifiedPlane2DDenialBasisLocus {
    match kind {
        ProjectPointToCertifiedPlane2DDenialKind::MissingSourcePointIdentity => {
            ProjectPointToCertifiedPlane2DDenialBasisLocus::SourcePointIdentity
        }
        ProjectPointToCertifiedPlane2DDenialKind::MissingSourcePointBasisDigest => {
            ProjectPointToCertifiedPlane2DDenialBasisLocus::SourcePointBasisDigest
        }
        ProjectPointToCertifiedPlane2DDenialKind::MissingLocalFrameReceipt => {
            ProjectPointToCertifiedPlane2DDenialBasisLocus::LocalFrameReceipt
        }
        ProjectPointToCertifiedPlane2DDenialKind::MissingMovementRotationPostureIdentity => {
            ProjectPointToCertifiedPlane2DDenialBasisLocus::MovementRotationPosture
        }
        ProjectPointToCertifiedPlane2DDenialKind::MissingTolerancePolicyIdentity => {
            ProjectPointToCertifiedPlane2DDenialBasisLocus::TolerancePolicy
        }
        ProjectPointToCertifiedPlane2DDenialKind::NonFiniteSourcePoint => {
            ProjectPointToCertifiedPlane2DDenialBasisLocus::SourcePoint
        }
        ProjectPointToCertifiedPlane2DDenialKind::NonFiniteLocalDelta => {
            ProjectPointToCertifiedPlane2DDenialBasisLocus::LocalDelta
        }
        ProjectPointToCertifiedPlane2DDenialKind::FrameBasisMismatch => {
            ProjectPointToCertifiedPlane2DDenialBasisLocus::FrameBasis
        }
        ProjectPointToCertifiedPlane2DDenialKind::OffPlanePoint => {
            ProjectPointToCertifiedPlane2DDenialBasisLocus::PlaneDistance
        }
        ProjectPointToCertifiedPlane2DDenialKind::SemanticRotationInvalidatedPlanarClass => {
            ProjectPointToCertifiedPlane2DDenialBasisLocus::MovementRotationPlanarClass
        }
    }
}
