//! Data types for the reactive signal graph.
//!
//! DOMAIN: Node state, aspect versioning, dependency edges.
//!
//! INVARIANTS:
//! - `NodeState` is always one of Clean/MaybeStale/Dirty
//! - `AspectVersion` tracks topology and geometry independently
//! - `DependencyEdge` records which aspect a downstream node reads
//!
//! DEPENDENCIES: `handles` (NodeId)

use serde::{Deserialize, Serialize};

use forge_core::TraceSummary;

use crate::handles::NodeId;

/// Three-state invalidation for a signal node.
///
/// This is the core reactive primitive:
/// - `Clean`: value is current, no recomputation needed
/// - `MaybeStale`: a transitive dependency changed — check before using
/// - `Dirty`: a direct dependency changed — must recompute
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeState {
    /// Value is current at the given version.
    Clean,
    /// A dependency's dependency changed. May or may not affect this node.
    /// Requires walking upstream to determine if recomputation is needed.
    MaybeStale,
    /// A direct dependency changed. This node MUST recompute.
    Dirty,
}

/// Which aspect of a feature output a downstream node subscribes to.
///
/// This enables the topology change firewall: a geometry-only change
/// (e.g., dragging an extrude depth) won't trigger re-evaluation of
/// nodes that only subscribe to the topology aspect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Aspect {
    /// Subscribes to topology changes (connectivity, face count, etc.).
    Topology,
    /// Subscribes to geometry changes (positions, dimensions, etc.).
    Geometry,
}

/// Per-aspect version counters carried by each signal node.
///
/// When a feature evaluates, it reports new aspect versions.
/// Downstream nodes compare these against their cached versions
/// to determine if they actually need to recompute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectVersion {
    topology: u64,
    geometry: u64,
}

impl AspectVersion {
    /// Create a new aspect version with both counters at zero.
    pub fn zero() -> Self {
        Self {
            topology: 0,
            geometry: 0,
        }
    }

    /// Create a new aspect version with explicit values.
    pub fn new(topology: u64, geometry: u64) -> Self {
        Self { topology, geometry }
    }

    /// The topology version counter.
    pub fn topology(self) -> u64 {
        self.topology
    }

    /// The geometry version counter.
    pub fn geometry(self) -> u64 {
        self.geometry
    }

    /// Read the version for a specific aspect.
    pub fn get(self, aspect: Aspect) -> u64 {
        match aspect {
            Aspect::Topology => self.topology,
            Aspect::Geometry => self.geometry,
        }
    }

    /// Bump the topology version by one.
    pub fn bump_topology(self) -> Self {
        Self {
            topology: self.topology + 1,
            geometry: self.geometry,
        }
    }

    /// Bump the geometry version by one.
    pub fn bump_geometry(self) -> Self {
        Self {
            topology: self.topology,
            geometry: self.geometry + 1,
        }
    }

    /// Bump both versions by one.
    pub fn bump_all(self) -> Self {
        Self {
            topology: self.topology + 1,
            geometry: self.geometry + 1,
        }
    }
}

/// A dependency edge recording which upstream node and aspect a downstream reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyEdge {
    source: NodeId,
    aspect: Aspect,
}

impl DependencyEdge {
    /// Create a new dependency edge.
    pub fn new(source: NodeId, aspect: Aspect) -> Self {
        Self { source, aspect }
    }

    /// The upstream node this edge points to.
    pub fn source(self) -> NodeId {
        self.source
    }

    /// Which aspect of the upstream node is subscribed to.
    pub fn aspect(self) -> Aspect {
        self.aspect
    }
}

/// A snapshot of upstream aspect versions at the time a node was last evaluated.
///
/// Used by the pull phase to determine if a `MaybeStale` node can revert
/// to `Clean`: if all upstream versions match the snapshot, no recomputation needed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencySnapshot {
    entries: Vec<(NodeId, Aspect, u64)>,
}

