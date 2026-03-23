use std::ops::{Deref, DerefMut};

use crate::data::graph::{EvaluationStrategy, SignalGraph};
use crate::data::telemetry::{RuntimeTelemetry, TransactionTelemetry};
use crate::logic::checkpoint::CheckpointRuntime;
use crate::logic::events::EventBus;
use crate::state::{SignalBranchHandle, SignalBranchId};

use super::super::config::SignalRuntimeConfig;
use super::branching::{BranchAncestryState, BranchManager, BranchState};
use super::merge::BranchMutationLedger;
use super::builder::SignalRuntimeBuilder;
use super::observer::RuntimeObserver;
use super::reconstructability::{AuthorityState, DerivedState};

#[derive(Debug)]
pub(in crate::logic::transaction::runtime) struct HeavyCaptureWitness(());

#[derive(Debug)]
pub(in crate::logic::transaction::runtime) struct AuthorityTransferPacket<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub branch_id: SignalBranchId,
    pub state: BranchState<D, I, T>,
}

#[derive(Debug)]
pub(in crate::logic::transaction::runtime) struct RestoreTransferPacket<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub branch_id: SignalBranchId,
    pub state: BranchState<D, I, T>,
}

#[derive(Debug)]
pub(in crate::logic::transaction::runtime) struct ExplicitBranchForkPacket<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub source_branch: SignalBranchId,
    pub branch_id: SignalBranchId,
    pub state: BranchState<D, I, T>,
}

#[derive(Debug)]
pub(in crate::logic::transaction::runtime) enum BranchLifecycleTransfer<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    Move(AuthorityTransferPacket<D, I, T>),
    Restore(RestoreTransferPacket<D, I, T>),
}

/// Full runtime surface for transactional evaluation, diagnostics, replay, and
/// keyed or tier-aware execution.
pub struct SignalRuntime<D, I, E, Ctx, T = ()>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(in crate::logic::transaction::runtime) config: SignalRuntimeConfig<T>,
    pub(in crate::logic::transaction::runtime) graph: SignalGraph,
    pub(in crate::logic::transaction::runtime) checkpoint: CheckpointRuntime<D, I>,
    pub(in crate::logic::transaction::runtime) event_bus: EventBus<E, D, Ctx>,
    pub(in crate::logic::transaction::runtime) telemetry: RuntimeTelemetry,
    pub(in crate::logic::transaction::runtime) branches: BranchManager<D, I, T>,
}

pub struct SignalGraphMut<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    runtime: &'a mut SignalRuntime<D, I, E, Ctx, T>,
}

impl<D, I, E, Ctx, T> Deref for SignalGraphMut<'_, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    type Target = SignalGraph;

    fn deref(&self) -> &Self::Target {
        &self.runtime.graph
    }
}

impl<D, I, E, Ctx, T> DerefMut for SignalGraphMut<'_, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.runtime.graph
    }
}

impl<D, I, E, Ctx, T> Drop for SignalGraphMut<'_, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn drop(&mut self) {
        self.runtime
            .config
            .prune_stale_node_meta(&self.runtime.graph);
    }
}

