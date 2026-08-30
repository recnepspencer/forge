use worth_ui_host_contract::{
    UiHostPresentationCostInput, UiHostPresentationCostReport, UiHostSurfacePresentationDenial,
    UiMountedFrameConsumptionView, UiMountedPaintCommand, UiMountedPresentationWorkView,
};

pub(crate) struct UiNativeColdReconstruction {
    cost: UiHostPresentationCostReport,
    retained: UiNativeRetainedDrawList,
    pixels: [[u8; 4]; 2],
    port_crossings: u8,
    recovery: crate::native::UiNativeRecoveryRequirement,
}

pub(crate) enum UiNativeReconstructionFailure {
    BeforeEffects {
        denial: UiHostSurfacePresentationDenial,
        recovery: crate::native::UiNativeRecoveryRequirement,
        successor_cause: Option<crate::native::UiNativeRecoveryCause>,
    },
    Pending(super::UiNativePendingPresentation),
}

impl UiNativeColdReconstruction {
    pub(crate) fn into_parts(
        self,
    ) -> (
        UiHostPresentationCostReport,
        UiNativeRetainedDrawList,
        [[u8; 4]; 2],
        u8,
        crate::native::UiNativeRecoveryRequirement,
    ) {
        let Self {
            cost,
            retained,
            pixels,
            port_crossings,
            recovery,
        } = self;
        (cost, retained, pixels, port_crossings, recovery)
    }
}

pub(crate) fn present_cold_reconstruction<Port: UiNativePresentationPort>(
    graphics: &mut UiNativePresentationAccess,
    resources: &mut UiNativeResourceRegistry,
    physical_signal: &mut crate::native::physical_work_signal::UiNativePhysicalSignalOwner,
    atlas: &crate::native::text_atlas::UiNativeTextAtlas,
    atlas_gpu: Option<&crate::native::text_atlas::UiNativeTextAtlasGpuPages>,
    view: &UiMountedFrameConsumptionView<'_>,
    recovery: crate::native::UiNativeRecoveryRequirement,
    defer_initial_observation: bool,
    lifecycle: &mut crate::native::lifecycle::UiNativeLifecycleOrchestrator,
) -> Result<UiNativeColdReconstruction, UiNativeReconstructionFailure> {
    let UiMountedPresentationWorkView::Reconstruction(work) = view.presentation_work() else {
        return Err(before_effects(malformed_denial(), recovery, None));
    };
    let glyph_runs = view
        .text_raster_work()
        .map(|work| work.glyph_runs())
        .unwrap_or_default();
    let retained = match UiNativeRetainedDrawList::reconstruction(work, glyph_runs) {
        Ok(retained) => retained,
        Err(_) => return Err(before_effects(malformed_denial(), recovery, None)),
    };
    let plan = match build_plan(graphics, atlas, &retained) {
        Ok(plan) => plan,
        Err(failure) => {
            let (denial, successor_cause) = presentation_before_effects(failure);
            return Err(before_effects(denial, recovery, successor_cause));
        }
    };
    let owners = match reserve_presentation_owners(
        resources,
        physical_signal,
        crate::native::physical_work_signal::UiNativePhysicalPresentationBasis::from_view(view),
    ) {
        Ok(owners) => owners,
        Err(failure) => {
            let (denial, successor_cause) = presentation_before_effects(failure);
            return Err(before_effects(denial, recovery, successor_cause));
        }
    };
    let observation = match settle_port_result(
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
    ) {
        Ok(observation) => observation,
        Err(UiNativePresentationFailure::Pending(pending)) => {
            return Err(UiNativeReconstructionFailure::Pending(
                pending.with_settlement(super::UiNativePendingSurfaceSettlement::Reconstruction {
                    retained: Box::new(retained),
                    recovery,
                }),
            ));
        }
        Err(failure) => {
            let (denial, successor_cause) = presentation_before_effects(failure);
            return Err(before_effects(denial, recovery, successor_cause));
        }
    };
    let (pixels, cost, port_crossings) = observation.into_parts();
    Ok(UiNativeColdReconstruction {
        cost,
        retained,
        pixels,
        port_crossings,
        recovery,
    })
}

