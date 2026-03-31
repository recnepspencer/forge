use crate::transactions::data::TransactionOptions;
use crate::transactions::logic::RelationalTransaction;

use std::collections::BTreeMap;

use super::RelationalRuntime;
use crate::commit_strategies::{
    data::CommitStrategyExecutionRegistration, FrozenCommitStrategyExecutorRegistry,
    FrozenCommitStrategyRegistry,
};
use crate::logic::runtime::{
    AspectSemanticsSubsystem, CommitStrategiesSubsystem, DurabilitySubsystem, HistorySubsystem,
    IndexingSubsystem, LineageSubsystem, PublicationSubsystem, RuntimeServices, RuntimeSubsystem,
    VisibilitySubsystem,
};
use crate::validation::data::CustomInvariantRegistration;
use crate::validation::logic::FrozenCustomInvariantRegistry;

impl RelationalRuntime {
    pub fn new(config: super::RelationalRuntimeConfig) -> Self {
        Self::new_with_extensions(config, Vec::new(), Vec::new())
    }

    pub fn new_with_custom_invariants(
        config: super::RelationalRuntimeConfig,
        custom_invariants: Vec<CustomInvariantRegistration>,
    ) -> Self {
        Self::new_with_extensions(config, custom_invariants, Vec::new())
    }

    pub fn new_with_extensions(
        config: super::RelationalRuntimeConfig,
        custom_invariants: Vec<CustomInvariantRegistration>,
        commit_strategy_executors: Vec<CommitStrategyExecutionRegistration>,
    ) -> Self {
        let mut aspect_semantics = <AspectSemanticsSubsystem as RuntimeSubsystem>::new(&config);
        aspect_semantics.custom_invariant_registries =
            FrozenCustomInvariantRegistry::from_registrations(custom_invariants).expect(
                "custom invariant registrations must have unique semantic identities at runtime construction",
            );
        let mut commit_strategy_subsystem =
            <CommitStrategiesSubsystem as RuntimeSubsystem>::new(&());
        commit_strategy_subsystem.registry =
            FrozenCommitStrategyRegistry::from_registrations(
                config.commit_strategies.registrations.clone(),
            )
            .expect(
                "commit strategy registrations must have unique runtime identities and persistent names at runtime construction",
            );
        commit_strategy_subsystem.executors = FrozenCommitStrategyExecutorRegistry::from_registrations(
            commit_strategy_executors,
            &commit_strategy_subsystem.registry,
        )
        .expect(
            "commit strategy executors must bind to registered strategy descriptors without digest drift at runtime construction",
        );
        Self {
            aspect_semantics,
            commit_strategies: commit_strategy_subsystem,
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
            aspect_semantics: RuntimeSubsystem::fork(&self.aspect_semantics),
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

    pub fn set_execution_model(
        &mut self,
        execution_model: crate::logic::planning::RelationalExecutionModel,
    ) {
        self.config.execution.execution_model = execution_model;
    }

    pub fn begin_transaction<'a>(
        &'a mut self,
        options: TransactionOptions,
    ) -> RelationalTransaction<'a> {
        let transaction_id = self.services.next_transaction_id();
        RelationalTransaction {
            runtime: self,
            transaction_id,
            options,
            batches: Vec::new(),
            savepoints: Vec::new(),
            last_merged_plan: None,
        }
    }
}
