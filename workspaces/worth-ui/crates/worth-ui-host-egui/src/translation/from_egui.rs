use worth_ui_runtime::host::{
    UiHostMeasurementAssumptionProfile, UiHostMeasurementNormalizationContext,
};

pub fn egui_text_intrinsic_normalization_context(
    assumption_profile: UiHostMeasurementAssumptionProfile,
) -> UiHostMeasurementNormalizationContext {
    UiHostMeasurementNormalizationContext::text_intrinsic_surface_logical_exact(
        assumption_profile,
    )
}

pub fn egui_text_baseline_normalization_context(
    assumption_profile: UiHostMeasurementAssumptionProfile,
) -> UiHostMeasurementNormalizationContext {
    UiHostMeasurementNormalizationContext::text_baseline_surface_logical_exact(
        assumption_profile,
    )
}

pub fn egui_font_metrics_normalization_context(
    assumption_profile: UiHostMeasurementAssumptionProfile,
) -> UiHostMeasurementNormalizationContext {
    UiHostMeasurementNormalizationContext::font_metrics_surface_logical_exact(
        assumption_profile,
    )
}

pub fn egui_native_control_normalization_context(
    assumption_profile: UiHostMeasurementAssumptionProfile,
) -> UiHostMeasurementNormalizationContext {
    UiHostMeasurementNormalizationContext::native_control_surface_logical_host_rounded(
        assumption_profile,
    )
}

pub fn egui_viewport_normalization_context(
    assumption_profile: UiHostMeasurementAssumptionProfile,
) -> UiHostMeasurementNormalizationContext {
    UiHostMeasurementNormalizationContext::viewport_logical_exact(assumption_profile)
}

pub fn egui_dpi_scale_normalization_context(
    assumption_profile: UiHostMeasurementAssumptionProfile,
) -> UiHostMeasurementNormalizationContext {
    UiHostMeasurementNormalizationContext::dpi_scale_window_exact(assumption_profile)
}

pub fn egui_portal_anchor_normalization_context(
    assumption_profile: UiHostMeasurementAssumptionProfile,
) -> UiHostMeasurementNormalizationContext {
    UiHostMeasurementNormalizationContext::portal_anchor_logical_exact(assumption_profile)
}

pub fn egui_scroll_container_normalization_context(
    assumption_profile: UiHostMeasurementAssumptionProfile,
) -> UiHostMeasurementNormalizationContext {
    UiHostMeasurementNormalizationContext::scroll_container_logical_exact(
        assumption_profile,
    )
}

#[cfg(test)]
mod tests {
    use worth_ui_host_contract::WorthUiHostCapabilityReport;
    use worth_ui_runtime::facade::{
        UiMeasurementCoordinateSpace, UiMeasurementEvidenceCategory,
        UiMeasurementRoundingPosture, UiMeasurementUnitPosture,
    };

    use super::*;
    use crate::translation::{
        egui_measurement_adapter_profile_digest, egui_measurement_assumption_profile,
    };

    #[test]
    fn egui_measurement_translation_surfaces_preserve_runtime_posture_explicitly() {
        let report = WorthUiHostCapabilityReport::available(vec![]);
        let profile = egui_measurement_assumption_profile(&report, 1, 2, 3);
        let cases = [
            (
                egui_text_intrinsic_normalization_context(profile),
                UiMeasurementEvidenceCategory::TextIntrinsicSize,
                UiMeasurementUnitPosture::LogicalPx,
                UiMeasurementCoordinateSpace::HostSurface,
                UiMeasurementRoundingPosture::ExactFloat,
            ),
            (
                egui_text_baseline_normalization_context(profile),
                UiMeasurementEvidenceCategory::TextBaselineMetrics,
                UiMeasurementUnitPosture::LogicalPx,
                UiMeasurementCoordinateSpace::HostSurface,
                UiMeasurementRoundingPosture::ExactFloat,
            ),
            (
                egui_font_metrics_normalization_context(profile),
                UiMeasurementEvidenceCategory::FontMetrics,
                UiMeasurementUnitPosture::LogicalPx,
                UiMeasurementCoordinateSpace::HostSurface,
                UiMeasurementRoundingPosture::ExactFloat,
            ),
            (
                egui_native_control_normalization_context(profile),
                UiMeasurementEvidenceCategory::NativeControlIntrinsicSize,
                UiMeasurementUnitPosture::LogicalPx,
                UiMeasurementCoordinateSpace::HostSurface,
                UiMeasurementRoundingPosture::HostRounded,
            ),
            (
                egui_viewport_normalization_context(profile),
                UiMeasurementEvidenceCategory::ViewportExtent,
                UiMeasurementUnitPosture::LogicalPx,
                UiMeasurementCoordinateSpace::Viewport,
                UiMeasurementRoundingPosture::ExactFloat,
            ),
            (
                egui_dpi_scale_normalization_context(profile),
                UiMeasurementEvidenceCategory::DpiScaleFactor,
                UiMeasurementUnitPosture::UnitlessScale,
                UiMeasurementCoordinateSpace::Window,
                UiMeasurementRoundingPosture::ExactFloat,
            ),
            (
                egui_portal_anchor_normalization_context(profile),
                UiMeasurementEvidenceCategory::PortalAnchorRect,
                UiMeasurementUnitPosture::LogicalPx,
                UiMeasurementCoordinateSpace::PortalLayer,
                UiMeasurementRoundingPosture::ExactFloat,
            ),
            (
                egui_scroll_container_normalization_context(profile),
                UiMeasurementEvidenceCategory::ScrollContainerViewport,
                UiMeasurementUnitPosture::LogicalPx,
                UiMeasurementCoordinateSpace::Viewport,
                UiMeasurementRoundingPosture::ExactFloat,
            ),
        ];

        assert_eq!(
            profile.adapter_profile_digest(),
            egui_measurement_adapter_profile_digest()
        );

        for (context, category, unit, space, rounding) in cases {
            assert_eq!(context.evidence_category(), category);
            assert_eq!(context.unit_posture(), unit);
            assert_eq!(context.coordinate_space(), space);
            assert_eq!(context.rounding_posture(), rounding);
        }
    }
}
