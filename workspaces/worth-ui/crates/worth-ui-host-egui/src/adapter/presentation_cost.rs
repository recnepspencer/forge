use worth_ui_host_contract::{
    UiHostPresentationCostInput, UiHostPresentationCostReport, UiHostSurfacePresentationDenial,
    UiMountedProjectionView,
};

pub(super) fn for_projection(
    projection: &UiMountedProjectionView,
) -> Result<UiHostPresentationCostReport, UiHostSurfacePresentationDenial> {
    let identity_overlays = projection
        .nodes()
        .iter()
        .filter(|node| {
            matches!(
                node.diagnostic(),
                worth_ui_host_contract::UiMountedDiagnosticProjection::IdentityOverlay(_)
            )
        })
        .count();
    let rows = [
        projection.nodes().len(),
        projection.clips().rows().len(),
        projection.layers().rows().len(),
        projection.filled_rects().rows().len(),
        projection.semantic_text().rows().len(),
        projection.hit_tests().rows().len(),
        projection.paint_batches().rows().len(),
        projection.spatial_batches().rows().len(),
        projection.realtime_batches().rows().len(),
        projection.resources().entries().len(),
        identity_overlays,
    ]
    .into_iter()
    .try_fold(0usize, usize::checked_add)
    .ok_or(UiHostSurfacePresentationDenial::CapacityExceeded)?;
    let structural_bytes = [
        std::mem::size_of_val(projection.nodes()),
        std::mem::size_of_val(projection.clips().rows()),
        std::mem::size_of_val(projection.layers().rows()),
        std::mem::size_of_val(projection.filled_rects().rows()),
        std::mem::size_of_val(projection.semantic_text().rows()),
        std::mem::size_of_val(projection.hit_tests().rows()),
        std::mem::size_of_val(projection.paint_batches().rows()),
        std::mem::size_of_val(projection.spatial_batches().rows()),
        std::mem::size_of_val(projection.realtime_batches().rows()),
        std::mem::size_of_val(projection.resources().entries()),
        identity_overlays
            .checked_mul(std::mem::size_of::<
                worth_ui_host_contract::UiMountedIdentityOverlayMechanic,
            >())
            .ok_or(UiHostSurfacePresentationDenial::CapacityExceeded)?,
    ]
    .into_iter()
    .try_fold(0usize, usize::checked_add)
    .ok_or(UiHostSurfacePresentationDenial::CapacityExceeded)?;
    let text_bytes = projection
        .semantic_text()
        .rows()
        .iter()
        .try_fold(0usize, |total, row| total.checked_add(row.text().len()))
        .ok_or(UiHostSurfacePresentationDenial::CapacityExceeded)?;
    let bytes = structural_bytes
        .checked_add(text_bytes)
        .ok_or(UiHostSurfacePresentationDenial::CapacityExceeded)?;
    Ok(UiHostPresentationCostReport::from_adapter(
        UiHostPresentationCostInput {
            presented_surfaces: 1,
            translated_rows: u64::try_from(rows)
                .map_err(|_| UiHostSurfacePresentationDenial::CapacityExceeded)?,
            translated_bytes: u64::try_from(bytes)
                .map_err(|_| UiHostSurfacePresentationDenial::CapacityExceeded)?,
            native_resource_cache_hits: 0,
            native_resource_cache_misses: 0,
            asynchronous_handoffs: 0,
        },
    ))
}
