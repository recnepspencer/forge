use worth_ui_host_contract::{
    UiHostSurfacePresentationMode, UiMountedEffectFamily, UiMountedPaintPrimitiveKind,
    UiMountedPaintProjection, UiMountedProjectionView,
};

pub(super) fn required_effects(
    mode: UiHostSurfacePresentationMode,
    projection: &UiMountedProjectionView,
) -> Vec<UiMountedEffectFamily> {
    if mode == UiHostSurfacePresentationMode::RecordOnly {
        return vec![UiMountedEffectFamily::RecordedProjection];
    }

    let mut effects = Vec::new();
    if projection.nodes().iter().any(|node| {
        matches!(node.paint(), UiMountedPaintProjection::Batch(_))
            || matches!(
                node.preview(),
                worth_ui_host_contract::UiMountedPreviewProjection::Resize { .. }
            )
    }) {
        effects.push(UiMountedEffectFamily::NativePaint);
    }
    if projection.nodes().iter().any(|node| {
        matches!(
            node.accessibility(),
            worth_ui_host_contract::UiMountedAccessibilityProjection::Admitted(_)
        )
    }) {
        effects.push(UiMountedEffectFamily::Accessibility);
    }
    if projection.nodes().iter().any(|node| {
        node.participation().focus().status()
            == worth_ui_host_contract::UiMountedParticipationStatus::Admitted
    }) {
        effects.push(UiMountedEffectFamily::Focus);
    }
    if has_canvas_effect(projection) {
        effects.push(UiMountedEffectFamily::CanvasSpatial);
    }
    if has_realtime_effect(projection) {
        effects.push(UiMountedEffectFamily::Realtime);
    }
    effects
}

fn has_canvas_effect(projection: &UiMountedProjectionView) -> bool {
    !projection.spatial_batches().rows().is_empty()
        || projection
            .paint_batches()
            .rows()
            .iter()
            .any(|row| row.primitive_kind() == UiMountedPaintPrimitiveKind::CanvasSpatialBatch)
}

fn has_realtime_effect(projection: &UiMountedProjectionView) -> bool {
    !projection.realtime_batches().rows().is_empty()
        || projection
            .paint_batches()
            .rows()
            .iter()
            .any(|row| row.primitive_kind() == UiMountedPaintPrimitiveKind::RealtimeBatch)
}
