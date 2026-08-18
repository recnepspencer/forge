use super::RelationalRuntime;

impl RelationalRuntime {
    pub fn config(&self) -> &crate::runtime::RelationalRuntimeConfig {
        &self.config
    }

    pub fn commit_strategy_registry(
        &self,
    ) -> &crate::commit_strategies::FrozenCommitStrategyRegistry {
        &self.commit_strategies.registry
    }

    pub(crate) fn commit_strategy_executor_registry(
        &self,
    ) -> &crate::commit_strategies::FrozenCommitStrategyExecutorRegistry {
        &self.commit_strategies.executors
    }

    pub fn commit_strategies(
        &self,
    ) -> crate::commit_strategies::facade::CommitStrategiesFacade<'_> {
        crate::commit_strategies::facade::CommitStrategiesFacade::new(self)
    }

    pub fn commit_strategies_authority(
        &mut self,
    ) -> crate::commit_strategies::facade::CommitStrategiesAuthorityFacade<'_> {
        crate::commit_strategies::facade::CommitStrategiesAuthorityFacade::new(self)
    }

    pub fn phase4_reference_cost_counters(
        &self,
    ) -> crate::runtime::RelationalPhase4ReferenceCostCounters {
        self.history.phase4_costs()
    }
}
