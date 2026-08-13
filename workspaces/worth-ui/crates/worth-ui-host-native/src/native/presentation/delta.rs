use worth_ui_host_contract::{
    UiHostPresentationCostInput, UiHostPresentationCostReport, UiHostSurfacePresentationDenial,
    UiMountedCanonicalBox, UiMountedCanonicalBoxInput, UiMountedFrameConsumptionView,
    UiMountedPaintCommand, UiMountedPresentationWorkView,
};

use super::port::UiNativePresentationPortObservation;
use super::raster::{raster_damage_for_basis, UiNativeRasterBasis};
use super::retained_draw_list::UiNativeRetainedDeltaUndo;
use super::{
    reserve_presentation_owners, settle_port_result, UiNativePresentationFailure,
    UiNativePresentationPort, UiNativePresentationPortPlan, UiNativeRasterOperation,
    UiNativeRetainedDrawList,
};
use crate::native::{UiNativeGraphics, UiNativeResourceRegistry};

pub(crate) struct UiNativeDeltaPresentation {
    cost: UiHostPresentationCostReport,
    painted: bool,
    pixels: Option<[[u8; 4]; 2]>,
    port_crossings: u8,
}

impl UiNativeDeltaPresentation {
    pub(crate) fn into_parts(
        self,
    ) -> (UiHostPresentationCostReport, bool, Option<[[u8; 4]; 2]>, u8) {
        (self.cost, self.painted, self.pixels, self.port_crossings)
    }
}

pub(crate) fn present_delta<Port: UiNativePresentationPort>(
    graphics: &mut UiNativeGraphics,
    resources: &mut UiNativeResourceRegistry,
    view: &UiMountedFrameConsumptionView<'_>,
    retained: &mut UiNativeRetainedDrawList,
) -> Result<UiNativeDeltaPresentation, UiNativePresentationFailure> {
    let UiMountedPresentationWorkView::Delta(delta) = view.presentation_work() else {
        return Err(before_effects(
            UiHostSurfacePresentationDenial::AdapterDeclined,
        ));
    };
    let basis = UiNativeRasterBasis::from_graphics(graphics);
    let (plan, undo) = prepare_delta_plan(basis, delta, retained)?;
    if plan.operations.is_empty() && !plan.clear_retained_target {
        return Ok(UiNativeDeltaPresentation {
            cost: plan.cost,
            painted: false,
            pixels: None,
            port_crossings: 0,
        });
    }
    let owners = match reserve_presentation_owners(resources) {
        Ok(owners) => owners,
        Err(failure) => {
            retained
                .rollback_delta(undo)
                .expect("capacity refusal must preserve retained state");
            return Err(failure);
        }
    };
    settle_staged_delta(
        retained,
        undo,
        settle_port_result(resources, owners, Port::present(graphics, plan)),
    )
}

fn prepare_delta_plan(
    basis: UiNativeRasterBasis,
    delta: &worth_ui_host_contract::UiMountedPresentationDelta,
    retained: &mut UiNativeRetainedDrawList,
) -> Result<(UiNativePresentationPortPlan, UiNativeRetainedDeltaUndo), UiNativePresentationFailure>
{
    let (replay, undo) = retained
        .stage_delta(delta)
        .map_err(|_| before_effects(UiHostSurfacePresentationDenial::MalformedProjection))?;
    match build_plan(basis, retained, replay, delta.nodes().len()).map_err(before_effects) {
        Ok(plan) => Ok((plan, undo)),
        Err(failure) => {
            retained
                .rollback_delta(undo)
                .expect("a prevalidated native delta must roll back exactly");
            Err(failure)
        }
    }
}

pub(crate) fn settle_staged_delta(
    retained: &mut UiNativeRetainedDrawList,
    undo: UiNativeRetainedDeltaUndo,
    result: Result<UiNativePresentationPortObservation, UiNativePresentationFailure>,
) -> Result<UiNativeDeltaPresentation, UiNativePresentationFailure> {
    match result {
        Ok(observation) => {
            let (pixels, cost, port_crossings) = observation.into_parts();
            Ok(UiNativeDeltaPresentation {
                cost,
                painted: true,
                pixels: Some(pixels),
                port_crossings,
            })
        }
        Err(failure @ UiNativePresentationFailure::BeforeEffects(_)) => {
            retained
                .rollback_delta(undo)
                .expect("before-effect port refusal must preserve retained state");
            Err(failure)
        }
        Err(failure @ UiNativePresentationFailure::Indeterminate(_)) => {
            retained
                .rollback_delta(undo)
                .expect("indeterminate effects preserve predecessor application truth");
            Err(failure)
        }
    }
}

fn build_plan(
    basis: UiNativeRasterBasis,
    retained: &UiNativeRetainedDrawList,
    replay: super::retained_draw_list::UiNativeRetainedReplayPlan,
    node_changes: usize,
) -> Result<UiNativePresentationPortPlan, UiHostSurfacePresentationDenial> {
    validate_replay_baseline(replay.baseline_rgba8)?;
    let mut operations = Vec::new();
    let mut cleared_pixels = 0_u64;
    let mut rendered_pixels = 0_u64;
    let mut replayed_commands = 0_u64;
    for region in &replay.regions {
        let Some(clear) =
            raster_damage_for_basis(region.damage.bounds(), basis).map_err(|_| malformed())?
        else {
            continue;
        };
        cleared_pixels = add_pixels(cleared_pixels, clear)?;
        operations.push(UiNativeRasterOperation::Clear(clear));
        for identity in &region.replay {
            let command = retained.command(*identity).ok_or_else(malformed)?;
            let Some(bounds) = clipped_damage(command, region.damage.bounds())? else {
                continue;
            };
            let Some(rect) = raster_damage_for_basis(bounds, basis).map_err(|_| malformed())?
            else {
                continue;
            };
            let UiMountedPaintCommand::FilledRect { mechanic, .. } = command else {
                return Err(UiHostSurfacePresentationDenial::AdapterDeclined);
            };
            rendered_pixels = add_pixels(rendered_pixels, rect)?;
            replayed_commands = replayed_commands.checked_add(1).ok_or_else(malformed)?;
            operations.push(UiNativeRasterOperation::FilledRect {
                rect,
                source_rgba8: mechanic.color().channels(),
            });
        }
    }
    let cost = delta_cost(
        basis.extent(),
        replay.counters,
        operations.len(),
        cleared_pixels,
        rendered_pixels,
        replayed_commands,
        node_changes,
    )?;
    Ok(UiNativePresentationPortPlan {
        clear_retained_target: false,
        operations: operations.into_boxed_slice(),
        cost,
    })
}

