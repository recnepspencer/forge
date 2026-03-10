//! Arena-based signal graph with dependency storage.

use serde::{Deserialize, Serialize};

use crate::data::dependency::DependencySnapshotStore;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::node::NodeEntry;
use crate::data::output::PartitionInterner;
use crate::data::telemetry::RuntimeTelemetry;
use crate::data::bitset::DenseBitset;
use crate::diagnostics::state::DiagnosticsState;

use super::scratch::{ScratchLeaseKind, TraversalScratch};
use super::slot::Slot;
use super::{DependencyEdgeStore, SubscriberEdgeStore};

/// The reactive signal graph.
///
/// An arena of `NodeEntry` values with graph-owned dependency, subscriber,
/// and snapshot storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalGraph {
    pub(super) nodes: Vec<Slot>,
    pub(super) free_list: Vec<u32>,
    #[serde(skip, default)]
    pub(super) free_slots: DenseBitset,
    pub(super) active_nodes: u32,
    pub(super) tombstone_count: u32,
    pub(super) gc_threshold: u32,
    #[serde(skip, default)]
    pub(super) scratch: TraversalScratch,
    #[serde(skip, default)]
    pub(super) scratch_lease: Option<ScratchLeaseKind>,
    #[serde(skip, default)]
    pub(super) telemetry: RuntimeTelemetry,
    #[serde(default)]
    pub(super) partition_interner: PartitionInterner,
    #[serde(default)]
    pub(super) dependency_snapshots: DependencySnapshotStore,
    #[serde(default)]
    pub(super) dependency_edges: DependencyEdgeStore,
    #[serde(default)]
    pub(super) subscriber_edges: SubscriberEdgeStore,
    #[serde(skip, default)]
    pub(super) diagnostics: DiagnosticsState,
}

const NODE_ARENA_RESERVE_CHUNK: usize = 1024;

impl Default for SignalGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl SignalGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            free_list: Vec::new(),
            free_slots: DenseBitset::default(),
            active_nodes: 0,
            tombstone_count: 0,
            gc_threshold: 1024,
            scratch: TraversalScratch::default(),
            scratch_lease: None,
            telemetry: RuntimeTelemetry::default(),
            partition_interner: PartitionInterner::default(),
            dependency_snapshots: DependencySnapshotStore::default(),
            dependency_edges: DependencyEdgeStore::default(),
            subscriber_edges: SubscriberEdgeStore::default(),
            diagnostics: DiagnosticsState::default(),
        }
    }

    pub fn with_gc_threshold(gc_threshold: u32) -> Self {
        Self {
            gc_threshold,
            ..Self::new()
        }
    }

    pub(crate) fn clone_stateful(&self) -> Self {
        let mut cloned = self.clone();
        cloned.telemetry = self.telemetry.clone();
        cloned.diagnostics = self.diagnostics.clone();
        cloned
    }

    pub(crate) fn acquire_scratch(
        &mut self,
        kind: ScratchLeaseKind,
    ) -> Result<TraversalScratch, SignalError> {
        if let Some(active) = self.scratch_lease {
            self.telemetry.scratch_reentry_error_count += 1;
            return Err(SignalError::invalid_input(format!(
                "signal scratch is already leased for {active:?}; re-entrant {kind:?} traversal is forbidden"
            )));
        }
        self.scratch_lease = Some(kind);
        Ok(std::mem::take(&mut self.scratch))
    }

    pub(crate) fn restore_scratch(
        &mut self,
        kind: ScratchLeaseKind,
        scratch: TraversalScratch,
    ) -> Result<(), SignalError> {
        match self.scratch_lease {
            Some(active) if active == kind => {
                self.scratch = scratch;
                self.scratch_lease = None;
                Ok(())
            }
            Some(active) => Err(SignalError::internal(format!(
                "signal scratch lease mismatch: expected {active:?}, restored {kind:?}"
            ))),
            None => Err(SignalError::internal(
                "signal scratch restore called without active lease",
            )),
        }
    }

    pub(super) fn allocate_node(&mut self, entry: NodeEntry) -> NodeId {
        while let Some(index) = self.free_list.pop() {
            if index as usize >= self.nodes.len() {
                continue;
            }
            self.free_slots.clear(index as usize);
            let slot = &mut self.nodes[index as usize];
            let generation = slot.occupy(entry);
            self.active_nodes += 1;
            return NodeId::new(index, generation);
        }

        let index = self.nodes.len() as u32;
        if self.nodes.len() == self.nodes.capacity() {
            self.nodes.reserve(NODE_ARENA_RESERVE_CHUNK);
        }
        let mut slot = Slot::vacant();
        let generation = slot.occupy(entry);
        self.nodes.push(slot);
        self.active_nodes += 1;
        NodeId::new(index, generation)
    }

    pub(crate) fn rollback_created_nodes(&mut self, created_nodes: &[NodeId]) {
        let mut indices = created_nodes
            .iter()
            .map(|node| node.index() as usize)
            .collect::<Vec<_>>();
        indices.sort_unstable();
        indices.dedup();

        for index in indices.iter().rev().copied() {
            let Some(slot) = self.nodes.get_mut(index) else {
                continue;
            };
            if slot.is_occupied() {
                slot.vacate();
                self.active_nodes = self.active_nodes.saturating_sub(1);
            }
            if !self.free_slots.contains(index) {
                self.free_list.push(index as u32);
                self.free_slots.mark(index);
            }
        }

        while self.nodes.last().is_some_and(|slot| !slot.is_occupied()) {
            self.free_slots.clear(self.nodes.len() - 1);
            self.nodes.pop();
        }
        self.free_list
            .retain(|index| (*index as usize) < self.nodes.len());
    }

    pub(super) fn validate_handle(&self, id: NodeId) -> Result<(), SignalError> {
        let idx = id.index() as usize;
        if idx >= self.nodes.len() {
            return Err(stale_error(id));
        }
        let slot = &self.nodes[idx];
        if slot.generation != id.generation() || !slot.is_occupied() {
            return Err(stale_error(id));
        }
        Ok(())
    }

    pub(crate) fn live_node_ids(&self) -> Vec<NodeId> {
        self.nodes
            .iter()
            .enumerate()
            .filter_map(|(index, slot)| {
                slot.data
                    .as_ref()
                    .map(|_| NodeId::new(index as u32, slot.generation))
            })
            .collect()
    }

    #[cfg(test)]
    pub(crate) fn storage_counts(&self) -> ((usize, usize), (usize, usize), usize) {
        (
            self.dependency_edges.storage_counts(),
            self.subscriber_edges.storage_counts(),
            self.dependency_snapshots.snapshot_count(),
        )
    }

    #[cfg(test)]
    pub(crate) fn free_list_snapshot(&self) -> Vec<u32> {
        self.free_list.clone()
    }
}

pub(super) fn stale_error(id: NodeId) -> SignalError {
    SignalError::invalid_input(format!("stale NodeId: {id}"))
}
