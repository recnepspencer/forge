use crate::workload_platform::vocabulary::{
    SpatialWorkloadStage, WorkloadStagePosture, WorkloadStageSupport,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnsupportedReplayReasonCode {
    MissingDeclaration,
    MissingRetainedArtifacts,
    MissingProjectionConsumedFacts,
    RetainedProjectionDrift,
    RetainedHistoricalReplayDenied,
    RetainedReplayStageReceiptDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnsupportedReplayWorkload {
    reason_code: UnsupportedReplayReasonCode,
    human_reason: String,
    posture: WorkloadStagePosture,
}

impl UnsupportedReplayWorkload {
    pub(crate) fn new(
        reason_code: UnsupportedReplayReasonCode,
        human_reason: impl Into<String>,
    ) -> Self {
        let human_reason = human_reason.into();
        Self {
            reason_code,
            posture: WorkloadStagePosture::unsupported(
                SpatialWorkloadStage::RetainedReplay,
                human_reason.clone(),
            ),
            human_reason,
        }
    }

    pub fn reason_code(&self) -> UnsupportedReplayReasonCode {
        self.reason_code
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }

    pub fn posture(&self) -> &WorkloadStagePosture {
        &self.posture
    }

    pub fn can_enter_diagnostics_workload(&self) -> bool {
        false
    }

    pub fn can_enter_operator_execution(&self) -> bool {
        false
    }

    pub fn support(&self) -> WorkloadStageSupport {
        self.posture.support()
    }
}
