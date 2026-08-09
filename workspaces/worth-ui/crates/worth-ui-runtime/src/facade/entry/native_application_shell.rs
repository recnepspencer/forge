use super::{WorthUiActiveApplicationSession, WorthUiApp};
use crate::facade::mounted::{
    UiHostSurfacePresentationMode, UiMountedFrameOutcome, UiPresentationDeadline,
    UiSurfaceBindingCoordinatePosture, UiSurfaceBindingGeneration, UiSurfaceBindingProfile,
};

/// High-level native lifecycle for one downstream application composition root.
pub struct WorthUiNativeApplicationShell {
    pub(super) session: WorthUiActiveApplicationSession,
    binding: UiSurfaceBindingGeneration,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiNativeApplicationShutdownReceipt {
    mounted_shutdown_attempt_count: usize,
    visual_capture: crate::inspection::visual_snapshot::UiVisualCaptureShutdownReport,
    visual_overlay: crate::inspection::visual_snapshot::UiVisualOverlayShutdownReport,
    host_session_released: bool,
    released_surface_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiNativeApplicationShellLaunchDenial {
    RuntimeLaunch,
    SemanticSurfaceCreation,
    HostSurfaceRegistration,
    MountedInstanceCreation,
    ViewportAllocation,
}

impl WorthUiApp {
    /// Launch one native surface without exposing mounted construction authority.
    pub fn launch_native_surface(
        self,
    ) -> Result<WorthUiNativeApplicationShell, WorthUiNativeApplicationShellLaunchDenial> {
        self.launch_native_surface_at_scale(1_000)
    }

    #[doc(hidden)]
    pub fn launch_native_surface_at_scale(
        self,
        scale_factor_milli: u32,
    ) -> Result<WorthUiNativeApplicationShell, WorthUiNativeApplicationShellLaunchDenial> {
        let mut session = self
            .launch()
            .map_err(|_| WorthUiNativeApplicationShellLaunchDenial::RuntimeLaunch)?;
        let surface = session
            .create_semantic_surface()
            .map_err(|_| WorthUiNativeApplicationShellLaunchDenial::SemanticSurfaceCreation)?;
        let profile = UiSurfaceBindingProfile::new(
            scale_factor_milli,
            UiSurfaceBindingCoordinatePosture::LogicalPoints,
            1,
        )
        .map_err(|_| WorthUiNativeApplicationShellLaunchDenial::HostSurfaceRegistration)?;
        let binding = session
            .register_host_surface(
                surface,
                UiHostSurfacePresentationMode::NativeDisplay,
                profile,
            )
            .map_err(|_| WorthUiNativeApplicationShellLaunchDenial::HostSurfaceRegistration)?;
        let graph_nodes = session.graph().node_identities().collect::<Vec<_>>();
        for graph_node in graph_nodes {
            let handle = session
                .mounted_graph_node(graph_node)
                .map_err(|_| WorthUiNativeApplicationShellLaunchDenial::MountedInstanceCreation)?;
            session
                .mount_instance(handle, surface)
                .map_err(|_| WorthUiNativeApplicationShellLaunchDenial::MountedInstanceCreation)?;
        }
        session
            .establish_native_viewport_allocation()
            .map_err(|_| WorthUiNativeApplicationShellLaunchDenial::ViewportAllocation)?;
        Ok(WorthUiNativeApplicationShell {
            session,
            binding: binding.binding_generation(),
        })
    }
}

impl WorthUiNativeApplicationShell {
    pub(crate) fn presentation_attribution(
        &self,
        outcome: &UiMountedFrameOutcome,
    ) -> Option<worth_ui_host_native::UiNativeClientPresentationAttribution> {
        let receipt = match outcome {
            UiMountedFrameOutcome::Published(receipt)
            | UiMountedFrameOutcome::Unchanged(receipt)
            | UiMountedFrameOutcome::Reconciled(receipt) => receipt,
            _ => return None,
        };
        let binding = *receipt.bindings().first()?;
        let (surface, mounted_instance, node_receipt) = self
            .session
            .mounted
            .native_filled_rect_attribution(receipt.frame(), binding)?;
        Some(
            worth_ui_host_native::UiNativeClientPresentationAttribution::reported(
                receipt.frame().diagnostic_value(),
                surface.diagnostic_value(),
                binding.diagnostic_value(),
                mounted_instance.diagnostic_value(),
                node_receipt.diagnostic_value(),
                receipt.attempt().diagnostic_value(),
            ),
        )
    }

    pub(crate) fn rebind_native_surface_scale(
        &mut self,
        scale_factor_milli: u32,
    ) -> Result<(), ()> {
        let profile = UiSurfaceBindingProfile::new(
            scale_factor_milli,
            UiSurfaceBindingCoordinatePosture::LogicalPoints,
            1,
        )
        .map_err(|_| ())?;
        self.binding = self
            .session
            .rebind_host_surface(
                self.binding,
                UiHostSurfacePresentationMode::NativeDisplay,
                profile,
            )
            .map_err(|_| ())?
            .binding_generation();
        Ok(())
    }

    pub fn update_intent_boolean_fact(
        &mut self,
        fact: &crate::facade::intent::UiIntentApplicationFact<
            crate::facade::intent::UiIntentBoolean,
        >,
        value: bool,
    ) -> Result<
        crate::facade::intent::UiIntentApplicationFactUpdateReceipt,
        crate::facade::intent::UiIntentApplicationFactUpdateDenial,
    > {
        self.session.update_intent_boolean_fact(fact, value)
    }

    pub fn update_intent_unsigned64_fact(
        &mut self,
        fact: &crate::facade::intent::UiIntentApplicationFact<
            crate::facade::intent::UiIntentUnsigned64,
        >,
        value: u64,
    ) -> Result<
        crate::facade::intent::UiIntentApplicationFactUpdateReceipt,
        crate::facade::intent::UiIntentApplicationFactUpdateDenial,
    > {
        self.session.update_intent_unsigned64_fact(fact, value)
    }

    pub const fn rebind_deadline_at(
        &self,
        tick: u64,
    ) -> crate::runtime::rebind::UiRebindSessionDeadline {
        self.session.rebind_deadline_at(tick)
    }

    pub const fn rebind_cancellation_request(
        &self,
    ) -> crate::runtime::rebind::UiRebindCancellationRequest {
        self.session.rebind_cancellation_request()
    }

    /// Execute and present one ordinary native frame.
    pub fn present_frame(
        &mut self,
        deadline_tick: u64,
        now_tick: u64,
    ) -> Result<UiMountedFrameOutcome, super::WorthUiMountedFrameExecutionStop<'_>> {
        let request = self.session.mounted_frame_request();
        self.session.execute_mounted_frame(
            request,
            UiPresentationDeadline::at_tick(deadline_tick),
            now_tick,
            |_| {},
        )
    }

    pub fn generation_identity(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity
    {
        self.session.generation_identity()
    }

    /// Consume the shell and report runtime, mounted, and host cleanup.
    pub fn shutdown(self) -> WorthUiNativeApplicationShutdownReceipt {
        let runtime = self.session.shutdown();
        let (host_session_released, released_surface_count) = match runtime.host_session_release() {
            Some(worth_ui_host_contract::UiHostSessionReleaseOutcome::Released(receipt)) => {
                (true, receipt.released_surface_count())
            }
            Some(worth_ui_host_contract::UiHostSessionReleaseOutcome::ReleaseIndeterminate(_))
            | None => (false, 0),
        };
        WorthUiNativeApplicationShutdownReceipt {
            mounted_shutdown_attempt_count: runtime.mounted_presentation().attempts().len(),
            visual_capture: runtime.visual_capture(),
            visual_overlay: runtime.visual_overlay(),
            host_session_released,
            released_surface_count,
        }
    }
}

impl WorthUiNativeApplicationShutdownReceipt {
    pub fn mounted_shutdown_attempt_count(self) -> usize {
        self.mounted_shutdown_attempt_count
    }

    pub fn host_session_released(self) -> bool {
        self.host_session_released
    }

    pub const fn visual_capture(
        self,
    ) -> crate::inspection::visual_snapshot::UiVisualCaptureShutdownReport {
        self.visual_capture
    }

    pub const fn visual_overlay(
        self,
    ) -> crate::inspection::visual_snapshot::UiVisualOverlayShutdownReport {
        self.visual_overlay
    }

    pub fn released_surface_count(self) -> usize {
        self.released_surface_count
    }
}
