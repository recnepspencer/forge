mod stream;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::timeline::ExecutionPhase;

pub use stream::{
    DeterministicFeedStreamGenerator, FeedShiftRange, FeedStreamBatch, FeedStreamEventKind,
    FeedStreamProfile, FeedStreamSample, FeedVolatilityRegime,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct WorkBudget {
    pub max_operations: Option<u64>,
    pub max_duration_millis: Option<u64>,
    pub frame_budget_micros: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkloadProfile {
    pub name: String,
    pub phase: Option<ExecutionPhase>,
    pub budget: WorkBudget,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BudgetUsage {
    pub operations_processed: Option<u64>,
    pub duration_millis: Option<u64>,
    pub frame_time_micros: Option<u64>,
    pub budget_exhausted: bool,
}

impl WorkloadProfile {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: None,
            budget: WorkBudget::default(),
            metadata: BTreeMap::new(),
        }
    }

    pub fn with_phase(mut self, phase: ExecutionPhase) -> Self {
        self.phase = Some(phase);
        self
    }

    pub fn with_budget(mut self, budget: WorkBudget) -> Self {
        self.budget = budget;
        self
    }
}