use super::{
    raster::raster_rect, reserve_presentation_owners, settle_port_result,
    UiNativePresentationAccess, UiNativePresentationFailure, UiNativePresentationPort,
    UiNativePresentationPortPlan, UiNativeRasterOperation, UiNativeResourceRegistry,
    UiNativeRetainedDrawList,
};

fn build_plan(
    graphics: &UiNativePresentationAccess,
    atlas: &crate::native::text_atlas::UiNativeTextAtlas,
    retained: &UiNativeRetainedDrawList,
) -> Result<UiNativePresentationPortPlan, UiNativePresentationFailure> {
    let commands = retained
        .reconstruction_commands()
        .map_err(|_| malformed())?;
    let mut operations = Vec::with_capacity(commands.len());
    let mut rendered_pixels = 0_u64;
    for command in commands {
        match command {
            UiMountedPaintCommand::FilledRect { mechanic, .. } => {
                let rect = raster_rect(*mechanic, graphics).map_err(|_| malformed())?;
                rendered_pixels = rendered_pixels
                    .checked_add(u64::from(rect.physical_width) * u64::from(rect.physical_height))
                    .ok_or_else(malformed)?;
                operations.push(UiNativeRasterOperation::FilledRect {
                    rect,
                    source_rgba8: mechanic.color().channels(),
                });
            }
            UiMountedPaintCommand::PortalOverlay { mechanic, .. } => {
                let rect = super::raster::raster_portal_overlay(*mechanic, graphics)
                    .map_err(|_| malformed())?;
                rendered_pixels = rendered_pixels
                    .checked_add(u64::from(rect.physical_width) * u64::from(rect.physical_height))
                    .ok_or_else(malformed)?;
                operations.push(UiNativeRasterOperation::FilledRect {
                    rect,
                    source_rgba8: mechanic.color().channels(),
                });
            }
            UiMountedPaintCommand::SemanticText { identity, .. } => {
                let glyphs = super::text::plan_glyph_commands(
                    retained.glyph_runs(*identity),
                    atlas,
                    graphics.extent(),
                )
                .map_err(|_| malformed())?;
                for glyph in glyphs {
                    rendered_pixels = rendered_pixels
                        .checked_add(
                            (glyph.target[2].ceil() as u64) * (glyph.target[3].ceil() as u64),
                        )
                        .ok_or_else(malformed)?;
                    operations.push(UiNativeRasterOperation::Glyph(glyph));
                }
            }
        }
    }
    operations.extend(
        retained
            .identity_overlay_operations(
                super::raster::UiNativeRasterBasis::from_presentation_access(graphics),
            )
            .map_err(UiNativePresentationFailure::BeforeEffects)?,
    );
    let rows = u64::try_from(operations.len()).map_err(|_| malformed())?;
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
    UiNativePresentationFailure::BeforeEffects(malformed_denial())
}

const fn malformed_denial() -> UiHostSurfacePresentationDenial {
    UiHostSurfacePresentationDenial::MalformedProjection
}

fn presentation_before_effects(
    failure: UiNativePresentationFailure,
) -> (
    UiHostSurfacePresentationDenial,
    Option<crate::native::UiNativeRecoveryCause>,
) {
    match failure {
        UiNativePresentationFailure::BeforeEffects(denial) => (denial, None),
        UiNativePresentationFailure::RecoveryRequired { denial, cause } => (denial, Some(cause)),
        UiNativePresentationFailure::Pending(_) => {
            unreachable!("pending reconstruction is handled with its recovery authority")
        }
    }
}

const fn before_effects(
    denial: UiHostSurfacePresentationDenial,
    recovery: crate::native::UiNativeRecoveryRequirement,
    successor_cause: Option<crate::native::UiNativeRecoveryCause>,
) -> UiNativeReconstructionFailure {
    UiNativeReconstructionFailure::BeforeEffects {
        denial,
        recovery,
        successor_cause,
    }
}
