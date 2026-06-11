use crate::workload_platform::vocabulary::{
    SpatialWorkloadStage, WorkloadStagePosture, WorkloadStageSupport,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnsupportedTransformReasonCode {
    MissingDeclaration,
    MissingTransformSequence,
    LabelOnlyMotionEvidence,
    InvalidCancellationStepCount,
    TransformStageReceiptDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnsupportedTransformWorkload {
    reason_code: UnsupportedTransformReasonCode,
    human_reason: String,
    posture: WorkloadStagePosture,
}

impl UnsupportedTransformWorkload {
    pub(crate) fn new(
        reason_code: UnsupportedTransformReasonCode,
        human_reason: impl Into<String>,
    ) -> Self {
        let human_reason = normalize_reason(human_reason);
        Self {
            reason_code,
            posture: WorkloadStagePosture::new(
                SpatialWorkloadStage::Transform,
                WorkloadStageSupport::Unsupported,
                human_reason.clone(),
            ),
            human_reason,
        }
    }

    pub fn reason_code(&self) -> UnsupportedTransformReasonCode {
        self.reason_code
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }

    pub fn posture(&self) -> &WorkloadStagePosture {
        &self.posture
    }

    pub fn can_enter_transform_posture_consumption(&self) -> bool {
        false
    }

    pub fn can_enter_operator_execution(&self) -> bool {
        false
    }
}

fn normalize_reason(reason: impl Into<String>) -> String {
    let reason = reason.into();
    if reason.trim().is_empty() {
        "Transform workload was denied before transformed evidence could be built.".to_string()
    } else {
        reason
    }
}
