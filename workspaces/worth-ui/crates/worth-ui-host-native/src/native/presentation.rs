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

use pipeline::{
    clear_target, draw_rectangle, draw_rectangle_after_clear, draw_retained_to_surface, pipeline,
    replace_pipeline, retained_transfer,
};
use raster::{raster_rect, rectangle_shader, RasterRect};
#[cfg(test)]
pub(crate) use readback_port::prove_nonuniform_readback_port;
use readback_port::{UiNativeReadbackPort, UiWgpuNativeReadbackPort};
use retained_evidence_copy::copy_evidence_pixels;

pub(crate) use delta::{present_delta, UiNativeDeltaPresentation};
pub(crate) use port::{
    UiNativePresentationPort, UiNativePresentationPortFailure, UiNativePresentationPortPlan,
    UiNativeRasterOperation, UiWgpuNativePresentationPort,
};
pub(crate) use reconstruction::present_reconstruction;
pub(crate) use retained_draw_list::UiNativeRetainedDrawList;

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
    frame: u64,
    mechanics: Box<[worth_ui_host_contract::UiMountedFilledRectMechanic]>,
}

pub(super) struct UiNativePresentationOwners {
    readback: super::UiNativeResourceOwner,
    submission: super::UiNativeResourceOwner,
}

pub(crate) struct UiNativePendingPresentation {
    external: Box<dyn UiNativePendingExternalObligation>,
    readback_owner: super::UiNativeResourceOwner,
    submission_owner: super::UiNativeResourceOwner,
}

pub(super) trait UiNativePendingExternalObligation {
    fn try_settle(&mut self, device: Option<&wgpu::Device>) -> bool;
}

pub(super) struct UiNativePendingWgpuObligation {
    _readback: wgpu::Buffer,
    _submission: wgpu::SubmissionIndex,
}

impl UiNativePendingWgpuObligation {
    pub(super) fn new(readback: wgpu::Buffer, submission: wgpu::SubmissionIndex) -> Self {
        Self {
            _readback: readback,
            _submission: submission,
        }
    }
}

impl UiNativePendingExternalObligation for UiNativePendingWgpuObligation {
    fn try_settle(&mut self, device: Option<&wgpu::Device>) -> bool {
        let Some(device) = device else {
            return false;
        };
        let settled = device
            .poll(wgpu::PollType::Wait {
                submission_index: Some(self._submission.clone()),
                timeout: Some(GPU_WAIT_DEADLINE),
            })
            .is_ok();
        if settled {
            self._readback.unmap();
        }
        settled
    }
}

impl UiNativePendingPresentation {
    fn external(
        external: Box<dyn UiNativePendingExternalObligation>,
        readback_owner: super::UiNativeResourceOwner,
        submission_owner: super::UiNativeResourceOwner,
    ) -> Self {
        Self {
            external,
            readback_owner,
            submission_owner,
        }
    }

    pub(crate) fn try_settle(&mut self, device: Option<&wgpu::Device>) -> bool {
        self.external.try_settle(device)
    }

    pub(crate) fn release(self, resources: &mut UiNativeResourceRegistry) {
        let Self {
            external,
            readback_owner,
            submission_owner,
        } = self;
        drop(external);
        resources
            .release(readback_owner)
            .expect("settled readback owner must remain exact");
        resources
            .release(submission_owner)
            .expect("settled submission owner must remain exact");
    }
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

pub(super) fn reserve_presentation_owners(
    resources: &mut UiNativeResourceRegistry,
) -> Result<UiNativePresentationOwners, UiNativePresentationFailure> {
    let mut owners = resources
        .reserve(&[
            UiNativeResourceClass::ReadbackBuffer,
            UiNativeResourceClass::PendingSubmission,
        ])
        .map_err(|_| {
            UiNativePresentationFailure::BeforeEffects(
                UiHostSurfacePresentationDenial::CapacityExceeded,
            )
        })?;
    Ok(UiNativePresentationOwners {
        readback: owners.remove(0),
        submission: owners.remove(0),
    })
}

pub(super) fn settle_port_result(
    resources: &mut UiNativeResourceRegistry,
    owners: UiNativePresentationOwners,
    result: Result<port::UiNativePresentationPortObservation, UiNativePresentationPortFailure>,
) -> Result<port::UiNativePresentationPortObservation, UiNativePresentationFailure> {
    match result {
        Ok(observation) => {
            resources
                .release(owners.readback)
                .expect("readback owner must remain exact");
            resources
                .release(owners.submission)
                .expect("submission owner must remain exact");
            Ok(observation)
        }
        Err(UiNativePresentationPortFailure::SurfaceUnavailable) => {
            resources
                .release(owners.readback)
                .expect("readback reservation must remain exact");
            resources
                .release(owners.submission)
                .expect("submission reservation must remain exact");
            Err(UiNativePresentationFailure::BeforeEffects(
                UiHostSurfacePresentationDenial::AdapterDeclined,
            ))
        }
        Err(UiNativePresentationPortFailure::ReadbackUnsettled(external)) => {
            Err(UiNativePresentationFailure::Indeterminate(
                UiNativePendingPresentation::external(external, owners.readback, owners.submission),
            ))
        }
    }
}

fn build_presented_frame(
    view: &UiMountedFrameConsumptionView<'_>,
    graphics: &UiNativeGraphics,
    initial: ValidatedInitial,
    external: port::UiNativePresentationPortObservation,
    retained: UiNativeRetainedDrawList,
) -> UiNativePresentedFrame {
    let ValidatedInitial { frame, mechanics } = initial;
    let mechanic = mechanics[0];
    let (pixels, cost, port_crossings) = external.into_parts();
    let [retained_baseline_rgba8, retained_center_rgba8] = pixels;
    let bounds = mechanic.bounds();
    let observation = UiNativePresentationObservation::new(UiNativePresentationInput {
        client_physical_size: graphics.extent(),
        scale_factor_milli: (graphics.scale_factor * 1_000.0).round() as u32,
        source_rgba8: mechanic.color().channels(),
        retained_center_rgba8,
        retained_baseline_rgba8,
        presented_frame: frame,
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
        order_ordinal: 0,
        port_crossings,
        cost,
    });
    UiNativePresentedFrame {
        observation,
        cost,
        retained,
    }
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
        gpu_writes: rows + 1,
        render_passes: rows + 1,
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
            UiMountedPaintCommand::FilledRect {
                identity,
                mechanic,
            } if *identity
                == worth_ui_host_contract::UiMountedPaintCommandIdentity::filled_rect(mechanic)
                && initial.projection().filled_rects().rows().contains(mechanic) =>
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
        frame: initial.affinity().successor().diagnostic_value(),
        mechanics: mechanics.into_boxed_slice(),
    })
}

#[cfg(test)]
#[path = "presentation_tests.rs"]
mod tests;
