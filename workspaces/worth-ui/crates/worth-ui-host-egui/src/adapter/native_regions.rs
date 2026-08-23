use std::collections::HashMap;

use worth_ui_host_contract::{
    UiHostRealizedGeometry, UiHostRealizedOrdering, UiHostRealizedRegion,
    UiHostRealizedRegionParticipation, UiHostSurfacePresentationDenial,
    UiMountedAllocationProjection, UiMountedCoordinateSpace, UiMountedGeometryPosture,
    UiMountedHitTestProjection, UiMountedPaintCommand, UiMountedPaintCommandChange,
    UiMountedPaintCommandIdentity, UiMountedPaintOrderIdentity, UiMountedParticipationStatus,
    UiMountedPresentationDelta, UiMountedProjectionView,
};

#[derive(Clone)]
pub(super) struct UiEguiRetainedNativeRegions {
    paint: HashMap<UiMountedPaintCommandIdentity, UiHostRealizedRegion>,
    paint_order: Vec<UiMountedPaintOrderIdentity>,
    hit_test: Vec<UiHostRealizedRegion>,
}

impl UiEguiRetainedNativeRegions {
    pub(super) fn prepare_projection(
        projection: &UiMountedProjectionView,
        commands: &HashMap<UiMountedPaintCommandIdentity, UiMountedPaintCommand>,
        paint_order: &[UiMountedPaintOrderIdentity],
    ) -> Result<Self, UiHostSurfacePresentationDenial> {
        let paint = commands
            .values()
            .filter_map(command_region)
            .collect::<HashMap<_, _>>();
        if paint.len() != paint_order.len()
            || paint_order
                .iter()
                .any(|identity| !paint.contains_key(&identity.command()))
        {
            return Err(UiHostSurfacePresentationDenial::MalformedProjection);
        }
        let hit_test = projection
            .hit_tests()
            .rows()
            .iter()
            .copied()
            .map(|row| hit_test_region(projection, row))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            paint,
            paint_order: paint_order.to_vec(),
            hit_test,
        })
    }

    pub(super) fn apply_delta(
        &self,
        delta: &UiMountedPresentationDelta,
        paint_order: &[UiMountedPaintOrderIdentity],
    ) -> Self {
        let mut candidate = self.clone();
        for change in delta.changes() {
            match change {
                UiMountedPaintCommandChange::Insert(command) => {
                    if let Some((identity, region)) = command_region(command) {
                        candidate.paint.insert(identity, region);
                    }
                }
                UiMountedPaintCommandChange::Replace {
                    predecessor,
                    successor,
                } => {
                    candidate.paint.remove(predecessor);
                    if let Some((identity, region)) = command_region(successor) {
                        candidate.paint.insert(identity, region);
                    }
                }
                UiMountedPaintCommandChange::Remove(identity) => {
                    candidate.paint.remove(identity);
                }
            }
        }
        candidate.paint_order = paint_order.to_vec();
        candidate
    }

    pub(super) fn realized(&self) -> Vec<UiHostRealizedRegion> {
        self.paint_order
            .iter()
            .map(|identity| {
                self.paint
                    .get(&identity.command())
                    .copied()
                    .expect("validated paint order identity must resolve")
            })
            .chain(self.hit_test.iter().copied())
            .collect()
    }
}

fn command_region(
    command: &UiMountedPaintCommand,
) -> Option<(UiMountedPaintCommandIdentity, UiHostRealizedRegion)> {
    match command {
        UiMountedPaintCommand::FilledRect {
            identity, mechanic, ..
        } => Some((
            *identity,
            UiHostRealizedRegion::observed_by_host(
                mechanic.node_receipt(),
                UiHostRealizedGeometry::observed_by_host(mechanic.bounds(), mechanic.clip_bounds()),
                UiHostRealizedOrdering::observed_by_host(
                    mechanic.layer_semantic_order(),
                    UiHostRealizedRegionParticipation::Paint,
                ),
            ),
        )),
        UiMountedPaintCommand::SemanticText {
            identity, mechanic, ..
        } => Some((
            *identity,
            UiHostRealizedRegion::observed_by_host(
                mechanic.node_receipt(),
                UiHostRealizedGeometry::observed_by_host(mechanic.bounds(), mechanic.clip_bounds()),
                UiHostRealizedOrdering::observed_by_host(
                    mechanic.layer_semantic_order(),
                    UiHostRealizedRegionParticipation::Paint,
                ),
            ),
        )),
    }
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
