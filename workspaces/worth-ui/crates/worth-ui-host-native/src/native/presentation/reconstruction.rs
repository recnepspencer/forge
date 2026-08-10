use worth_ui_host_contract::{
    UiHostPresentationCostInput, UiHostPresentationCostReport, UiHostSurfacePresentationDenial,
    UiMountedPaintCommand,
};

use super::{
    raster::raster_rect, reserve_presentation_owners, settle_port_result, UiNativeGraphics,
    UiNativePresentationFailure, UiNativePresentationPort, UiNativePresentationPortPlan,
    UiNativeRasterOperation, UiNativeResourceRegistry, UiNativeRetainedDrawList,
};

pub(crate) fn present_reconstruction<Port: UiNativePresentationPort>(
    graphics: &mut UiNativeGraphics,
    resources: &mut UiNativeResourceRegistry,
    retained: &UiNativeRetainedDrawList,
) -> Result<UiHostPresentationCostReport, UiNativePresentationFailure> {
    let plan = build_plan(graphics, retained)?;
    let owners = reserve_presentation_owners(resources)?;
    settle_port_result(resources, owners, Port::present(graphics, plan))
        .map(|observation| observation.into_parts().1)
}

pub(super) fn build_plan(
    graphics: &UiNativeGraphics,
    retained: &UiNativeRetainedDrawList,
) -> Result<UiNativePresentationPortPlan, UiNativePresentationFailure> {
    let commands = retained
        .reconstruction_commands()
        .map_err(|_| malformed())?;
    let rows = u64::try_from(commands.len()).map_err(|_| malformed())?;
    let render_passes = rows.max(1) + 1;
    let mut operations = Vec::with_capacity(commands.len());
    let mut rendered_pixels = 0_u64;
    for command in commands {
        let UiMountedPaintCommand::FilledRect { mechanic, .. } = command else {
            return Err(UiNativePresentationFailure::BeforeEffects(
                UiHostSurfacePresentationDenial::AdapterDeclined,
            ));
        };
        let rect = raster_rect(*mechanic, graphics).map_err(|_| malformed())?;
        rendered_pixels = rendered_pixels
            .checked_add(u64::from(rect.physical_width) * u64::from(rect.physical_height))
            .ok_or_else(malformed)?;
        operations.push(UiNativeRasterOperation::FilledRect {
            rect,
            source_rgba8: mechanic.color().channels(),
        });
    }
    let pixels = u64::from(graphics.extent()[0]) * u64::from(graphics.extent()[1]);
    Ok(UiNativePresentationPortPlan {
        clear_retained_target: true,
        operations: operations.into_boxed_slice(),
        cost: UiHostPresentationCostReport::from_adapter(UiHostPresentationCostInput {
            presented_surfaces: 1,
            translated_rows: rows,
            retained_command_scans: rows,
            intersecting_commands: rows,
            replayed_commands: rows,
            cleared_pixels: pixels,
            rendered_pixels,
            presented_pixels: pixels,
            gpu_writes: render_passes,
            render_passes,
            surface_copies: 1,
            surface_acquisitions: 1,
            queue_submissions: 1,
            presents: 1,
            ..Default::default()
        }),
    })
}

fn malformed() -> UiNativePresentationFailure {
    UiNativePresentationFailure::BeforeEffects(UiHostSurfacePresentationDenial::MalformedProjection)
}