impl SignalRuntime<(), (), (), (), ()> {
    /// Create a runtime builder from a graph.
    ///
    /// This is the recommended entrypoint for most applications.
    pub fn builder(
        graph: SignalGraph,
    ) -> SignalRuntimeBuilder<super::builder::Missing, super::builder::Missing, (), (), (), (), ()>
    {
        SignalRuntimeBuilder::new(graph)
    }
}

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(in crate::logic::transaction::runtime::state) fn merge_global_transaction_telemetry(
        current: TransactionTelemetry,
        restored: &mut TransactionTelemetry,
    ) {
        restored.transaction_begin_count = restored
            .transaction_begin_count
            .max(current.transaction_begin_count);
        restored.transaction_commit_count = restored
            .transaction_commit_count
            .max(current.transaction_commit_count);
        restored.transaction_rollback_count = restored
            .transaction_rollback_count
            .max(current.transaction_rollback_count);
        restored.transaction_poison_count = restored
            .transaction_poison_count
            .max(current.transaction_poison_count);
        restored.rollback_packet_breadth = restored
            .rollback_packet_breadth
            .max(current.rollback_packet_breadth);
        restored.rollback_packet_config_count = restored
            .rollback_packet_config_count
            .max(current.rollback_packet_config_count);
        restored.rollback_packet_diagnostics_count = restored
            .rollback_packet_diagnostics_count
            .max(current.rollback_packet_diagnostics_count);
        restored.rollback_packet_graph_patch_count = restored
            .rollback_packet_graph_patch_count
            .max(current.rollback_packet_graph_patch_count);
        restored.rollback_packet_created_node_count = restored
            .rollback_packet_created_node_count
            .max(current.rollback_packet_created_node_count);
        restored.rollback_packet_subscriber_repair_count = restored
            .rollback_packet_subscriber_repair_count
            .max(current.rollback_packet_subscriber_repair_count);
        restored.move_transfer_count =
            restored.move_transfer_count.max(current.move_transfer_count);
        restored.explicit_fork_count =
            restored.explicit_fork_count.max(current.explicit_fork_count);
        restored.restore_transfer_count = restored
            .restore_transfer_count
            .max(current.restore_transfer_count);
        restored.heavy_capture_count =
            restored.heavy_capture_count.max(current.heavy_capture_count);
        restored.decision_log_event_count = restored
            .decision_log_event_count
            .max(current.decision_log_event_count);
        restored.staged_node_patch_count = restored
            .staged_node_patch_count
            .max(current.staged_node_patch_count);
        restored.max_touched_nodes_in_txn = restored
            .max_touched_nodes_in_txn
            .max(current.max_touched_nodes_in_txn);
        restored.transaction_mark_dirty_candidate_visits = restored
            .transaction_mark_dirty_candidate_visits
            .max(current.transaction_mark_dirty_candidate_visits);
    }

    pub(crate) fn new(
        graph: SignalGraph,
        checkpoint: CheckpointRuntime<D, I>,
        event_bus: EventBus<E, D, Ctx>,
    ) -> Self {
        let mut config = SignalRuntimeConfig::default();
        config.sync_graph_capacity(&graph);
        Self {
            config,
            graph,
            checkpoint,
            event_bus,
            telemetry: RuntimeTelemetry::default(),
            branches: BranchManager::<D, I, T>::new(),
        }
    }

    pub fn config(&self) -> &SignalRuntimeConfig<T> {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut SignalRuntimeConfig<T> {
        self.config.sync_graph_capacity(&self.graph);
        &mut self.config
    }

    pub fn graph(&self) -> &SignalGraph {
        &self.graph
    }

    pub fn observe(&self) -> RuntimeObserver<'_, D, I, E, Ctx, T> {
        RuntimeObserver::new(self)
    }

    pub fn derive_evaluation_strategy(&self) -> EvaluationStrategy {
        self.graph.derive_evaluation_strategy()
    }

    pub fn graph_mut(&mut self) -> SignalGraphMut<'_, D, I, E, Ctx, T> {
        self.config.sync_graph_capacity(&self.graph);
        SignalGraphMut { runtime: self }
    }

    pub fn checkpoint(&self) -> &CheckpointRuntime<D, I> {
        &self.checkpoint
    }

    pub fn event_bus(&self) -> &EventBus<E, D, Ctx> {
        &self.event_bus
    }

    pub fn event_bus_mut(&mut self) -> &mut EventBus<E, D, Ctx> {
        &mut self.event_bus
    }

    pub fn telemetry(&self) -> &RuntimeTelemetry {
        &self.telemetry
    }

    pub(super) fn capture_full_authority_state(&self) -> AuthorityState<T> {
        AuthorityState::capture(&self.graph, &self.config)
    }

    pub(super) fn capture_full_derived_state(&self) -> DerivedState<D, I> {
        DerivedState::capture(&self.checkpoint, &self.telemetry)
    }

    fn heavy_capture_witness(&mut self) -> HeavyCaptureWitness {
        self.telemetry.transaction.heavy_capture_count += 1;
        HeavyCaptureWitness(())
    }

    pub(super) fn capture_heavy_branch_state(&mut self) -> BranchState<D, I, T> {
        let _witness = self.heavy_capture_witness();
        let handle = self.graph.current_branch();
        let ancestry = self
            .branches
            .branch_ancestry_state(handle.id)
            .cloned()
            .unwrap_or(BranchAncestryState {
                branch_id: handle.id,
                parent_branch_id: handle.parent_branch_id,
                forked_from_snapshot_id: handle.head_snapshot_id,
                latest_merge_reference: None,
            });
        let mut mutation_ledger = self
            .branches
            .branch_state(handle.id)
            .map(|state| state.mutation_ledger.clone())
            .unwrap_or_else(|| {
                BranchMutationLedger::default().with_baseline_snapshot(handle.head_snapshot_id)
            });
        mutation_ledger.absorb_records(self.graph.branch_mutation_records());
        self.graph.clear_branch_mutation_nodes();
        self.branches.capture_active_state(
            self.capture_full_authority_state(),
            self.capture_full_derived_state(),
            ancestry,
            mutation_ledger,
        )
    }

    pub(super) fn take_heavy_active_branch_state(&mut self) -> BranchState<D, I, T> {
        let _witness = self.heavy_capture_witness();
        let handle = self.graph.current_branch();
        let ancestry = self
            .branches
            .branch_ancestry_state(handle.id)
            .cloned()
            .unwrap_or(BranchAncestryState {
                branch_id: handle.id,
                parent_branch_id: handle.parent_branch_id,
                forked_from_snapshot_id: handle.head_snapshot_id,
                latest_merge_reference: None,
            });
        let mut mutation_ledger = self
            .branches
            .branch_state(handle.id)
            .map(|state| state.mutation_ledger.clone())
            .unwrap_or_else(|| {
                BranchMutationLedger::default().with_baseline_snapshot(handle.head_snapshot_id)
            });
        mutation_ledger.absorb_records(self.graph.branch_mutation_records());
        self.graph.clear_branch_mutation_nodes();

        let authority = AuthorityState {
            graph: std::mem::take(&mut self.graph),
            config: std::mem::take(&mut self.config),
        };
        let checkpoint_policy = self.checkpoint.policy().clone();
        let derived = DerivedState {
            checkpoint: std::mem::replace(
                &mut self.checkpoint,
                CheckpointRuntime::new(checkpoint_policy),
            ),
            telemetry: std::mem::take(&mut self.telemetry),
        };
        self.branches
            .capture_active_state(authority, derived, ancestry, mutation_ledger)
    }

    fn load_branch_state(
        &mut self,
        packet: AuthorityTransferPacket<D, I, T>,
    ) -> Result<(), crate::data::error::SignalError> {
        let preserved_transaction = self.telemetry.transaction;
        if packet.branch_id != packet.state.ancestry.branch_id {
            return Err(crate::data::error::SignalError::internal(format!(
                "branch lifecycle transfer mismatch: packet branch {} does not match state branch {}",
                packet.branch_id.0,
                packet.state.ancestry.branch_id.0
            )));
        }
        self.branches.restore_active_state(
            packet.state,
            &mut self.graph,
            &mut self.config,
            &mut self.checkpoint,
            &mut self.telemetry,
        );
        Self::merge_global_transaction_telemetry(
            preserved_transaction,
            &mut self.telemetry.transaction,
        );
        Ok(())
    }

    fn load_restored_branch_state(
        &mut self,
        packet: RestoreTransferPacket<D, I, T>,
    ) -> Result<(), crate::data::error::SignalError> {
        self.telemetry.transaction.restore_transfer_count += 1;
        self.load_branch_state(AuthorityTransferPacket {
            branch_id: packet.branch_id,
            state: packet.state,
        })
    }

    pub(super) fn apply_branch_lifecycle_transfer(
        &mut self,
        transfer: BranchLifecycleTransfer<D, I, T>,
    ) -> Result<(), crate::data::error::SignalError> {
        match transfer {
            BranchLifecycleTransfer::Move(packet) => self.load_branch_state(packet),
            BranchLifecycleTransfer::Restore(packet) => self.load_restored_branch_state(packet),
        }
    }

    pub(super) fn synchronize_branch_catalogs(
        &mut self,
        branch_catalog: std::collections::BTreeMap<SignalBranchId, SignalBranchHandle>,
    ) {
        let active_branch = self.graph.current_branch().id;
        self.branches
            .synchronize_catalogs(branch_catalog, active_branch, &mut self.graph);
    }
}
