use crate::evidence::{
    UiMeasurementCoordinateSpace, UiMeasurementEvidenceCategory, UiMeasurementRoundingPosture,
    UiMeasurementUnitPosture,
};

use super::UiHostMeasurementAssumptionProfile;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostMeasurementNormalizationContext {
    evidence_category: UiMeasurementEvidenceCategory,
    unit_posture: UiMeasurementUnitPosture,
    coordinate_space: UiMeasurementCoordinateSpace,
    rounding_posture: UiMeasurementRoundingPosture,
    assumption_profile: UiHostMeasurementAssumptionProfile,
}

impl UiHostMeasurementNormalizationContext {
    pub fn text_intrinsic_surface_logical_exact(
        assumption_profile: UiHostMeasurementAssumptionProfile,
    ) -> Self {
        Self::new(
            UiMeasurementEvidenceCategory::TextIntrinsicSize,
            UiMeasurementUnitPosture::LogicalPx,
            UiMeasurementCoordinateSpace::HostSurface,
            UiMeasurementRoundingPosture::ExactFloat,
            assumption_profile,
        )
    }

    pub fn text_baseline_surface_logical_exact(
        assumption_profile: UiHostMeasurementAssumptionProfile,
    ) -> Self {
        Self::new(
            UiMeasurementEvidenceCategory::TextBaselineMetrics,
            UiMeasurementUnitPosture::LogicalPx,
            UiMeasurementCoordinateSpace::HostSurface,
            UiMeasurementRoundingPosture::ExactFloat,
            assumption_profile,
        )
    }

    pub fn font_metrics_surface_logical_exact(
        assumption_profile: UiHostMeasurementAssumptionProfile,
    ) -> Self {
        Self::new(
            UiMeasurementEvidenceCategory::FontMetrics,
            UiMeasurementUnitPosture::LogicalPx,
            UiMeasurementCoordinateSpace::HostSurface,
            UiMeasurementRoundingPosture::ExactFloat,
            assumption_profile,
        )
    }

    pub fn native_control_surface_logical_host_rounded(
        assumption_profile: UiHostMeasurementAssumptionProfile,
    ) -> Self {
        Self::new(
            UiMeasurementEvidenceCategory::NativeControlIntrinsicSize,
            UiMeasurementUnitPosture::LogicalPx,
            UiMeasurementCoordinateSpace::HostSurface,
            UiMeasurementRoundingPosture::HostRounded,
            assumption_profile,
        )
    }

    pub fn viewport_logical_exact(assumption_profile: UiHostMeasurementAssumptionProfile) -> Self {
        Self::new(
            UiMeasurementEvidenceCategory::ViewportExtent,
            UiMeasurementUnitPosture::LogicalPx,
            UiMeasurementCoordinateSpace::Viewport,
            UiMeasurementRoundingPosture::ExactFloat,
            assumption_profile,
        )
    }

    pub fn dpi_scale_window_exact(assumption_profile: UiHostMeasurementAssumptionProfile) -> Self {
        Self::new(
            UiMeasurementEvidenceCategory::DpiScaleFactor,
            UiMeasurementUnitPosture::UnitlessScale,
            UiMeasurementCoordinateSpace::Window,
            UiMeasurementRoundingPosture::ExactFloat,
            assumption_profile,
        )
    }

    pub fn portal_anchor_logical_exact(
        assumption_profile: UiHostMeasurementAssumptionProfile,
    ) -> Self {
        Self::new(
            UiMeasurementEvidenceCategory::PortalAnchorRect,
            UiMeasurementUnitPosture::LogicalPx,
            UiMeasurementCoordinateSpace::PortalLayer,
            UiMeasurementRoundingPosture::ExactFloat,
            assumption_profile,
        )
    }

    pub fn scroll_container_logical_exact(
        assumption_profile: UiHostMeasurementAssumptionProfile,
    ) -> Self {
        Self::new(
            UiMeasurementEvidenceCategory::ScrollContainerViewport,
            UiMeasurementUnitPosture::LogicalPx,
            UiMeasurementCoordinateSpace::Viewport,
            UiMeasurementRoundingPosture::ExactFloat,
            assumption_profile,
        )
    }

    const fn new(
        evidence_category: UiMeasurementEvidenceCategory,
        unit_posture: UiMeasurementUnitPosture,
        coordinate_space: UiMeasurementCoordinateSpace,
        rounding_posture: UiMeasurementRoundingPosture,
        assumption_profile: UiHostMeasurementAssumptionProfile,
    ) -> Self {
        Self {
            evidence_category,
            unit_posture,
            coordinate_space,
            rounding_posture,
            assumption_profile,
        }
    }

    pub fn evidence_category(self) -> UiMeasurementEvidenceCategory {
        self.evidence_category
    }

    pub fn unit_posture(self) -> UiMeasurementUnitPosture {
        self.unit_posture
    }

    pub fn coordinate_space(self) -> UiMeasurementCoordinateSpace {
        self.coordinate_space
    }

    pub fn rounding_posture(self) -> UiMeasurementRoundingPosture {
        self.rounding_posture
    }

    pub fn assumption_profile(self) -> UiHostMeasurementAssumptionProfile {
        self.assumption_profile
    }
}
