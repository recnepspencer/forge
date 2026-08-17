use worth_ui_host_contract::{
    UiHostSurfacePresentationDenial, UiMountedAllocationProjection, UiMountedPaintProjection,
    UiMountedProjectionView, UiMountedStaticPaintSchemaVersion,
};

pub(super) fn validate_protocol(
    view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
    projection: &UiMountedProjectionView,
) -> Result<(), worth_ui_host_contract::UiHostSurfacePresentationDenial> {
    if !projection.filled_rects().rows().is_empty()
        && view.protocol().contract().mounted_frame().revision()
            < worth_ui_host_contract::UiMountedStaticPaintSchemaVersion::REQUIRED_MOUNTED_FRAME_REVISION
    {
        return Err(worth_ui_host_contract::UiHostSurfacePresentationDenial::Protocol(
            worth_ui_host_contract::UiHostProtocolDenial::SchemaTooOld(
                worth_ui_host_contract::UiHostProtocolSchemaFamily::MountedFrame,
            ),
        ));
    }
    Ok(())
}

pub(crate) fn translate_command(
    row: worth_ui_host_contract::UiMountedFilledRectMechanic,
) -> UiHeadlessFilledRectMechanic {
    UiHeadlessFilledRectMechanic::new(UiHeadlessFilledRectMechanicInput {
        command_identity: worth_ui_host_contract::UiMountedPaintCommandIdentity::filled_rect(&row),
        schema: row.schema(),
        frame: row.frame(),
        surface: row.surface(),
        binding: row.binding(),
        mounted_instance: row.mounted_instance(),
        node_receipt: row.node_receipt(),
        allocation_basis: row.allocation_basis(),
        bounds: row.bounds(),
        color: row.color(),
        layer_semantic_order: row.layer_semantic_order(),
        clip_bounds: row.clip_bounds(),
        semantic_digest: row.semantic_digest(),
    })
}

use super::super::headless_transcript::{
    UiHeadlessFilledRectMechanic, UiHeadlessFilledRectMechanicInput,
};

pub(super) fn translate_filled_rects(
    projection: &UiMountedProjectionView,
) -> Result<Vec<UiHeadlessFilledRectMechanic>, UiHostSurfacePresentationDenial> {
    if !projection.filled_rects().rows().is_empty()
        && projection.filled_rects().schema().revision()
            != UiMountedStaticPaintSchemaVersion::current().revision()
    {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    projection
        .filled_rects()
        .rows()
        .iter()
        .copied()
        .map(|row| {
            validate_row_basis(projection, row)?;
            Ok(translate_command(row))
        })
        .collect()
}

fn validate_row_basis(
    projection: &UiMountedProjectionView,
    row: worth_ui_host_contract::UiMountedFilledRectMechanic,
) -> Result<(), UiHostSurfacePresentationDenial> {
    if row.schema() != UiMountedStaticPaintSchemaVersion::current()
        || projection.filled_rects().schema() != UiMountedStaticPaintSchemaVersion::current()
        || row.frame() != projection.frame()
        || row.surface() != projection.surface()
        || row.binding() != projection.binding()
        || row.clip_bounds() != row.bounds()
    {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    let node = projection
        .nodes()
        .iter()
        .find(|node| node.mounted_instance() == row.mounted_instance())
        .ok_or(UiHostSurfacePresentationDenial::MalformedProjection)?;
    let matching_reference = matches!(
        node.paint(),
        UiMountedPaintProjection::FilledRect(reference)
            if projection.filled_rects().resolve(reference) == Some(&row)
    );
    let matching_allocation = matches!(
        node.allocation(),
        UiMountedAllocationProjection::Known { bounds, basis }
            if bounds == row.bounds() && basis == row.allocation_basis()
    );
    if node.node_receipt() != row.node_receipt() || !matching_reference || !matching_allocation {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    Ok(())
}
