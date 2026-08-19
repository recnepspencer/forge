use super::definition::SignalRuntimePolicy;
use worth_foundational::ExecutionObjectiveProfile;

impl SignalRuntimePolicy {
    pub const fn execution_objective(self) -> ExecutionObjectiveProfile {
        self.execution_objective
    }

    pub fn with_execution_objective(mut self, objective: ExecutionObjectiveProfile) -> Self {
        self.execution_objective = objective;
        self
    }

    pub(crate) fn default_execution_strategy(
        self,
    ) -> crate::logic::planner::ResolvedExecutionStrategy {
        match self.execution_objective {
            ExecutionObjectiveProfile::Throughput => {
                crate::logic::planner::ResolvedExecutionStrategy::SparseIncremental
            }
            ExecutionObjectiveProfile::Balanced => {
                crate::logic::planner::ResolvedExecutionStrategy::DenseStageBatched
            }
            ExecutionObjectiveProfile::LatencyBounded => {
                crate::logic::planner::ResolvedExecutionStrategy::SparseIncremental
            }
        }
    }

    pub(crate) fn default_maintenance_strategy(
        self,
    ) -> crate::logic::planner::ResolvedMaintenanceStrategy {
        match self.execution_objective {
            ExecutionObjectiveProfile::Throughput => {
                crate::logic::planner::ResolvedMaintenanceStrategy::DensityAdaptive
            }
            ExecutionObjectiveProfile::Balanced | ExecutionObjectiveProfile::LatencyBounded => {
                crate::logic::planner::ResolvedMaintenanceStrategy::Incremental
            }
        }
    }
}
