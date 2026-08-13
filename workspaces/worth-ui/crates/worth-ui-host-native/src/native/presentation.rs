use worth_ui_host_contract::{
    UiHostPresentationCostInput, UiHostPresentationCostReport, UiHostSurfacePresentationDenial,
    UiMountedFrameConsumptionView, UiMountedPaintCommand, UiMountedPresentationWorkView,
};

use super::{
    UiNativeGraphics, UiNativePresentationInput, UiNativePresentationObservation,
    UiNativeResourceClass, UiNativeResourceRegistry,
};

mod damage_index;
mod damage_regions;
mod delta;
mod pipeline;
mod port;
mod raster;
mod readback_port;
mod reconstruction;
mod retained_draw_list;
mod retained_evidence_copy;
mod retained_order;
mod transaction_state;

use pipeline::{
    draw_raster_operations, draw_retained_to_surface, raster_pipelines, retained_transfer,
};
use raster::{raster_rect, rectangle_vertices, RasterRect, RasterVertex};
#[cfg(test)]
pub(crate) use readback_port::prove_nonuniform_readback_port;
use readback_port::{UiNativeReadbackPort, UiWgpuNativeReadbackPort};
use retained_evidence_copy::copy_evidence_pixels;

pub(crate) use delta::{present_delta, UiNativeDeltaPresentation};
pub(crate) use port::{
    UiNativePresentationPort, UiNativePresentationPortFailure, UiNativePresentationPortPlan,
    UiNativeRasterOperation, UiWgpuNativePresentationPort,
};
pub(crate) use reconstruction::present_cold_reconstruction;
pub(crate) use retained_draw_list::UiNativeRetainedDrawList;
pub(crate) use transaction_state::UiNativePendingPresentation;
pub(crate) use transaction_state::{
    reserve_presentation_owners, settle_port_result, UiNativePendingExternalObligation,
    UiNativePendingWgpuObligation,
};

pub(crate) const GPU_WAIT_DEADLINE: std::time::Duration = std::time::Duration::from_millis(5_000);

pub(crate) enum UiNativePresentationFailure {
    BeforeEffects(UiHostSurfacePresentationDenial),
    Indeterminate(UiNativePendingPresentation),
}

pub(crate) struct UiNativePresentedFrame {
    observation: UiNativePresentationObservation,
    cost: UiHostPresentationCostReport,
    retained: UiNativeRetainedDrawList,
}

struct ValidatedInitial {
    mechanics: Box<[worth_ui_host_contract::UiMountedFilledRectMechanic]>,
}

impl UiNativePresentedFrame {
    pub(crate) fn into_parts(
        self,
    ) -> (
        UiNativePresentationObservation,
        UiHostPresentationCostReport,
        UiNativeRetainedDrawList,
    ) {
        (self.observation, self.cost, self.retained)
    }
}

