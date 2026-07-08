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

    #[cfg(test)]
    pub(crate) fn next(self) -> Self {
        Self {
            value: self.value + 1,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiPendingActivation {
    frame_epoch: WorthUiRuntimeFrameEpoch,
    staged_replacement: crate::runtime::WorthUiStagedReplacement,
    readiness: crate::runtime::WorthUiActivationReadiness,
    staging_report: crate::runtime::WorthUiActivationStagingReport,
}

impl WorthUiPendingActivation {
    pub(crate) fn new(
        frame_epoch: WorthUiRuntimeFrameEpoch,
        staged_replacement: crate::runtime::WorthUiStagedReplacement,
        readiness: crate::runtime::WorthUiActivationReadiness,
        staging_report: crate::runtime::WorthUiActivationStagingReport,
    ) -> Self {
        Self {
            frame_epoch,
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
}

/// Receipt emitted when the runtime host is consumed during shutdown.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiRuntimeShutdownReceipt {
    final_frame_epoch: WorthUiRuntimeFrameEpoch,
}

impl WorthUiRuntimeShutdownReceipt {
    pub(crate) fn new(final_frame_epoch: WorthUiRuntimeFrameEpoch) -> Self {
        Self { final_frame_epoch }
    }

    pub fn final_frame_epoch(&self) -> WorthUiRuntimeFrameEpoch {
        self.final_frame_epoch
    }
}
