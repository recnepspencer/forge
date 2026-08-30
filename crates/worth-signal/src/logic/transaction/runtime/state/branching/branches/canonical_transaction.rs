use crate::clock::RuntimeInstant;
use crate::data::error::SignalError;
use crate::logic::events::EventBus;
use crate::logic::transaction::runtime::transaction::{
    SignalTransaction, TransactionCommitPosture, TransactionExecutionState, TransactionResult,
    TransactionRollbackPacketSet, TransactionScratch,
};
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};

use super::super::super::runtime_observation::RuntimeObservationRegistry;
use super::authority::BranchState;

impl<D, I, T> BranchState<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    /// Runs the canonical synchronous transaction engine against one owned
    /// branch state. Owner-service issuance permits this path only when the
    /// runtime has no callback or observation configuration, so the execution
    /// scope owns fresh empty callback registries and cannot acquire an
    /// owner-wide callback lock.
    pub(crate) fn execute_canonical_transaction<E, Ctx, F>(
        &mut self,
        branch_head_generation: &mut u64,
        branch_restore_snapshot_id: &mut Option<crate::state::SignalSnapshotId>,
        runtime_ctx: &mut Ctx,
        apply: F,
    ) -> Result<TransactionResult, SignalError>
    where
        F: FnOnce(&mut SignalTransaction<'_, D, I, E, Ctx, T>) -> Result<(), SignalError>,
    {
        let captures_telemetry = self.authority.graph.captures_observation_surface(
            crate::logic::transaction::SignalObservationSurface::OptionalTelemetry,
        );
        if captures_telemetry {
            self.derived.telemetry.transaction.transaction_begin_count += 1;
        }
        self.authority
            .config
            .sync_graph_capacity(&self.authority.graph);

        let mut event_bus = EventBus::new();
        event_bus.set_telemetry_capture(captures_telemetry);
        let observations = RuntimeObservationRegistry::default();
        let mut transaction = SignalTransaction {
            runtime_ctx,
            observations: &observations,
            config: &mut self.authority.config,
            graph: &mut self.authority.graph,
            checkpoint: &mut self.derived.checkpoint,
            event_bus: &mut event_bus,
            resource: &mut self.derived.resource,
            temporal: &mut self.derived.temporal,
            telemetry: captures_telemetry.then_some(&mut self.derived.telemetry),
            branch_mutation_ledger: &mut self.mutation_ledger,
            branch_head_generation,
            branch_restore_snapshot_id,
            scratch: TransactionScratch::new(),
            rollback_packets: TransactionRollbackPacketSet::default(),
            poisoned: false,
            finished: false,
            execution_state: TransactionExecutionState::default(),
            started_at: RuntimeInstant::now(),
            commit_posture: TransactionCommitPosture::Visible,
        };
        match catch_unwind(AssertUnwindSafe(|| apply(&mut transaction))) {
            Ok(Ok(())) => transaction.commit(),
            Ok(Err(error)) => match transaction.rollback() {
                Ok(_) => Err(error),
                Err(rollback_error) => Err(rollback_error),
            },
            Err(payload) => {
                let rollback = transaction.rollback();
                if let Err(rollback_error) = rollback {
                    panic!(
                        "Signal canonical transaction rollback failed while containing a caller panic: {rollback_error}"
                    );
                }
                resume_unwind(payload)
            }
        }
    }
}
