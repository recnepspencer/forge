//! Arena-based signal graph with dependency storage.

use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::data::aspect::Aspect;
use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::node::{NodeEntry, NodeEvaluationConfig, NodeState};
use crate::data::telemetry::RuntimeTelemetry;

use super::scratch::{ScratchLeaseKind, TraversalScratch};
use super::slot::Slot;

/// The reactive signal graph.
///
/// An arena of `NodeEntry` values with dependency edges.
/// Nodes are allocated with generational handles (`NodeId`) for
/// safe, stale-proof access.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalGraph {
    /// Arena slots (generational).
    nodes: Vec<Slot>,
    /// Free list of vacant slot indices for reuse.
    free_list: Vec<u32>,
    /// Count of tombstoned nodes awaiting GC.
    tombstone_count: u32,
    /// Threshold for triggering a GC epoch.
    gc_threshold: u32,
    /// Reusable traversal scratch to avoid hot-path allocations.
    #[serde(skip, default)]
    scratch: TraversalScratch,
    /// Active scratch lease, if any.
    #[serde(skip, default)]
    scratch_lease: Option<ScratchLeaseKind>,
    /// Lightweight runtime counters for evaluation/invalidation behavior.
    #[serde(skip, default)]
    telemetry: RuntimeTelemetry,
}

