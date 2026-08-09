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
}

#[cfg(test)]
pub(super) fn prepare(
    view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
) -> Result<Vec<UiEguiPreparedSemanticText>, UiHostSurfacePresentationDenial> {
    let worth_ui_host_contract::UiMountedPresentationWorkView::Initial(initial) =
        view.presentation_work()
    else {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    };
    let projection = initial.projection();
    validate_projection(view, projection)?;
    Ok(projection
        .semantic_text()
        .rows()
        .iter()
        .map(translate)
        .collect())
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

pub(super) fn translate(row: &UiMountedSemanticTextMechanic) -> UiEguiPreparedSemanticText {
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
    }
}

impl UiEguiPreparedSemanticText {
    pub(super) fn paint(&self, painter: &egui::Painter) {
        painter.clone().with_clip_rect(self.clip_rect).text(
            self.origin,
            egui::Align2::LEFT_TOP,
            self.text.as_ref(),
            self.font.clone(),
            self.color,
        );
    }
}

#[cfg(test)]
#[path = "semantic_text_tests.rs"]
mod tests;
