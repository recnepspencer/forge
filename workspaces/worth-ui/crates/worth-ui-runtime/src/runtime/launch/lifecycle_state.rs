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
#[derive(Debug)]
pub struct WorthUiRuntimeShutdownReceipt {
    final_frame_epoch: WorthUiRuntimeFrameEpoch,
    query_retirement: worth_ui_query_binding::WorthUiOperationLiveRetirement,
    mounted_presentation: crate::mounting::UiMountedPresentationShutdownReport,
    visual_capture: crate::inspection::visual_snapshot::UiVisualCaptureShutdownReport,
    visual_overlay: crate::inspection::visual_snapshot::UiVisualOverlayShutdownReport,
    host_session_release: Option<crate::host::adapter::UiHostSessionReleaseOutcome>,
}

impl WorthUiRuntimeShutdownReceipt {
    pub(crate) fn new(
        final_frame_epoch: WorthUiRuntimeFrameEpoch,
        _queue_disposition: crate::runtime::UiAllocationFrameQueueDisposition,
        query_retirement: worth_ui_query_binding::WorthUiOperationLiveRetirement,
    ) -> Self {
        Self {
            final_frame_epoch,
            query_retirement,
            mounted_presentation: Default::default(),
            visual_capture: Default::default(),
            visual_overlay: Default::default(),
            host_session_release: None,
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

    pub fn host_session_release(
        &self,
    ) -> Option<crate::host::adapter::UiHostSessionReleaseOutcome> {
        self.host_session_release
    }

    pub(crate) fn bind_mounted_presentation(
        mut self,
        report: crate::mounting::UiMountedPresentationShutdownReport,
    ) -> Self {
        self.mounted_presentation = report;
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

    pub fn into_operation_live_retirement(
        self,
    ) -> worth_ui_query_binding::WorthUiOperationLiveRetirement {
        self.query_retirement
    }
}
