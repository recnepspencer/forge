use worth_ui_host_contract::{
    UiHostSurfacePresentationDenial, UiMountedCanonicalBox, UiMountedCanonicalBoxInput,
    UiMountedFrameConsumptionView, UiMountedPaintCommand, UiMountedPresentationSampleChange,
    UiMountedPresentationTransform, UiMountedPresentationWorkView,
};

use super::port::UiNativePresentationPortObservation;
use super::raster::{raster_damage_for_basis, UiNativeRasterBasis};
use super::retained_draw_list::{UiNativeRetainedReplayPlan, UiNativeRetainedSampleUndo};
use super::{
    reserve_presentation_owners, settle_port_result, UiNativePresentationFailure,
    UiNativePresentationPort, UiNativePresentationPortPlan, UiNativeRasterOperation,
    UiNativeRetainedDrawList,
};
use crate::native::{UiNativePresentationAccess, UiNativeResourceRegistry};

pub(crate) struct UiNativeSamplePresentation {
    cost: worth_ui_host_contract::UiHostPresentationCostReport,
    painted: bool,
    pixels: Option<[[u8; 4]; 2]>,
    port_crossings: u8,
    effects: super::UiNativePresentationEffects,
}

impl UiNativeSamplePresentation {
    pub(crate) const fn into_parts(
        self,
    ) -> (
        worth_ui_host_contract::UiHostPresentationCostReport,
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

#[allow(clippy::too_many_arguments)]
pub(crate) fn present_sample<Port: UiNativePresentationPort>(
    graphics: &mut UiNativePresentationAccess,
    resources: &mut UiNativeResourceRegistry,
    physical_signal: &mut crate::native::physical_work_signal::UiNativePhysicalSignalOwner,
    atlas: &crate::native::text_atlas::UiNativeTextAtlas,
    atlas_gpu: Option<&crate::native::text_atlas::UiNativeTextAtlasGpuPages>,
    view: &UiMountedFrameConsumptionView<'_>,
    retained: &mut UiNativeRetainedDrawList,
    defer_initial_observation: bool,
    lifecycle: &mut crate::native::lifecycle::UiNativeLifecycleOrchestrator,
) -> Result<UiNativeSamplePresentation, UiNativePresentationFailure> {
    let UiMountedPresentationWorkView::Sample(sample) = view.presentation_work() else {
        return Err(before_effects(
            UiHostSurfacePresentationDenial::AdapterDeclined,
        ));
    };
    let basis = UiNativeRasterBasis::from_presentation_access(graphics);
    let (replay, undo) = retained
        .stage_sample(sample)
        .map_err(|_| before_effects(malformed()))?;
    let plan = match build_plan(basis, retained, replay, atlas) {
        Ok(plan) => plan,
        Err(denial) => {
            retained
                .rollback_sample(undo)
                .expect("a prevalidated native sample rolls back exactly");
            return Err(before_effects(denial));
        }
    };
    if plan.operations.is_empty() && !plan.clear_retained_target {
        return Ok(UiNativeSamplePresentation {
            cost: plan.cost,
            painted: false,
            pixels: None,
            port_crossings: 0,
            effects: super::UiNativePresentationEffects::new(true, false).without_native_paint(),
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
                .rollback_sample(undo)
                .expect("capacity refusal preserves the previous sample");
            return Err(failure);
        }
    };
    settle_staged_sample(
        retained,
        undo,
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

fn settle_staged_sample(
    retained: &mut UiNativeRetainedDrawList,
    undo: UiNativeRetainedSampleUndo,
    result: Result<UiNativePresentationPortObservation, UiNativePresentationFailure>,
) -> Result<UiNativeSamplePresentation, UiNativePresentationFailure> {
    match result {
        Ok(observation) => {
            let (pixels, cost, port_crossings) = observation.into_parts();
            Ok(UiNativeSamplePresentation {
                cost,
                painted: true,
                pixels: Some(pixels),
                port_crossings,
                effects: super::UiNativePresentationEffects::new(true, false),
            })
        }
        Err(
            failure @ (UiNativePresentationFailure::BeforeEffects(_)
            | UiNativePresentationFailure::RecoveryRequired { .. }),
        ) => {
            retained
                .rollback_sample(undo)
                .expect("before-effect refusal preserves the previous sample");
            Err(failure)
        }
        Err(UiNativePresentationFailure::Pending(pending)) => {
            Err(UiNativePresentationFailure::Pending(
                pending.with_settlement(super::UiNativePendingSurfaceSettlement::Sample(undo)),
            ))
        }
    }
}

pub(super) fn build_plan(
    basis: UiNativeRasterBasis,
    retained: &UiNativeRetainedDrawList,
    replay: UiNativeRetainedReplayPlan,
    atlas: &crate::native::text_atlas::UiNativeTextAtlas,
) -> Result<UiNativePresentationPortPlan, UiHostSurfacePresentationDenial> {
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
            let change = retained.sample_override(*identity);
            let opacity = change.map_or(1.0, |change| change.opacity().factor());
            match command {
                UiMountedPaintCommand::FilledRect { mechanic, .. } => {
                    let Some((rect, color)) = solid_operation(
                        basis,
                        command,
                        change,
                        region.damage.bounds(),
                        mechanic.color().channels(),
                        opacity,
                    )?
                    else {
                        continue;
                    };
                    rendered_pixels = add_pixels(rendered_pixels, rect)?;
                    operations.push(UiNativeRasterOperation::FilledRect {
                        rect,
                        source_rgba8: color,
                    });
                }
                UiMountedPaintCommand::PortalOverlay { mechanic, .. } => {
                    let Some((rect, color)) = solid_operation(
                        basis,
                        command,
                        change,
                        region.damage.bounds(),
                        mechanic.color().channels(),
                        opacity,
                    )?
                    else {
                        continue;
                    };
                    rendered_pixels = add_pixels(rendered_pixels, rect)?;
                    operations.push(UiNativeRasterOperation::FilledRect {
                        rect,
                        source_rgba8: color,
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
                        if let Some(transform) = change.and_then(|change| change.transform()) {
                            glyph.target = transform_physical_box(glyph.target, transform, basis)?;
                        }
                        glyph.opacity = opacity;
                        let Some(glyph) =
                            super::text::clip_glyph_command(glyph, clear.physical_bounds())
                        else {
                            continue;
                        };
                        rendered_pixels = rendered_pixels
                            .checked_add(
                                (glyph.target[2].ceil() as u64)
                                    .saturating_mul(glyph.target[3].ceil() as u64),
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
    let cost = super::delta::delta_cost(
        basis.extent(),
        replay.counters,
        operations.len(),
        cleared_pixels,
        rendered_pixels,
        replayed_commands,
        0,
    )?;
    Ok(UiNativePresentationPortPlan {
        clear_retained_target: false,
        operations: operations.into_boxed_slice(),
        cost,
    })
}

pub(super) fn sampled_command_bounds(
    command: &UiMountedPaintCommand,
    change: Option<UiMountedPresentationSampleChange>,
) -> Result<UiMountedCanonicalBox, UiHostSurfacePresentationDenial> {
    super::retained_draw_list::sampled_visible_bounds(command, change)
        .map_err(|_| malformed())?
        .ok_or_else(malformed)
}

fn solid_operation(
    basis: UiNativeRasterBasis,
    command: &UiMountedPaintCommand,
    change: Option<UiMountedPresentationSampleChange>,
    damage: UiMountedCanonicalBox,
    mut color: [u8; 4],
    opacity: f32,
) -> Result<Option<(super::RasterRect, [u8; 4])>, UiHostSurfacePresentationDenial> {
    let bounds = sampled_command_bounds(command, change)?;
    let Some(bounds) = intersect_box(bounds, damage)? else {
        return Ok(None);
    };
    let Some(rect) = raster_damage_for_basis(bounds, basis).map_err(|_| malformed())? else {
        return Ok(None);
    };
    color[3] = (f32::from(color[3]) * opacity).round() as u8;
    Ok(Some((rect, color)))
}

pub(super) fn transform_physical_box(
    bounds: [f32; 4],
    transform: UiMountedPresentationTransform,
    basis: UiNativeRasterBasis,
) -> Result<[f32; 4], UiHostSurfacePresentationDenial> {
    let scale = basis.scale_factor();
    let source = transform.source();
    let sampled = transform.sampled();
    let source = [
        source.x() * scale,
        source.y() * scale,
        source.width() * scale,
        source.height() * scale,
    ];
    let sampled = [
        sampled.x() * scale,
        sampled.y() * scale,
        sampled.width() * scale,
        sampled.height() * scale,
    ];
    if source[2] <= 0.0 || source[3] <= 0.0 {
        return Err(malformed());
    }
    Ok([
        sampled[0] + (bounds[0] - source[0]) * sampled[2] / source[2],
        sampled[1] + (bounds[1] - source[1]) * sampled[3] / source[3],
        bounds[2] * sampled[2] / source[2],
        bounds[3] * sampled[3] / source[3],
    ])
}

fn intersect_box(
    left: UiMountedCanonicalBox,
    right: UiMountedCanonicalBox,
) -> Result<Option<UiMountedCanonicalBox>, UiHostSurfacePresentationDenial> {
    if left.coordinate_space() != right.coordinate_space() {
        return Err(malformed());
    }
    let x = left.x().max(right.x());
    let y = left.y().max(right.y());
    let far_x = (left.x() + left.width()).min(right.x() + right.width());
    let far_y = (left.y() + left.height()).min(right.y() + right.height());
    if far_x <= x || far_y <= y {
        return Ok(None);
    }
    UiMountedCanonicalBox::canonicalize(UiMountedCanonicalBoxInput {
        x,
        y,
        width: far_x - x,
        height: far_y - y,
        coordinate_space: left.coordinate_space(),
    })
    .map(Some)
    .map_err(|_| malformed())
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
#[path = "sample_tests.rs"]
mod tests;
