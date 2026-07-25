//! Host-owned mechanics admitted by the runtime's authority-bearing adapter boundary.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostSessionReleaseReceipt {
    host_session_identity: u64,
    released_surface_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostSessionReleaseIndeterminate {
    host_session_identity: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostSessionReleaseOutcome {
    Released(UiHostSessionReleaseReceipt),
    ReleaseIndeterminate(UiHostSessionReleaseIndeterminate),
}

/// Mechanical host operations that cannot publish or validate Worth UI truth.
///
/// The runtime implements its governed operational adapter contract for these
/// mechanics and checks concrete runtime authority before delegating effects.
pub trait WorthUiHostMechanicsAdapter: crate::WorthUiMeasurementHostAdapter {
    fn mechanical_host_contract(&self) -> crate::WorthUiHostContract;

    fn mechanical_protocol_contract(&self) -> crate::UiHostProtocolContract {
        crate::UiHostProtocolContract::current()
    }

    fn mechanical_capability_report(&self) -> crate::WorthUiHostCapabilityReport;

    fn mechanical_measurement_environment_report(
        &self,
    ) -> crate::UiHostMeasurementEnvironmentReport {
        crate::UiHostMeasurementEnvironmentReport::unsupported()
    }

    fn perform_mounted_surface_presentation(
        &self,
        _view: &crate::UiMountedFrameConsumptionView<'_>,
    ) -> crate::UiHostSurfacePresentationOutcome {
        crate::UiHostSurfacePresentationOutcome::RejectedBeforeEffects(
            crate::UiHostSurfacePresentationDenial::AdapterDeclined,
        )
    }

    fn perform_mounted_surface_completion(
        &self,
        _token: crate::UiHostPresentationCompletionToken,
    ) -> crate::UiHostSurfaceInFlightCompletion {
        crate::UiHostSurfaceInFlightCompletion::PresentationIndeterminate
    }

    fn perform_mounted_surface_cancellation(
        &self,
        _token: crate::UiHostPresentationCompletionToken,
    ) -> crate::UiHostSurfaceCancellationOutcome {
        crate::UiHostSurfaceCancellationOutcome::EffectsMayHaveBegun
    }

    fn perform_surface_registration(
        &self,
        _request: crate::UiHostSurfaceRegistrationRequest,
    ) -> crate::UiHostSurfaceRegistrationOutcome {
        crate::UiHostSurfaceRegistrationOutcome::RejectedBeforeEffects(
            crate::UiHostSurfaceRegistrationDenial::Unsupported,
        )
    }

    fn perform_surface_deregistration(
        &self,
        _request: crate::UiHostSurfaceRegistrationRequest,
    ) -> crate::UiHostSurfaceDeregistrationOutcome {
        crate::UiHostSurfaceDeregistrationOutcome::RejectedBeforeEffects(
            crate::UiHostSurfaceRegistrationDenial::Unsupported,
        )
    }

    fn release_mechanical_host_session(
        &self,
        host_session_identity: u64,
    ) -> UiHostSessionReleaseOutcome;
}

impl UiHostSessionReleaseReceipt {
    pub fn released(host_session_identity: u64, released_surface_count: usize) -> Self {
        Self {
            host_session_identity,
            released_surface_count,
        }
    }

    pub fn host_session_identity(self) -> u64 {
        self.host_session_identity
    }

    pub fn released_surface_count(self) -> usize {
        self.released_surface_count
    }
}

impl UiHostSessionReleaseIndeterminate {
    pub fn after_effects_may_have_begun(host_session_identity: u64) -> Self {
        Self {
            host_session_identity,
        }
    }

    pub fn host_session_identity(self) -> u64 {
        self.host_session_identity
    }
}
