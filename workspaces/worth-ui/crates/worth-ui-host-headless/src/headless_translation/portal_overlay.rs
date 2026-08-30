pub(super) fn validate(
    view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
    projection: &worth_ui_host_contract::UiMountedProjectionView,
) -> Result<(), worth_ui_host_contract::UiHostSurfacePresentationDenial> {
    let table = projection.portal_overlays();
    if !table.rows().is_empty()
        && view.protocol().contract().mounted_frame().revision()
            < worth_ui_host_contract::UiMountedPortalOverlaySchemaVersion::REQUIRED_MOUNTED_FRAME_REVISION
    {
        return Err(worth_ui_host_contract::UiHostSurfacePresentationDenial::Protocol(
            worth_ui_host_contract::UiHostProtocolDenial::SchemaTooOld(
                worth_ui_host_contract::UiHostProtocolSchemaFamily::MountedFrame,
            ),
        ));
    }
    if table.schema() != worth_ui_host_contract::UiMountedPortalOverlaySchemaVersion::current() {
        return Err(worth_ui_host_contract::UiHostSurfacePresentationDenial::MalformedProjection);
    }
    for row in table.rows() {
        let owner = projection
            .nodes()
            .iter()
            .find(|node| node.mounted_instance() == row.owner())
            .ok_or(worth_ui_host_contract::UiHostSurfacePresentationDenial::MalformedProjection)?;
        if row.schema() != table.schema()
            || row.frame() != projection.frame()
            || row.surface() != projection.surface()
            || row.binding() != projection.binding()
            || row.owner_receipt() != owner.node_receipt()
        {
            return Err(
                worth_ui_host_contract::UiHostSurfacePresentationDenial::MalformedProjection,
            );
        }
    }
    Ok(())
}
