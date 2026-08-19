use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ExecutionObjectiveProfile {
    LatencyBounded,
    Balanced,
    Throughput,
}

impl ExecutionObjectiveProfile {
    pub const fn token(self) -> &'static str {
        match self {
            Self::LatencyBounded => "latency-bounded",
            Self::Balanced => "balanced",
            Self::Throughput => "throughput",
        }
    }
}
