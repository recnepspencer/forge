use worth_ui_host_contract::{
    UiHostPresentationCostReport, UiHostSurfacePresentationDenial, UiMountedCanonicalBox,
    UiMountedCanonicalBoxInput, UiMountedFrameConsumptionView, UiMountedPaintCommand,
    UiMountedPresentationWorkView,
};

use super::port::UiNativePresentationPortObservation;
use super::raster::{raster_damage_for_basis, UiNativeRasterBasis};
use super::retained_draw_list::UiNativeRetainedDeltaUndo;
use super::{
    reserve_presentation_owners, settle_port_result, UiNativePresentationFailure,
    UiNativePresentationPort, UiNativePresentationPortPlan, UiNativeRasterOperation,
    UiNativeRetainedDrawList,
};
use crate::native::{UiNativePresentationAccess, UiNativeResourceRegistry};

#[path = "delta/cost.rs"]
mod cost;
pub(super) use cost::delta_cost;

pub(crate) struct UiNativeDeltaPresentation {
    cost: UiHostPresentationCostReport,
    painted: bool,
    pixels: Option<[[u8; 4]; 2]>,
    port_crossings: u8,
    effects: super::UiNativePresentationEffects,
}

impl UiNativeDeltaPresentation {
    pub(crate) fn into_parts(
        self,
    ) -> (
        UiHostPresentationCostReport,
        bool,
        Option<[[u8; 4]; 2]>,
        u8,
        super::UiNativePresentationEffects,
    ) {
        (
            self.cost,
            self.painted,
            self.pixels,
            self.port_crossings,
            self.effects,
        )
    }
}

pub(crate) fn present_delta<Port: UiNativePresentationPort>(
    graphics: &mut UiNativePresentationAccess,
    resources: &mut UiNativeResourceRegistry,
    physical_signal: &mut crate::native::physical_work_signal::UiNativePhysicalSignalOwner,
    atlas: &crate::native::text_atlas::UiNativeTextAtlas,
    atlas_gpu: Option<&crate::native::text_atlas::UiNativeTextAtlasGpuPages>,
    view: &UiMountedFrameConsumptionView<'_>,
    retained: &mut UiNativeRetainedDrawList,
    defer_initial_observation: bool,
    lifecycle: &mut crate::native::lifecycle::UiNativeLifecycleOrchestrator,
) -> Result<UiNativeDeltaPresentation, UiNativePresentationFailure> {
    let UiMountedPresentationWorkView::Delta(delta) = view.presentation_work() else {
        return Err(before_effects(
            UiHostSurfacePresentationDenial::AdapterDeclined,
        ));
    };
    let basis = UiNativeRasterBasis::from_presentation_access(graphics);
    let glyph_runs = view
        .text_raster_work()
        .map(|work| work.glyph_runs())
        .unwrap_or_default();
    let (plan, undo, effects) = prepare_delta_plan(basis, delta, glyph_runs, atlas, retained)?;
    if plan.operations.is_empty() && !plan.clear_retained_target {
        return Ok(UiNativeDeltaPresentation {
            cost: plan.cost,
            painted: false,
            pixels: None,
            port_crossings: 0,
            effects: effects.without_native_paint(),
        });
    }
    let owners = match reserve_presentation_owners(
        resources,
        physical_signal,
        crate::native::physical_work_signal::UiNativePhysicalPresentationBasis::from_view(view),
    ) {
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
        effects,
        settle_port_result(
            resources,
            physical_signal,
            owners,
            Port::present(
                graphics,
                atlas_gpu,
                plan,
                defer_initial_observation,
                lifecycle,
            ),
        ),
    )
}

fn prepare_delta_plan(
    basis: UiNativeRasterBasis,
    delta: &worth_ui_host_contract::UiMountedPresentationDelta,
    glyph_runs: &[worth_ui_host_contract::UiGlyphRunView],
    atlas: &crate::native::text_atlas::UiNativeTextAtlas,
    retained: &mut UiNativeRetainedDrawList,
) -> Result<
    (
        UiNativePresentationPortPlan,
        UiNativeRetainedDeltaUndo,
        super::UiNativePresentationEffects,
    ),
    UiNativePresentationFailure,
