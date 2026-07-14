use std::collections::BTreeMap;

use crate::commit_strategies::{
    data::CommitStrategyExecutionRegistration, FrozenCommitStrategyExecutorRegistry,
    FrozenCommitStrategyRegistry,
};
use crate::logic::runtime::{
    CommitStrategiesSubsystem, DurabilitySubsystem, HistorySubsystem, IndexingSubsystem,
    LineageSubsystem, PublicationSubsystem, RuntimeServices, RuntimeSubsystem,
    SchemaContractRuntimeSubsystem, VisibilitySubsystem,
};
use crate::validation::data::CustomInvariantRegistration;
use crate::validation::logic::FrozenCustomInvariantRegistry;

use super::RelationalRuntime;

#[derive(Debug, Default)]
pub(crate) struct RuntimeExtensions {
    custom_invariants: Vec<CustomInvariantRegistration>,
    commit_strategy_executors: Vec<CommitStrategyExecutionRegistration>,
}

impl RuntimeExtensions {
    pub(crate) fn new(
        custom_invariants: Vec<CustomInvariantRegistration>,
        commit_strategy_executors: Vec<CommitStrategyExecutionRegistration>,
    ) -> Self {
        Self {
            custom_invariants,
            commit_strategy_executors,
        }
    }

    fn build_schema_contract_runtime_subsystem(
        &self,
        config: &super::RelationalRuntimeConfig,
    ) -> SchemaContractRuntimeSubsystem {
        let mut schema_contract_runtime =
            <SchemaContractRuntimeSubsystem as RuntimeSubsystem>::new(config);
        schema_contract_runtime.custom_invariant_registries =
            FrozenCustomInvariantRegistry::from_registrations(self.custom_invariants.clone())
                .expect(
                    "custom invariant registrations must have unique semantic identities per execution point at runtime construction",
                );
        schema_contract_runtime
    }

    fn build_commit_strategy_subsystem(
        &self,
        config: &super::RelationalRuntimeConfig,
    ) -> CommitStrategiesSubsystem {
        let mut commit_strategy_subsystem =
            <CommitStrategiesSubsystem as RuntimeSubsystem>::new(&());
        commit_strategy_subsystem.registry = FrozenCommitStrategyRegistry::from_registrations(
            config.commit_strategies.registrations.clone(),
        )
        .expect(
            "commit strategy registrations must have unique runtime identities and persistent names at runtime construction",
        );
        commit_strategy_subsystem.executors =
            FrozenCommitStrategyExecutorRegistry::from_registrations(
                self.commit_strategy_executors.clone(),
                &commit_strategy_subsystem.registry,
            )
            .expect(
                "commit strategy executors must bind to registered strategy descriptors without digest drift at runtime construction",
            );
        commit_strategy_subsystem
    }
}

impl RelationalRuntime {
    pub fn new(config: super::RelationalRuntimeConfig) -> Self {
        Self::build_from_extensions(config, RuntimeExtensions::default())
    }

    pub fn new_with_custom_invariants(
        config: super::RelationalRuntimeConfig,
        custom_invariants: Vec<CustomInvariantRegistration>,
    ) -> Self {
        Self::build_from_extensions(
            config,
            RuntimeExtensions::new(custom_invariants, Vec::new()),
        )
    }

    pub fn new_with_extensions(
        config: super::RelationalRuntimeConfig,
        custom_invariants: Vec<CustomInvariantRegistration>,
        commit_strategy_executors: Vec<CommitStrategyExecutionRegistration>,
    ) -> Self {
        Self::build_from_extensions(
            config,
            RuntimeExtensions::new(custom_invariants, commit_strategy_executors),
        )
    }

    pub(crate) fn build_from_extensions(
        config: super::RelationalRuntimeConfig,
        extensions: RuntimeExtensions,
    ) -> Self {
        Self {
            schema_contract_runtime: extensions.build_schema_contract_runtime_subsystem(&config),
            commit_strategies: extensions.build_commit_strategy_subsystem(&config),
            history: <HistorySubsystem as RuntimeSubsystem>::new(&config.history.main_branch),
            indexes: <IndexingSubsystem as RuntimeSubsystem>::new(&()),
            lineage: <LineageSubsystem as RuntimeSubsystem>::new(&()),
            durability: <DurabilitySubsystem as RuntimeSubsystem>::new(&config),
            services: <RuntimeServices as RuntimeSubsystem>::new(&()),
            partitions: BTreeMap::new(),
            visibility: <VisibilitySubsystem as RuntimeSubsystem>::new(&config),
            publication: <PublicationSubsystem as RuntimeSubsystem>::new(&()),
            config,
        }
    }

    pub fn fork(&self) -> Self {
        Self {
            config: self.config.clone(),
            schema_contract_runtime: RuntimeSubsystem::fork(&self.schema_contract_runtime),
            commit_strategies: RuntimeSubsystem::fork(&self.commit_strategies),
            partitions: self.partitions.clone(),
            visibility: RuntimeSubsystem::fork(&self.visibility),
            publication: RuntimeSubsystem::fork(&self.publication),
            history: RuntimeSubsystem::fork(&self.history),
            indexes: RuntimeSubsystem::fork(&self.indexes),
            lineage: RuntimeSubsystem::fork(&self.lineage),
            durability: RuntimeSubsystem::fork(&self.durability),
            services: RuntimeSubsystem::fork(&self.services),
        }
    }
}