impl Default for SignalGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl SignalGraph {
    /// Create an empty signal graph.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            free_list: Vec::new(),
            tombstone_count: 0,
            gc_threshold: 1024,
            scratch: TraversalScratch::default(),
            scratch_lease: None,
            telemetry: RuntimeTelemetry::default(),
        }
    }

    /// Create a signal graph with a custom GC threshold.
    pub fn with_gc_threshold(gc_threshold: u32) -> Self {
        Self {
            nodes: Vec::new(),
            free_list: Vec::new(),
            tombstone_count: 0,
            gc_threshold,
            scratch: TraversalScratch::default(),
            scratch_lease: None,
            telemetry: RuntimeTelemetry::default(),
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

    /// Allocate a new signal node, returning its stable handle.
    pub fn create_node(&mut self) -> NodeId {
        let entry = NodeEntry::new();
        self.allocate_node(entry)
    }

    /// Allocate a new node with explicit evaluation config.
    pub fn create_node_with_config(&mut self, config: NodeEvaluationConfig) -> NodeId {
        let mut entry = NodeEntry::new();
        entry.set_eval_config(config);
        self.allocate_node(entry)
    }

    fn allocate_node(&mut self, entry: NodeEntry) -> NodeId {
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

    /// Wire a dependency: `downstream` reads `aspect` from `upstream`.
    pub fn add_dependency(
        &mut self,
        downstream: NodeId,
        upstream: NodeId,
        aspect: Aspect,
    ) -> Result<(), SignalError> {
        self.validate_handle(downstream)?;
        self.validate_handle(upstream)?;

        let edge = DependencyEdge::new(upstream, aspect);
        self.get_entry_mut(downstream)?.add_dependency(edge);
        self.get_entry_mut(upstream)?.add_subscriber(downstream);
        Ok(())
    }

    /// Remove all dependency edges from `downstream` to `upstream`.
    pub fn remove_dependency(
        &mut self,
        downstream: NodeId,
        upstream: NodeId,
    ) -> Result<(), SignalError> {
        self.validate_handle(downstream)?;
        self.validate_handle(upstream)?;

        self.get_entry_mut(downstream)?
            .remove_dependencies_on(upstream);
        self.get_entry_mut(upstream)?.remove_subscriber(downstream);
        Ok(())
    }

    /// Read the state of a node.
    pub fn get_state(&self, id: NodeId) -> Result<NodeState, SignalError> {
        Ok(*self.get_entry(id)?.get_state())
    }

    /// Read-only access to a node entry.
    pub fn get_entry(&self, id: NodeId) -> Result<&NodeEntry, SignalError> {
        self.validate_handle(id)?;
        let slot = &self.nodes[id.index() as usize];
        slot.data.as_ref().ok_or_else(|| stale_error(id))
    }

    /// Mutable access to a node entry.
    pub fn get_entry_mut(&mut self, id: NodeId) -> Result<&mut NodeEntry, SignalError> {
        self.validate_handle(id)?;
        let slot = &mut self.nodes[id.index() as usize];
        slot.data.as_mut().ok_or_else(|| stale_error(id))
    }

    /// Check whether a node handle is valid (alive and generation matches).
    pub fn is_alive(&self, id: NodeId) -> bool {
        let idx = id.index() as usize;
        if idx >= self.nodes.len() {
            return false;
        }
        let slot = &self.nodes[idx];
        slot.generation == id.generation() && slot.is_occupied()
    }

    /// The total number of active (non-tombstoned, occupied) nodes.
    pub fn active_node_count(&self) -> usize {
        self.nodes.iter().filter(|s| s.is_occupied()).count()
    }

    /// The number of allocated slots (including vacant ones).
    pub fn arena_capacity(&self) -> usize {
        self.nodes.len()
    }

    /// Resolve a live `NodeId` at slot index if occupied.
    pub(crate) fn live_node_id_at(&self, index: usize) -> Option<NodeId> {
        let slot = self.nodes.get(index)?;
        if !slot.is_occupied() {
            return None;
        }
        Some(NodeId::new(index as u32, slot.generation))
    }

    /// Replace full node entry payload for an existing live node.
    pub(crate) fn replace_entry(&mut self, id: NodeId, entry: NodeEntry) -> Result<(), SignalError> {
        let target = self.get_entry_mut(id)?;
        *target = entry;
        Ok(())
    }

    /// The count of tombstoned nodes awaiting GC.
    pub fn tombstone_count(&self) -> u32 {
        self.tombstone_count
    }

    /// The GC threshold.
    pub fn gc_threshold(&self) -> u32 {
        self.gc_threshold
    }

    /// Remove a node from the arena, severing all dependency edges.
    pub fn unregister_node(&mut self, id: NodeId) -> Result<(), SignalError> {
        self.validate_handle(id)?;
        let mut scratch = self.acquire_scratch(ScratchLeaseKind::Churn)?;
        scratch.node_buffer_a.clear();
        scratch.node_buffer_b.clear();

        {
            let entry = self.get_entry(id)?;
            scratch
                .node_buffer_a
                .extend(entry.get_dependencies().iter().map(|edge| edge.source()));
            scratch
                .node_buffer_b
                .extend(entry.get_subscribers().iter().copied());
        }

        for &source in &scratch.node_buffer_a {
            if self.is_alive(source) {
                self.get_entry_mut(source)?.remove_subscriber(id);
            }
        }

        for &subscriber in &scratch.node_buffer_b {
            if self.is_alive(subscriber) {
                self.get_entry_mut(subscriber)?.remove_dependencies_on(id);
                self.get_entry_mut(subscriber)?.set_state(NodeState::Dirty);
            }
        }

        debug_assert!(
            !self.free_list.contains(&id.index()),
            "free list already contained slot {} before unregister",
            id.index()
        );
        self.nodes[id.index() as usize].vacate();
        self.tombstone_count += 1;
        self.free_list.push(id.index());
        self.restore_scratch(ScratchLeaseKind::Churn, scratch)?;
        Ok(())
    }

    /// Run a garbage collection epoch.
    pub fn run_gc_epoch(&mut self) {
        let gc_start = Instant::now();
        let mut scratch = self
            .acquire_scratch(ScratchLeaseKind::Gc)
            .expect("GC scratch lease must succeed");
        let len = self.nodes.len();
        if scratch.gc_liveness_generations.len() < len {
            scratch.gc_liveness_generations.resize(len, 0);
        }
        scratch.gc_liveness_alive.clear_all();
        scratch.gc_liveness_alive.ensure_len(len);

        for (index, slot) in self.nodes.iter().enumerate() {
            scratch.gc_liveness_generations[index] = slot.generation;
            if slot.is_occupied() {
                scratch.gc_liveness_alive.mark(index);
            }
        }

        let generations = &scratch.gc_liveness_generations;
        let alive_bits = &scratch.gc_liveness_alive;
        let alive_checker = |node_id: NodeId| -> bool {
            let idx = node_id.index() as usize;
            idx < generations.len()
                && generations[idx] == node_id.generation()
                && alive_bits.contains(idx)
        };

        for slot in &mut self.nodes {
            if let Some(ref mut entry) = slot.data {
                entry.purge_stale_subscribers(alive_checker);
            }
        }

        self.tombstone_count = 0;
        self.restore_scratch(ScratchLeaseKind::Gc, scratch)
            .expect("GC scratch restore must succeed");
        self.telemetry.gc_epoch_count += 1;
        self.telemetry.gc_epoch_nanos += gc_start.elapsed().as_nanos();
    }

    /// Whether a GC epoch should be triggered.
    pub fn should_gc(&self) -> bool {
        self.tombstone_count >= self.gc_threshold
    }

    /// Validate that a handle refers to a live node.
    fn validate_handle(&self, id: NodeId) -> Result<(), SignalError> {
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

    /// Immutable telemetry snapshot.
    pub fn telemetry(&self) -> &RuntimeTelemetry {
        &self.telemetry
    }

    /// Mutable telemetry reference.
    pub fn telemetry_mut(&mut self) -> &mut RuntimeTelemetry {
        &mut self.telemetry
    }

    /// Reset runtime telemetry counters.
    pub fn reset_telemetry(&mut self) {
        self.telemetry = RuntimeTelemetry::default();
    }

}

/// Produce a structured error for a stale or invalid node handle.
fn stale_error(id: NodeId) -> SignalError {
    SignalError::InvalidInput {
        message: format!("Stale or invalid signal node handle: {}", id),
        context: None,
    }
}
