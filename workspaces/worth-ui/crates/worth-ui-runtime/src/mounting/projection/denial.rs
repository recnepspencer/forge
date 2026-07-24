#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedProjectionDenial {
    Identity(super::super::UiMountedIdentityDenial),
    UnknownGraphNode,
    MissingSurfaceBinding,
    ForeignPlan,
    ForeignGraphWorld,
    ForeignMountIncarnation,
    ForeignAllocation,
    PreviewInstanceMismatch,
    CoordinateBasisMismatch,
    NonFiniteGeometry,
    NegativeExtent,
    TableCapacityExceeded,
    DuplicateLaneContribution,
}

impl From<super::super::UiMountedIdentityDenial> for UiMountedProjectionDenial {
    fn from(denial: super::super::UiMountedIdentityDenial) -> Self {
        Self::Identity(denial)
    }
}
