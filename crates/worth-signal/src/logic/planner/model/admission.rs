use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParallelAdmissionReason {
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

impl ParallelAdmissionReason {
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
            Self::AdmittedOperational => "admitted-operational",
            Self::AdmittedDevelopment => "admitted-development",
            Self::AdmittedForensic => "admitted-forensic",
            Self::AdmittedProofSafeGroupedConcurrent => "admitted-proof-safe-grouped-concurrent",
        }
    }

    pub fn message(self) -> &'static str {
        match self {
            Self::SerialExecutor => "parallelism was not requested for this stage",
            Self::BelowMinStageWidth => {
                "stage stayed serial because it did not meet the executor's minimum stage width"
            }
            Self::BelowPolicyWorkThreshold => {
                "stage stayed serial because the active runtime policy estimated the work was too small to amortize parallel overhead"
            }
            Self::ValidationHeavyStage => {
                "stage stayed serial because it was validation-heavy and unlikely to benefit from parallel overhead"
            }
            Self::BelowFullParallelThreshold => {
                "stage stayed out of full parallel mode because the active policy requires a larger stage for grouped concurrent apply"
            }
            Self::FullParallelUnsupportedByMutableEngine => {
                "stage stayed out of full parallel mode because the current mutable graph engine does not support concurrent apply yet"
            }
            Self::AdmittedOperational => {
                "stage ran in parallel under the low-overhead operational policy"
            }
            Self::AdmittedDevelopment => "stage ran in parallel under the development policy",
            Self::AdmittedForensic => "stage ran in parallel under the forensic policy",
            Self::AdmittedProofSafeGroupedConcurrent => {
                "stage ran through proof-safe grouped concurrent apply with deterministic reduction-only publication"
            }
        }
    }
}
