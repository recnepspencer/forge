use worth_ui_host_contract::{
    UiHostSurfacePresentationDenial, UiMountedAllocationProjection, UiMountedCoordinateSpace,
    UiMountedGeometryPosture, UiMountedParticipationStatus, UiMountedSemanticTextMechanic,
    UiMountedTextSchemaVersion, UiSemanticTextBaselinePosture, UiSemanticTextProfile,
    UiSemanticTextSlot, UiSemanticTextWrapPosture,
};

#[cfg(test)]
pub(super) fn prepare(
    view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
) -> Result<(), UiHostSurfacePresentationDenial> {
    let worth_ui_host_contract::UiMountedPresentationWorkView::Initial(initial) =
        view.presentation_work()
    else {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    };
    let projection = initial.projection();
    validate_projection(view, projection)
}

pub(super) fn validate_projection(
    view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
    projection: &worth_ui_host_contract::UiMountedProjectionView,
) -> Result<(), UiHostSurfacePresentationDenial> {
    let rows = projection.semantic_text().rows();
    let mut visited = vec![false; rows.len()];
    for node in projection.nodes() {
        for reference in node.semantic_text() {
            let index = usize::from(reference.index());
            let row = projection
                .semantic_text()
                .resolve(*reference)
                .ok_or(UiHostSurfacePresentationDenial::MalformedProjection)?;
            if std::mem::replace(
                visited
                    .get_mut(index)
                    .ok_or(UiHostSurfacePresentationDenial::MalformedProjection)?,
                true,
            ) {
                return Err(UiHostSurfacePresentationDenial::MalformedProjection);
            }
            validate_row(view, projection, node, row)?;
        }
    }
    if visited.iter().any(|visited| !visited) {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    Ok(())
}

fn validate_row(
    view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
    projection: &worth_ui_host_contract::UiMountedProjectionView,
    node: &worth_ui_host_contract::UiMountedNodeProjectionView,
    row: &UiMountedSemanticTextMechanic,
) -> Result<(), UiHostSurfacePresentationDenial> {
    if row.schema() != UiMountedTextSchemaVersion::current()
        || row.frame() != projection.frame()
        || row.surface() != projection.surface()
        || row.binding() != projection.binding()
        || row.content_generation() != projection.content_generation()
        || row.capability_generation() != view.capability_generation()
        || row.capability_profile_digest() != view.capability_profile_digest()
        || row.bounds() != row.clip_bounds()
        || row.bounds().posture() != UiMountedGeometryPosture::Area
        || row.bounds().coordinate_space() != UiMountedCoordinateSpace::Viewport
        || !valid_origin(row)
        || row.mounted_instance() != node.mounted_instance()
        || row.node_receipt() != node.node_receipt()
        || !matching_allocation(node, row)
        || node.participation().paint().status() != UiMountedParticipationStatus::Admitted
        || node.participation().clip().status() != UiMountedParticipationStatus::Admitted
        || row.profile() != UiSemanticTextProfile::BodyDefault
        || row.profile().wrap() != UiSemanticTextWrapPosture::Clip
        || row.profile().baseline() != UiSemanticTextBaselinePosture::Alphabetic
        || row.foregrounds().len() != 1
        || !matching_collection_identity(row)
        || view.qualified_text_layout(row).is_none()
    {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    Ok(())
}

fn matching_collection_identity(row: &UiMountedSemanticTextMechanic) -> bool {
    matches!(row.slot(), UiSemanticTextSlot::CollectionValue { .. })
        == row.collection_row().is_some()
}

fn valid_origin(row: &UiMountedSemanticTextMechanic) -> bool {
    let bounds = row.bounds();
    row.origin_x().is_finite()
        && row.origin_y().is_finite()
        && row.origin_x() >= bounds.x()
        && row.origin_x() <= bounds.x() + bounds.width()
        && row.origin_y() >= bounds.y()
        && row.origin_y() <= bounds.y() + bounds.height()
}

fn matching_allocation(
    node: &worth_ui_host_contract::UiMountedNodeProjectionView,
    row: &UiMountedSemanticTextMechanic,
) -> bool {
    matches!(
        node.allocation(),
        UiMountedAllocationProjection::Known { bounds, basis }
            if bounds == row.bounds() && basis == row.allocation_basis()
    )
}

#[cfg(test)]
#[path = "semantic_text_tests.rs"]
mod tests;
