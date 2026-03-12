use serde::{Deserialize, Serialize};

use crate::logic::planner::StageExecutor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParallelismHint {
    Serial,
    Preferred,
}

impl ParallelismHint {
    pub fn stage_executor(self) -> StageExecutor {
        match self {
            Self::Serial => StageExecutor::Serial,
            Self::Preferred => {
                #[cfg(feature = "parallel")]
                {
                    StageExecutor::parallel(1)
                }
                #[cfg(not(feature = "parallel"))]
                {
                    StageExecutor::Serial
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GcPressure {
    Deferred,
    CompactAfterEvaluation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObservationLevel {
    Minimal,
    Full,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluationStrategy {
    pub parallelism: ParallelismHint,
    pub gc_pressure: GcPressure,
    pub observation_level: ObservationLevel,
}
