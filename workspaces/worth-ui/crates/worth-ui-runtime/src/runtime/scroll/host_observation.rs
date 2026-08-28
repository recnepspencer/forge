#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum UiHostScrollObservationOutcome {
    Applied(super::UiScrollRouteReceipt),
    Denied(UiHostScrollObservationDenial),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiHostScrollObservationDenial {
    Targeting(crate::runtime::interaction::UiInteractionTargetingDenial),
    PresentedSurfaceFallbackIsAmbiguous,
    MountedBasisUnavailable,
    Ownership(super::UiScrollOwnershipResolutionDenial),
    NoDeclaredScrollOwner,
    AllocationUnavailable,
    ViewportUnavailable,
    BoundsOutOfRange,
    DeltaOutOfRange,
    Route(super::UiScrollRouteDenial),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiScrollBoundsResolutionDenial {
    AllocationUnavailable,
    ViewportUnavailable,
    OutOfRange,
}
