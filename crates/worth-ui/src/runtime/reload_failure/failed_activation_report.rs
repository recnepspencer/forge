use crate::runtime::{
    WorthUiReloadCheckedStopPosture, WorthUiReloadFailureCounters, WorthUiReloadFailureStage,
    WorthUiReloadPreservationReceipt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiFailedActivationReport {
    stage: WorthUiReloadFailureStage,
    checked_stop_posture: WorthUiReloadCheckedStopPosture,
    preserved_active_artifact_digest: u64,
    preserved_active_plan_digest: u64,
    fallback_runtime_created: bool,
    counters: WorthUiReloadFailureCounters,
}

impl WorthUiFailedActivationReport {
    pub(crate) fn new(
        stage: WorthUiReloadFailureStage,
        checked_stop_posture: WorthUiReloadCheckedStopPosture,
        preservation_receipt: WorthUiReloadPreservationReceipt,
        counters: WorthUiReloadFailureCounters,
    ) -> Self {
        Self {
            stage,
            checked_stop_posture,
            preserved_active_artifact_digest: preservation_receipt.active_artifact_digest(),
            preserved_active_plan_digest: preservation_receipt.active_plan_digest(),
            fallback_runtime_created: counters.fallback_runtime_creation_count() > 0,
            counters,
        }
    }

    pub fn stage(self) -> WorthUiReloadFailureStage {
        self.stage
    }

    pub fn checked_stop_posture(self) -> WorthUiReloadCheckedStopPosture {
        self.checked_stop_posture
    }

    pub fn preserved_active_artifact_digest(self) -> u64 {
        self.preserved_active_artifact_digest
    }

    pub fn preserved_active_plan_digest(self) -> u64 {
        self.preserved_active_plan_digest
    }

    pub fn fallback_runtime_created(self) -> bool {
        self.fallback_runtime_created
    }

    pub fn counters(self) -> WorthUiReloadFailureCounters {
        self.counters
    }
}