> {
    let (replay, undo) = retained
        .stage_delta(delta, glyph_runs)
        .map_err(|_| before_effects(UiHostSurfacePresentationDenial::MalformedProjection))?;
    let effects = super::UiNativePresentationEffects::new(
        !delta.changes().is_empty() || !delta.order().is_empty() || !delta.damage().is_empty(),
        replay.identity_overlay_effect,
    );
    match build_plan(basis, retained, replay, delta.nodes().len(), atlas).map_err(before_effects) {
        Ok(plan) => Ok((plan, undo, effects)),
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
    effects: super::UiNativePresentationEffects,
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
                effects,
            })
        }
        Err(
            failure @ (UiNativePresentationFailure::BeforeEffects(_)
            | UiNativePresentationFailure::RecoveryRequired { .. }),
        ) => {
            retained
                .rollback_delta(undo)
                .expect("before-effect port refusal must preserve retained state");
            Err(failure)
        }
        Err(UiNativePresentationFailure::Pending(pending)) => {
            Err(UiNativePresentationFailure::Pending(
                pending.with_settlement(super::UiNativePendingSurfaceSettlement::Delta(
                    super::UiNativePendingDeltaSettlement::new(undo, effects),
                )),
            ))
        }
    }
}

fn build_plan(
    basis: UiNativeRasterBasis,
    retained: &UiNativeRetainedDrawList,
    replay: super::retained_draw_list::UiNativeRetainedReplayPlan,
    node_changes: usize,
    atlas: &crate::native::text_atlas::UiNativeTextAtlas,
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
            let sample = retained.sample_override(*identity);
            let opacity = sample.map_or(1.0, |sample| sample.opacity().factor());
            match command {
                UiMountedPaintCommand::FilledRect { mechanic, .. } => {
                    let sampled = super::sample::sampled_command_bounds(command, sample)?;
                    let Some(bounds) = clipped_sampled_damage(sampled, region.damage.bounds())?
                    else {
                        continue;
                    };
                    let Some(rect) =
                        raster_damage_for_basis(bounds, basis).map_err(|_| malformed())?
                    else {
                        continue;
                    };
                    rendered_pixels = add_pixels(rendered_pixels, rect)?;
                    operations.push(UiNativeRasterOperation::FilledRect {
                        rect,
                        source_rgba8: sampled_color(mechanic.color().channels(), opacity),
                    });
                }
                UiMountedPaintCommand::PortalOverlay { mechanic, .. } => {
                    let sampled = super::sample::sampled_command_bounds(command, sample)?;
                    let Some(bounds) = clipped_sampled_damage(sampled, region.damage.bounds())?
                    else {
                        continue;
                    };
                    let Some(rect) =
                        raster_damage_for_basis(bounds, basis).map_err(|_| malformed())?
                    else {
                        continue;
                    };
                    rendered_pixels = add_pixels(rendered_pixels, rect)?;
                    operations.push(UiNativeRasterOperation::FilledRect {
                        rect,
                        source_rgba8: sampled_color(mechanic.color().channels(), opacity),
                    });
                }
                UiMountedPaintCommand::SemanticText { .. } => {
                    let glyphs = super::text::plan_glyph_commands(
                        retained.glyph_runs(*identity),
                        atlas,
                        basis.extent(),
                    )
                    .map_err(|_| malformed())?;
                    for mut glyph in glyphs.iter().copied() {
                        if let Some(transform) = sample.and_then(|sample| sample.transform()) {
                            glyph.target = super::sample::transform_physical_box(
                                glyph.target,
                                transform,
                                basis,
                            )?;
                        }
                        glyph.opacity = opacity;
                        let Some(glyph) =
                            super::text::clip_glyph_command(glyph, clear.physical_bounds())
                        else {
                            continue;
                        };
                        rendered_pixels = rendered_pixels
                            .checked_add(
                                (glyph.target[2].ceil() as u64) * (glyph.target[3].ceil() as u64),
                            )
                            .ok_or_else(malformed)?;
                        operations.push(UiNativeRasterOperation::Glyph(glyph));
                    }
                }
            }
            replayed_commands = replayed_commands.checked_add(1).ok_or_else(malformed)?;
        }
    }
    operations.extend(retained.identity_overlay_operations(basis)?);
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

fn clipped_sampled_damage(
    bounds: UiMountedCanonicalBox,
    damage: UiMountedCanonicalBox,
) -> Result<Option<UiMountedCanonicalBox>, UiHostSurfacePresentationDenial> {
    if bounds.coordinate_space() != damage.coordinate_space() {
        return Err(malformed());
    }
    let left = bounds.x().max(damage.x());
    let top = bounds.y().max(damage.y());
    let right = edge(bounds, true).min(edge(damage, true));
    let bottom = edge(bounds, false).min(edge(damage, false));
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

fn sampled_color(mut color: [u8; 4], opacity: f32) -> [u8; 4] {
    color[3] = (f32::from(color[3]) * opacity).round() as u8;
    color
}

fn edge(bounds: UiMountedCanonicalBox, horizontal: bool) -> f32 {
    if horizontal {
        bounds.x() + bounds.width()
    } else {
        bounds.y() + bounds.height()
    }
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
