use worth_ui_host_contract::{
    UiHostSurfacePresentationDenial, UiMountedAccessibilityProjection, UiMountedClipProjection,
    UiMountedFrameConsumptionView, UiMountedLayerProjection, UiMountedPaintProjection,
    UiMountedParticipationStatus, UiMountedProjectionView, UiMountedTableProjectionStatus,
};

use super::headless_transcript::UiHeadlessMountedFrameTranscriptInput;
use super::{
    UiHeadlessClipMechanic, UiHeadlessLayerMechanic, UiHeadlessMountedFrameTranscript,
    UiHeadlessNodeMechanic, UiHeadlessNodePaintMechanic, UiHeadlessPaintBatchMechanic,
    UiHeadlessRecorderCapacity, UiHeadlessResolvedClip, UiHeadlessResourceContact,
    UiHeadlessUnperformedEffect,
};

pub(super) fn translate_headless_frame(
    view: &UiMountedFrameConsumptionView<'_>,
    capacity: UiHeadlessRecorderCapacity,
) -> Result<UiHeadlessMountedFrameTranscript, UiHostSurfacePresentationDenial> {
    let projection = view.projection();
    validate_mechanic_capacity(projection, capacity)?;
    validate_external_batch_alignment(projection)?;
    let clips = translate_clips(projection)?;
    let mut paint_batches = translate_paint_batches(projection)?;
    paint_batches.sort_by_key(paint_order);
    let nodes = translate_nodes(projection)?;
    let unperformed_effects = unperformed_effects(projection)?;
    Ok(UiHeadlessMountedFrameTranscript::new(
        UiHeadlessMountedFrameTranscriptInput {
            host_session_identity: view.host_session_identity(),
            protocol: view.protocol(),
            attempt: view.attempt(),
            frame: projection.frame(),
            binding: view.requirement().binding(),
            nodes,
            clips,
            paint_batches,
            unperformed_effects,
        },
    ))
}

fn validate_external_batch_alignment(
    projection: &UiMountedProjectionView,
) -> Result<(), UiHostSurfacePresentationDenial> {
    let canvas = projection.paint_batches().rows().iter().filter(|row| {
        row.primitive_kind()
            == worth_ui_host_contract::UiMountedPaintPrimitiveKind::CanvasSpatialBatch
    });
    let realtime = projection.paint_batches().rows().iter().filter(|row| {
        row.primitive_kind() == worth_ui_host_contract::UiMountedPaintPrimitiveKind::RealtimeBatch
    });
    let canvas_count = canvas.clone().count();
    let realtime_count = realtime.clone().count();
    let canvas_aligned = canvas
        .zip(projection.spatial_batches().rows())
        .all(|(paint, spatial)| paint.primitive_count() == spatial.primitive_count());
    let realtime_aligned = realtime
        .zip(projection.realtime_batches().rows())
        .all(|(paint, overlay)| paint.primitive_count() == u32::from(overlay.overlay_row_count()));
    if canvas_count != projection.spatial_batches().rows().len()
        || realtime_count != projection.realtime_batches().rows().len()
        || !canvas_aligned
        || !realtime_aligned
    {
        Err(UiHostSurfacePresentationDenial::MalformedProjection)
    } else {
        Ok(())
    }
}

fn validate_mechanic_capacity(
    projection: &UiMountedProjectionView,
    capacity: UiHeadlessRecorderCapacity,
) -> Result<(), UiHostSurfacePresentationDenial> {
    let count = [
        projection.nodes().len(),
        projection.clips().rows().len(),
        projection.paint_batches().rows().len(),
        projection.spatial_batches().rows().len(),
        projection.realtime_batches().rows().len(),
        1,
        usize::from(has_accessibility(projection)),
        usize::from(has_focus(projection)),
    ]
    .into_iter()
    .try_fold(0usize, usize::checked_add)
    .ok_or(UiHostSurfacePresentationDenial::CapacityExceeded)?;
    if count > capacity.mechanics_per_frame() {
        Err(UiHostSurfacePresentationDenial::CapacityExceeded)
    } else {
        Ok(())
    }
}

