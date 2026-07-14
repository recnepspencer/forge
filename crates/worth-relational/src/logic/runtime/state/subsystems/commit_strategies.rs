use crate::commit_strategies::{
    FrozenCommitStrategyExecutorRegistry, FrozenCommitStrategyRegistry,
};
use crate::logic::runtime::state::subsystems::RuntimeSubsystem;

#[derive(Debug, Clone, Default)]
pub(crate) struct CommitStrategiesSubsystem {
    pub(crate) registry: FrozenCommitStrategyRegistry,
    pub(crate) executors: FrozenCommitStrategyExecutorRegistry,
}

impl RuntimeSubsystem for CommitStrategiesSubsystem {
    type Config = ();

    fn new(_: &Self::Config) -> Self {
        Self::default()
    }

    fn fork(&self) -> Self {
        self.clone()
    }
}
