//! Arena-based signal graph with dependency storage.
//!
//! DOMAIN: Node allocation, dependency wiring, and generation-safe access.
//!
//! INVARIANTS:
//! - Handles are generation-checked on every access
//! - Tombstoned nodes are skipped during push traversal
//! - Free slots are reused with bumped generation
//!
//! DEPENDENCIES: `handles` (NodeId), `schema` (NodeEntry, DependencyEdge, Aspect)

use forge_core::KernelError;

use serde::{Deserialize, Serialize};

use crate::handles::NodeId;
use crate::schema::{Aspect, DependencyEdge, NodeEntry, NodeState};

/// A slot in the node arena that may be occupied or vacant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slot {
    /// The node data, if occupied.
    data: Option<NodeEntry>,
    /// Generation counter (bumped on each reuse).
    generation: u32,
}

impl Slot {
    /// Create a new vacant slot at generation 0.
    fn vacant() -> Self {
        Self {
            data: None,
            generation: 0,
        }
    }

    /// Occupy this slot, returning the current generation.
    fn occupy(&mut self, entry: NodeEntry) -> u32 {
        self.data = Some(entry);
        self.generation
    }

    /// Vacate this slot, bumping the generation.
    fn vacate(&mut self) -> Option<NodeEntry> {
        self.generation += 1;
        self.data.take()
    }

    /// Whether this slot is occupied.
    fn is_occupied(&self) -> bool {
        self.data.is_some()
    }
}

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
        }
    }

    /// Create a signal graph with a custom GC threshold.
    pub fn with_gc_threshold(gc_threshold: u32) -> Self {
        Self {
            nodes: Vec::new(),
            free_list: Vec::new(),
            tombstone_count: 0,
            gc_threshold,
        }
    }

    /// Allocate a new signal node, returning its stable handle.
    pub fn create_node(&mut self) -> NodeId {
        let entry = NodeEntry::new();

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
    ///
    /// Adds the dependency edge to the downstream node and registers
    /// the downstream as a subscriber on the upstream node.
    pub fn add_dependency(
        &mut self,
        downstream: NodeId,
        upstream: NodeId,
        aspect: Aspect,
    ) -> Result<(), KernelError> {
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
    ) -> Result<(), KernelError> {
        self.validate_handle(downstream)?;
        self.validate_handle(upstream)?;

        self.get_entry_mut(downstream)?
            .remove_dependencies_on(upstream);
        self.get_entry_mut(upstream)?
            .remove_subscriber(downstream);
        Ok(())
    }

    /// Read the state of a node.
    pub fn get_state(&self, id: NodeId) -> Result<NodeState, KernelError> {
        Ok(*self.get_entry(id)?.get_state())
    }

    /// Read-only access to a node entry.
    pub fn get_entry(&self, id: NodeId) -> Result<&NodeEntry, KernelError> {
        self.validate_handle(id)?;
        let slot = &self.nodes[id.index() as usize];
        slot.data.as_ref().ok_or_else(|| stale_error(id))
    }

    /// Mutable access to a node entry.
    pub fn get_entry_mut(&mut self, id: NodeId) -> Result<&mut NodeEntry, KernelError> {
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
        self.nodes
            .iter()
            .filter(|s| s.is_occupied())
            .count()
    }

    /// The number of allocated slots (including vacant ones).
    pub fn arena_capacity(&self) -> usize {
        self.nodes.len()
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
    ///
    /// The node is tombstoned: its slot is vacated and generation bumped.
    /// All upstream nodes have this node removed from their subscriber lists.
    /// All downstream subscribers have their dependency on this node removed
    /// and are marked `Dirty`.
    pub fn unregister_node(&mut self, id: NodeId) -> Result<(), KernelError> {
        self.validate_handle(id)?;

        let entry = self.get_entry(id)?;
        let upstream_sources: Vec<NodeId> = entry
            .get_dependencies()
            .iter()
            .map(|e| e.source())
            .collect();
        let downstream_subs: Vec<NodeId> = entry.get_subscribers().to_vec();

        for source in upstream_sources {
            if self.is_alive(source) {
                self.get_entry_mut(source)?.remove_subscriber(id);
            }
        }

        for sub in downstream_subs {
            if self.is_alive(sub) {
                self.get_entry_mut(sub)?.remove_dependencies_on(id);
                self.get_entry_mut(sub)?.set_state(NodeState::Dirty);
            }
        }

        self.nodes[id.index() as usize].vacate();
        self.tombstone_count += 1;
        self.free_list.push(id.index());

        Ok(())
    }

    /// Run a garbage collection epoch.
    ///
    /// Purges stale subscriber references from all active nodes,
    /// compacts the free list, and resets the tombstone counter.
    pub fn run_gc_epoch(&mut self) {
        let alive_snapshot: Vec<(u32, bool)> = self
            .nodes
            .iter()
            .map(|slot| (slot.generation, slot.is_occupied()))
            .collect();

        let alive_checker = |node_id: NodeId| -> bool {
            let idx = node_id.index() as usize;
            if idx >= alive_snapshot.len() {
                return false;
            }
            let (gen, occupied) = alive_snapshot[idx];
            gen == node_id.generation() && occupied
        };

        for slot in &mut self.nodes {
            if let Some(ref mut entry) = slot.data {
                entry.purge_stale_subscribers(alive_checker);
            }
        }

        self.free_list.sort_unstable();
        self.free_list.dedup();
        self.tombstone_count = 0;
    }

    /// Whether a GC epoch should be triggered.
    pub fn should_gc(&self) -> bool {
        self.tombstone_count >= self.gc_threshold
    }

    /// Validate that a handle refers to a live node.
    fn validate_handle(&self, id: NodeId) -> Result<(), KernelError> {
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

/// Produce a structured error for a stale or invalid node handle.
fn stale_error(id: NodeId) -> KernelError {
    KernelError::InvalidInput {
        message: format!(
            "Stale or invalid signal node handle: {}",
            id
        ),
        context: None,
    }
}