fn validate_replay_baseline(
    baseline_rgba8: [u8; 4],
) -> Result<(), UiHostSurfacePresentationDenial> {
    (baseline_rgba8 == [0, 0, 0, 0])
        .then_some(())
        .ok_or(UiHostSurfacePresentationDenial::MalformedProjection)
}

fn clipped_damage(
    command: &UiMountedPaintCommand,
    damage: UiMountedCanonicalBox,
) -> Result<Option<UiMountedCanonicalBox>, UiHostSurfacePresentationDenial> {
    let bounds = command.bounds();
    let clip = command.clip_bounds();
    if bounds.coordinate_space() != clip.coordinate_space()
        || bounds.coordinate_space() != damage.coordinate_space()
    {
        return Err(malformed());
    }
    let left = bounds.x().max(clip.x()).max(damage.x());
    let top = bounds.y().max(clip.y()).max(damage.y());
    let right = edge(bounds, true)
        .min(edge(clip, true))
        .min(edge(damage, true));
    let bottom = edge(bounds, false)
        .min(edge(clip, false))
        .min(edge(damage, false));
    if right <= left || bottom <= top {
        return Ok(None);
    }
    UiMountedCanonicalBox::canonicalize(UiMountedCanonicalBoxInput {
        x: left,
        y: top,
        width: right - left,
        height: bottom - top,
        coordinate_space: bounds.coordinate_space(),
    })
    .map(Some)
    .map_err(|_| malformed())
}

fn edge(bounds: UiMountedCanonicalBox, horizontal: bool) -> f32 {
    if horizontal {
        bounds.x() + bounds.width()
    } else {
        bounds.y() + bounds.height()
    }
}

fn delta_cost(
    extent: [u32; 2],
    counters: super::retained_draw_list::UiNativeRetainedMutationCounters,
    operation_count: usize,
    cleared_pixels: u64,
    rendered_pixels: u64,
    replayed_commands: u64,
    node_changes: usize,
) -> Result<UiHostPresentationCostReport, UiHostSurfacePresentationDenial> {
    let physical = operation_count > 0;
    let operations = u64::try_from(operation_count).map_err(|_| malformed())?;
    let node_changes = u64::try_from(node_changes).map_err(|_| malformed())?;
    let damage_index_probes = counters
        .damage_index_branch_aabb_probes
        .checked_add(counters.damage_index_leaf_command_bounds_probes)
        .ok_or_else(malformed)?;
    let presented_pixels = u64::from(extent[0]) * u64::from(extent[1]);
    Ok(UiHostPresentationCostReport::from_adapter(
        UiHostPresentationCostInput {
            presented_surfaces: u64::from(physical),
            translated_rows: counters
                .draw_mutations
                .checked_add(node_changes)
                .ok_or_else(malformed)?,
            native_resource_cache_hits: counters.replayed_commands,
            delta_rows_carried: counters
                .draw_mutations
                .checked_add(node_changes)
                .and_then(|value| value.checked_add(counters.order_mutations))
                .and_then(|value| value.checked_add(counters.damage_rows_carried))
                .ok_or_else(malformed)?,
            draw_list_mutations: counters.draw_mutations,
            order_mutations: counters.order_mutations,
            order_index_lookups: counters.order_index_lookups,
            order_index_node_touches: counters.order_index_node_touches,
            order_index_rotations: counters.order_index_rotations,
            order_index_high_water: counters.order_index_high_water,
            logical_damage_regions: counters.damage_regions,
            damage_index_probes,
            damage_index_stored_records: counters.damage_index_stored_records,
            damage_index_high_water: counters.damage_index_high_water,
            damage_region_command_checks: counters.damage_region_command_checks,
            intersecting_commands: counters.replayed_commands,
            replayed_commands,
            cleared_pixels,
            rendered_pixels,
            presented_pixels: physical.then_some(presented_pixels).unwrap_or(0),
            gpu_writes: u64::from(physical && operations > 0),
            render_passes: physical.then_some(2).unwrap_or(0),
            surface_copies: u64::from(physical),
            surface_acquisitions: u64::from(physical),
            queue_submissions: u64::from(physical),
            presents: u64::from(physical),
            ..Default::default()
        },
    ))
}

fn add_pixels(total: u64, rect: super::RasterRect) -> Result<u64, UiHostSurfacePresentationDenial> {
    total
        .checked_add(u64::from(rect.physical_width) * u64::from(rect.physical_height))
        .ok_or_else(malformed)
}

fn malformed() -> UiHostSurfacePresentationDenial {
    UiHostSurfacePresentationDenial::MalformedProjection
}

fn before_effects(denial: UiHostSurfacePresentationDenial) -> UiNativePresentationFailure {
    UiNativePresentationFailure::BeforeEffects(denial)
}

#[cfg(test)]
#[path = "delta_tests.rs"]
mod tests;