impl DependencySnapshot {
    /// Create an empty snapshot.
    pub fn empty() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Record an upstream version.
    pub fn record(&mut self, source: NodeId, aspect: Aspect, version: u64) {
        self.entries.push((source, aspect, version));
    }

    /// All recorded entries.
    pub fn entries(&self) -> &[(NodeId, Aspect, u64)] {
        &self.entries
    }
}

/// Internal storage for a single signal node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeEntry {
    state: NodeState,
    aspect_version: AspectVersion,
    /// Upstream dependencies this node reads from.
    dependencies: Vec<DependencyEdge>,
    /// Downstream subscribers (generation-checked before push).
    subscribers: Vec<NodeId>,
    /// Snapshot of upstream versions at last clean evaluation.
    dep_snapshot: DependencySnapshot,
    /// Whether this node has been tombstoned (deleted but not yet GC'd).
    tombstoned: bool,
    /// Last evaluation trace summary (for diff-on-re-eval).
    #[serde(default)]
    trace_summary: Option<TraceSummary>,
}

impl Default for NodeEntry {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeEntry {
    /// Create a new node entry in the `Dirty` state.
    pub fn new() -> Self {
        Self {
            state: NodeState::Dirty,
            aspect_version: AspectVersion::zero(),
            dependencies: Vec::new(),
            subscribers: Vec::new(),
            dep_snapshot: DependencySnapshot::empty(),
            tombstoned: false,
            trace_summary: None,
        }
    }

    /// The current state of this node.
    pub fn get_state(&self) -> &NodeState {
        &self.state
    }

    /// Set the node state.
    pub fn set_state(&mut self, state: NodeState) {
        self.state = state;
    }

    /// The current aspect versions.
    pub fn get_aspect_version(&self) -> AspectVersion {
        self.aspect_version
    }

    /// Set the aspect version after evaluation.
    pub fn set_aspect_version(&mut self, version: AspectVersion) {
        self.aspect_version = version;
    }

    /// The upstream dependencies.
    pub fn get_dependencies(&self) -> &[DependencyEdge] {
        &self.dependencies
    }

    /// Add an upstream dependency.
    pub fn add_dependency(&mut self, edge: DependencyEdge) {
        self.dependencies.push(edge);
    }

    /// Remove all dependencies on a specific upstream node.
    pub fn remove_dependencies_on(&mut self, source: NodeId) {
        self.dependencies.retain(|e| e.source() != source);
    }

    /// The downstream subscribers.
    pub fn get_subscribers(&self) -> &[NodeId] {
        &self.subscribers
    }

    /// Add a downstream subscriber.
    pub fn add_subscriber(&mut self, subscriber: NodeId) {
        self.subscribers.push(subscriber);
    }

    /// Remove a specific downstream subscriber.
    pub fn remove_subscriber(&mut self, subscriber: NodeId) {
        self.subscribers.retain(|s| *s != subscriber);
    }

    /// Purge subscribers whose generation doesn't match the graph.
    pub fn purge_stale_subscribers(&mut self, is_alive: impl Fn(NodeId) -> bool) {
        self.subscribers.retain(|s| is_alive(*s));
    }

    /// The dependency snapshot from the last clean evaluation.
    pub fn get_dep_snapshot(&self) -> &DependencySnapshot {
        &self.dep_snapshot
    }

    /// Replace the dependency snapshot.
    pub fn set_dep_snapshot(&mut self, snapshot: DependencySnapshot) {
        self.dep_snapshot = snapshot;
    }

    /// Whether this node is tombstoned.
    pub fn is_tombstoned(&self) -> bool {
        self.tombstoned
    }

    /// Mark this node as tombstoned.
    pub fn set_tombstoned(&mut self, tombstoned: bool) {
        self.tombstoned = tombstoned;
    }

    /// The last evaluation trace summary.
    pub fn get_trace_summary(&self) -> Option<&TraceSummary> {
        self.trace_summary.as_ref()
    }

    /// Set or clear the trace summary.
    pub fn set_trace_summary(&mut self, summary: Option<TraceSummary>) {
        self.trace_summary = summary;
    }
}
