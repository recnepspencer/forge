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
        crate::runtime::allocation_planning::WorthUiAllocationPlanningProjection,
    staged_replacement: crate::runtime::WorthUiStagedReplacement,
    readiness: crate::runtime::WorthUiActivationReadiness,
    staging_report: crate::runtime::WorthUiActivationStagingReport,
}

impl WorthUiPendingActivation {
    pub(crate) fn new(
        frame_epoch: WorthUiRuntimeFrameEpoch,
        candidate_application_authority: crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority,
        staged_replacement: crate::runtime::WorthUiStagedReplacement,
        readiness: crate::runtime::WorthUiActivationReadiness,
        staging_report: crate::runtime::WorthUiActivationStagingReport,
    ) -> Self {
        let allocation_planning_projection =
            crate::runtime::allocation_planning::WorthUiAllocationPlanningProjection::seal(
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
    ) -> &crate::runtime::allocation_planning::WorthUiAllocationPlanningProjection {
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
#[derive(Debug, PartialEq)]
pub struct WorthUiRuntimeShutdownReceipt {
    final_frame_epoch: WorthUiRuntimeFrameEpoch,
}

impl WorthUiRuntimeShutdownReceipt {
    pub(crate) fn new(
        final_frame_epoch: WorthUiRuntimeFrameEpoch,
        _queue_disposition: crate::runtime::UiAllocationFrameQueueDisposition,
    ) -> Self {
        Self { final_frame_epoch }
    }

    pub fn final_frame_epoch(&self) -> WorthUiRuntimeFrameEpoch {
        self.final_frame_epoch
    }
}
