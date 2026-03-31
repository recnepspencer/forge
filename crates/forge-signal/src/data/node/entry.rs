use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::data::aspect::{
    Aspect, AspectMask, AspectVersion, AspectVersionHeader, PartitionVersionMap,
    PartitionVersionOverrides, MAX_ASPECTS,
};
use crate::data::core_profile::HOT_VEC_INLINE_CAPACITY;
use crate::data::dependency::DependencySnapshotId;
use crate::data::graph::{DependencySetId, SubscriberSetId};
use crate::data::output::{ChangedRegion, PartitionSubscription};
#[cfg(test)]
use crate::data::trace::ArtifactMergeAuthority;
use crate::data::trace::{
    assemble_historical_artifact_record, assemble_trace_summary_with_execution, ArtifactWriteDelta,
    CausalityMetadata, ExecutionTraceStamp, HistoricalArtifactRecord, RetainedDiagnosticArtifact,
    RuntimeArtifactState, TraceSummary,
};

use super::checkpoint_image::CheckpointNodeImageParts;
use super::condition::NodeEvaluationConfig;
use super::CheckpointNodeImage;

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
pub(crate) struct NodeColdData {
    #[serde(default)]
    pub(crate) retained_artifact: Option<RetainedDiagnosticArtifact>,
    #[serde(default)]
    pub(crate) causality: Option<CausalityMetadata>,
    #[serde(default)]
    pub(crate) execution_trace: Option<ExecutionTraceStamp>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct NodeHotData {
    pub(crate) state: NodeState,
    pub(crate) dirty_aspects: AspectMask,
    #[serde(default)]
    pub(crate) dirty_partition_scope_aspects: AspectMask,
    pub(crate) aspect_version_header: AspectVersionHeader,
    pub(crate) dependencies_id: DependencySetId,
    pub(crate) subscribers_id: SubscriberSetId,
    pub(crate) dep_snapshot_id: DependencySnapshotId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub(crate) struct NodeWarmData {
    #[serde(default)]
    pub(crate) tombstoned: bool,
    #[serde(default)]
    pub(crate) aspect_version_overrides: PartitionVersionOverrides,
    #[serde(default)]
    pub(crate) dirty_partition_scope_payload:
        SmallVec<[(crate::data::aspect::Aspect, PartitionSubscription); HOT_VEC_INLINE_CAPACITY]>,
    #[serde(default)]
    pub(crate) runtime_artifact_state: Option<RuntimeArtifactState>,
    #[serde(default)]
    pub(crate) eval_config: NodeEvaluationConfig,
}

pub(crate) fn node_hot_inline_size_bytes() -> u64 {
    std::mem::size_of::<NodeHotData>() as u64
}

pub(crate) fn node_warm_inline_size_bytes() -> u64 {
    std::mem::size_of::<NodeWarmData>() as u64
}

/// Internal storage for a single signal node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeEntry {
    #[serde(flatten)]
    hot: NodeHotData,
    #[serde(flatten)]
    warm: NodeWarmData,
    /// Cold diagnostics- and explanation-facing data kept off the hot path.
    #[serde(default)]
    cold: Option<Box<NodeColdData>>,
}

impl Default for NodeEntry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(not(test), allow(dead_code))]
impl NodeEntry {
    /// Create a new node entry in the `Dirty` state.
    pub fn new() -> Self {
        Self {
            hot: NodeHotData {
                state: NodeState::Dirty,
                dirty_aspects: AspectMask::from_bits(((1u32 << MAX_ASPECTS) - 1) as _),
                dirty_partition_scope_aspects: AspectMask::EMPTY,
                aspect_version_header: AspectVersionHeader::zero(),
                dependencies_id: DependencySetId::EMPTY,
                subscribers_id: SubscriberSetId::EMPTY,
                dep_snapshot_id: DependencySnapshotId::EMPTY,
            },
            warm: NodeWarmData::default(),
            cold: None,
        }
    }

    /// The current state of this node.
    pub fn get_state(&self) -> &NodeState {
        &self.hot.state
    }

