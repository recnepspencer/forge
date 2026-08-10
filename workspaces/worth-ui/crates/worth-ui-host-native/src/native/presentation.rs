use worth_ui_host_contract::{
    UiHostPresentationCostReport, UiHostSurfacePresentationDenial, UiMountedFrameConsumptionView,
    UiMountedPaintCommand, UiMountedPresentationWorkView,
};

use super::{
    UiNativeGraphics, UiNativePresentationInput, UiNativePresentationObservation,
    UiNativeResourceClass, UiNativeResourceRegistry,
};

mod pipeline;
mod port;
mod raster;
mod readback_port;
mod retained_evidence_copy;

use pipeline::{draw_rectangle, draw_retained_to_surface, pipeline, retained_transfer};
use raster::{raster_rect, rectangle_shader, RasterRect};
#[cfg(test)]
pub(crate) use readback_port::prove_nonuniform_readback_port;
use readback_port::{UiNativeReadbackPort, UiWgpuNativeReadbackPort};
use retained_evidence_copy::copy_evidence_pixels;

pub(crate) use port::{
    UiNativePresentationPort, UiNativePresentationPortFailure, UiNativePresentationPortPlan,
    UiWgpuNativePresentationPort,
};

pub(crate) const GPU_WAIT_DEADLINE: std::time::Duration = std::time::Duration::from_millis(5_000);

pub(crate) enum UiNativePresentationFailure {
    BeforeEffects(UiHostSurfacePresentationDenial),
    Indeterminate(UiNativePendingPresentation),
}

pub(crate) struct UiNativePresentedFrame {
    observation: UiNativePresentationObservation,
    cost: UiHostPresentationCostReport,
}

struct UiNativePresentationOwners {
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

    #[cfg(test)]
    pub(super) fn scripted(
        resources: &mut UiNativeResourceRegistry,
        dropped: std::rc::Rc<std::cell::Cell<bool>>,
    ) -> Self {
        Self::scripted_controllable(
            resources,
            dropped,
            std::rc::Rc::new(std::cell::Cell::new(false)),
        )
    }

