use super::{WorthUiActiveApplicationSession, WorthUiRuntimeShutdownReceipt};

impl WorthUiActiveApplicationSession {
    pub fn shutdown(mut self) -> WorthUiRuntimeShutdownReceipt {
        let rebind = self.rebind.shutdown();
        let visual_capture = self.visual_captures.shutdown();
        let visual_overlay = self.visual_overlays.shutdown();
        let interaction = self.interaction.shutdown();
        let confirmation = self.intent_confirmation.shutdown();
        let (admission, execution) = self.intent_admission.shutdown(&mut self.intent_execution);
        let observation_resources = self.application.retire_observation_resources(
            crate::runtime::observation::UiObservationResourceRetirementCause::ApplicationShutdown,
        );
        let intent_evidence = self
            .intent_evidence
            .retire(worth_ui_inspection::UiIntentEvidenceRetirementCause::ApplicationShutdown);
        let final_intent_resource_census = self.intent_resource_census();
        debug_assert!(final_intent_resource_census.is_empty());
        let (mounted_presentation, outcomes, presentation_async_cleanup) =
            self.mounted.shutdown_presentation(&self.host_session);
        for outcome in outcomes {
            let _ = self.finish_mounted_presentation(outcome);
        }
        self.mounted.assert_shutdown_resolved();
        self.host_exchange.shutdown();
        let host_session_release = self.host_session.release_adapter_session();
        let host_session_recovery = matches!(
            host_session_release,
            worth_ui_host_contract::UiHostSessionReleaseOutcome::ReleaseIndeterminate(_)
        )
        .then(|| crate::facade::WorthUiHostSessionReleaseRecovery::retain(self.host_session));
        self.application
            .shutdown()
            .bind_visual_capture(visual_capture)
            .bind_visual_overlay(visual_overlay)
            .bind_mounted_presentation(mounted_presentation)
            .bind_presentation_async_cleanup(presentation_async_cleanup)
            .bind_host_session_release(host_session_release)
            .bind_host_session_recovery(host_session_recovery)
            .bind_interaction(interaction)
            .bind_intent_confirmation(confirmation)
            .bind_intent_admission(admission)
            .bind_intent_execution(execution)
            .bind_observation_resources(observation_resources)
            .bind_intent_evidence(intent_evidence)
            .bind_intent_resource_census(final_intent_resource_census)
            .bind_rebind(rebind)
    }
}
