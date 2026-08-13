use worth_ui_host_contract::UiHostSurfacePresentationDenial;

use super::{
    port, UiNativePresentationFailure, UiNativePresentationPortFailure, UiNativeResourceClass,
    UiNativeResourceRegistry, GPU_WAIT_DEADLINE,
};

pub(crate) struct UiNativePresentationOwners {
    readback: crate::native::UiNativeResourceOwner,
    submission: crate::native::UiNativeResourceOwner,
}

pub(crate) struct UiNativePendingPresentation {
    external: Box<dyn UiNativePendingExternalObligation>,
    readback_owner: crate::native::UiNativeResourceOwner,
    submission_owner: crate::native::UiNativeResourceOwner,
}

pub(crate) trait UiNativePendingExternalObligation {
    fn try_settle(&mut self, device: Option<&wgpu::Device>) -> bool;
}

pub(crate) struct UiNativePendingWgpuObligation {
    readback: wgpu::Buffer,
    submission: wgpu::SubmissionIndex,
}

impl UiNativePendingWgpuObligation {
    pub(crate) fn new(readback: wgpu::Buffer, submission: wgpu::SubmissionIndex) -> Self {
        Self {
            readback,
            submission,
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
                submission_index: Some(self.submission.clone()),
                timeout: Some(GPU_WAIT_DEADLINE),
            })
            .is_ok();
        if settled {
            self.readback.unmap();
        }
        settled
    }
}

impl UiNativePendingPresentation {
    fn external(
        external: Box<dyn UiNativePendingExternalObligation>,
        readback_owner: crate::native::UiNativeResourceOwner,
        submission_owner: crate::native::UiNativeResourceOwner,
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

pub(crate) fn reserve_presentation_owners(
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

pub(crate) fn settle_port_result(
    resources: &mut UiNativeResourceRegistry,
    owners: UiNativePresentationOwners,
    result: Result<port::UiNativePresentationPortObservation, UiNativePresentationPortFailure>,
) -> Result<port::UiNativePresentationPortObservation, UiNativePresentationFailure> {
    match result {
        Ok(observation) => {
            release_reserved(resources, owners);
            Ok(observation)
        }
        Err(UiNativePresentationPortFailure::SurfaceUnavailable) => {
            release_reserved(resources, owners);
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

fn release_reserved(resources: &mut UiNativeResourceRegistry, owners: UiNativePresentationOwners) {
    resources
        .release(owners.readback)
        .expect("readback reservation must remain exact");
    resources
        .release(owners.submission)
        .expect("submission reservation must remain exact");
}
