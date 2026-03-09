//! Arena-based signal graph with dependency storage.

use serde::{Deserialize, Serialize};

use crate::data::dependency::DependencySnapshotStore;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::node::NodeEntry;
use crate::data::output::PartitionInterner;
use crate::data::telemetry::RuntimeTelemetry;
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
        if let Some(index) = self.free_list.pop() {
            let slot = &mut self.nodes[index as usize];
            let generation = slot.occupy(entry);
            return NodeId::new(index, generation);
        }

        let index = self.nodes.len() as u32;
        let mut slot = Slot::vacant();
        let generation = slot.occupy(entry);
        self.nodes.push(slot);
        NodeId::new(index, generation)
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
}

pub(super) fn stale_error(id: NodeId) -> SignalError {
    SignalError::invalid_input(format!("stale NodeId: {id}"))
}
