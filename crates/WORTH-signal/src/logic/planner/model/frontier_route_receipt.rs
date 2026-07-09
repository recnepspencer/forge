use serde::{Deserialize, Serialize};

use super::{ParallelAdmissionReason, StageExecutionRecord};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrontierRouteSerialFallbackReason {
    SerialExecutor,
    BelowMinStageWidth,
    BelowPolicyWorkThreshold,
    ValidationHeavyStage,
    BelowFullParallelThreshold,
    FullParallelUnsupportedByMutableEngine,
}

impl FrontierRouteSerialFallbackReason {
    pub fn code(self) -> &'static str {
        match self {
            Self::SerialExecutor => "serial-executor",
            Self::BelowMinStageWidth => "below-min-stage-width",
            Self::BelowPolicyWorkThreshold => "below-policy-work-threshold",
            Self::ValidationHeavyStage => "validation-heavy-stage",
            Self::BelowFullParallelThreshold => "below-full-parallel-threshold",
            Self::FullParallelUnsupportedByMutableEngine => {
                "full-parallel-unsupported-by-mutable-engine"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrontierRouteEvidenceReason {
    SerialExecutor,
    BelowMinStageWidth,
    BelowPolicyWorkThreshold,
    ValidationHeavyStage,
    BelowFullParallelThreshold,
    FullParallelUnsupportedByMutableEngine,
    AdmittedOperational,
    AdmittedDevelopment,
    AdmittedForensic,
    AdmittedProofSafeGroupedConcurrent,
}

impl FrontierRouteEvidenceReason {
    pub fn is_parallel_admitted(self) -> bool {
        matches!(
            self,
            Self::AdmittedOperational
                | Self::AdmittedDevelopment
                | Self::AdmittedForensic
                | Self::AdmittedProofSafeGroupedConcurrent
        )
    }

    pub fn serial_fallback_reason(self) -> Option<FrontierRouteSerialFallbackReason> {
        match self {
            Self::SerialExecutor => Some(FrontierRouteSerialFallbackReason::SerialExecutor),
            Self::BelowMinStageWidth => Some(FrontierRouteSerialFallbackReason::BelowMinStageWidth),
            Self::BelowPolicyWorkThreshold => {
                Some(FrontierRouteSerialFallbackReason::BelowPolicyWorkThreshold)
            }
            Self::ValidationHeavyStage => {
                Some(FrontierRouteSerialFallbackReason::ValidationHeavyStage)
            }
            Self::BelowFullParallelThreshold => {
                Some(FrontierRouteSerialFallbackReason::BelowFullParallelThreshold)
            }
            Self::FullParallelUnsupportedByMutableEngine => {
                Some(FrontierRouteSerialFallbackReason::FullParallelUnsupportedByMutableEngine)
            }
            Self::AdmittedOperational
            | Self::AdmittedDevelopment
            | Self::AdmittedForensic
            | Self::AdmittedProofSafeGroupedConcurrent => None,
        }
    }
}

impl From<ParallelAdmissionReason> for FrontierRouteEvidenceReason {
    fn from(value: ParallelAdmissionReason) -> Self {
        match value {
            ParallelAdmissionReason::SerialExecutor => Self::SerialExecutor,
            ParallelAdmissionReason::BelowMinStageWidth => Self::BelowMinStageWidth,
            ParallelAdmissionReason::BelowPolicyWorkThreshold => Self::BelowPolicyWorkThreshold,
            ParallelAdmissionReason::ValidationHeavyStage => Self::ValidationHeavyStage,
            ParallelAdmissionReason::BelowFullParallelThreshold => Self::BelowFullParallelThreshold,
            ParallelAdmissionReason::FullParallelUnsupportedByMutableEngine => {
                Self::FullParallelUnsupportedByMutableEngine
            }
            ParallelAdmissionReason::AdmittedOperational => Self::AdmittedOperational,
            ParallelAdmissionReason::AdmittedDevelopment => Self::AdmittedDevelopment,
            ParallelAdmissionReason::AdmittedForensic => Self::AdmittedForensic,
            ParallelAdmissionReason::AdmittedProofSafeGroupedConcurrent => {
                Self::AdmittedProofSafeGroupedConcurrent
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrontierRouteEvidenceReceipt {
    reason: FrontierRouteEvidenceReason,
}

impl FrontierRouteEvidenceReceipt {
    pub fn from_reason(reason: FrontierRouteEvidenceReason) -> Self {
        Self { reason }
    }

    pub fn from_stage_execution_record(
        stage: &StageExecutionRecord,
    ) -> Result<Self, FrontierRouteEvidenceReceiptError> {
        let reason = stage
            .parallel_admission_reason
            .ok_or(FrontierRouteEvidenceReceiptError::MissingParallelAdmissionReason)?;
        Ok(Self::from_reason(reason.into()))
    }

    pub fn reason(&self) -> FrontierRouteEvidenceReason {
        self.reason
    }

    pub fn is_parallel_admitted(&self) -> bool {
        self.reason.is_parallel_admitted()
    }

    pub fn serial_fallback_reason(&self) -> Option<FrontierRouteSerialFallbackReason> {
        self.reason.serial_fallback_reason()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontierRouteEvidenceReceiptError {
    MissingParallelAdmissionReason,
}
