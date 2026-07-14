use super::{
    UiMeasurementRequestFamily, WorthUiHostCapabilityObservationGeneration,
    WorthUiHostCapabilityReport,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum UiMeasurementEvidenceCategory {
    TextIntrinsicSize,
    TextBaselineMetrics,
    FontMetrics,
    NativeControlIntrinsicSize,
    ViewportExtent,
    DpiScaleFactor,
    PortalAnchorRect,
    ScrollContainerViewport,
}

impl UiMeasurementEvidenceCategory {
    pub const fn from_request_family(family: UiMeasurementRequestFamily) -> Self {
        match family {
            UiMeasurementRequestFamily::TextIntrinsicSize => Self::TextIntrinsicSize,
            UiMeasurementRequestFamily::TextBaselineMetrics => Self::TextBaselineMetrics,
            UiMeasurementRequestFamily::FontMetrics => Self::FontMetrics,
            UiMeasurementRequestFamily::NativeControlIntrinsicSize => {
                Self::NativeControlIntrinsicSize
            }
            UiMeasurementRequestFamily::ViewportExtent => Self::ViewportExtent,
            UiMeasurementRequestFamily::DpiScaleFactor => Self::DpiScaleFactor,
            UiMeasurementRequestFamily::PortalAnchorRect => Self::PortalAnchorRect,
            UiMeasurementRequestFamily::ScrollContainerViewport => Self::ScrollContainerViewport,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum UiMeasurementUnitPosture {
    LogicalPx,
    PhysicalPx,
    UnitlessScale,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum UiMeasurementCoordinateSpace {
    Viewport,
    Window,
    GraphNodeLocal,
    HostSurface,
    PortalLayer,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum UiMeasurementRoundingPosture {
    ExactFloat,
    HostRounded,
    RuntimeRounded,
    DeferredToAllocation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostMeasurementAssumptionProfile {
    capability_observation_generation: WorthUiHostCapabilityObservationGeneration,
    capability_profile_digest: u64,
    viewport_assumption_digest: u64,
    dpi_assumption_digest: u64,
    font_assumption_digest: u64,
    adapter_profile_digest: u64,
}

impl UiHostMeasurementAssumptionProfile {
    pub const fn new(
        capability_observation_generation: WorthUiHostCapabilityObservationGeneration,
        capability_profile_digest: u64,
        viewport_assumption_digest: u64,
        dpi_assumption_digest: u64,
        font_assumption_digest: u64,
        adapter_profile_digest: u64,
    ) -> Self {
        Self {
            capability_observation_generation,
            capability_profile_digest,
            viewport_assumption_digest,
            dpi_assumption_digest,
            font_assumption_digest,
            adapter_profile_digest,
        }
    }

    pub fn from_capability_report(
        capability_report: &WorthUiHostCapabilityReport,
        viewport_assumption_digest: u64,
        dpi_assumption_digest: u64,
        font_assumption_digest: u64,
        adapter_profile_digest: u64,
    ) -> Self {
        Self::new(
            capability_report.observation_generation(),
            capability_report.profile_identity_digest(),
            viewport_assumption_digest,
            dpi_assumption_digest,
            font_assumption_digest,
            adapter_profile_digest,
        )
    }

    pub fn adapter_profile_digest(self) -> u64 {
        self.adapter_profile_digest
    }
}

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
