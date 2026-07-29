use super::{UiHostAdapterSessionAuthority, UiHostSessionReleaseOutcome};

pub trait WorthUiOperationalHostAdapter:
    worth_ui_host_contract::WorthUiMeasurementHostAdapter
{
    fn operational_host_contract(&self) -> worth_ui_host_contract::WorthUiHostContract;

    fn operational_protocol_contract(&self) -> worth_ui_host_contract::UiHostProtocolContract {
        worth_ui_host_contract::UiHostProtocolContract::current()
    }

    fn operational_capability_report(&self) -> worth_ui_host_contract::WorthUiHostCapabilityReport;

    fn measurement_environment_report(
        &self,
    ) -> worth_ui_host_contract::UiHostMeasurementEnvironmentReport {
        worth_ui_host_contract::UiHostMeasurementEnvironmentReport::unsupported()
    }

    fn visual_capture_capability(&self) -> worth_ui_host_contract::UiHostCaptureCapability {
        worth_ui_host_contract::UiHostCaptureCapability::Unsupported
    }

    fn drain_host_observations(
        &self,
        _authority: &UiHostAdapterSessionAuthority,
    ) -> Result<
        worth_ui_host_contract::UiHostObservationDrain,
        worth_ui_host_contract::UiHostObservationDrainDenial,
    > {
        Ok(worth_ui_host_contract::UiHostObservationDrain::empty())
    }

    fn capture_visual_presentation(
        &self,
        _authority: &UiHostAdapterSessionAuthority,
        _request: worth_ui_host_contract::UiHostVisualCaptureRequest,
    ) -> worth_ui_host_contract::UiHostCaptureObservationOutcome {
        worth_ui_host_contract::UiHostCaptureObservationOutcome::Unsupported
    }

    fn cancel_visual_capture(
        &self,
        _authority: &UiHostAdapterSessionAuthority,
        _request: worth_ui_host_contract::UiHostVisualCaptureRequest,
    ) -> worth_ui_host_contract::UiHostCaptureCancellationOutcome {
        worth_ui_host_contract::UiHostCaptureCancellationOutcome::ReadbackMayHaveBegun
    }

    fn present_mounted_surface(
        &self,
        _authority: &UiHostAdapterSessionAuthority,
        _view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
    ) -> worth_ui_host_contract::UiHostSurfacePresentationOutcome {
        worth_ui_host_contract::UiHostSurfacePresentationOutcome::RejectedBeforeEffects(
            worth_ui_host_contract::UiHostSurfacePresentationDenial::AdapterDeclined,
        )
    }

    fn complete_mounted_surface(
        &self,
        _authority: &UiHostAdapterSessionAuthority,
        _token: worth_ui_host_contract::UiHostPresentationCompletionToken,
    ) -> worth_ui_host_contract::UiHostSurfaceInFlightCompletion {
        worth_ui_host_contract::UiHostSurfaceInFlightCompletion::PresentationIndeterminate
    }

    fn cancel_mounted_surface(
        &self,
        _authority: &UiHostAdapterSessionAuthority,
        _token: worth_ui_host_contract::UiHostPresentationCompletionToken,
    ) -> worth_ui_host_contract::UiHostSurfaceCancellationOutcome {
        worth_ui_host_contract::UiHostSurfaceCancellationOutcome::EffectsMayHaveBegun
    }

    fn register_surface(
        &self,
        _authority: &UiHostAdapterSessionAuthority,
        _request: worth_ui_host_contract::UiHostSurfaceRegistrationRequest,
    ) -> worth_ui_host_contract::UiHostSurfaceRegistrationOutcome {
        worth_ui_host_contract::UiHostSurfaceRegistrationOutcome::RejectedBeforeEffects(
            worth_ui_host_contract::UiHostSurfaceRegistrationDenial::Unsupported,
        )
    }

    fn deregister_surface(
        &self,
        _authority: &UiHostAdapterSessionAuthority,
        _request: worth_ui_host_contract::UiHostSurfaceRegistrationRequest,
    ) -> worth_ui_host_contract::UiHostSurfaceDeregistrationOutcome {
        worth_ui_host_contract::UiHostSurfaceDeregistrationOutcome::RejectedBeforeEffects(
            worth_ui_host_contract::UiHostSurfaceRegistrationDenial::Unsupported,
        )
    }

    /// Terminalize every adapter-owned resource for this session, including
    /// registrations and in-flight completion state. The returned count names
    /// released surface registrations; other resources are still mandatory.
    fn release_host_session(
        &self,
        authority: &UiHostAdapterSessionAuthority,
    ) -> UiHostSessionReleaseOutcome;
}

