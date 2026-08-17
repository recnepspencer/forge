use worth_ui_host_contract::UiHostSurfacePresentationDenial;

use super::{
    port, UiNativePresentationFailure, UiNativePresentationPortFailure, UiNativeResourceClass,
    UiNativeResourceRegistry, GPU_WAIT_DEADLINE,
};

pub(crate) struct UiNativePresentationOwners {
    readback: crate::native::UiNativeResourceOwner,
    submission: crate::native::UiNativeResourceOwner,
    physical_work: crate::native::physical_work_signal::UiNativePhysicalPresentationIdentity,
    physical_token: crate::native::physical_work_signal::UiNativePhysicalSignalRequestToken,
}

pub(crate) struct UiNativePendingPresentation {
    external: Box<dyn UiNativePendingExternalObligation>,
    readback_owner: crate::native::UiNativeResourceOwner,
    submission_owner: crate::native::UiNativeResourceOwner,
    physical_work: crate::native::physical_work_signal::UiNativePhysicalPresentationIdentity,
    physical_token: crate::native::physical_work_signal::UiNativePhysicalSignalRequestToken,
}

pub(crate) trait UiNativePendingExternalObligation {
    fn poll_observation(
        &mut self,
        basis: crate::native::physical_work_signal::UiNativePhysicalSignalExternalBasis,
        device: Option<&wgpu::Device>,
    ) -> crate::native::physical_work_signal::UiNativePhysicalSignalExternalObservation;
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
    fn poll_observation(
        &mut self,
        basis: crate::native::physical_work_signal::UiNativePhysicalSignalExternalBasis,
        device: Option<&wgpu::Device>,
    ) -> crate::native::physical_work_signal::UiNativePhysicalSignalExternalObservation {
        let Some(device) = device else {
            return basis.observe(
                crate::native::physical_work_signal::UiNativePhysicalSignalStatus::Pending,
            );
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
        basis.observe(if settled {
            crate::native::physical_work_signal::UiNativePhysicalSignalStatus::Completed
        } else {
            crate::native::physical_work_signal::UiNativePhysicalSignalStatus::Pending
        })
    }
}

impl UiNativePendingPresentation {
    fn external(
        external: Box<dyn UiNativePendingExternalObligation>,
        readback_owner: crate::native::UiNativeResourceOwner,
        submission_owner: crate::native::UiNativeResourceOwner,
        physical_work: crate::native::physical_work_signal::UiNativePhysicalPresentationIdentity,
        physical_token: crate::native::physical_work_signal::UiNativePhysicalSignalRequestToken,
    ) -> Self {
        Self {
            external,
            readback_owner,
            submission_owner,
            physical_work,
            physical_token,
        }
    }

    pub(crate) const fn physical_work(
        &self,
    ) -> crate::native::physical_work_signal::UiNativePhysicalPresentationIdentity {
        self.physical_work
    }

    pub(crate) const fn physical_token(
        &self,
    ) -> crate::native::physical_work_signal::UiNativePhysicalSignalRequestToken {
        self.physical_token
    }

    pub(crate) fn refresh_physical_token(
        &mut self,
        token: crate::native::physical_work_signal::UiNativePhysicalSignalRequestToken,
    ) -> bool {
        if token.work()
            != crate::native::physical_work_signal::UiNativePhysicalSignalWork::Presentation(
                self.physical_work,
            )
        {
            return false;
        }
        self.physical_token = token;
        true
    }

    pub(crate) fn poll_observation(
        &mut self,
        device: Option<&wgpu::Device>,
    ) -> crate::native::physical_work_signal::UiNativePhysicalSignalExternalObservation {
        self.external
            .poll_observation(self.physical_token.external_basis(), device)
    }

    pub(crate) fn release(self, resources: &mut UiNativeResourceRegistry) {
        let Self {
            external,
            readback_owner,
            submission_owner,
            physical_work: _,
            physical_token: _,
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
    physical_signal: &mut crate::native::physical_work_signal::UiNativePhysicalSignalOwner,
    basis: crate::native::physical_work_signal::UiNativePhysicalPresentationBasis,
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
    let readback = owners.remove(0);
    let submission = owners.remove(0);
    let physical_work = match physical_signal.admit_presentation(basis) {
        Ok(work) => work,
        Err(()) => {
            resources
                .release(readback)
                .expect("unused readback reservation must release exactly");
            resources
                .release(submission)
                .expect("unused submission reservation must release exactly");
            return Err(UiNativePresentationFailure::BeforeEffects(
                UiHostSurfacePresentationDenial::CapacityExceeded,
            ));
        }
    };
    let physical_token = physical_signal
        .take_ready_presentation(physical_work)
        .expect("new physical presentation work must issue one exact wake");
    Ok(UiNativePresentationOwners {
        readback,
        submission,
        physical_work,
        physical_token,
    })
}

pub(crate) fn settle_port_result(
    resources: &mut UiNativeResourceRegistry,
    physical_signal: &mut crate::native::physical_work_signal::UiNativePhysicalSignalOwner,
    owners: UiNativePresentationOwners,
    result: Result<port::UiNativePresentationPortObservation, UiNativePresentationPortFailure>,
) -> Result<port::UiNativePresentationPortObservation, UiNativePresentationFailure> {
    match result {
        Ok(observation) => {
            let settled = physical_signal.reconcile(owners.physical_token.observe(
                crate::native::physical_work_signal::UiNativePhysicalSignalStatus::Completed,
            ));
            assert!(matches!(
                settled,
                crate::native::physical_work_signal::UiNativePhysicalSignalSettlement::Completed
            ));
            release_reserved(resources, owners);
            Ok(observation)
        }
        Err(UiNativePresentationPortFailure::SurfaceUnavailable) => {
            let settled = physical_signal.reconcile(owners.physical_token.observe(
                crate::native::physical_work_signal::UiNativePhysicalSignalStatus::RejectedBeforeEffects,
            ));
            assert!(matches!(
                settled,
                crate::native::physical_work_signal::UiNativePhysicalSignalSettlement::Rejected
            ));
            release_reserved(resources, owners);
            Err(UiNativePresentationFailure::BeforeEffects(
                UiHostSurfacePresentationDenial::AdapterDeclined,
            ))
        }
        Err(UiNativePresentationPortFailure::ReadbackUnsettled(external)) => {
            let settled = physical_signal.reconcile(owners.physical_token.observe(
                crate::native::physical_work_signal::UiNativePhysicalSignalStatus::Pending,
            ));
            assert!(matches!(
                settled,
                crate::native::physical_work_signal::UiNativePhysicalSignalSettlement::Pending
            ));
            Err(UiNativePresentationFailure::Indeterminate(
                UiNativePendingPresentation::external(
                    external,
                    owners.readback,
                    owners.submission,
                    owners.physical_work,
                    owners.physical_token,
                ),
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
