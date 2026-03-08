use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::data::aspect::{AspectMask, AspectVersion};
use crate::data::dependency::{DependencyEdge, DependencySnapshot};
use crate::data::handle::NodeId;
use crate::data::trace::TraceSummary;

use super::condition::NodeEvaluationConfig;

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

/// Internal storage for a single signal node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeEntry {
    state: NodeState,
    dirty_aspects: AspectMask,
    aspect_version: AspectVersion,
    /// Upstream dependencies this node reads from.
    dependencies: SmallVec<[DependencyEdge; 4]>,
    /// Downstream subscribers (generation-checked before push).
    subscribers: SmallVec<[NodeId; 4]>,
    /// Snapshot of upstream versions at last clean evaluation.
    dep_snapshot: DependencySnapshot,
    /// Whether this node has been tombstoned (deleted but not yet GC'd).
    tombstoned: bool,
    /// Last evaluation trace summary (for diff-on-re-eval).
    #[serde(default)]
    trace_summary: Option<TraceSummary>,
    /// Evaluation condition/config descriptor for this node.
    #[serde(default)]
    eval_config: NodeEvaluationConfig,
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
            dirty_aspects: AspectMask::EMPTY,
            aspect_version: AspectVersion::zero(),
            dependencies: SmallVec::new(),
            subscribers: SmallVec::new(),
            dep_snapshot: DependencySnapshot::empty(),
            tombstoned: false,
            trace_summary: None,
            eval_config: NodeEvaluationConfig::default(),
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

    /// Dirty aspects currently pending recomputation for this node.
    pub fn get_dirty_aspects(&self) -> AspectMask {
        self.dirty_aspects
    }

    /// Replace the dirty aspect mask.
    pub fn set_dirty_aspects(&mut self, dirty_aspects: AspectMask) {
        self.dirty_aspects = dirty_aspects;
    }

    /// Add one dirty aspect to the current mask.
    pub fn add_dirty_aspect(&mut self, aspect: crate::data::aspect::Aspect) {
        self.dirty_aspects.insert(aspect);
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
    pub fn add_dependency(&mut self, edge: DependencyEdge) -> bool {
        if self.dependencies.contains(&edge) {
            return false;
        }
        self.dependencies.push(edge);
        true
    }

    /// Remove one specific dependency edge.
    pub fn remove_dependency(&mut self, edge: DependencyEdge) -> bool {
        let original_len = self.dependencies.len();
        self.dependencies.retain(|candidate| *candidate != edge);
        self.dependencies.len() != original_len
    }

    /// Remove all dependencies on a specific upstream node.
    pub fn remove_dependencies_on(&mut self, source: NodeId) -> bool {
        let original_len = self.dependencies.len();
        self.dependencies.retain(|e| e.source() != source);
        self.dependencies.len() != original_len
    }

    /// Whether any dependency remains on the specified upstream node.
    pub fn has_dependency_on(&self, source: NodeId) -> bool {
        self.dependencies.iter().any(|edge| edge.source() == source)
    }

    /// The downstream subscribers.
    pub fn get_subscribers(&self) -> &[NodeId] {
        &self.subscribers
    }

    /// Add a downstream subscriber.
    pub fn add_subscriber(&mut self, subscriber: NodeId) -> bool {
        if self.subscribers.contains(&subscriber) {
            return false;
        }
        self.subscribers.push(subscriber);
        true
    }

    /// Remove a specific downstream subscriber.
    pub fn remove_subscriber(&mut self, subscriber: NodeId) -> bool {
        let original_len = self.subscribers.len();
        self.subscribers.retain(|s| *s != subscriber);
        self.subscribers.len() != original_len
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

    /// Per-node evaluation policy descriptor.
    pub fn get_eval_config(&self) -> &NodeEvaluationConfig {
        &self.eval_config
    }

    /// Replace per-node evaluation policy descriptor.
    pub fn set_eval_config(&mut self, config: NodeEvaluationConfig) {
        self.eval_config = config;
    }
}