    #[cfg(test)]
    pub(crate) fn scripted_controllable(
        resources: &mut UiNativeResourceRegistry,
        dropped: std::rc::Rc<std::cell::Cell<bool>>,
        settles: std::rc::Rc<std::cell::Cell<bool>>,
    ) -> Self {
        let mut owners = resources
            .reserve(&[
                crate::native::UiNativeResourceClass::ReadbackBuffer,
                crate::native::UiNativeResourceClass::PendingSubmission,
            ])
            .expect("scripted indeterminate presentation reserves exact owners");
        let submission_owner = owners.pop().expect("submission owner");
        let readback_owner = owners.pop().expect("readback owner");
        Self {
            external: Box::new(UiNativePendingDropProbe { dropped, settles }),
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
    ) {
        (self.observation, self.cost)
    }
}

pub(crate) fn present_initial<Port: UiNativePresentationPort>(
    graphics: &mut UiNativeGraphics,
    resources: &mut UiNativeResourceRegistry,
    view: &UiMountedFrameConsumptionView<'_>,
) -> Result<UiNativePresentedFrame, UiNativePresentationFailure> {
    let (frame, mechanic) =
        validate_initial(view).map_err(UiNativePresentationFailure::BeforeEffects)?;
    let rect = raster_rect(mechanic, graphics).map_err(|_| {
        UiNativePresentationFailure::BeforeEffects(
            UiHostSurfacePresentationDenial::MalformedProjection,
        )
    })?;
    let owners = reserve_presentation_owners(resources)?;
    let result = Port::present(
        graphics,
        UiNativePresentationPortPlan {
            rect,
            source_rgba8: mechanic.color().channels(),
        },
    );
    let external = settle_port_result(resources, owners, result)?;
    Ok(build_presented_frame(
        view, graphics, frame, mechanic, external,
    ))
}

fn reserve_presentation_owners(
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

fn settle_port_result(
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
    frame: u64,
    mechanic: worth_ui_host_contract::UiMountedFilledRectMechanic,
    external: port::UiNativePresentationPortObservation,
) -> UiNativePresentedFrame {
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
    UiNativePresentedFrame { observation, cost }
}

#[cfg(test)]
struct UiNativePendingDropProbe {
    dropped: std::rc::Rc<std::cell::Cell<bool>>,
    settles: std::rc::Rc<std::cell::Cell<bool>>,
}

#[cfg(test)]
impl UiNativePendingExternalObligation for UiNativePendingDropProbe {
    fn try_settle(&mut self, _device: Option<&wgpu::Device>) -> bool {
        self.settles.get()
    }
}

#[cfg(test)]
impl Drop for UiNativePendingDropProbe {
    fn drop(&mut self) {
        self.dropped.set(true);
    }
}

fn milli(value: f32) -> i64 {
    (f64::from(value) * 1_000.0).round() as i64
}

fn validate_initial(
    view: &UiMountedFrameConsumptionView<'_>,
) -> Result<
    (u64, worth_ui_host_contract::UiMountedFilledRectMechanic),
    UiHostSurfacePresentationDenial,
> {
    let UiMountedPresentationWorkView::Initial(initial) = view.presentation_work() else {
        return Err(UiHostSurfacePresentationDenial::AdapterDeclined);
    };
    if initial.commands().len() != 1
        || initial.order().len() != 1
        || initial.projection().filled_rects().rows().len() != 1
        || !initial.projection().semantic_text().rows().is_empty()
    {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    let UiMountedPaintCommand::FilledRect {
        table_index,
        mechanic,
        ..
    } = &initial.commands()[0]
    else {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    };
    if *table_index != 0
        || initial.projection().filled_rects().rows()[0] != *mechanic
        || initial.order()[0].command() != initial.commands()[0].identity()
    {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    Ok((initial.affinity().successor().diagnostic_value(), *mechanic))
}

#[cfg(test)]
mod tests {
    use super::{
        reserve_presentation_owners, settle_port_result, UiNativePendingDropProbe,
        UiNativePresentationFailure, UiNativePresentationPortFailure,
    };

    #[test]
    fn external_port_failures_cross_the_real_framework_settlement_transition() {
        let mut resources = crate::native::UiNativeResourceRegistry::new();
        let owners = reserve_presentation_owners(&mut resources)
            .unwrap_or_else(|_| panic!("empty registry must reserve presentation owners"));
        let denied = settle_port_result(
            &mut resources,
            owners,
            Err(UiNativePresentationPortFailure::SurfaceUnavailable),
        );
        assert!(matches!(
            denied,
            Err(UiNativePresentationFailure::BeforeEffects(
                worth_ui_host_contract::UiHostSurfacePresentationDenial::AdapterDeclined
            ))
        ));
        assert!(resources.current().is_zero());

        let dropped = std::rc::Rc::new(std::cell::Cell::new(false));
        let owners = reserve_presentation_owners(&mut resources)
            .unwrap_or_else(|_| panic!("released registry must reserve presentation owners"));
        let pending = Box::new(UiNativePendingDropProbe {
            dropped: std::rc::Rc::clone(&dropped),
            settles: std::rc::Rc::new(std::cell::Cell::new(false)),
        });
        let unsettled = settle_port_result(
            &mut resources,
            owners,
            Err(UiNativePresentationPortFailure::ReadbackUnsettled(pending)),
        );
        let Err(UiNativePresentationFailure::Indeterminate(pending)) = unsettled else {
            panic!("readback failure must remain indeterminate");
        };
        assert_eq!(resources.current().readback_buffers, 1);
        assert_eq!(resources.current().pending_submissions, 1);
        assert!(!dropped.get());
        pending.release(&mut resources);
        assert!(dropped.get());
        assert!(resources.current().is_zero());
    }
}
