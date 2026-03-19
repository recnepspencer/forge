use crate::transactions::data::TransactionOptions;
use crate::transactions::logic::RelationalTransaction;

use std::collections::BTreeMap;

use super::RelationalRuntime;
use crate::logic::runtime::{
    AspectSemanticsSubsystem, DurabilitySubsystem, HistorySubsystem, IndexingSubsystem,
    LineageSubsystem, PublicationSubsystem, RuntimeServices, RuntimeSubsystem, VisibilitySubsystem,
};

impl RelationalRuntime {
    pub fn new(config: super::RelationalRuntimeConfig) -> Self {
        Self {
            aspect_semantics: <AspectSemanticsSubsystem as RuntimeSubsystem>::new(&config),
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
