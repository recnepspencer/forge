use crate::clock::RuntimeInstant;
use crate::data::handle::NodeId;
use crate::data::output::ChangedRegion;
use crate::data::proof::DirtyBatchEntry;
use crate::data::telemetry::RuntimeTelemetry;
use crate::logic::checkpoint::CheckpointRuntime;
use crate::logic::events::EventBus;

use super::super::super::config::SignalRuntimeConfig;
use super::super::super::state::{
    BranchManager, ResourceRuntimeState, RuntimeObservationRegistry, TemporalRuntimeState,
};

use super::rollback::TransactionRollbackPacketSet;
use super::state::{TransactionExecutionState, TransactionScratch};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::logic::transaction::runtime) enum TransactionCommitPosture {
    Visible,
    BranchLocal,
}

pub struct SignalTransaction<'a, D, I, E, Ctx, T = ()>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(in crate::logic::transaction::runtime) runtime_ctx: &'a mut Ctx,
    pub(in crate::logic::transaction::runtime) observations:
        &'a RuntimeObservationRegistry<D, I, E, Ctx, T>,
    pub(in crate::logic::transaction::runtime) config: &'a mut SignalRuntimeConfig<T>,
    pub(in crate::logic::transaction::runtime) graph: &'a mut crate::data::graph::SignalGraph,
    pub(in crate::logic::transaction::runtime) checkpoint: &'a mut CheckpointRuntime<D, I>,
    pub(in crate::logic::transaction::runtime) event_bus: &'a mut EventBus<E, D, Ctx>,
    pub(in crate::logic::transaction::runtime) resource: &'a mut ResourceRuntimeState,
    pub(in crate::logic::transaction::runtime) temporal: &'a mut TemporalRuntimeState,
    pub(in crate::logic::transaction::runtime) telemetry: &'a mut RuntimeTelemetry,
    pub(in crate::logic::transaction::runtime) branches: &'a mut BranchManager<D, I, T>,
    pub(in crate::logic::transaction::runtime) scratch: TransactionScratch<D, I, E>,
    pub(in crate::logic::transaction::runtime) rollback_packets: TransactionRollbackPacketSet<T>,
    pub(in crate::logic::transaction::runtime) poisoned: bool,
    pub(in crate::logic::transaction::runtime) finished: bool,
    pub(in crate::logic::transaction::runtime) execution_state: TransactionExecutionState,
    pub(in crate::logic::transaction::runtime) started_at: RuntimeInstant,
    pub(in crate::logic::transaction::runtime) commit_posture: TransactionCommitPosture,
}

pub struct BatchChangeSession<'tx, 'a, D, I, E, Ctx, T = ()>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(in crate::logic::transaction::runtime) tx: &'tx mut SignalTransaction<'a, D, I, E, Ctx, T>,
    pub(in crate::logic::transaction::runtime) entries: Vec<DirtyBatchEntry>,
    pub(in crate::logic::transaction::runtime) applied: bool,
}

impl<'tx, 'a, D, I, E, Ctx, T> BatchChangeSession<'tx, 'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(in crate::logic::transaction::runtime) fn new(
        tx: &'tx mut SignalTransaction<'a, D, I, E, Ctx, T>,
    ) -> Self {
        Self {
            tx,
            entries: Vec::new(),
            applied: false,
        }
    }

    pub fn mark(mut self, source: NodeId, changed_aspect: crate::data::aspect::Aspect) -> Self {
        self.entries
            .push(DirtyBatchEntry::without_regions(source, changed_aspect));
        self
    }

    pub fn mark_regions(
        mut self,
        source: NodeId,
        changed_aspect: crate::data::aspect::Aspect,
        changed_regions: &[ChangedRegion],
    ) -> Self {
        self.entries.push(DirtyBatchEntry::new(
            source,
            changed_aspect,
            changed_regions.to_vec(),
        ));
        self
    }
}
