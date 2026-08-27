/// Runtime lifecycle state owned by Worth UI.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiRuntimeLifecycle {
    Starting,
    Active,
    PausedForReplacement,
    PendingActivation,
    FailedActivationPreserved,
    ShuttingDown,
    Shutdown,
}

/// Monotonic frame epoch used to reject stale activation work.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthUiRuntimeFrameEpoch {
    value: u64,
}

impl WorthUiRuntimeFrameEpoch {
    pub fn initial() -> Self {
        Self { value: 0 }
    }

    pub fn as_u64(self) -> u64 {
        self.value
    }

    pub(crate) fn checked_next(self) -> Option<Self> {
        self.value.checked_add(1).map(|value| Self { value })
    }

    #[cfg(test)]
    pub(crate) fn next(self) -> Self {
        self.checked_next().expect("test epoch must not exhaust")
    }

    #[cfg(test)]
    pub(crate) fn for_test(value: u64) -> Self {
        Self { value }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiPendingActivation {
    frame_epoch: WorthUiRuntimeFrameEpoch,
    candidate_application_authority:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority,
    allocation_planning_projection:
        crate::runtime::planning::allocation_planning::WorthUiAllocationPlanningProjection,
    staged_replacement: crate::runtime::WorthUiStagedReplacement,
    readiness: crate::runtime::WorthUiActivationReadiness,
    staging_report: crate::runtime::WorthUiActivationStagingReport,
}

pub(crate) struct WorthUiPendingActivationInput {
    pub(crate) frame_epoch: WorthUiRuntimeFrameEpoch,
    pub(crate) candidate_application_authority:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority,
    pub(crate) staged_replacement: crate::runtime::WorthUiStagedReplacement,
    pub(crate) readiness: crate::runtime::WorthUiActivationReadiness,
    pub(crate) staging_report: crate::runtime::WorthUiActivationStagingReport,
}

impl WorthUiPendingActivation {
    pub(crate) fn new(input: WorthUiPendingActivationInput) -> Self {
        let WorthUiPendingActivationInput {
            frame_epoch,
            candidate_application_authority,
            staged_replacement,
            readiness,
            staging_report,
        } = input;
        let allocation_planning_projection =
            crate::runtime::planning::allocation_planning::WorthUiAllocationPlanningProjection::seal(
                frame_epoch,
                staged_replacement.candidate_artifact_digest(),
                candidate_application_authority.graph_authority_identity(),
            );
        Self {
            frame_epoch,
            candidate_application_authority,
            allocation_planning_projection,
            staged_replacement,
            readiness,
            staging_report,
        }
    }

    pub fn frame_epoch(&self) -> WorthUiRuntimeFrameEpoch {
        self.frame_epoch
    }

    pub fn staged_replacement(&self) -> &crate::runtime::WorthUiStagedReplacement {
        &self.staged_replacement
    }

    pub fn readiness(&self) -> crate::runtime::WorthUiActivationReadiness {
        self.readiness
    }

    pub fn staging_report(&self) -> &crate::runtime::WorthUiActivationStagingReport {
        &self.staging_report
    }

    pub(crate) fn allocation_planning_projection(
        &self,
    ) -> &crate::runtime::planning::allocation_planning::WorthUiAllocationPlanningProjection {
        &self.allocation_planning_projection
    }

    pub(crate) fn candidate_application_authority(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority
    {
        &self.candidate_application_authority
    }
}

/// Receipt emitted when the runtime host is consumed during shutdown.
#[path = "lifecycle_state/service_shutdown.rs"]
mod service_shutdown;

#[derive(Debug)]
pub struct WorthUiRuntimeShutdownReceipt {
    final_frame_epoch: WorthUiRuntimeFrameEpoch,
    query_retirement: worth_ui_query_binding::WorthUiOperationLiveRetirement,
    service_proposals:
        crate::runtime::session::service_proposal::UiServiceProposalCompilerShutdownReceipt,
    mounted_presentation: crate::mounting::UiMountedPresentationShutdownReport,
    presentation_async_cleanup:
        Option<crate::native_platform::text_presentation::UiPresentationAsyncTerminalCleanup>,
    visual_capture: crate::inspection::visual_snapshot::UiVisualCaptureShutdownReport,
    visual_overlay: crate::inspection::visual_snapshot::UiVisualOverlayShutdownReport,
    interaction: crate::runtime::interaction::UiInteractionShutdownReport,
    focus_placement: crate::mounting::UiFocusHostPlacementShutdownReport,
    portal: crate::runtime::portal::UiPortalShutdownReport,
    motion: crate::runtime::motion::UiMotionShutdownReport,
    intent_confirmation: crate::runtime::intent::UiIntentConfirmationShutdownReport,
    intent_admission: crate::runtime::intent::UiIntentAdmissionShutdownReport,
    intent_execution: crate::facade::intent::UiIntentExecutionShutdownReport,
    observation_resources: crate::runtime::observation::UiObservationResourceRetirementReport,
    intent_evidence: worth_ui_inspection::UiIntentEvidenceRetirementReport,
    intent_resource_census: crate::runtime::session::UiIntentResourceCensus,
    rebind: crate::runtime::rebind::UiRebindShutdownReport,
    host_session_release: Option<crate::host::adapter::UiHostSessionReleaseOutcome>,
    host_session_recovery: Option<crate::facade::WorthUiHostSessionReleaseRecovery>,
}

impl WorthUiRuntimeShutdownReceipt {
    pub(in crate::runtime) fn new(
        final_frame_epoch: WorthUiRuntimeFrameEpoch,
        _queue_disposition: crate::runtime::UiAllocationFrameQueueDisposition,
        query_retirement: worth_ui_query_binding::WorthUiOperationLiveRetirement,
        service_proposals:
            crate::runtime::session::service_proposal::UiServiceProposalCompilerShutdownReceipt,
    ) -> Self {
        Self {
            final_frame_epoch,
            query_retirement,
            service_proposals,
            mounted_presentation: Default::default(),
            presentation_async_cleanup: None,
            visual_capture: Default::default(),
            visual_overlay: Default::default(),
            interaction: Default::default(),
            focus_placement: Default::default(),
            portal: Default::default(),
            motion: Default::default(),
            intent_confirmation: Default::default(),
            intent_admission: Default::default(),
            intent_execution: Default::default(),
            observation_resources: Default::default(),
            intent_evidence: Default::default(),
            intent_resource_census: crate::runtime::session::UiIntentResourceCensus::EMPTY,
            rebind: Default::default(),
            host_session_release: None,
            host_session_recovery: None,
        }
    }

    pub fn final_frame_epoch(&self) -> WorthUiRuntimeFrameEpoch {
        self.final_frame_epoch
    }

    pub fn operation_live_retirement(
        &self,
    ) -> &worth_ui_query_binding::WorthUiOperationLiveRetirement {
        &self.query_retirement
    }

    pub fn mounted_presentation(&self) -> &crate::mounting::UiMountedPresentationShutdownReport {
        &self.mounted_presentation
    }

    pub(crate) fn take_presentation_async_cleanup(
        &mut self,
    ) -> Option<crate::native_platform::text_presentation::UiPresentationAsyncTerminalCleanup> {
        self.presentation_async_cleanup.take()
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

    pub const fn rebind(&self) -> crate::runtime::rebind::UiRebindShutdownReport {
        self.rebind
    }

    pub const fn interaction(&self) -> &crate::runtime::interaction::UiInteractionShutdownReport {
        &self.interaction
    }

    pub const fn intent_confirmation(
        &self,
    ) -> crate::runtime::intent::UiIntentConfirmationShutdownReport {
        self.intent_confirmation
    }

    pub const fn intent_admission(
        &self,
    ) -> crate::runtime::intent::UiIntentAdmissionShutdownReport {
        self.intent_admission
    }

    pub const fn intent_execution(&self) -> crate::facade::intent::UiIntentExecutionShutdownReport {
        self.intent_execution
    }

    pub const fn intent_evidence(&self) -> worth_ui_inspection::UiIntentEvidenceRetirementReport {
        self.intent_evidence
    }

    pub const fn observation_resources(
        &self,
    ) -> crate::runtime::observation::UiObservationResourceRetirementReport {
        self.observation_resources
    }

    pub const fn intent_resource_census(&self) -> crate::runtime::session::UiIntentResourceCensus {
        self.intent_resource_census
    }

    pub fn host_session_release(
        &self,
    ) -> Option<crate::host::adapter::UiHostSessionReleaseOutcome> {
        self.host_session_release
    }

    pub(crate) fn take_host_session_recovery(
        &mut self,
    ) -> Option<crate::facade::WorthUiHostSessionReleaseRecovery> {
        self.host_session_recovery.take()
    }

    pub(crate) fn bind_mounted_presentation(
        mut self,
        report: crate::mounting::UiMountedPresentationShutdownReport,
    ) -> Self {
        self.mounted_presentation = report;
        self
    }

    pub(crate) fn bind_presentation_async_cleanup(
        mut self,
        cleanup: Option<
            crate::native_platform::text_presentation::UiPresentationAsyncTerminalCleanup,
        >,
    ) -> Self {
        self.presentation_async_cleanup = cleanup;
        self
    }

    pub(crate) fn bind_visual_capture(
        mut self,
        report: crate::inspection::visual_snapshot::UiVisualCaptureShutdownReport,
    ) -> Self {
        self.visual_capture = report;
        self
    }

    pub(crate) fn bind_visual_overlay(
        mut self,
        report: crate::inspection::visual_snapshot::UiVisualOverlayShutdownReport,
    ) -> Self {
        self.visual_overlay = report;
        self
    }

    pub(crate) fn bind_host_session_release(
        mut self,
        outcome: crate::host::adapter::UiHostSessionReleaseOutcome,
    ) -> Self {
        self.host_session_release = Some(outcome);
        self
    }

    pub(crate) fn bind_host_session_recovery(
        mut self,
        recovery: Option<crate::facade::WorthUiHostSessionReleaseRecovery>,
    ) -> Self {
        self.host_session_recovery = recovery;
        self
    }

    pub(crate) fn bind_interaction(
        mut self,
        report: crate::runtime::interaction::UiInteractionShutdownReport,
    ) -> Self {
        self.interaction = report;
        self
    }

    pub(crate) fn bind_intent_confirmation(
        mut self,
        report: crate::runtime::intent::UiIntentConfirmationShutdownReport,
    ) -> Self {
        self.intent_confirmation = report;
        self
    }

    pub(crate) fn bind_intent_admission(
        mut self,
        report: crate::runtime::intent::UiIntentAdmissionShutdownReport,
    ) -> Self {
        self.intent_admission = report;
        self
    }

    pub(crate) fn bind_intent_execution(
        mut self,
        report: crate::facade::intent::UiIntentExecutionShutdownReport,
    ) -> Self {
        self.intent_execution = report;
        self
    }

    pub(crate) fn bind_intent_evidence(
        mut self,
        report: worth_ui_inspection::UiIntentEvidenceRetirementReport,
    ) -> Self {
        self.intent_evidence = report;
        self
    }

    pub(crate) fn bind_observation_resources(
        mut self,
        report: crate::runtime::observation::UiObservationResourceRetirementReport,
    ) -> Self {
        self.observation_resources = report;
        self
    }

    pub(crate) fn bind_intent_resource_census(
        mut self,
        census: crate::runtime::session::UiIntentResourceCensus,
    ) -> Self {
        self.intent_resource_census = census;
        self
    }

    pub fn into_operation_live_retirement(
        self,
    ) -> worth_ui_query_binding::WorthUiOperationLiveRetirement {
        self.query_retirement
    }
}