pub(crate) fn present_initial<Port: UiNativePresentationPort>(
    graphics: &mut UiNativeGraphics,
    resources: &mut UiNativeResourceRegistry,
    view: &UiMountedFrameConsumptionView<'_>,
) -> Result<UiNativePresentedFrame, UiNativePresentationFailure> {
    let UiMountedPresentationWorkView::Initial(initial_work) = view.presentation_work() else {
        return Err(UiNativePresentationFailure::BeforeEffects(
            UiHostSurfacePresentationDenial::AdapterDeclined,
        ));
    };
    let retained = UiNativeRetainedDrawList::initial(initial_work).map_err(|_| {
        UiNativePresentationFailure::BeforeEffects(
            UiHostSurfacePresentationDenial::MalformedProjection,
        )
    })?;
    let initial = validate_initial(view).map_err(UiNativePresentationFailure::BeforeEffects)?;
    let rects = initial
        .mechanics
        .iter()
        .copied()
        .map(|mechanic| {
            raster_rect(mechanic, graphics).map(|rect| (rect, mechanic.color().channels()))
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| {
            UiNativePresentationFailure::BeforeEffects(
                UiHostSurfacePresentationDenial::MalformedProjection,
            )
        })?;
    let cost = initial_presentation_cost(graphics.extent(), &rects);
    let owners = reserve_presentation_owners(resources)?;
    let result = Port::present(
        graphics,
        UiNativePresentationPortPlan {
            clear_retained_target: true,
            operations: rects
                .iter()
                .map(|(rect, source_rgba8)| UiNativeRasterOperation::FilledRect {
                    rect: *rect,
                    source_rgba8: *source_rgba8,
                })
                .collect(),
            cost,
        },
    );
    let external = settle_port_result(resources, owners, result)?;
    Ok(build_presented_frame(
        view, graphics, initial, external, retained,
    ))
}

fn build_presented_frame(
    view: &UiMountedFrameConsumptionView<'_>,
    graphics: &UiNativeGraphics,
    initial: ValidatedInitial,
    external: port::UiNativePresentationPortObservation,
    retained: UiNativeRetainedDrawList,
) -> UiNativePresentedFrame {
    let ValidatedInitial { mechanics } = initial;
    let order_ordinal = mechanics.len().saturating_sub(1);
    let mechanic = mechanics[order_ordinal];
    let (pixels, cost, port_crossings) = external.into_parts();
    let observation = observation_for_mechanic(
        view,
        graphics,
        mechanic,
        order_ordinal,
        pixels,
        cost,
        port_crossings,
    );
    UiNativePresentedFrame {
        observation,
        cost,
        retained,
    }
}

pub(crate) fn observation_for_retained(
    view: &UiMountedFrameConsumptionView<'_>,
    graphics: &UiNativeGraphics,
    retained: &UiNativeRetainedDrawList,
    pixels: [[u8; 4]; 2],
    cost: UiHostPresentationCostReport,
    port_crossings: u8,
) -> Option<UiNativePresentationObservation> {
    let (ordinal, mechanic) = retained.top_filled_rect()?;
    Some(observation_for_mechanic(
        view,
        graphics,
        mechanic,
        ordinal,
        pixels,
        cost,
        port_crossings,
    ))
}

fn observation_for_mechanic(
    view: &UiMountedFrameConsumptionView<'_>,
    graphics: &UiNativeGraphics,
    mechanic: worth_ui_host_contract::UiMountedFilledRectMechanic,
    order_ordinal: usize,
    pixels: [[u8; 4]; 2],
    cost: UiHostPresentationCostReport,
    port_crossings: u8,
) -> UiNativePresentationObservation {
    let [retained_baseline_rgba8, retained_center_rgba8] = pixels;
    let bounds = mechanic.bounds();
    UiNativePresentationObservation::new(UiNativePresentationInput {
        client_physical_size: graphics.extent(),
        scale_factor_milli: (graphics.scale_factor * 1_000.0).round() as u32,
        source_rgba8: mechanic.color().channels(),
        retained_center_rgba8,
        retained_baseline_rgba8,
        presented_frame: view.frame().diagnostic_value(),
        semantic_surface: view.surface().diagnostic_value(),
        binding_generation: view.binding().diagnostic_value(),
        mounted_instance: mechanic.mounted_instance().diagnostic_value(),
        node_receipt: mechanic.node_receipt().diagnostic_value(),
        presentation_attempt: view.attempt().diagnostic_value(),
        logical_bounds_milli: [
            milli(bounds.x()),
            milli(bounds.y()),
            milli(bounds.width()),
            milli(bounds.height()),
        ],
        order_ordinal: u16::try_from(order_ordinal).expect("native profile bounds paint order"),
        port_crossings,
        cost,
    })
}

fn milli(value: f32) -> i64 {
    (f64::from(value) * 1_000.0).round() as i64
}

fn initial_presentation_cost(
    extent: [u32; 2],
    rects: &[(RasterRect, [u8; 4])],
) -> UiHostPresentationCostReport {
    let pixels = u64::from(extent[0]) * u64::from(extent[1]);
    let rows = u64::try_from(rects.len()).expect("native profile bounds rectangle rows");
    let rendered_pixels = rects.iter().fold(0_u64, |total, (rect, _)| {
        total + u64::from(rect.physical_width) * u64::from(rect.physical_height)
    });
    UiHostPresentationCostReport::from_adapter(UiHostPresentationCostInput {
        presented_surfaces: 1,
        translated_rows: rows,
        native_resource_cache_misses: 1,
        intersecting_commands: rows,
        replayed_commands: rows,
        cleared_pixels: pixels,
        rendered_pixels,
        presented_pixels: pixels,
        gpu_writes: u64::from(!rects.is_empty()),
        render_passes: 2,
        surface_acquisitions: 1,
        queue_submissions: 1,
        presents: 1,
        ..Default::default()
    })
}

fn validate_initial(
    view: &UiMountedFrameConsumptionView<'_>,
) -> Result<ValidatedInitial, UiHostSurfacePresentationDenial> {
    let UiMountedPresentationWorkView::Initial(initial) = view.presentation_work() else {
        return Err(UiHostSurfacePresentationDenial::AdapterDeclined);
    };
    if !initial.projection().semantic_text().rows().is_empty()
        || initial
            .commands()
            .iter()
            .any(|command| matches!(command, UiMountedPaintCommand::SemanticText { .. }))
    {
        return Err(UiHostSurfacePresentationDenial::AdapterDeclined);
    }
    if initial.commands().is_empty()
        || initial.commands().len() != initial.projection().filled_rects().rows().len()
        || initial.order().len() != initial.commands().len()
        || !initial.order_integrity().admits(initial.order())
    {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    let commands = initial
        .commands()
        .iter()
        .map(|command| match command {
            UiMountedPaintCommand::FilledRect { identity, mechanic }
                if *identity
                    == worth_ui_host_contract::UiMountedPaintCommandIdentity::filled_rect(
                        mechanic,
                    )
                    && initial
                        .projection()
                        .filled_rects()
                        .rows()
                        .contains(mechanic) =>
            {
                Ok((*identity, *mechanic))
            }
            _ => Err(UiHostSurfacePresentationDenial::MalformedProjection),
        })
        .collect::<Result<std::collections::HashMap<_, _>, _>>()?;
    if commands.len() != initial.commands().len() {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    let mechanics = initial
        .order()
        .iter()
        .map(|order| {
            commands
                .get(&order.command())
                .copied()
                .ok_or(UiHostSurfacePresentationDenial::MalformedProjection)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(ValidatedInitial {
        mechanics: mechanics.into_boxed_slice(),
    })
}

#[cfg(test)]
#[path = "presentation_tests.rs"]
mod tests;
