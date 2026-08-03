use std::sync::Arc;

use worth_ui_host_contract::{
    UiHostProtocolDenial, UiHostProtocolSchemaFamily, UiHostSurfacePresentationDenial,
    UiMountedAllocationProjection, UiMountedCoordinateSpace, UiMountedFrameConsumptionView,
    UiMountedGeometryPosture, UiMountedParticipationStatus, UiMountedSemanticTextMechanic,
    UiMountedTextSchemaVersion, UiSemanticTextBaselinePosture, UiSemanticTextProfile,
    UiSemanticTextSlot, UiSemanticTextWrapPosture,
};

use super::super::headless_transcript::{
    UiHeadlessSemanticTextMechanic, UiHeadlessSemanticTextMechanicInput,
};

pub(super) fn translate(
    view: &UiMountedFrameConsumptionView<'_>,
) -> Result<Vec<UiHeadlessSemanticTextMechanic>, UiHostSurfacePresentationDenial> {
    validate_protocol(view)?;
    let projection = view.projection();
    let rows = projection.semantic_text().rows();
    if !rows.is_empty()
        && projection.semantic_text().schema() != UiMountedTextSchemaVersion::current()
    {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    let mut visited = vec![false; rows.len()];
    let mut translated = Vec::with_capacity(rows.len());
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
            translated.push(translate_row(row));
        }
    }
    if visited.iter().any(|visited| !visited) {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    Ok(translated)
}

fn validate_protocol(
    view: &UiMountedFrameConsumptionView<'_>,
) -> Result<(), UiHostSurfacePresentationDenial> {
    if !view.projection().semantic_text().rows().is_empty()
        && view.protocol().contract().mounted_frame().revision()
            < UiMountedTextSchemaVersion::REQUIRED_MOUNTED_FRAME_REVISION
    {
        return Err(UiHostSurfacePresentationDenial::Protocol(
            UiHostProtocolDenial::SchemaTooOld(UiHostProtocolSchemaFamily::MountedFrame),
        ));
    }
    Ok(())
}

fn validate_row(
    view: &UiMountedFrameConsumptionView<'_>,
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
        || (matches!(row.slot(), UiSemanticTextSlot::CollectionValue { .. })
            != row.collection_row().is_some())
    {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    Ok(())
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

fn translate_row(row: &UiMountedSemanticTextMechanic) -> UiHeadlessSemanticTextMechanic {
    UiHeadlessSemanticTextMechanic::new(UiHeadlessSemanticTextMechanicInput {
        content_generation: row.content_generation(),
        mounted_instance: row.mounted_instance(),
        node_receipt: row.node_receipt(),
        allocation_basis: row.allocation_basis(),
        bounds: row.bounds(),
        origin_x: row.origin_x(),
        origin_y: row.origin_y(),
        text: Arc::from(row.text()),
        slot: row.slot(),
        collection_row: row.collection_row().cloned(),
        color: row.color(),
        profile: row.profile(),
        layer_semantic_order: row.layer_semantic_order(),
        semantic_digest: row.semantic_digest(),
    })
}