    /// Set the node state.
    pub fn set_state(&mut self, state: NodeState) {
        self.hot.state = state;
    }

    /// Transition to `Clean` and clear all dirty tracking.
    #[allow(dead_code)]
    pub fn transition_clean(&mut self) {
        self.set_state(NodeState::Clean);
        self.set_dirty_aspects(AspectMask::EMPTY);
        self.clear_dirty_partition_scopes();
    }

    /// Transition to `Dirty` for one aspect and merge any scoped dirty regions.
    pub fn transition_dirty(&mut self, aspect: Aspect, scopes: &[PartitionSubscription]) {
        let was_clean = matches!(self.hot.state, NodeState::Clean);
        let already_dirty_for_aspect = self
            .hot
            .dirty_aspects
            .contains(AspectMask::from_aspect(aspect));
        self.set_state(NodeState::Dirty);
        self.add_dirty_aspect(aspect);
        self.merge_dirty_partition_scopes(aspect, scopes, was_clean, already_dirty_for_aspect);
    }

    /// Transition to `MaybeStale` for one aspect.
    #[allow(dead_code)]
    pub fn transition_maybe_stale(&mut self, aspect: Aspect) {
        let was_clean = matches!(self.hot.state, NodeState::Clean);
        let already_dirty_for_aspect = self
            .hot
            .dirty_aspects
            .contains(AspectMask::from_aspect(aspect));
        self.set_state(NodeState::MaybeStale);
        self.add_dirty_aspect(aspect);
        if was_clean || !already_dirty_for_aspect {
            self.clear_dirty_partition_scopes_for(aspect);
        }
    }

    /// Dirty aspects currently pending recomputation for this node.
    pub fn get_dirty_aspects(&self) -> AspectMask {
        self.hot.dirty_aspects
    }

    /// Replace the dirty aspect mask.
    pub fn set_dirty_aspects(&mut self, dirty_aspects: AspectMask) {
        self.hot.dirty_aspects = dirty_aspects;
    }

    #[cfg(test)]
    pub fn get_dirty_partition_scopes(
        &self,
    ) -> SmallVec<[PartitionSubscription; HOT_VEC_INLINE_CAPACITY]> {
        self.dirty_partition_scopes().cloned().collect()
    }

    #[cfg(test)]
    pub fn dirty_partition_scopes(&self) -> impl Iterator<Item = &PartitionSubscription> {
        self.warm
            .dirty_partition_scope_payload
            .iter()
            .map(|(_, scope)| scope)
    }

    pub fn clear_dirty_partition_scopes(&mut self) {
        self.hot.dirty_partition_scope_aspects = AspectMask::EMPTY;
        self.warm.dirty_partition_scope_payload.clear();
    }

    pub fn get_dirty_partition_scopes_for(
        &self,
        aspect: crate::data::aspect::Aspect,
    ) -> impl Iterator<Item = &PartitionSubscription> {
        self.warm
            .dirty_partition_scope_payload
            .iter()
            .filter(move |(candidate_aspect, _)| *candidate_aspect == aspect)
            .map(|(_, scope)| scope)
    }

    pub fn clear_dirty_partition_scopes_for(&mut self, aspect: crate::data::aspect::Aspect) {
        self.warm
            .dirty_partition_scope_payload
            .retain(|(candidate_aspect, _)| *candidate_aspect != aspect);
        self.sync_dirty_partition_scope_flag(aspect);
    }

    pub fn add_dirty_partition_scope(
        &mut self,
        aspect: crate::data::aspect::Aspect,
        scope: PartitionSubscription,
    ) {
        if !self.warm.dirty_partition_scope_payload.iter().any(
            |(candidate_aspect, candidate_scope)| {
                *candidate_aspect == aspect && *candidate_scope == scope
            },
        ) {
            self.warm
                .dirty_partition_scope_payload
                .push((aspect, scope));
            self.hot.dirty_partition_scope_aspects.insert(aspect);
        }
    }

