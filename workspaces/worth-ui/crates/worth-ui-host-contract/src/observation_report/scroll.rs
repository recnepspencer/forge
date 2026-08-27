#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostScrollDeltaSource {
    PointerWheel,
    Touch,
    Pen,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostScrollDeltaPhase {
    Started,
    Updated,
    Ended,
    Cancelled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostScrollDeltaPrecision {
    Line,
    Pixel,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostScrollDeltaTargetAffinity {
    ExactCoordinate {
        presentation: super::UiHostObservationPresentationBasis,
        position: super::UiHostSurfacePosition,
    },
    ExactMountedTarget {
        presentation: super::UiHostObservationPresentationBasis,
        mounted: super::UiHostObservationMountedBasis,
    },
    PresentedSurfaceFallback {
        presentation: super::UiHostObservationPresentationBasis,
    },
}

impl UiHostScrollDeltaTargetAffinity {
    pub const fn exact_coordinate(
        presentation: super::UiHostObservationPresentationBasis,
        position: super::UiHostSurfacePosition,
    ) -> Self {
        Self::ExactCoordinate {
            presentation,
            position,
        }
    }

    pub const fn exact_mounted_target(
        presentation: super::UiHostObservationPresentationBasis,
        mounted: super::UiHostObservationMountedBasis,
    ) -> Self {
        Self::ExactMountedTarget {
            presentation,
            mounted,
        }
    }

    pub const fn presented_surface_fallback(
        presentation: super::UiHostObservationPresentationBasis,
    ) -> Self {
        Self::PresentedSurfaceFallback { presentation }
    }

    pub const fn presentation(self) -> super::UiHostObservationPresentationBasis {
        match self {
            Self::ExactCoordinate { presentation, .. }
            | Self::ExactMountedTarget { presentation, .. }
            | Self::PresentedSurfaceFallback { presentation } => presentation,
        }
    }

    pub const fn position(self) -> Option<super::UiHostSurfacePosition> {
        match self {
            Self::ExactCoordinate { position, .. } => Some(position),
            Self::ExactMountedTarget { .. } | Self::PresentedSurfaceFallback { .. } => None,
        }
    }

    pub const fn mounted_target(self) -> Option<super::UiHostObservationMountedBasis> {
        match self {
            Self::ExactMountedTarget { mounted, .. } => Some(mounted),
            Self::ExactCoordinate { .. } | Self::PresentedSurfaceFallback { .. } => None,
        }
    }

    pub const fn is_surface_fallback(self) -> bool {
        matches!(self, Self::PresentedSurfaceFallback { .. })
    }

    pub(super) const fn encoded_len(self) -> usize {
        match self {
            Self::ExactCoordinate { .. } => 51,
            Self::ExactMountedTarget { .. } => 49,
            Self::PresentedSurfaceFallback { .. } => 33,
        }
    }
}
