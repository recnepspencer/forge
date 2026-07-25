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