    /// Add one dirty aspect to the current mask.
    pub fn add_dirty_aspect(&mut self, aspect: crate::data::aspect::Aspect) {
        self.hot.dirty_aspects.insert(aspect);
    }

    /// The current aspect versions.
    pub fn get_aspect_version(&self) -> AspectVersion {
        self.hot.aspect_version_header.global()
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub fn get_partitioned_aspect_version(&self, scope: &PartitionSubscription) -> AspectVersion {
        self.warm
            .aspect_version_overrides
            .scoped_or_global(scope, self.hot.aspect_version_header.global())
    }

    pub fn version_for_scope(&self, aspect: Aspect, scope: Option<&PartitionSubscription>) -> u64 {
        self.warm.aspect_version_overrides.version_for_scope(
            aspect,
            scope,
            self.hot.aspect_version_header.global(),
        )
    }

    /// Set the aspect version after evaluation.
    pub fn set_aspect_version(&mut self, version: AspectVersion) {
        self.hot.aspect_version_header.set_global(version);
        self.warm.aspect_version_overrides.set_global(version);
        self.hot
            .aspect_version_header
            .set_has_partition_overrides(self.warm.aspect_version_overrides.has_overrides());
    }

    pub fn apply_aspect_version(
        &mut self,
        version: AspectVersion,
        changed_regions: &[ChangedRegion],
    ) {
        self.hot.aspect_version_header.set_global(version);
        self.warm
            .aspect_version_overrides
            .apply_evaluation(version, changed_regions);
        self.hot
            .aspect_version_header
            .set_has_partition_overrides(self.warm.aspect_version_overrides.has_overrides());
    }

    /// Graph-owned dependency set handle.
    pub fn get_dependencies_id(&self) -> DependencySetId {
        self.hot.dependencies_id
    }

    /// Replace the dependency set handle.
    pub fn set_dependencies_id(&mut self, dependencies_id: DependencySetId) {
        self.hot.dependencies_id = dependencies_id;
    }

    /// Graph-owned subscriber set handle.
    pub fn get_subscribers_id(&self) -> SubscriberSetId {
        self.hot.subscribers_id
    }

    /// Replace the subscriber set handle.
    pub fn set_subscribers_id(&mut self, subscribers_id: SubscriberSetId) {
        self.hot.subscribers_id = subscribers_id;
    }

    /// The graph-owned dependency snapshot handle from the last clean evaluation.
    pub fn get_dep_snapshot_id(&self) -> DependencySnapshotId {
        self.hot.dep_snapshot_id
    }

    /// Replace the dependency snapshot handle.
    pub fn set_dep_snapshot_id(&mut self, snapshot_id: DependencySnapshotId) {
        self.hot.dep_snapshot_id = snapshot_id;
    }

    /// Whether this node is tombstoned.
    pub fn is_tombstoned(&self) -> bool {
        self.warm.tombstoned
    }

    /// Mark this node as tombstoned.
    #[cfg(test)]
    pub fn set_tombstoned(&mut self, tombstoned: bool) {
        self.warm.tombstoned = tombstoned;
    }

    /// The last operational artifact state.
    pub fn get_runtime_artifact_state(&self) -> Option<&RuntimeArtifactState> {
        self.warm.runtime_artifact_state.as_ref()
    }

    /// Set or clear the runtime artifact state.
    pub fn set_runtime_artifact_state(&mut self, state: Option<RuntimeArtifactState>) {
        self.warm.runtime_artifact_state = state;
    }

    /// Mutably access the runtime artifact state when an operation needs to
    /// update warm metadata in place without rebuilding the whole carrier.
    #[allow(dead_code)]
    pub fn runtime_artifact_state_mut(&mut self) -> Option<&mut RuntimeArtifactState> {
        self.warm.runtime_artifact_state.as_mut()
    }

    /// Retained diagnostic artifact payload, if any.
    pub fn retained_diagnostic_artifact(&self) -> Option<&RetainedDiagnosticArtifact> {
        self.cold.as_ref()?.retained_artifact.as_ref()
    }

    /// Cold retained artifact record, if any.
    pub fn cold_artifact_record(&self) -> Option<&RetainedDiagnosticArtifact> {
        self.retained_diagnostic_artifact()
    }

    /// Assemble a cold historical artifact record from the published hot/cold
    /// facades for this node entry.
    pub fn historical_artifact_record(
        &self,
        node: crate::data::handle::NodeId,
    ) -> Option<HistoricalArtifactRecord> {
        assemble_historical_artifact_record(
            node,
            self.get_runtime_artifact_state(),
            self.cold_artifact_record(),
            self.get_causality(),
        )
    }

    /// Assemble a trace summary from the published hot/cold facades for this
    /// node entry.
    pub fn trace_summary(&self) -> Option<TraceSummary> {
        assemble_trace_summary_with_execution(
            self.get_runtime_artifact_state(),
            self.cold_artifact_record(),
            self.execution_trace_stamp(),
        )
    }

    /// Set or clear the retained diagnostic artifact payload.
    pub fn set_retained_diagnostic_artifact(
        &mut self,
        artifact: Option<RetainedDiagnosticArtifact>,
    ) {
        self.cold_mut().retained_artifact = artifact;
        self.trim_cold_if_empty();
    }

    /// Cold execution/segment stamp, if any.
    pub fn execution_trace_stamp(&self) -> Option<ExecutionTraceStamp> {
        self.cold.as_ref()?.execution_trace
    }

    /// Set or clear the cold execution/segment stamp.
    pub fn set_execution_trace_stamp(&mut self, stamp: Option<ExecutionTraceStamp>) {
        self.cold_mut().execution_trace = stamp;
        self.trim_cold_if_empty();
    }

    /// Apply explicit hot/cold artifact lane updates without implying that the
    /// lanes are a single ambient payload.
    #[allow(dead_code)]
    pub fn apply_artifact_write_delta(&mut self, delta: ArtifactWriteDelta) {
        self.set_runtime_artifact_state(delta.runtime);
        self.set_retained_diagnostic_artifact(delta.retained);
    }

    /// Split a materialized trace summary back into runtime and retained
    /// storage lanes.
    #[cfg(test)]
    pub fn set_trace_summary(&mut self, summary: Option<TraceSummary>) {
        match summary {
            Some(summary) => {
                let retained_changed_regions = crate::data::output::CanonicalChangedRegions::from(
                    summary.changed_regions.clone(),
                );
                self.warm.runtime_artifact_state = Some(RuntimeArtifactState::new(
                    crate::data::trace::RuntimeArtifactHot {
                        output_hash: summary.output_hash,
                        output_change: summary.output_change,
                        recomputed: summary.recomputed,
                        dependency_count: summary.dependency_count,
                        meaningful_input_changes: summary.meaningful_input_changes,
                        changed_partition_count: summary.changed_partition_count,
                        propagation_suppressed: summary.propagation_suppressed,
                        changed_scopes: crate::data::trace::CompactChangedScopeProof::new(
                            crate::data::proof::PartitionScopeSet::from_changed_regions(
                                &retained_changed_regions,
                            ),
                        ),
                    },
                    crate::data::trace::RuntimeArtifactWarm {
                        output_identity: summary.output_identity,
                        continuity_token: crate::data::trace::ContinuityAuthorityToken::new(
                            summary.continuity_token,
                        ),
                        memoized_origin: summary.memoized_origin,
                        reuse_basis: crate::data::trace::ReuseOperationalBasis::new(
                            summary.reuse_basis,
                        ),
                        reuse_origin: summary.reuse_origin,
                        reuse_boundary_authority: summary
                            .reuse_boundary_context
                            .as_ref()
                            .map(|context| context.authority()),
                        lineage_artifact_id: crate::data::trace::ArtifactTransitionKey::new(
                            summary.lineage_artifact_id,
                        ),
                        merge_authority: ArtifactMergeAuthority::default(),
                    },
                ));
                self.set_execution_trace_stamp(Some(ExecutionTraceStamp {
                    execution_record_id: summary.execution_record_id,
                    semantic_segment_id: summary.semantic_segment_id,
                }));
                let retained = RetainedDiagnosticArtifact {
                    changed_regions: retained_changed_regions,
                    labels: summary.labels,
                    keyed_family: summary.keyed_family,
                    keyed_key: summary.keyed_key,
                    reuse_certification: None,
                    reuse_boundary_context: summary.reuse_boundary_context,
                };
                if retained.changed_regions.is_empty()
                    && retained.labels.is_empty()
                    && retained.keyed_family.is_none()
                    && retained.keyed_key.is_none()
                    && retained.reuse_certification.is_none()
                    && retained.reuse_boundary_context.is_none()
                {
                    self.set_retained_diagnostic_artifact(None);
                } else {
                    self.set_retained_diagnostic_artifact(Some(retained));
                }
            }
            None => {
                self.warm.runtime_artifact_state = None;
                self.set_retained_diagnostic_artifact(None);
                self.set_execution_trace_stamp(None);
            }
        }
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
        &self.warm.eval_config
    }

    /// Replace per-node evaluation policy descriptor.
    pub fn set_eval_config(&mut self, config: NodeEvaluationConfig) {
        self.warm.eval_config = config;
    }

    pub(crate) fn to_checkpoint_image(&self) -> CheckpointNodeImage {
        CheckpointNodeImage::from_parts(CheckpointNodeImageParts {
            state: self.hot.state,
            dirty_aspects: self.hot.dirty_aspects,
            dirty_partition_scopes: self
                .warm
                .dirty_partition_scope_payload
                .iter()
                .cloned()
                .collect(),
            aspect_versions: PartitionVersionMap::from_storage_parts(
                self.hot.aspect_version_header,
                self.warm.aspect_version_overrides.clone(),
            ),
            dependencies_id: self.hot.dependencies_id,
            subscribers_id: self.hot.subscribers_id,
            dep_snapshot_id: self.hot.dep_snapshot_id,
            tombstoned: self.warm.tombstoned,
            runtime_artifact_state: self.warm.runtime_artifact_state.clone(),
            retained_artifact: self.cold_artifact_record().cloned(),
            causality: self.get_causality().cloned(),
            execution_trace: self.execution_trace_stamp(),
            eval_config: self.warm.eval_config.clone(),
        })
    }

    pub(crate) fn from_checkpoint_image(image: CheckpointNodeImage) -> Self {
        let image = image.into_parts();
        let (aspect_version_header, aspect_version_overrides) =
            image.aspect_versions.into_storage_parts();
        let mut entry = Self {
            hot: NodeHotData {
                state: image.state,
                dirty_aspects: image.dirty_aspects,
                dirty_partition_scope_aspects: AspectMask::EMPTY,
                aspect_version_header,
                dependencies_id: image.dependencies_id,
                subscribers_id: image.subscribers_id,
                dep_snapshot_id: image.dep_snapshot_id,
            },
            warm: NodeWarmData {
                tombstoned: image.tombstoned,
                aspect_version_overrides,
                dirty_partition_scope_payload: image.dirty_partition_scopes.into_iter().collect(),
                runtime_artifact_state: image.runtime_artifact_state,
                eval_config: image.eval_config,
            },
            cold: None,
        };
        entry.sync_all_dirty_partition_scope_flags();
        entry.set_retained_diagnostic_artifact(image.retained_artifact);
        entry.set_causality(image.causality);
        entry.set_execution_trace_stamp(image.execution_trace);
        entry
    }

    pub(crate) fn from_storage_parts(
        hot: NodeHotData,
        warm: NodeWarmData,
        cold: Option<Box<NodeColdData>>,
    ) -> Self {
        Self { hot, warm, cold }
    }

    pub(crate) fn into_storage_parts(
        self,
    ) -> (NodeHotData, NodeWarmData, Option<Box<NodeColdData>>) {
        (self.hot, self.warm, self.cold)
    }

    fn cold_mut(&mut self) -> &mut NodeColdData {
        self.cold
            .get_or_insert_with(|| Box::new(NodeColdData::default()))
            .as_mut()
    }

    fn trim_cold_if_empty(&mut self) {
        if self.cold.as_ref().is_some_and(|cold| {
            cold.retained_artifact.is_none()
                && cold.causality.is_none()
                && cold.execution_trace.is_none()
        }) {
            self.cold = None;
        }
    }

    fn merge_dirty_partition_scopes(
        &mut self,
        changed_aspect: Aspect,
        changed_scopes: &[PartitionSubscription],
        was_clean: bool,
        already_dirty_for_aspect: bool,
    ) {
        if changed_scopes.is_empty() {
            // Whole-aspect invalidation supersedes scoped dirtiness for this aspect only.
            self.clear_dirty_partition_scopes_for(changed_aspect);
            return;
        }
        if !was_clean
            && already_dirty_for_aspect
            && self
                .get_dirty_partition_scopes_for(changed_aspect)
                .next()
                .is_none()
        {
            // An existing whole-aspect dirty mark is already stronger than any scoped follow-up.
            return;
        }
        for scope in changed_scopes {
            self.add_dirty_partition_scope(changed_aspect, scope.clone());
        }
    }

    fn sync_all_dirty_partition_scope_flags(&mut self) {
        self.hot.dirty_partition_scope_aspects = AspectMask::EMPTY;
        for (aspect, _) in &self.warm.dirty_partition_scope_payload {
            self.hot.dirty_partition_scope_aspects.insert(*aspect);
        }
    }

    fn sync_dirty_partition_scope_flag(&mut self, aspect: Aspect) {
        if self
            .warm
            .dirty_partition_scope_payload
            .iter()
            .any(|(candidate_aspect, _)| *candidate_aspect == aspect)
        {
            self.hot.dirty_partition_scope_aspects.insert(aspect);
        } else {
            self.hot.dirty_partition_scope_aspects = AspectMask::from_bits(
                self.hot.dirty_partition_scope_aspects.bits()
                    & !AspectMask::from_aspect(aspect).bits(),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NodeEntry;
    use crate::data::aspect::{Aspect, AspectVersion};
    use crate::data::output::{ChangedRegion, PartitionSubscription};
    use crate::data::trace::{CausalityMetadata, RuntimeArtifactState};

    #[test]
    fn checkpoint_image_round_trips_node_entry() {
        let mut entry = NodeEntry::new();
        entry.transition_dirty(
            Aspect::new(0),
            &[PartitionSubscription::partition_and_detail(
                "wing", "rib-12",
            )],
        );
        entry.apply_aspect_version(
            AspectVersion::from_updates([(Aspect::new(0), 7)]),
            &[ChangedRegion::new("wing").with_detail("rib-12")],
        );
        entry.set_tombstoned(true);
        let mut runtime = RuntimeArtifactState::default();
        runtime.hot_mut().dependency_count = 3;
        entry.set_runtime_artifact_state(Some(runtime));
        entry.set_causality(Some(CausalityMetadata {
            kind: "checkpoint-test".to_string(),
            fields: Default::default(),
        }));

        let image = entry.to_checkpoint_image();
        let restored = NodeEntry::from_checkpoint_image(image);

        assert_eq!(restored.get_state(), entry.get_state());
        assert_eq!(restored.get_dirty_aspects(), entry.get_dirty_aspects());
        assert_eq!(
            restored.get_dirty_partition_scopes(),
            entry.get_dirty_partition_scopes()
        );
        assert_eq!(restored.get_aspect_version(), entry.get_aspect_version());
        assert_eq!(restored.is_tombstoned(), entry.is_tombstoned());
        assert_eq!(
            restored.get_runtime_artifact_state(),
            entry.get_runtime_artifact_state()
        );
        assert_eq!(restored.get_causality(), entry.get_causality());
    }
}
