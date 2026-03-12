use crate::data::error::SignalError;
use crate::logic::transaction::patch_buffer::SparsePatchBuffer;
use std::time::Instant;

use super::runtime_state::SignalRuntime;
use super::super::computation::{ComputationSpec, DefinedComputation};
use super::super::transaction::{
    SignalTransaction, TransactionExecutionState, TransactionResult, TransactionSemanticDelta,
};

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn define_computation<F>(
        &mut self,
        spec: ComputationSpec<T, F>,
    ) -> Result<DefinedComputation<T, F>, SignalError> {
        self.config.define_computation(
            spec.family.clone(),
            spec.contract.clone(),
            spec.tier,
            spec.comparator.clone(),
        )?;
        Ok(DefinedComputation::from_spec(spec))
    }

    pub fn begin<'a>(&'a mut self) -> SignalTransaction<'a, D, I, E, Ctx, T> {
        self.telemetry.transaction.transaction_begin_count += 1;
        self.config.sync_graph_capacity(&self.graph);
        let baseline_config = self.config.clone();
        let baseline_diagnostics_state = self.graph.diagnostics_state().clone();
        SignalTransaction {
            config: &mut self.config,
            graph: &mut self.graph,
            checkpoint: &mut self.checkpoint,
            event_bus: &mut self.event_bus,
            telemetry: &mut self.telemetry,
            staged_dirty: crate::data::dirty_set::BatchedDirtySet::new(),
            staged_checkpoint_flushes: 0,
            staged_checkpoint_flush_nanos: 0,
            staged_event_flush_nanos: 0,
            staged_event_operations: Vec::new(),
            staged_memo_writes: std::collections::BTreeMap::new(),
            graph_patches: SparsePatchBuffer::new(),
            created_nodes: Vec::new(),
            baseline_config,
            baseline_diagnostics_state,
            semantic_delta: TransactionSemanticDelta::default(),
            mark_dirty_seen: crate::data::bitset::DenseBitset::new(),
            mark_dirty_staged: crate::data::bitset::DenseBitset::new(),
            evaluate_seen: crate::data::bitset::DenseBitset::new(),
            dirty_targets: crate::data::bitset::DenseBitset::new(),
            poisoned: false,
            finished: false,
            staged_patch_count: 0,
            execution_state: TransactionExecutionState::default(),
            started_at: Instant::now(),
        }
    }

    pub fn transaction<F>(
        &mut self,
        runtime_ctx: &mut Ctx,
        apply: F,
    ) -> Result<TransactionResult, SignalError>
    where
        F: FnOnce(&mut SignalTransaction<'_, D, I, E, Ctx, T>) -> Result<(), SignalError>,
    {
        let mut transaction = self.begin();
        match apply(&mut transaction) {
            Ok(()) => transaction.commit(runtime_ctx),
            Err(err) => {
                let rollback_result = transaction.rollback(runtime_ctx);
                match rollback_result {
                    Ok(_) => Err(err),
                    Err(rollback_err) => Err(rollback_err),
                }
            }
        }
    }
}
