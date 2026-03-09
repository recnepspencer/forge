use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::data::aspect::{AspectMask, AspectVersion};
use crate::data::dependency::DependencySnapshotId;
use crate::data::graph::{DependencySetId, SubscriberSetId};
use crate::data::output::PartitionSubscription;
use crate::data::trace::{CausalityMetadata, TraceSummary};

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
struct NodeColdData {
    #[serde(default)]
    trace_summary: Option<TraceSummary>,
    #[serde(default)]
    causality: Option<CausalityMetadata>,
}

/// Internal storage for a single signal node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeEntry {
    state: NodeState,
    dirty_aspects: AspectMask,
    #[serde(default)]
    dirty_partition_scopes: SmallVec<[PartitionSubscription; 4]>,
    aspect_version: AspectVersion,
    /// Handle to graph-owned dependency edge storage.
    dependencies_id: DependencySetId,
    /// Handle to graph-owned subscriber storage.
    subscribers_id: SubscriberSetId,
    /// Handle to graph-owned dependency snapshot storage.
    dep_snapshot_id: DependencySnapshotId,
    /// Whether this node has been tombstoned (deleted but not yet GC'd).
    tombstoned: bool,
    /// Cold diagnostics- and explanation-facing data kept off the hot path.
    #[serde(default)]
    cold: Option<Box<NodeColdData>>,
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
            dirty_partition_scopes: SmallVec::new(),
            aspect_version: AspectVersion::zero(),
            dependencies_id: DependencySetId::EMPTY,
            subscribers_id: SubscriberSetId::EMPTY,
            dep_snapshot_id: DependencySnapshotId::EMPTY,
            tombstoned: false,
            cold: None,
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

    pub fn get_dirty_partition_scopes(&self) -> &[PartitionSubscription] {
        &self.dirty_partition_scopes
    }

    pub fn set_dirty_partition_scopes(
        &mut self,
        scopes: impl IntoIterator<Item = PartitionSubscription>,
    ) {
        self.dirty_partition_scopes.clear();
        self.dirty_partition_scopes.extend(scopes);
    }

    pub fn clear_dirty_partition_scopes(&mut self) {
        self.dirty_partition_scopes.clear();
    }

    pub fn add_dirty_partition_scope(&mut self, scope: PartitionSubscription) {
        if !self.dirty_partition_scopes.contains(&scope) {
            self.dirty_partition_scopes.push(scope);
        }
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

    /// Graph-owned dependency set handle.
    pub fn get_dependencies_id(&self) -> DependencySetId {
        self.dependencies_id
    }

    /// Replace the dependency set handle.
    pub fn set_dependencies_id(&mut self, dependencies_id: DependencySetId) {
        self.dependencies_id = dependencies_id;
    }

    /// Graph-owned subscriber set handle.
    pub fn get_subscribers_id(&self) -> SubscriberSetId {
        self.subscribers_id
    }

    /// Replace the subscriber set handle.
    pub fn set_subscribers_id(&mut self, subscribers_id: SubscriberSetId) {
        self.subscribers_id = subscribers_id;
    }

    /// The graph-owned dependency snapshot handle from the last clean evaluation.
    pub fn get_dep_snapshot_id(&self) -> DependencySnapshotId {
        self.dep_snapshot_id
    }

    /// Replace the dependency snapshot handle.
    pub fn set_dep_snapshot_id(&mut self, snapshot_id: DependencySnapshotId) {
        self.dep_snapshot_id = snapshot_id;
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
        self.cold.as_ref()?.trace_summary.as_ref()
    }

    /// Set or clear the trace summary.
    pub fn set_trace_summary(&mut self, summary: Option<TraceSummary>) {
        self.cold_mut().trace_summary = summary;
        self.trim_cold_if_empty();
    }

    /// Optional host-provided causality payload.
    pub fn get_causality(&self) -> Option<&CausalityMetadata> {
        self.cold.as_ref()?.causality.as_ref()
    }

    /// Set or clear the causality payload.
    pub fn set_causality(&mut self, causality: Option<CausalityMetadata>) {
        self.cold_mut().causality = causality;
        self.trim_cold_if_empty();
    }

    /// Per-node evaluation policy descriptor.
    pub fn get_eval_config(&self) -> &NodeEvaluationConfig {
        &self.eval_config
    }

    /// Replace per-node evaluation policy descriptor.
    pub fn set_eval_config(&mut self, config: NodeEvaluationConfig) {
        self.eval_config = config;
    }

    fn cold_mut(&mut self) -> &mut NodeColdData {
        self.cold
            .get_or_insert_with(|| Box::new(NodeColdData::default()))
            .as_mut()
    }

    fn trim_cold_if_empty(&mut self) {
        if self
            .cold
            .as_ref()
            .is_some_and(|cold| cold.trace_summary.is_none() && cold.causality.is_none())
        {
            self.cold = None;
        }
    }
}
