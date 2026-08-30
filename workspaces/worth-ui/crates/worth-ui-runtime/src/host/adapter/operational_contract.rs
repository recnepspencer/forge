use super::{UiHostAdapterSessionAuthority, UiHostSessionReleaseOutcome};

pub trait WorthUiOperationalHostAdapter:
    worth_ui_host_contract::WorthUiMeasurementHostAdapter
{
    fn operational_host_contract(&self) -> worth_ui_host_contract::WorthUiHostContract;

    fn operational_protocol_contract(&self) -> worth_ui_host_contract::UiHostProtocolContract {
        worth_ui_host_contract::UiHostProtocolContract::current()
    }

    fn operational_capability_report(&self) -> worth_ui_host_contract::WorthUiHostCapabilityReport;

    /// Open the concrete authority's adapter session before session effects.
    ///
    /// Denial is effect-free. A successful open remains owned by the adapter
    /// until `release_host_session` terminalizes the same authority.
    fn open_host_session(
        &self,
        _authority: &UiHostAdapterSessionAuthority,
    ) -> Result<(), worth_ui_host_contract::UiHostObservationSessionRegistrationDenial> {
        Ok(())
    }

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

    fn install_input_recipient(
        &self,
        _authority: &UiHostAdapterSessionAuthority,
        _binding: worth_ui_host_contract::UiHostInputRecipientBindingReceipt,
    ) -> bool {
        false
    }

    fn clear_input_recipient(
        &self,
        _authority: &UiHostAdapterSessionAuthority,
        _binding: worth_ui_host_contract::UiHostInputRecipientBindingReceipt,
    ) -> bool {
        false
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

    fn place_semantic_focus(
        &self,
        _authority: &UiHostAdapterSessionAuthority,
        request: worth_ui_host_contract::UiHostFocusPlacementRequest,
    ) -> worth_ui_host_contract::UiHostFocusPlacementAcknowledgement {
        worth_ui_host_contract::UiHostFocusPlacementAcknowledgement::settled(
            request,
            worth_ui_host_contract::UiHostFocusPlacementDisposition::RejectedBeforeEffect(
                worth_ui_host_contract::UiHostFocusPlacementRejection::Unsupported,
            ),
        )
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
        _reason: worth_ui_host_contract::UiHostSurfaceStopReason,
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

    fn open_host_session(
        &self,
        authority: &UiHostAdapterSessionAuthority,
    ) -> Result<(), worth_ui_host_contract::UiHostObservationSessionRegistrationDenial> {
        self.register_mechanical_host_session(authority.host_session_identity())
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

    fn install_input_recipient(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        binding: worth_ui_host_contract::UiHostInputRecipientBindingReceipt,
    ) -> bool {
        binding.host_session() == authority.host_session_identity()
            && self.install_mechanical_input_recipient(binding)
    }

    fn clear_input_recipient(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        binding: worth_ui_host_contract::UiHostInputRecipientBindingReceipt,
    ) -> bool {
        binding.host_session() == authority.host_session_identity()
            && self.clear_mechanical_input_recipient(binding)
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

    fn place_semantic_focus(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        request: worth_ui_host_contract::UiHostFocusPlacementRequest,
    ) -> worth_ui_host_contract::UiHostFocusPlacementAcknowledgement {
        if request.host_session() != authority.host_session_identity() {
            return worth_ui_host_contract::UiHostFocusPlacementAcknowledgement::settled(
                request,
                worth_ui_host_contract::UiHostFocusPlacementDisposition::RejectedBeforeEffect(
                    worth_ui_host_contract::UiHostFocusPlacementRejection::ForeignSurface,
                ),
            );
        }
        self.perform_focus_placement(request)
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
        reason: worth_ui_host_contract::UiHostSurfaceStopReason,
    ) -> worth_ui_host_contract::UiHostSurfaceCancellationOutcome {
        if !authority.admits_mounted_completion_token(&token) {
            return worth_ui_host_contract::UiHostSurfaceCancellationOutcome::EffectsMayHaveBegun;
        }
        self.perform_mounted_surface_cancellation(token, reason)
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
#[path = "operational_contract_tests.rs"]
mod tests;