pub trait WorthUiHostAdapter: WorthUiOperationalHostAdapter {}

impl<Adapter> WorthUiHostAdapter for Adapter where Adapter: WorthUiOperationalHostAdapter + ?Sized {}

impl<Adapter> WorthUiOperationalHostAdapter for Adapter
where
    Adapter: worth_ui_host_contract::WorthUiHostMechanicsAdapter + ?Sized,
{
    fn operational_host_contract(&self) -> worth_ui_host_contract::WorthUiHostContract {
        self.mechanical_host_contract()
    }

    fn operational_protocol_contract(&self) -> worth_ui_host_contract::UiHostProtocolContract {
        self.mechanical_protocol_contract()
    }

    fn operational_capability_report(&self) -> worth_ui_host_contract::WorthUiHostCapabilityReport {
        self.mechanical_capability_report()
    }

    fn measurement_environment_report(
        &self,
    ) -> worth_ui_host_contract::UiHostMeasurementEnvironmentReport {
        self.mechanical_measurement_environment_report()
    }

    fn visual_capture_capability(&self) -> worth_ui_host_contract::UiHostCaptureCapability {
        self.mechanical_visual_capture_capability()
    }

    fn drain_host_observations(
        &self,
        authority: &UiHostAdapterSessionAuthority,
    ) -> Result<
        worth_ui_host_contract::UiHostObservationDrain,
        worth_ui_host_contract::UiHostObservationDrainDenial,
    > {
        self.drain_mechanical_host_observations(authority.host_session_identity())
    }

    fn capture_visual_presentation(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        request: worth_ui_host_contract::UiHostVisualCaptureRequest,
    ) -> worth_ui_host_contract::UiHostCaptureObservationOutcome {
        if !authority.admits_visual_capture(request) {
            return worth_ui_host_contract::UiHostCaptureObservationOutcome::Unsupported;
        }
        self.perform_visual_capture(request)
    }

    fn cancel_visual_capture(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        request: worth_ui_host_contract::UiHostVisualCaptureRequest,
    ) -> worth_ui_host_contract::UiHostCaptureCancellationOutcome {
        if !authority.admits_visual_capture(request) {
            return worth_ui_host_contract::UiHostCaptureCancellationOutcome::CleanupIndeterminate;
        }
        self.perform_visual_capture_cancellation(request)
    }

    fn present_mounted_surface(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
    ) -> worth_ui_host_contract::UiHostSurfacePresentationOutcome {
        if !authority.admits_mounted_presentation(view) {
            return worth_ui_host_contract::UiHostSurfacePresentationOutcome::RejectedBeforeEffects(
                worth_ui_host_contract::UiHostSurfacePresentationDenial::SurfaceBindingChanged,
            );
        }
        self.perform_mounted_surface_presentation(view)
    }

    fn complete_mounted_surface(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        token: worth_ui_host_contract::UiHostPresentationCompletionToken,
    ) -> worth_ui_host_contract::UiHostSurfaceInFlightCompletion {
        if !authority.admits_mounted_completion_token(&token) {
            return worth_ui_host_contract::UiHostSurfaceInFlightCompletion::RejectedBeforeEffects(
                worth_ui_host_contract::UiHostSurfacePresentationDenial::SurfaceBindingChanged,
            );
        }
        self.perform_mounted_surface_completion(token)
    }

    fn cancel_mounted_surface(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        token: worth_ui_host_contract::UiHostPresentationCompletionToken,
    ) -> worth_ui_host_contract::UiHostSurfaceCancellationOutcome {
        if !authority.admits_mounted_completion_token(&token) {
            return worth_ui_host_contract::UiHostSurfaceCancellationOutcome::EffectsMayHaveBegun;
        }
        self.perform_mounted_surface_cancellation(token)
    }

    fn register_surface(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        request: worth_ui_host_contract::UiHostSurfaceRegistrationRequest,
    ) -> worth_ui_host_contract::UiHostSurfaceRegistrationOutcome {
        if request.host_session_identity() != authority.host_session_identity() {
            return worth_ui_host_contract::UiHostSurfaceRegistrationOutcome::RejectedBeforeEffects(
                worth_ui_host_contract::UiHostSurfaceRegistrationDenial::ForeignRegistration,
            );
        }
        self.perform_surface_registration(request)
    }

    fn deregister_surface(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        request: worth_ui_host_contract::UiHostSurfaceRegistrationRequest,
    ) -> worth_ui_host_contract::UiHostSurfaceDeregistrationOutcome {
        if request.host_session_identity() != authority.host_session_identity() {
            return worth_ui_host_contract::UiHostSurfaceDeregistrationOutcome::RejectedBeforeEffects(
                worth_ui_host_contract::UiHostSurfaceRegistrationDenial::ForeignRegistration,
            );
        }
        self.perform_surface_deregistration(request)
    }

    fn release_host_session(
        &self,
        authority: &UiHostAdapterSessionAuthority,
    ) -> UiHostSessionReleaseOutcome {
        self.release_mechanical_host_session(authority.host_session_identity())
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use worth_ui_host_contract::{
        UiHostCaptureArtifactBudget, UiHostCaptureFrameAffinity, UiHostCaptureObservationOutcome,
        UiHostCaptureRequestIdentity, UiHostCaptureSurfaceAffinity,
        UiHostMeasurementObservationValue, UiHostMeasurementRequest, UiHostPresentationEpoch,
        UiHostSessionReleaseOutcome, UiHostSessionReleaseReceipt, UiHostSurfaceIdentity,
        UiHostVisualCaptureRequest, UiMountedFrameIdentity, UiMountedPresentationAttemptIdentity,
        UiSurfaceBindingGeneration, WorthUiHostCapabilityReport, WorthUiHostContract,
        WorthUiHostMechanicsAdapter, WorthUiMeasurementHostAdapter,
    };

    use super::{UiHostAdapterSessionAuthority, WorthUiOperationalHostAdapter};

    #[derive(Default)]
    struct PendingCaptureMechanics {
        capture_calls: Cell<usize>,
    }

    impl WorthUiMeasurementHostAdapter for PendingCaptureMechanics {
        fn observe_measurement(
            &self,
            _request: &UiHostMeasurementRequest,
        ) -> UiHostMeasurementObservationValue {
            unreachable!("the focused capture mechanics declare no measurement capability")
        }
    }

    impl WorthUiHostMechanicsAdapter for PendingCaptureMechanics {
        fn mechanical_host_contract(&self) -> WorthUiHostContract {
            WorthUiHostContract::headless()
        }

        fn mechanical_capability_report(&self) -> WorthUiHostCapabilityReport {
            WorthUiHostCapabilityReport::available(Vec::new())
        }

        fn perform_visual_capture(
            &self,
            _request: UiHostVisualCaptureRequest,
        ) -> UiHostCaptureObservationOutcome {
            self.capture_calls.set(self.capture_calls.get() + 1);
            UiHostCaptureObservationOutcome::Pending
        }

        fn release_mechanical_host_session(
            &self,
            host_session_identity: u64,
        ) -> UiHostSessionReleaseOutcome {
            UiHostSessionReleaseOutcome::Released(UiHostSessionReleaseReceipt::released(
                host_session_identity,
                0,
            ))
        }
    }

    #[test]
    fn mechanics_capture_port_rejects_foreign_session_before_adapter_effects() {
        let mechanics = PendingCaptureMechanics::default();
        let authority = UiHostAdapterSessionAuthority::activate(7);
        assert!(matches!(
            mechanics.capture_visual_presentation(&authority, capture_request(7)),
            UiHostCaptureObservationOutcome::Pending
        ));
        assert!(matches!(
            mechanics.capture_visual_presentation(&authority, capture_request(8)),
            UiHostCaptureObservationOutcome::Unsupported
        ));
        assert_eq!(mechanics.capture_calls.get(), 1);
    }

    fn capture_request(host_session_identity: u64) -> UiHostVisualCaptureRequest {
        UiHostVisualCaptureRequest::admitted_by_runtime(
            UiHostCaptureRequestIdentity::issued_by_runtime(1),
            UiHostCaptureFrameAffinity::observed_by_runtime(
                UiMountedFrameIdentity::mint_unbound().unwrap(),
                UiMountedPresentationAttemptIdentity::mint_unbound().unwrap(),
            ),
            UiHostCaptureSurfaceAffinity::observed_by_runtime(
                host_session_identity,
                UiHostSurfaceIdentity::mint_unbound().unwrap(),
                UiSurfaceBindingGeneration::mint_unbound().unwrap(),
                UiHostPresentationEpoch::issued_by_host(1),
            ),
            UiHostCaptureArtifactBudget::admitted_by_runtime(false, 0),
        )
    }
}
