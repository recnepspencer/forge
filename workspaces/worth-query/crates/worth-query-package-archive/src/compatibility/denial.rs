//! Self-describing unsupported reader-version posture.

use worth_foundational::facade::{
    BoundaryProtocolCompatibilityWindow, BoundaryProtocolUnsupportedVersion,
    BoundaryProtocolUnsupportedVersionPosture,
};

use super::WorthQueryPackageArchiveProtocolLayer;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPackageArchiveCompatibilityPosture {
    InvalidZero,
    PredatesWindow,
    ExceedsWindow,
    Retired,
}

/// Exact protocol incompatibility observed before body interpretation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryPackageArchiveCompatibilityDenial {
    layer: WorthQueryPackageArchiveProtocolLayer,
    observed_version: u16,
    supported_window: BoundaryProtocolCompatibilityWindow,
    posture: WorthQueryPackageArchiveCompatibilityPosture,
}

impl WorthQueryPackageArchiveCompatibilityDenial {
    pub(super) const fn invalid_zero(
        layer: WorthQueryPackageArchiveProtocolLayer,
        supported_window: BoundaryProtocolCompatibilityWindow,
    ) -> Self {
        Self {
            layer,
            observed_version: 0,
            supported_window,
            posture: WorthQueryPackageArchiveCompatibilityPosture::InvalidZero,
        }
    }

    pub(super) const fn unsupported(
        layer: WorthQueryPackageArchiveProtocolLayer,
        observed_version: u16,
        supported_window: BoundaryProtocolCompatibilityWindow,
        unsupported: BoundaryProtocolUnsupportedVersion,
    ) -> Self {
        Self {
            layer,
            observed_version,
            supported_window,
            posture: map_unsupported_posture(unsupported.posture()),
        }
    }

    pub const fn layer(self) -> WorthQueryPackageArchiveProtocolLayer {
        self.layer
    }

    pub const fn observed_version(self) -> u16 {
        self.observed_version
    }

    pub const fn supported_window(self) -> BoundaryProtocolCompatibilityWindow {
        self.supported_window
    }

    pub const fn posture(self) -> WorthQueryPackageArchiveCompatibilityPosture {
        self.posture
    }
}

const fn map_unsupported_posture(
    posture: BoundaryProtocolUnsupportedVersionPosture,
) -> WorthQueryPackageArchiveCompatibilityPosture {
    match posture {
        BoundaryProtocolUnsupportedVersionPosture::PredatesWindow => {
            WorthQueryPackageArchiveCompatibilityPosture::PredatesWindow
        }
        BoundaryProtocolUnsupportedVersionPosture::ExceedsWindow => {
            WorthQueryPackageArchiveCompatibilityPosture::ExceedsWindow
        }
        BoundaryProtocolUnsupportedVersionPosture::Retired => {
            WorthQueryPackageArchiveCompatibilityPosture::Retired
        }
    }
}
