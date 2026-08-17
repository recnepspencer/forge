use super::WorthUiActiveApplicationSession;
use crate::facade::mounted::{
    UiHostSurfacePresentationMode, UiMountedFrameOutcome, UiPresentationDeadline,
    UiSurfaceBindingCoordinatePosture, UiSurfaceBindingGeneration, UiSurfaceBindingProfile,
};
use std::collections::HashMap;

#[path = "native_application_shell/component_presence.rs"]
mod component_presence;
#[path = "native_application_shell/launch.rs"]
mod launch;

/// High-level native lifecycle for one downstream application composition root.
pub struct WorthUiNativeApplicationShell {
    pub(super) session: WorthUiActiveApplicationSession,
    binding: UiSurfaceBindingGeneration,
    surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    mounted_rows: Vec<NativeMountedRow>,
    mounted_row_indices: HashMap<Box<str>, usize>,
    semantic_text_values: HashMap<Box<str>, std::sync::Arc<str>>,
}

struct NativeMountedRow {
    graph_node: crate::graph::UiGraphNodeIdentity,
    mounted: Option<worth_ui_host_contract::UiMountedInstanceIdentity>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiNativeApplicationShutdownReceipt {
    mounted_shutdown_attempt_count: usize,
    visual_capture: crate::inspection::visual_snapshot::UiVisualCaptureShutdownReport,
    visual_overlay: crate::inspection::visual_snapshot::UiVisualOverlayShutdownReport,
    host_session_released: bool,
    released_surface_count: usize,
    host_cleanup: Option<crate::facade::WorthUiHostSessionReleaseRecovery>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiNativeApplicationCleanup {
    host_cleanup: Option<crate::facade::WorthUiHostSessionReleaseRecovery>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum WorthUiNativeApplicationShellLaunchDenial {
    RuntimeLaunch,
    RuntimeLaunchCleanup(crate::runtime::WorthUiRuntimeLaunchDenial),
    SemanticSurfaceCreation,
    HostSurfaceRegistration,
    MountedInstanceCreation,
    ViewportAllocation,
    ApplicationCleanup(WorthUiNativeApplicationCleanup),
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
        let attribution = self
            .session
            .mounted
            .native_filled_rect_attribution(receipt.frame(), binding)?;
        Some(
            worth_ui_host_native::UiNativeClientPresentationAttribution::reported(
                [
                    receipt.frame().diagnostic_value(),
                    attribution.surface.diagnostic_value(),
                    binding.diagnostic_value(),
                    attribution.mounted_instance.diagnostic_value(),
                    attribution.node_receipt.diagnostic_value(),
                    receipt.attempt().diagnostic_value(),
                ],
                [
                    attribution.authored_provenance_digest,
                    attribution.authored_semantic_identity_digest,
                ],
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
        let mut semantic_content = crate::mounting::UiMountedSemanticContentInput::empty();
        for (identity, text) in &self.semantic_text_values {
            let index = *self
                .mounted_row_indices
                .get(identity.as_ref())
                .ok_or_else(|| {
                    super::WorthUiMountedFrameExecutionStop::Preparation(
                        crate::mounting::UiMountedFramePreparationDenial::Projection(
                            crate::mounting::UiMountedProjectionDenial::UnknownGraphNode,
                        ),
                    )
                })?;
            semantic_content
                .insert_scalar(
                    self.mounted_rows[index].graph_node,
                    crate::mounting::UiMountedSemanticTextValueDirective::Replace(
                        std::sync::Arc::clone(text),
                    ),
                    std::sync::Arc::from("native-application-program"),
                )
                .map_err(|_| {
                    super::WorthUiMountedFrameExecutionStop::Preparation(
                        crate::mounting::UiMountedFramePreparationDenial::Projection(
                            crate::mounting::UiMountedProjectionDenial::UnknownGraphNode,
                        ),
                    )
                })?;
        }
        self.session.execute_mounted_frame_with_content(
            request,
            UiPresentationDeadline::at_tick(deadline_tick),
            now_tick,
            semantic_content,
            |_| {},
        )
    }

    pub(crate) fn apply_component_semantic_text(
        &mut self,
        changes: &[super::UiNativeComponentSemanticTextChange],
    ) -> Result<(), ()> {
        for change in changes {
            if !self
                .mounted_row_indices
                .contains_key(change.authored_semantic_identity())
            {
                return Err(());
            }
        }
        for change in changes {
            self.semantic_text_values.insert(
                Box::from(change.authored_semantic_identity()),
                std::sync::Arc::from(change.text()),
            );
        }
        Ok(())
    }

    pub(crate) fn complete_frame_presentation(
        &mut self,
        in_flight: crate::mounting::UiMountedPresentationInFlight,
        now_tick: u64,
    ) -> UiMountedFrameOutcome {
        self.session
            .complete_mounted_presentation(in_flight, now_tick)
    }

    pub fn generation_identity(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity
    {
        self.session.generation_identity()
    }

    /// Consume the shell and report runtime, mounted, and host cleanup.
    pub fn shutdown(self) -> WorthUiNativeApplicationShutdownReceipt {
        let mut runtime = self.session.shutdown();
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
            host_cleanup: runtime.take_host_session_recovery(),
        }
    }
}

impl WorthUiNativeApplicationShutdownReceipt {
    pub fn mounted_shutdown_attempt_count(&self) -> usize {
        self.mounted_shutdown_attempt_count
    }

    pub fn host_session_released(&self) -> bool {
        self.host_session_released
    }

    pub const fn visual_capture(
        &self,
    ) -> crate::inspection::visual_snapshot::UiVisualCaptureShutdownReport {
        self.visual_capture
    }

    pub const fn visual_overlay(
        &self,
    ) -> crate::inspection::visual_snapshot::UiVisualOverlayShutdownReport {
        self.visual_overlay
    }

    pub fn released_surface_count(&self) -> usize {
        self.released_surface_count
    }

    pub(crate) fn into_host_cleanup(
        self,
    ) -> Option<crate::facade::WorthUiHostSessionReleaseRecovery> {
        self.host_cleanup
    }
}

impl WorthUiNativeApplicationCleanup {
    pub(crate) fn retry(mut self) -> Result<(), Self> {
        let Some(recovery) = self.host_cleanup.take() else {
            return Err(self);
        };
        match recovery.retry() {
            Ok(_) => Ok(()),
            Err(recovery) => {
                self.host_cleanup = Some(recovery);
                Err(self)
            }
        }
    }
}
