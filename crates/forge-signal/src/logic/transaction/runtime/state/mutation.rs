use crate::data::error::SignalError;
use std::time::Instant;

use super::super::computation::{ComputationSpec, DefinedComputation};
use super::super::transaction::{
    SignalTransaction, TransactionExecutionState, TransactionResult, TransactionScratch,
};
use super::runtime_state::SignalRuntime;

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

    pub fn begin<'a>(
        &'a mut self,
        runtime_ctx: &'a mut Ctx,
    ) -> SignalTransaction<'a, D, I, E, Ctx, T> {
        self.telemetry.transaction.transaction_begin_count += 1;
        self.config.sync_graph_capacity(&self.graph);
        SignalTransaction {
            runtime_ctx,
            config: &mut self.config,
            graph: &mut self.graph,
            checkpoint: &mut self.checkpoint,
            event_bus: &mut self.event_bus,
            telemetry: &mut self.telemetry,
            scratch: TransactionScratch::new(),
            rollback_baseline: super::super::transaction::TransactionRollbackBaseline::default(),
            poisoned: false,
            finished: false,
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
        let mut transaction = self.begin(runtime_ctx);
        match apply(&mut transaction) {
            Ok(()) => transaction.commit(),
            Err(err) => {
                let rollback_result = transaction.rollback();
                match rollback_result {
                    Ok(_) => Err(err),
                    Err(rollback_err) => Err(rollback_err),
                }
            }
        }
    }
}
