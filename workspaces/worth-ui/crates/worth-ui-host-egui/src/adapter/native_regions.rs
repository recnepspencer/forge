use worth_ui_host_contract::{
    UiHostRealizedGeometry, UiHostRealizedOrdering, UiHostRealizedRegion,
    UiHostRealizedRegionParticipation, UiHostSurfacePresentationDenial,
    UiMountedAllocationProjection, UiMountedCoordinateSpace, UiMountedGeometryPosture,
    UiMountedHitTestProjection, UiMountedParticipationStatus, UiMountedProjectionView,
};

pub(super) fn prepare(
    projection: &UiMountedProjectionView,
) -> Result<Vec<UiHostRealizedRegion>, UiHostSurfacePresentationDenial> {
    let paint = projection
        .filled_rects()
        .rows()
        .iter()
        .copied()
        .map(paint_region);
    let hit_test = projection
        .hit_tests()
        .rows()
        .iter()
        .copied()
        .map(|row| hit_test_region(projection, row));
    paint.map(Ok).chain(hit_test).collect::<Result<Vec<_>, _>>()
}

fn paint_region(row: worth_ui_host_contract::UiMountedFilledRectMechanic) -> UiHostRealizedRegion {
    UiHostRealizedRegion::observed_by_host(
        row.node_receipt(),
        UiHostRealizedGeometry::observed_by_host(row.bounds(), row.clip_bounds()),
        UiHostRealizedOrdering::observed_by_host(
            row.layer_semantic_order(),
            UiHostRealizedRegionParticipation::Paint,
        ),
    )
}

fn hit_test_region(
    projection: &UiMountedProjectionView,
    row: worth_ui_host_contract::UiMountedHitTestMechanic,
) -> Result<UiHostRealizedRegion, UiHostSurfacePresentationDenial> {
    validate_hit_test_row(projection, row)?;
    Ok(UiHostRealizedRegion::observed_by_host(
        row.node_receipt(),
        UiHostRealizedGeometry::observed_by_host(row.bounds(), row.clip_bounds()),
        UiHostRealizedOrdering::observed_by_host(
            row.order().rank(),
            UiHostRealizedRegionParticipation::HitTest,
        ),
    ))
}

fn validate_hit_test_row(
    projection: &UiMountedProjectionView,
    row: worth_ui_host_contract::UiMountedHitTestMechanic,
) -> Result<(), UiHostSurfacePresentationDenial> {
    if row.frame() != projection.frame()
        || row.surface() != projection.surface()
        || row.binding() != projection.binding()
        || row.bounds().posture() != UiMountedGeometryPosture::Area
        || row.clip_bounds().posture() != UiMountedGeometryPosture::Area
        || row.bounds().coordinate_space() != UiMountedCoordinateSpace::Viewport
    {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    let node = projection
        .nodes()
        .iter()
        .find(|node| node.mounted_instance() == row.mounted_instance())
        .ok_or(UiHostSurfacePresentationDenial::MalformedProjection)?;
    let matching_reference = matches!(
        node.hit_test(),
        UiMountedHitTestProjection::Region(reference)
            if projection.hit_tests().resolve(reference) == Some(&row)
    );
    let matching_allocation = matches!(
        node.allocation(),
        UiMountedAllocationProjection::Known { bounds, .. } if bounds == row.bounds()
    );
    if node.node_receipt() != row.node_receipt()
        || node.participation().hit_test().status() != UiMountedParticipationStatus::Admitted
        || !matching_reference
        || !matching_allocation
    {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    Ok(())
}
