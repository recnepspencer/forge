use worth_ui_host_contract::{
    UiMountedAccessibilityProjection, UiMountedDiagnosticProjection, UiMountedEffectFamily,
    UiMountedMotionProjection, UiMountedPaintPrimitiveKind, UiMountedPaintProjection,
    UiMountedParticipationStatus, UiMountedProjectionView,
};

pub(super) fn unsupported_projection_effect(
    projection: &UiMountedProjectionView,
) -> Option<UiMountedEffectFamily> {
    if !projection.spatial_batches().rows().is_empty()
        || projection
            .paint_batches()
            .rows()
            .iter()
            .any(|row| row.primitive_kind() == UiMountedPaintPrimitiveKind::CanvasSpatialBatch)
    {
        return Some(UiMountedEffectFamily::CanvasSpatial);
    }
    if !projection.realtime_batches().rows().is_empty()
        || projection
            .paint_batches()
            .rows()
            .iter()
            .any(|row| row.primitive_kind() == UiMountedPaintPrimitiveKind::RealtimeBatch)
    {
        return Some(UiMountedEffectFamily::Realtime);
    }
    if projection
        .nodes()
        .iter()
        .any(|node| matches!(node.paint(), UiMountedPaintProjection::Batch(_)))
    {
        return Some(UiMountedEffectFamily::NativePaint);
    }
    if projection.nodes().iter().any(|node| {
        matches!(
            node.accessibility(),
            UiMountedAccessibilityProjection::Admitted(_)
        )
    }) {
        return Some(UiMountedEffectFamily::Accessibility);
    }
    if projection
        .nodes()
        .iter()
        .any(|node| node.participation().focus().status() == UiMountedParticipationStatus::Admitted)
    {
        return Some(UiMountedEffectFamily::Focus);
    }
    if projection
        .nodes()
        .iter()
        .any(|node| matches!(node.motion(), UiMountedMotionProjection::Admitted))
    {
        return Some(UiMountedEffectFamily::Motion);
    }
    projection
        .nodes()
        .iter()
        .any(|node| {
            matches!(
                node.diagnostic(),
                UiMountedDiagnosticProjection::Reference(_)
            )
        })
        .then_some(UiMountedEffectFamily::Diagnostic)
}
