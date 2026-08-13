use worth_ui_host_contract::{
    UiHostPresentationCostInput, UiHostPresentationCostReport, UiHostSurfacePresentationDenial,
    UiMountedFrameConsumptionView, UiMountedPaintCommand, UiMountedPresentationWorkView,
};

pub(crate) struct UiNativeColdReconstruction {
    cost: UiHostPresentationCostReport,
    retained: UiNativeRetainedDrawList,
    pixels: [[u8; 4]; 2],
    port_crossings: u8,
}

impl UiNativeColdReconstruction {
    pub(crate) fn into_parts(
        self,
    ) -> (
        UiHostPresentationCostReport,
        UiNativeRetainedDrawList,
        [[u8; 4]; 2],
        u8,
    ) {
        (self.cost, self.retained, self.pixels, self.port_crossings)
    }
}

pub(crate) fn present_cold_reconstruction<Port: UiNativePresentationPort>(
    graphics: &mut UiNativeGraphics,
    resources: &mut UiNativeResourceRegistry,
    view: &UiMountedFrameConsumptionView<'_>,
) -> Result<UiNativeColdReconstruction, UiNativePresentationFailure> {
    let UiMountedPresentationWorkView::Reconstruction(work) = view.presentation_work() else {
        return Err(malformed());
    };
    let retained = UiNativeRetainedDrawList::reconstruction(work).map_err(|_| malformed())?;
    let plan = build_plan(graphics, &retained)?;
    let owners = reserve_presentation_owners(resources)?;
    let observation = settle_port_result(resources, owners, Port::present(graphics, plan))?;
    let (pixels, cost, port_crossings) = observation.into_parts();
    Ok(UiNativeColdReconstruction {
        cost,
        retained,
        pixels,
        port_crossings,
    })
}

use super::{
    raster::raster_rect, reserve_presentation_owners, settle_port_result, UiNativeGraphics,
    UiNativePresentationFailure, UiNativePresentationPort, UiNativePresentationPortPlan,
    UiNativeRasterOperation, UiNativeResourceRegistry, UiNativeRetainedDrawList,
};

fn build_plan(
    graphics: &UiNativeGraphics,
    retained: &UiNativeRetainedDrawList,
) -> Result<UiNativePresentationPortPlan, UiNativePresentationFailure> {
    let commands = retained
        .reconstruction_commands()
        .map_err(|_| malformed())?;
    let rows = u64::try_from(commands.len()).map_err(|_| malformed())?;
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
            draw_list_mutations: rows,
            order_mutations: rows,
            retained_command_scans: rows,
            damage_index_stored_records: rows,
            damage_index_high_water: rows,
            intersecting_commands: rows,
            replayed_commands: rows,
            cleared_pixels: pixels,
            rendered_pixels,
            presented_pixels: pixels,
            gpu_writes: u64::from(rows > 0),
            render_passes: 2,
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
