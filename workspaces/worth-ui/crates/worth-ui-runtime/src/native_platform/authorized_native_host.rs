//! Runtime-owned binding of the qualified native platform effect capability.

use crate::host::adapter::{UiHostAdapterSessionAuthority, WorthUiOperationalHostAdapter};
use worth_ui_host_contract::WorthUiHostMechanicsAdapter;

pub(crate) struct UiAuthorizedNativeHostAdapter {
    adapter: worth_ui_host_native::WorthUiPreparedNativeMechanics,
}

impl UiAuthorizedNativeHostAdapter {
    pub(crate) fn bind(prepared: worth_ui_host_native::WorthUiPreparedNativeMechanics) -> Self {
        Self { adapter: prepared }
    }
}

impl worth_ui_host_contract::WorthUiMeasurementHostAdapter for UiAuthorizedNativeHostAdapter {
    fn observe_measurement(
        &self,
        request: &worth_ui_host_contract::UiHostMeasurementRequest,
    ) -> worth_ui_host_contract::UiHostMeasurementObservationValue {
        worth_ui_host_contract::WorthUiMeasurementHostAdapter::observe_measurement(
            &self.adapter,
            request,
        )
    }
}

impl WorthUiOperationalHostAdapter for UiAuthorizedNativeHostAdapter {
    fn operational_host_contract(&self) -> worth_ui_host_contract::WorthUiHostContract {
        self.adapter.mechanical_host_contract()
    }

    fn operational_protocol_contract(&self) -> worth_ui_host_contract::UiHostProtocolContract {
        self.adapter.mechanical_protocol_contract()
    }

    fn operational_capability_report(&self) -> worth_ui_host_contract::WorthUiHostCapabilityReport {
        self.adapter.mechanical_capability_report()
    }

    fn open_host_session(
        &self,
        authority: &UiHostAdapterSessionAuthority,
    ) -> Result<(), worth_ui_host_contract::UiHostObservationSessionRegistrationDenial> {
        self.adapter
            .register_mechanical_host_session(authority.host_session_identity())
    }

    fn measurement_environment_report(
        &self,
    ) -> worth_ui_host_contract::UiHostMeasurementEnvironmentReport {
        self.adapter.mechanical_measurement_environment_report()
    }

    fn visual_capture_capability(&self) -> worth_ui_host_contract::UiHostCaptureCapability {
        self.adapter.mechanical_visual_capture_capability()
    }

    fn drain_host_observations(
        &self,
        authority: &UiHostAdapterSessionAuthority,
    ) -> Result<
        worth_ui_host_contract::UiHostObservationDrain,
        worth_ui_host_contract::UiHostObservationDrainDenial,
    > {
        self.adapter
            .drain_mechanical_host_observations(authority.host_session_identity())
    }

    fn install_input_recipient(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        binding: worth_ui_host_contract::UiHostInputRecipientBindingReceipt,
    ) -> bool {
        binding.host_session() == authority.host_session_identity()
            && self.adapter.install_mechanical_input_recipient(binding)
    }

    fn clear_input_recipient(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        binding: worth_ui_host_contract::UiHostInputRecipientBindingReceipt,
    ) -> bool {
        binding.host_session() == authority.host_session_identity()
            && self.adapter.clear_mechanical_input_recipient(binding)
    }

    fn capture_visual_presentation(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        request: worth_ui_host_contract::UiHostVisualCaptureRequest,
    ) -> worth_ui_host_contract::UiHostCaptureObservationOutcome {
        if !authority.admits_visual_capture(request) {
            return worth_ui_host_contract::UiHostCaptureObservationOutcome::Unsupported;
        }
        self.adapter.perform_visual_capture(request)
    }

    fn cancel_visual_capture(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        request: worth_ui_host_contract::UiHostVisualCaptureRequest,
    ) -> worth_ui_host_contract::UiHostCaptureCancellationOutcome {
        if !authority.admits_visual_capture(request) {
            return worth_ui_host_contract::UiHostCaptureCancellationOutcome::CleanupIndeterminate;
        }
        self.adapter.perform_visual_capture_cancellation(request)
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
        self.adapter.perform_mounted_surface_presentation(view)
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
        self.adapter.perform_mounted_surface_completion(token)
    }

    fn cancel_mounted_surface(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        token: worth_ui_host_contract::UiHostPresentationCompletionToken,
        reason: worth_ui_host_contract::UiHostSurfaceStopReason,
    ) -> worth_ui_host_contract::UiHostSurfaceCancellationOutcome {
        if !authority.admits_mounted_completion_token(&token) {
            return worth_ui_host_contract::UiHostSurfaceCancellationOutcome::EffectsMayHaveBegun;
        }
        self.adapter
            .perform_mounted_surface_cancellation(token, reason)
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
        self.adapter.perform_surface_registration(request)
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
        self.adapter.perform_surface_deregistration(request)
    }

    fn release_host_session(
        &self,
        authority: &UiHostAdapterSessionAuthority,
    ) -> crate::host::adapter::UiHostSessionReleaseOutcome {
        self.adapter
            .release_mechanical_host_session(authority.host_session_identity())
    }
}
