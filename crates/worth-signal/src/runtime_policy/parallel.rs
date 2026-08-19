use serde::{Deserialize, Serialize};

use worth_foundational::ExecutionObjectiveProfile;

/// Runtime scheduling admission thresholds owned by the runtime-policy compiler.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParallelAdmissionPolicy {
    pub throughput_min_parallel_tasks: usize,
    pub balanced_min_parallel_tasks: usize,
    pub latency_bounded_min_parallel_tasks: usize,
    pub full_parallel_min_tasks: usize,
}

impl Default for ParallelAdmissionPolicy {
    fn default() -> Self {
        Self {
            throughput_min_parallel_tasks: 2,
            balanced_min_parallel_tasks: 4,
            latency_bounded_min_parallel_tasks: 8,
            full_parallel_min_tasks: 8,
        }
    }
}

impl ParallelAdmissionPolicy {
    pub(crate) fn min_parallel_tasks_for_objective(
        self,
        objective: ExecutionObjectiveProfile,
    ) -> usize {
        match objective {
            ExecutionObjectiveProfile::Throughput => self.throughput_min_parallel_tasks,
            ExecutionObjectiveProfile::Balanced => self.balanced_min_parallel_tasks,
            ExecutionObjectiveProfile::LatencyBounded => self.latency_bounded_min_parallel_tasks,
        }
    }
}
