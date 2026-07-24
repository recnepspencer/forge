use worth_ui_host_contract::{
    UiHostProtocolAgreement, UiMountedFrameConsumptionInput, UiMountedFrameConsumptionView,
    UiMountedPresentationAttemptIdentity, UiMountedPresentationLease, UiMountedProjectionView,
    UiMountedSurfaceBindingRequirement, UiPresentationDeadline, WorthUiHostCapabilityReport,
};

#[derive(Clone, Copy)]
pub(crate) struct UiMountedHostPresentationAuthority<'authority> {
    host_session_identity: u64,
    protocol: UiHostProtocolAgreement,
    capability_report: &'authority WorthUiHostCapabilityReport,
    presentation: &'authority UiMountedPresentationLease,
}

pub(super) struct UiRuntimeMountedFrameConsumptionInput<'frame> {
    pub attempt: UiMountedPresentationAttemptIdentity,
    pub deadline: UiPresentationDeadline,
    pub requirement: UiMountedSurfaceBindingRequirement,
    pub projection: &'frame UiMountedProjectionView,
}

impl<'authority> UiMountedHostPresentationAuthority<'authority> {
    pub(crate) fn new(
        host_session_identity: u64,
        protocol: UiHostProtocolAgreement,
        capability_report: &'authority WorthUiHostCapabilityReport,
        presentation: &'authority UiMountedPresentationLease,
    ) -> Self {
        Self {
            host_session_identity,
            protocol,
            capability_report,
            presentation,
        }
    }

    pub(super) fn protocol(self) -> UiHostProtocolAgreement {
        self.protocol
    }

    pub(super) fn capability_report(self) -> &'authority WorthUiHostCapabilityReport {
        self.capability_report
    }

    pub(super) fn bind<'frame>(
        self,
        input: UiRuntimeMountedFrameConsumptionInput<'frame>,
    ) -> UiMountedFrameConsumptionView<'frame> {
        self.presentation.open(UiMountedFrameConsumptionInput {
            host_session_identity: self.host_session_identity,
            protocol: self.protocol,
            capability_generation: self.capability_report.observation_generation(),
            capability_profile_digest: self.capability_report.profile_identity_digest(),
            attempt: input.attempt,
            deadline: input.deadline,
            requirement: input.requirement,
            projection: input.projection,
        })
    }
}
