use crate::workload_platform::vocabulary::{
    SpatialWorkloadStage, WorkloadStagePosture, WorkloadStageSupport,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnsupportedProjectionReasonCode {
    MissingDeclaration,
    MissingCertifiedSurfaceSupport,
    MissingLocalFrameBasis,
    ProjectionStageReceiptDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnsupportedProjectionWorkload {
    reason_code: UnsupportedProjectionReasonCode,
    human_reason: String,
    posture: WorkloadStagePosture,
}

impl UnsupportedProjectionWorkload {
    pub(crate) fn new(
        reason_code: UnsupportedProjectionReasonCode,
        human_reason: impl Into<String>,
    ) -> Self {
        let human_reason = normalize_reason(human_reason);
        Self {
            reason_code,
            posture: WorkloadStagePosture::new(
                SpatialWorkloadStage::Projection,
                WorkloadStageSupport::Unsupported,
                human_reason.clone(),
            ),
            human_reason,
        }
    }

    pub fn reason_code(&self) -> UnsupportedProjectionReasonCode {
        self.reason_code
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }

    pub fn posture(&self) -> &WorkloadStagePosture {
        &self.posture
    }

    pub fn can_enter_projection_consumed_planar_facts(&self) -> bool {
        false
    }

    pub fn can_enter_operator_execution(&self) -> bool {
        false
    }
}

fn normalize_reason(reason: impl Into<String>) -> String {
    let reason = reason.into();
    if reason.trim().is_empty() {
        "Projection workload was denied before a projected planar workload could be built."
            .to_string()
    } else {
        reason
    }
}
