use crate::symbols::data::StringInterner;
use crate::transactions::data::TransactionOptions;
use crate::transactions::logic::RelationalTransaction;

use std::collections::BTreeMap;

use super::{RelationalRuntime, RuntimeInstrumentation, RuntimeSequenceState, SimulationState, SnapshotRegistry};
use super::{DurabilityState, HistoryState, IndexState, LineageState, PublicationState};

impl RelationalRuntime {
    pub fn new(config: super::RelationalRuntimeConfig) -> Self {
        Self {
            history: HistoryState::new(config.main_branch.clone()),
            indexes: IndexState::new(),
            lineage: LineageState::new(),
            durability: DurabilityState::new(&config),
            sequence: RuntimeSequenceState::new(),
            instrumentation: RuntimeInstrumentation::new(),
            simulation: SimulationState::new(),
            partitions: BTreeMap::new(),
            snapshots: SnapshotRegistry::new(&config),
            publication: PublicationState::default(),
            symbols: StringInterner::default(),
            config,
        }
    }

    pub fn fork(&self) -> Self {
        Self {
            config: self.config.clone(),
            partitions: self.partitions.clone(),
            snapshots: self.snapshots.fork(),
            publication: self.publication.clone(),
            history: self.history.clone(),
            indexes: self.indexes.clone(),
            lineage: self.lineage.clone(),
            durability: self.durability.clone(),
            sequence: self.sequence.clone(),
            symbols: self.symbols.clone(),
            instrumentation: self.instrumentation.fork(),
            simulation: self.simulation.clone(),
        }
    }

    pub fn begin_transaction<'a>(
        &'a mut self,
        options: TransactionOptions,
    ) -> RelationalTransaction<'a> {
        let transaction_id =
            crate::transactions::data::TransactionId(self.sequence.next_transaction_id);
        self.sequence.next_transaction_id += 1;
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
