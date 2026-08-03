use std::sync::Arc;

use worth_ui_host_contract::{
    UiHostSurfacePresentationDenial, UiMountedAllocationProjection, UiMountedCoordinateSpace,
    UiMountedGeometryPosture, UiMountedParticipationStatus, UiMountedSemanticTextMechanic,
    UiMountedTextSchemaVersion, UiSemanticTextBaselinePosture, UiSemanticTextProfile,
    UiSemanticTextSlot, UiSemanticTextWrapPosture,
};

#[derive(Clone)]
pub(super) struct UiEguiPreparedSemanticText {
    pub(super) origin: egui::Pos2,
    pub(super) clip_rect: egui::Rect,
    pub(super) text: Arc<str>,
    pub(super) color: egui::Color32,
    pub(super) font: egui::FontId,
    pub(super) layer_semantic_order: u32,
}

pub(super) fn prepare(
    view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
) -> Result<Vec<UiEguiPreparedSemanticText>, UiHostSurfacePresentationDenial> {
    let projection = view.projection();
    let rows = projection.semantic_text().rows();
    let mut visited = vec![false; rows.len()];
    let mut prepared = Vec::with_capacity(rows.len());
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
            validate_row(view, node, row)?;
            prepared.push(translate(row));
        }
    }
    if visited.iter().any(|visited| !visited) {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    Ok(prepared)
}

fn validate_row(
    view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
    node: &worth_ui_host_contract::UiMountedNodeProjectionView,
    row: &UiMountedSemanticTextMechanic,
) -> Result<(), UiHostSurfacePresentationDenial> {
    let projection = view.projection();
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
        || !matching_collection_identity(row)
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

fn translate(row: &UiMountedSemanticTextMechanic) -> UiEguiPreparedSemanticText {
    let channels = row.color().channels();
    UiEguiPreparedSemanticText {
        origin: egui::pos2(row.origin_x(), row.origin_y()),
        clip_rect: super::native_paint::egui_rect(row.clip_bounds()),
        text: Arc::from(row.text()),
        color: egui::Color32::from_rgba_unmultiplied(
            channels[0],
            channels[1],
            channels[2],
            channels[3],
        ),
        font: egui::FontId::new(
            f32::from(row.profile().size_millipoints()) / 1_000.0,
            egui::FontFamily::Proportional,
        ),
        layer_semantic_order: row.layer_semantic_order(),
    }
}

#[cfg(test)]
#[path = "semantic_text_tests.rs"]
mod tests;
