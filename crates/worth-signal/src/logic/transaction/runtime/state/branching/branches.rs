mod authority;
mod catalog;
mod lifecycle;
mod retention;
mod selection;
mod snapshot_storage;
mod transfer;

pub(in crate::logic::transaction::runtime) use authority::{
    BranchAncestryState, BranchState, LatestMergeReference,
};
pub(in crate::logic::transaction::runtime) use catalog::BranchManager;
pub(in crate::logic::transaction::runtime::state) use catalog::DEFAULT_MAXIMUM_STORED_SIGNAL_BRANCH_SNAPSHOTS;
pub(in crate::logic::transaction::runtime) use snapshot_storage::SignalBranchSnapshotStorageDenial;

use crate::data::graph::SignalGraph;
use crate::data::telemetry::RuntimeTelemetry;
use crate::logic::transaction::runtime::config::SignalRuntimeConfig;
use crate::state::{SignalBranchId, SignalSnapshotId};

use super::super::merge::BranchMutationLedger;
use super::super::reconstructability::{AuthorityState, DerivedState};

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct SnapshotBranchState<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    config: SignalRuntimeConfig<T>,
    derived: DerivedState<D, I>,
    ancestry: BranchAncestryState,
    mutation_ledger: BranchMutationLedger,
}

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime) struct SnapshotStatePacket<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    branch_id: SignalBranchId,
    snapshot_id: SignalSnapshotId,
    state: SnapshotBranchState<D, I, T>,
}

impl<D, I, T> SnapshotBranchState<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(in crate::logic::transaction::runtime::state) fn resource(
        &self,
    ) -> &super::super::resource::ResourceRuntimeState {
        &self.derived.resource
    }

    pub fn from_branch_state(state: &BranchState<D, I, T>) -> Self {
        Self {
            config: state.config().clone(),
            derived: state.derived.clone(),
            ancestry: state.ancestry().clone(),
            mutation_ledger: state.mutation_ledger().clone(),
        }
    }

    pub fn into_branch_state(
        self,
        graph: SignalGraph,
        runtime_telemetry: Option<RuntimeTelemetry>,
    ) -> BranchState<D, I, T> {
        let telemetry = if graph.captures_observation_surface(
            crate::logic::transaction::SignalObservationSurface::OptionalTelemetry,
        ) {
            runtime_telemetry.unwrap_or(self.derived.telemetry)
        } else {
            RuntimeTelemetry::default()
        };
        BranchState::new(
            AuthorityState {
                graph,
                config: self.config,
            },
            DerivedState {
                checkpoint: self.derived.checkpoint,
                resource: self.derived.resource,
                temporal: self.derived.temporal,
                telemetry,
            },
            self.ancestry,
            self.mutation_ledger,
        )
    }

    pub fn packet(self, snapshot_id: SignalSnapshotId) -> SnapshotStatePacket<D, I, T> {
        SnapshotStatePacket {
            branch_id: self.ancestry.branch_id(),
            snapshot_id,
            state: self,
        }
    }
}

impl<D, I, T> SnapshotStatePacket<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn into_parts(
        self,
    ) -> (
        SignalBranchId,
        SignalSnapshotId,
        SnapshotBranchState<D, I, T>,
    ) {
        (self.branch_id, self.snapshot_id, self.state)
    }
}