fn translate_clips(
    projection: &UiMountedProjectionView,
) -> Result<Vec<UiHeadlessClipMechanic>, UiHostSurfacePresentationDenial> {
    if matches!(
        projection.clips().status(),
        UiMountedTableProjectionStatus::Omitted(_)
    ) && !projection.clips().rows().is_empty()
    {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    projection
        .clips()
        .rows()
        .iter()
        .enumerate()
        .map(|(index, row)| {
            let parent = row.parent().map(|reference| reference.index());
            if parent.is_some_and(|parent| usize::from(parent) >= index) {
                return Err(UiHostSurfacePresentationDenial::MalformedProjection);
            }
            Ok(UiHeadlessClipMechanic::new(row.bounds(), parent))
        })
        .collect()
}

fn translate_paint_batches(
    projection: &UiMountedProjectionView,
) -> Result<Vec<UiHeadlessPaintBatchMechanic>, UiHostSurfacePresentationDenial> {
    projection
        .paint_batches()
        .rows()
        .iter()
        .enumerate()
        .map(|(index, batch)| {
            let batch_index = u16::try_from(index)
                .map_err(|_| UiHostSurfacePresentationDenial::CapacityExceeded)?;
            let layer = resolve_layer(projection, batch.layer())?;
            let resource = match batch.resource() {
                Some(reference) => {
                    let entry = projection
                        .resources()
                        .resolve(reference)
                        .ok_or(UiHostSurfacePresentationDenial::MalformedProjection)?;
                    Some(UiHeadlessResourceContact::new(
                        entry.content_identity(),
                        entry.kind(),
                        entry.byte_len(),
                    ))
                }
                None => None,
            };
            Ok(UiHeadlessPaintBatchMechanic::new(
                batch_index,
                batch.primitive_kind(),
                batch.primitive_count(),
                layer,
                resource,
            ))
        })
        .collect()
}

fn resolve_layer(
    projection: &UiMountedProjectionView,
    layer: UiMountedLayerProjection,
) -> Result<UiHeadlessLayerMechanic, UiHostSurfacePresentationDenial> {
    match layer {
        UiMountedLayerProjection::Omitted(reason) => Ok(UiHeadlessLayerMechanic::Omitted(reason)),
        UiMountedLayerProjection::Layer(reference) => {
            let row = projection
                .layers()
                .rows()
                .get(usize::from(reference.index()))
                .ok_or(UiHostSurfacePresentationDenial::MalformedProjection)?;
            Ok(UiHeadlessLayerMechanic::Ordered {
                semantic_order: row.semantic_order(),
                clip: resolve_clip(projection, row.clip())?,
            })
        }
    }
}

fn resolve_clip(
    projection: &UiMountedProjectionView,
    clip: UiMountedClipProjection,
) -> Result<UiHeadlessResolvedClip, UiHostSurfacePresentationDenial> {
    match clip {
        UiMountedClipProjection::Unclipped => Ok(UiHeadlessResolvedClip::Unclipped),
        UiMountedClipProjection::Omitted(reason) => Ok(UiHeadlessResolvedClip::Omitted(reason)),
        UiMountedClipProjection::Clip(reference) => projection
            .clips()
            .rows()
            .get(usize::from(reference.index()))
            .map(|_| UiHeadlessResolvedClip::Clip(reference.index()))
            .ok_or(UiHostSurfacePresentationDenial::MalformedProjection),
    }
}

fn translate_nodes(
    projection: &UiMountedProjectionView,
) -> Result<Vec<UiHeadlessNodeMechanic>, UiHostSurfacePresentationDenial> {
    projection
        .nodes()
        .iter()
        .map(|node| {
            let paint = match node.paint() {
                UiMountedPaintProjection::Omitted(reason) => {
                    UiHeadlessNodePaintMechanic::Omitted(reason)
                }
                UiMountedPaintProjection::Batch(reference) => {
                    projection
                        .paint_batches()
                        .rows()
                        .get(usize::from(reference.index()))
                        .ok_or(UiHostSurfacePresentationDenial::MalformedProjection)?;
                    UiHeadlessNodePaintMechanic::Batch(reference.index())
                }
            };
            Ok(UiHeadlessNodeMechanic::new(
                node.mounted_instance(),
                node.role(),
                node.participation(),
                node.allocation(),
                node.preview(),
                paint,
                node.accessibility(),
            ))
        })
        .collect()
}

fn unperformed_effects(
    projection: &UiMountedProjectionView,
) -> Result<Vec<UiHeadlessUnperformedEffect>, UiHostSurfacePresentationDenial> {
    let mut effects = vec![UiHeadlessUnperformedEffect::NativePaint {
        paint_batch_count: u32::try_from(projection.paint_batches().rows().len())
            .map_err(|_| UiHostSurfacePresentationDenial::CapacityExceeded)?,
        preview_node_count: matching_node_count(projection, |node| {
            matches!(
                node.preview(),
                worth_ui_host_contract::UiMountedPreviewProjection::Resize { .. }
            )
        })?,
    }];
    let accessibility_count = matching_node_count(projection, |node| {
        matches!(
            node.accessibility(),
            UiMountedAccessibilityProjection::Admitted(_)
        )
    })?;
    if accessibility_count > 0 {
        effects.push(UiHeadlessUnperformedEffect::Accessibility {
            node_count: accessibility_count,
        });
    }
    let focus_count = matching_node_count(projection, |node| {
        node.participation().focus().status() == UiMountedParticipationStatus::Admitted
    })?;
    if focus_count > 0 {
        effects.push(UiHeadlessUnperformedEffect::Focus {
            node_count: focus_count,
        });
    }
    for (index, batch) in projection.spatial_batches().rows().iter().enumerate() {
        effects.push(UiHeadlessUnperformedEffect::CanvasSpatial {
            batch_index: u16::try_from(index)
                .map_err(|_| UiHostSurfacePresentationDenial::CapacityExceeded)?,
            primitive_count: batch.primitive_count(),
            hit_region_count: batch.hit_region_count(),
            overlay_row_count: batch.overlay_row_count(),
            tool_state_row_count: batch.tool_state_row_count(),
        });
    }
    for (index, batch) in projection.realtime_batches().rows().iter().enumerate() {
        effects.push(UiHeadlessUnperformedEffect::Realtime {
            batch_index: u16::try_from(index)
                .map_err(|_| UiHostSurfacePresentationDenial::CapacityExceeded)?,
            overlay_row_count: batch.overlay_row_count(),
        });
    }
    Ok(effects)
}

fn matching_node_count(
    projection: &UiMountedProjectionView,
    predicate: impl Fn(&worth_ui_host_contract::UiMountedNodeProjectionView) -> bool,
) -> Result<u32, UiHostSurfacePresentationDenial> {
    u32::try_from(
        projection
            .nodes()
            .iter()
            .filter(|node| predicate(node))
            .count(),
    )
    .map_err(|_| UiHostSurfacePresentationDenial::CapacityExceeded)
}

fn has_accessibility(projection: &UiMountedProjectionView) -> bool {
    projection.nodes().iter().any(|node| {
        matches!(
            node.accessibility(),
            UiMountedAccessibilityProjection::Admitted(_)
        )
    })
}

fn has_focus(projection: &UiMountedProjectionView) -> bool {
    projection
        .nodes()
        .iter()
        .any(|node| node.participation().focus().status() == UiMountedParticipationStatus::Admitted)
}

fn paint_order(batch: &UiHeadlessPaintBatchMechanic) -> (u8, u32, u16) {
    match batch.layer() {
        UiHeadlessLayerMechanic::Ordered { semantic_order, .. } => {
            (0, semantic_order, batch.batch_index())
        }
        UiHeadlessLayerMechanic::Omitted(_) => (1, u32::MAX, batch.batch_index()),
    }
}
