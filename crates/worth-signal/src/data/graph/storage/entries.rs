use crate::clock::RuntimeInstant;
use crate::data::dependency::{
    CommittedSnapshotUpdate, DependencySnapshot, DependencySnapshotShapeStore, SnapshotDeltaRecord,
    SnapshotStorageStrategy,
};
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::node::{
    CheckpointNodeImage, CheckpointNodeImageParts, NodeColdData, NodeContract, NodeEntry,
    NodeEvaluationConfig, NodeHotData, NodeState, NodeWarmData,
};
use crate::data::output::PartitionSubscription;
use crate::data::proof::{
    ClassifiedSnapshotBatchCommit, MixedSnapshotBatchCommit, PendingSnapshotBatch,
    SnapshotBatchCommit, StableShapeSnapshotBatchCommit,
};
use crate::data::reuse::{PersistentCorrespondenceKind, ReuseBasis};
use crate::data::trace::{
    CausalityMetadata, ColdArtifactRecord, ExecutionTraceStamp, RetainedDiagnosticArtifact,
    RuntimeArtifactFinalizeImage, RuntimeArtifactHot, RuntimeArtifactOperationalSummary,
    RuntimeArtifactReuseBoundarySnapshot, RuntimeArtifactWarm, TraceSummary,
};
use crate::data::{aspect::AspectVersion, core_profile::StableHashValue, output::ChangedRegion};
use std::ops::{Deref, DerefMut};

use super::super::node_builder::NodeBuilder;
use super::super::signal_graph::stale_error;
use super::super::signal_graph::{DependencySnapshotStructuralDelta, SignalGraph};

pub(crate) struct MaterializedEntryRef(NodeEntry);

impl Deref for MaterializedEntryRef {
    type Target = NodeEntry;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(crate) struct MaterializedEntryGuard<'a> {
    graph: &'a mut SignalGraph,
    id: NodeId,
    entry: NodeEntry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct NodeReplayProjection {
    pub lineage_artifact_id: Option<crate::diagnostics::lineage::LineageArtifactId>,
    pub persistent_correspondence_kind: Option<PersistentCorrespondenceKind>,
    pub composition_region_count: Option<u32>,
}

impl Deref for MaterializedEntryGuard<'_> {
    type Target = NodeEntry;

    fn deref(&self) -> &Self::Target {
        &self.entry
    }
}

impl DerefMut for MaterializedEntryGuard<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entry
    }
}

impl Drop for MaterializedEntryGuard<'_> {
    fn drop(&mut self) {
        let entry = std::mem::take(&mut self.entry);
        self.graph.write_back_materialized_entry(self.id, entry);
    }
}

impl SignalGraph {
    #[doc(hidden)]
    pub fn create_node(&mut self) -> NodeId {
        let entry = NodeEntry::new();
        self.allocate_node(entry)
    }

    pub(crate) fn create_node_from_checkpoint_image(
        &mut self,
        image: CheckpointNodeImage,
    ) -> NodeId {
        self.allocate_node(NodeEntry::from_checkpoint_image(image))
    }

    pub fn node(&mut self) -> NodeBuilder<'_> {
        NodeBuilder::new(self)
    }

    #[doc(hidden)]
    pub fn create_node_with_config(&mut self, config: NodeEvaluationConfig) -> NodeId {
        let mut entry = NodeEntry::new();
        entry.set_eval_config(config);
        self.allocate_node(entry)
    }

    pub fn get_state(&self, id: NodeId) -> Result<NodeState, SignalError> {
        self.validate_handle(id)?;
        self.arena
            .hot
            .get(id.index() as usize)
            .and_then(Option::as_ref)
            .map(|hot| hot.state)
            .ok_or_else(|| stale_error(id, id.generation()))
    }

    pub(crate) fn node_dependency_ids(
        &self,
        id: NodeId,
    ) -> Result<
        (
            super::super::DependencySetId,
            crate::data::dependency::DependencySnapshotId,
        ),
        SignalError,
    > {
        let hot = self.hot_ref(id)?;
        Ok((hot.dependencies_id, hot.dep_snapshot_id))
    }

    pub(crate) fn node_subscribers_id(
        &self,
        id: NodeId,
    ) -> Result<super::super::SubscriberSetId, SignalError> {
        Ok(self.hot_ref(id)?.subscribers_id)
    }

    pub(crate) fn node_eval_config(
        &self,
        id: NodeId,
    ) -> Result<&NodeEvaluationConfig, SignalError> {
        Ok(&self.warm_ref(id)?.eval_config)
    }

    pub(crate) fn node_dirty_aspects(
        &self,
        id: NodeId,
    ) -> Result<crate::data::aspect::AspectMask, SignalError> {
        Ok(self.hot_ref(id)?.dirty_aspects)
    }

    pub(crate) fn node_dirty_partition_scopes(
        &self,
        id: NodeId,
    ) -> Result<Vec<PartitionSubscription>, SignalError> {
        Ok(self
            .warm_ref(id)?
            .dirty_partition_scope_payload
            .iter()
            .map(|(_, scope)| scope.clone())
            .collect())
    }

    pub(crate) fn node_dirty_partition_scopes_present(
        &self,
        id: NodeId,
    ) -> Result<bool, SignalError> {
        Ok(!self.hot_ref(id)?.dirty_partition_scope_aspects.is_empty())
    }

    pub(crate) fn node_state(&self, id: NodeId) -> Result<NodeState, SignalError> {
        Ok(self.hot_ref(id)?.state)
    }

    pub(crate) fn node_runtime_artifact_hot(
        &self,
        id: NodeId,
    ) -> Result<Option<&RuntimeArtifactHot>, SignalError> {
        Ok(self
            .warm_ref(id)?
            .runtime_artifact_state
            .as_ref()
            .map(|state| state.hot()))
    }

    pub(crate) fn node_runtime_artifact_warm(
        &self,
        id: NodeId,
    ) -> Result<Option<&RuntimeArtifactWarm>, SignalError> {
        crate::data::access_counters::note_runtime_artifact_warm_read();
        Ok(self
            .warm_ref(id)?
            .runtime_artifact_state
            .as_ref()
            .map(|state| state.warm()))
    }

    pub(crate) fn node_runtime_artifact_reuse_boundary_snapshot(
        &self,
        id: NodeId,
    ) -> Result<Option<RuntimeArtifactReuseBoundarySnapshot>, SignalError> {
        Ok(self
            .warm_ref(id)?
            .runtime_artifact_state
            .as_ref()
            .map(crate::data::trace::RuntimeArtifactState::reuse_boundary_snapshot))
    }

    pub(crate) fn node_runtime_artifact_operational_summary(
        &self,
        id: NodeId,
    ) -> Result<Option<RuntimeArtifactOperationalSummary>, SignalError> {
        Ok(self
            .warm_ref(id)?
            .runtime_artifact_state
            .as_ref()
            .map(crate::data::trace::RuntimeArtifactState::operational_summary))
    }

    pub(crate) fn node_runtime_artifact_state(
        &self,
        id: NodeId,
    ) -> Result<Option<&crate::data::trace::RuntimeArtifactState>, SignalError> {
        crate::data::access_counters::note_runtime_artifact_state_read();
        Ok(self.warm_ref(id)?.runtime_artifact_state.as_ref())
    }

    pub(crate) fn node_runtime_artifact_finalize_image(
        &self,
        id: NodeId,
    ) -> Result<Option<RuntimeArtifactFinalizeImage>, SignalError> {
        Ok(self
            .warm_ref(id)?
            .runtime_artifact_state
            .as_ref()
            .map(RuntimeArtifactFinalizeImage::from_runtime_state))
    }

    pub(crate) fn node_runtime_artifact_state_present(
        &self,
        id: NodeId,
    ) -> Result<bool, SignalError> {
        Ok(self.warm_ref(id)?.runtime_artifact_state.is_some())
    }

    pub(crate) fn node_checkpoint_image(
        &self,
        id: NodeId,
    ) -> Result<CheckpointNodeImage, SignalError> {
        let hot = self.hot_ref(id)?;
        let warm = self.warm_ref(id)?;
        let cold = self.cold_ref(id)?;
        Ok(CheckpointNodeImage::from_parts(CheckpointNodeImageParts {
            state: hot.state,
            dirty_aspects: hot.dirty_aspects,
            dirty_partition_scopes: warm.dirty_partition_scope_payload.iter().cloned().collect(),
            aspect_versions: crate::data::aspect::PartitionVersionMap::from_storage_parts(
                hot.aspect_version_header,
                warm.aspect_version_overrides.clone(),
            ),
            dependencies_id: hot.dependencies_id,
            subscribers_id: hot.subscribers_id,
            dep_snapshot_id: hot.dep_snapshot_id,
            tombstoned: warm.tombstoned,
            runtime_artifact_state: warm.runtime_artifact_state.clone(),
            retained_artifact: cold.and_then(|cold| cold.retained_artifact.clone()),
            causality: cold.and_then(|cold| cold.causality.clone()),
            execution_trace: cold.and_then(|cold| cold.execution_trace),
            eval_config: warm.eval_config.clone(),
        }))
    }

    pub(crate) fn node_condition(
        &self,
        id: NodeId,
    ) -> Result<crate::data::node::EvaluationCondition, SignalError> {
        Ok(self.warm_ref(id)?.eval_config.condition.clone())
    }

    pub fn node_aspect_version(
        &self,
        id: NodeId,
    ) -> Result<crate::data::aspect::AspectVersion, SignalError> {
        Ok(self.hot_ref(id)?.aspect_version_header.global())
    }

    pub fn set_trace_summary(
        &mut self,
        id: NodeId,
        summary: Option<TraceSummary>,
    ) -> Result<(), SignalError> {
        let mut entry = self.get_entry_mut(id)?;
        entry.set_retained_diagnostic_artifact(summary.map(|summary| RetainedDiagnosticArtifact {
            labels: summary.labels,
            ..RetainedDiagnosticArtifact::default()
        }));
        Ok(())
    }

    pub(crate) fn node_partitioned_aspect_version(
        &self,
        id: NodeId,
        scope: &PartitionSubscription,
    ) -> Result<AspectVersion, SignalError> {
        let hot = self.hot_ref(id)?;
        let warm = self.warm_ref(id)?;
        Ok(warm
            .aspect_version_overrides
            .scoped_or_global(scope, hot.aspect_version_header.global()))
    }

    pub(crate) fn node_version_for_scope(
        &self,
        id: NodeId,
        aspect: crate::data::aspect::Aspect,
        scope: Option<&PartitionSubscription>,
    ) -> Result<u64, SignalError> {
        let hot = self.hot_ref(id)?;
        let warm = self.warm_ref(id)?;
        Ok(warm.aspect_version_overrides.version_for_scope(
            aspect,
            scope,
            hot.aspect_version_header.global(),
        ))
    }

    pub(crate) fn get_entry(&self, id: NodeId) -> Result<MaterializedEntryRef, SignalError> {
        Ok(MaterializedEntryRef(self.materialize_entry(id)?))
    }

    pub(crate) fn get_entry_mut(
        &mut self,
        id: NodeId,
    ) -> Result<MaterializedEntryGuard<'_>, SignalError> {
        let entry = self.materialize_entry(id)?;
        Ok(MaterializedEntryGuard {
            graph: self,
            id,
            entry,
        })
    }

    fn hot_ref(&self, id: NodeId) -> Result<&NodeHotData, SignalError> {
        self.validate_handle(id)?;
        self.arena
            .hot
            .get(id.index() as usize)
            .and_then(Option::as_ref)
            .ok_or_else(|| stale_error(id, id.generation()))
    }

    fn warm_ref(&self, id: NodeId) -> Result<&NodeWarmData, SignalError> {
        self.validate_handle(id)?;
        self.arena
            .warm
            .get(id.index() as usize)
            .ok_or_else(|| stale_error(id, id.generation()))
    }

    fn cold_ref(&self, id: NodeId) -> Result<Option<&NodeColdData>, SignalError> {
        self.validate_handle(id)?;
        Ok(self
            .arena
            .cold
            .get(id.index() as usize)
            .and_then(|cold| cold.as_deref()))
    }

    fn hot_mut(&mut self, id: NodeId) -> Result<&mut NodeHotData, SignalError> {
        self.validate_handle(id)?;
        self.arena
            .hot
            .get_mut(id.index() as usize)
            .and_then(Option::as_mut)
            .ok_or_else(|| stale_error(id, id.generation()))
    }

    fn warm_mut(&mut self, id: NodeId) -> Result<&mut NodeWarmData, SignalError> {
        self.validate_handle(id)?;
        self.arena
            .warm
            .get_mut(id.index() as usize)
            .ok_or_else(|| stale_error(id, id.generation()))
    }

    fn cold_mut(&mut self, id: NodeId) -> Result<&mut NodeColdData, SignalError> {
        self.validate_handle(id)?;
        Ok(self.arena.cold[id.index() as usize]
            .get_or_insert_with(|| Box::new(NodeColdData::default()))
            .as_mut())
    }

    fn trim_cold_if_empty(&mut self, id: NodeId) {
        let index = id.index() as usize;
        if self.arena.cold[index].as_ref().is_some_and(|cold| {
            cold.retained_artifact.is_none()
                && cold.causality.is_none()
                && cold.execution_trace.is_none()
        }) {
            self.arena.cold[index] = None;
        }
    }

    fn set_dep_snapshot_id_direct(
        &mut self,
        id: NodeId,
        snapshot_id: crate::data::dependency::DependencySnapshotId,
    ) -> Result<(), SignalError> {
        self.hot_mut(id)?.dep_snapshot_id = snapshot_id;
        Ok(())
    }

    pub(crate) fn set_dependencies_id_direct(
        &mut self,
        id: NodeId,
        dependencies_id: super::super::DependencySetId,
    ) -> Result<(), SignalError> {
        self.hot_mut(id)?.dependencies_id = dependencies_id;
        Ok(())
    }

    pub(crate) fn set_subscribers_id_direct(
        &mut self,
        id: NodeId,
        subscribers_id: super::super::SubscriberSetId,
    ) -> Result<(), SignalError> {
        self.hot_mut(id)?.subscribers_id = subscribers_id;
        Ok(())
    }

    fn materialize_entry(&self, id: NodeId) -> Result<NodeEntry, SignalError> {
        crate::data::access_counters::note_materialized_entry_read();
        Ok(NodeEntry::from_storage_parts(
            self.hot_ref(id)?.clone(),
            self.warm_ref(id)?.clone(),
            self.cold_ref(id)?.map(|cold| Box::new(cold.clone())),
        ))
    }

    fn write_back_materialized_entry(&mut self, id: NodeId, entry: NodeEntry) {
        crate::data::access_counters::note_materialized_entry_write();
        let index = id.index() as usize;
        let (hot, warm, cold) = entry.into_storage_parts();
        self.arena.hot[index] = Some(hot);
        self.arena.warm[index] = warm;
        self.arena.cold[index] = cold;
    }

    pub fn get_contract(&self, id: NodeId) -> Result<&NodeContract, SignalError> {
        Ok(&self.warm_ref(id)?.eval_config.contract)
    }

    pub fn node_schema_binding(
        &self,
        id: NodeId,
    ) -> Result<Option<&crate::schema::data::SignalSchemaBinding>, SignalError> {
        Ok(self.warm_ref(id)?.eval_config.schema_binding.as_ref())
    }

    pub fn node_merge_strategy_name(
        &self,
        id: NodeId,
    ) -> Result<Option<&crate::logic::transaction::MergeStrategyName>, SignalError> {
        Ok(self.warm_ref(id)?.eval_config.merge_strategy_name.as_ref())
    }

    pub fn node_conflict_policy_name(
        &self,
        id: NodeId,
    ) -> Result<Option<&crate::logic::transaction::ConflictPolicyName>, SignalError> {
        Ok(self.warm_ref(id)?.eval_config.conflict_policy_name.as_ref())
    }

    pub fn node_identity_matcher_name(
        &self,
        id: NodeId,
    ) -> Result<Option<&crate::logic::transaction::IdentityMatcherName>, SignalError> {
        Ok(self
            .warm_ref(id)?
            .eval_config
            .identity_matcher_name
            .as_ref())
    }

    pub fn node_source_only_policy_name(
        &self,
        id: NodeId,
    ) -> Result<Option<&crate::logic::transaction::SourceOnlyPolicyName>, SignalError> {
        Ok(self
            .warm_ref(id)?
            .eval_config
            .source_only_policy_name
            .as_ref())
    }

    pub fn node_deletion_policy_name(
        &self,
        id: NodeId,
    ) -> Result<Option<&crate::logic::transaction::DeletionPolicyName>, SignalError> {
        Ok(self.warm_ref(id)?.eval_config.deletion_policy_name.as_ref())
    }

    pub fn node_conflict_isolation_policy_name(
        &self,
        id: NodeId,
    ) -> Result<Option<&crate::logic::transaction::ConflictIsolationPolicyName>, SignalError> {
        Ok(self
            .warm_ref(id)?
            .eval_config
            .conflict_isolation_policy_name
            .as_ref())
    }

    pub fn node_aspect_merge_policy_bindings(
        &self,
        id: NodeId,
    ) -> Result<&[crate::logic::transaction::AspectMergePolicyBinding], SignalError> {
        Ok(&self.warm_ref(id)?.eval_config.aspect_merge_policy_bindings)
    }

    pub fn validate_schema_bindings_against(
        &self,
        schema_registry: &crate::schema::data::SignalSchemaRegistry,
    ) -> Result<(), SignalError> {
        for node in self.live_node_ids() {
            let Some(binding) = self.node_schema_binding(node)? else {
                continue;
            };
            let descriptor = schema_registry
                .resolve_by_id(binding.schema_id())
                .ok_or_else(|| {
                    SignalError::invalid_input(format!(
                        "node {} references unknown schema id `{}`",
                        node,
                        binding.schema_id().0
                    ))
                })?;
            if descriptor.semantic_name() != binding.semantic_name() {
                return Err(SignalError::invalid_input(format!(
                    "node {} schema binding name mismatch: binding=`{}`, registry=`{}`",
                    node,
                    binding.semantic_name().as_str(),
                    descriptor.semantic_name().as_str()
                )));
            }
            if descriptor.version() != binding.version() {
                return Err(SignalError::invalid_input(format!(
                    "node {} schema binding version mismatch for `{}`",
                    node,
                    binding.semantic_name().as_str()
                )));
            }
            if descriptor.digest() != binding.descriptor_digest() {
                return Err(SignalError::invalid_input(format!(
                    "node {} schema binding digest mismatch for `{}`",
                    node,
                    binding.semantic_name().as_str()
                )));
            }
        }
        Ok(())
    }

    pub fn validate_merge_semantics_against(
        &self,
        schema_registry: &crate::schema::data::SignalSchemaRegistry,
        merge_strategy_registry: &crate::logic::transaction::FrozenMergeStrategyRegistry,
        aspect_merge_policy_registry: &crate::logic::transaction::FrozenAspectMergePolicyRegistry,
        conflict_isolation_registry: &crate::logic::transaction::FrozenConflictIsolationRegistry,
        conflict_policy_registry: &crate::logic::transaction::FrozenConflictPolicyRegistry,
        identity_matcher_registry: &crate::logic::transaction::FrozenIdentityMatcherRegistry,
        source_only_policy_registry: &crate::logic::transaction::FrozenSourceOnlyPolicyRegistry,
        deletion_policy_registry: &crate::logic::transaction::FrozenDeletionPolicyRegistry,
    ) -> Result<(), SignalError> {
        for registration in schema_registry.iter() {
            let descriptor = registration.descriptor();
            if let Some(strategy_name) = descriptor.default_merge_strategy_name() {
                if merge_strategy_registry
                    .resolve_by_name(strategy_name)
                    .is_none()
                {
                    return Err(SignalError::invalid_input(format!(
                        "schema `{}` references unknown default merge strategy `{}`",
                        descriptor.semantic_name().as_str(),
                        strategy_name.as_str()
                    )));
                }
            }
            if let Some(policy_name) = descriptor.default_conflict_policy_name() {
                if conflict_policy_registry
                    .resolve_by_name(policy_name)
                    .is_none()
                {
                    return Err(SignalError::invalid_input(format!(
                        "schema `{}` references unknown default conflict policy `{}`",
                        descriptor.semantic_name().as_str(),
                        policy_name.as_str()
                    )));
                }
            }
            if let Some(matcher_name) = descriptor.default_identity_matcher_name() {
                if identity_matcher_registry
                    .resolve_by_name(matcher_name)
                    .is_none()
                {
                    return Err(SignalError::invalid_input(format!(
                        "schema `{}` references unknown default identity matcher `{}`",
                        descriptor.semantic_name().as_str(),
                        matcher_name.as_str()
                    )));
                }
            }
            if let Some(policy_name) = descriptor.default_source_only_policy_name() {
                if source_only_policy_registry
                    .resolve_by_name(policy_name)
                    .is_none()
                {
                    return Err(SignalError::invalid_input(format!(
                        "schema `{}` references unknown default source-only policy `{}`",
                        descriptor.semantic_name().as_str(),
                        policy_name.as_str()
                    )));
                }
            }
            if let Some(policy_name) = descriptor.default_deletion_policy_name() {
                if deletion_policy_registry
                    .resolve_by_name(policy_name)
                    .is_none()
                {
                    return Err(SignalError::invalid_input(format!(
                        "schema `{}` references unknown default deletion policy `{}`",
                        descriptor.semantic_name().as_str(),
                        policy_name.as_str()
                    )));
                }
            }
            if let Some(policy_name) = descriptor.default_conflict_isolation_policy_name() {
                if conflict_isolation_registry
                    .resolve_by_name(policy_name)
                    .is_none()
                {
                    return Err(SignalError::invalid_input(format!(
                        "schema `{}` references unknown default conflict isolation policy `{}`",
                        descriptor.semantic_name().as_str(),
                        policy_name.as_str()
                    )));
                }
            }
            for binding in descriptor.default_aspect_merge_policy_bindings() {
                if aspect_merge_policy_registry
                    .resolve_by_name(&binding.policy_name)
                    .is_none()
                {
                    return Err(SignalError::invalid_input(format!(
                        "schema `{}` references unknown aspect merge policy `{}` for aspect {}",
                        descriptor.semantic_name().as_str(),
                        binding.policy_name.as_str(),
                        binding.aspect.id()
                    )));
                }
            }
        }

        for node in self.live_node_ids() {
            if let Some(strategy_name) = self.node_merge_strategy_name(node)? {
                if merge_strategy_registry
                    .resolve_by_name(strategy_name)
                    .is_none()
                {
                    return Err(SignalError::invalid_input(format!(
                        "node {} references unknown merge strategy `{}`",
                        node,
                        strategy_name.as_str()
                    )));
                }
            }
            if let Some(policy_name) = self.node_conflict_policy_name(node)? {
                if conflict_policy_registry
                    .resolve_by_name(policy_name)
                    .is_none()
                {
                    return Err(SignalError::invalid_input(format!(
                        "node {} references unknown conflict policy `{}`",
                        node,
                        policy_name.as_str()
                    )));
                }
            }
            if let Some(matcher_name) = self.node_identity_matcher_name(node)? {
                if identity_matcher_registry
                    .resolve_by_name(matcher_name)
                    .is_none()
                {
                    return Err(SignalError::invalid_input(format!(
                        "node {} references unknown identity matcher `{}`",
                        node,
                        matcher_name.as_str()
                    )));
                }
            }
            if let Some(policy_name) = self.node_source_only_policy_name(node)? {
                if source_only_policy_registry
                    .resolve_by_name(policy_name)
                    .is_none()
                {
                    return Err(SignalError::invalid_input(format!(
                        "node {} references unknown source-only policy `{}`",
                        node,
                        policy_name.as_str()
                    )));
                }
            }
            if let Some(policy_name) = self.node_deletion_policy_name(node)? {
                if deletion_policy_registry
                    .resolve_by_name(policy_name)
                    .is_none()
                {
                    return Err(SignalError::invalid_input(format!(
                        "node {} references unknown deletion policy `{}`",
                        node,
                        policy_name.as_str()
                    )));
                }
            }
            if let Some(policy_name) = self.node_conflict_isolation_policy_name(node)? {
                if conflict_isolation_registry
                    .resolve_by_name(policy_name)
                    .is_none()
                {
                    return Err(SignalError::invalid_input(format!(
                        "node {} references unknown conflict isolation policy `{}`",
                        node,
                        policy_name.as_str()
                    )));
                }
            }
            for binding in self.node_aspect_merge_policy_bindings(node)? {
                if aspect_merge_policy_registry
                    .resolve_by_name(&binding.policy_name)
                    .is_none()
                {
                    return Err(SignalError::invalid_input(format!(
                        "node {} references unknown aspect merge policy `{}` for aspect {}",
                        node,
                        binding.policy_name.as_str(),
                        binding.aspect.id()
                    )));
                }
            }
        }

        Ok(())
    }

    pub(crate) fn get_dep_snapshot(&self, id: NodeId) -> Result<&DependencySnapshot, SignalError> {
        Ok(self
            .topology
            .dependency_snapshots
            .get(self.hot_ref(id)?.dep_snapshot_id))
    }

    pub(crate) fn dependency_snapshot_shapes_mut(&mut self) -> &mut DependencySnapshotShapeStore {
        &mut self.topology.dependency_snapshot_shapes
    }

    pub(crate) fn dependency_snapshot_shape_handle(
        &mut self,
        id: crate::data::dependency::DependencySnapshotId,
    ) -> crate::data::dependency::SnapshotShapeHandle {
        self.topology
            .dependency_snapshots
            .shape_handle_for(id, &mut self.topology.dependency_snapshot_shapes)
    }

    fn insert_dependency_snapshot(
        &mut self,
        snapshot: DependencySnapshot,
    ) -> crate::data::dependency::DependencySnapshotId {
        self.topology
            .dependency_snapshots
            .insert_with_shape_handle(snapshot, &mut self.topology.dependency_snapshot_shapes)
            .0
    }

    pub(crate) fn set_dep_snapshot(
        &mut self,
        id: NodeId,
        snapshot: DependencySnapshot,
    ) -> Result<(), SignalError> {
        let previous = self.get_dep_snapshot(id)?.clone();
        let (_, previous_snapshot_id) = self.node_dependency_ids(id)?;
        let previous_shape_handle = self.dependency_snapshot_shape_handle(previous_snapshot_id);
        let (update, delta) = CommittedSnapshotUpdate::between(
            id,
            previous_snapshot_id,
            previous_shape_handle,
            &previous,
            snapshot,
            self.dependency_snapshot_shapes_mut(),
        );
        if !delta.changed() {
            return Ok(());
        }
        match update.storage_strategy() {
            SnapshotStorageStrategy::SharedReplacement => {
                self.telemetry_mut()
                    .storage
                    .shared_snapshot_replacement_count += 1;
                self.telemetry_mut()
                    .storage
                    .structural_replace_batch_commit_count += 1;
            }
            SnapshotStorageStrategy::VersionOnlyDelta => {
                self.telemetry_mut()
                    .storage
                    .version_only_snapshot_update_count += 1;
                self.telemetry_mut().storage.stable_shape_batch_commit_count += 1;
                self.telemetry_mut().storage.snapshot_shape_reuse_count += 1;
            }
        }
        let snapshot_id =
            self.insert_dependency_snapshot(update.apply_to(&previous).into_snapshot());
        self.set_dep_snapshot_id_direct(id, snapshot_id)?;
        self.record_branch_mutation_snapshot(
            id,
            DependencySnapshotStructuralDelta::from_snapshot_delta(delta),
        );
        self.record_graph_storage_pressure();
        Ok(())
    }

    pub(crate) fn replace_dep_snapshot_committed(
        &mut self,
        id: NodeId,
        update: CommittedSnapshotUpdate,
    ) -> Result<SnapshotDeltaRecord, SignalError> {
        let previous = self.get_dep_snapshot(id)?.clone();
        let delta = match &update {
            CommittedSnapshotUpdate::VersionOnly(version_only) => {
                SnapshotDeltaRecord::for_version_update(
                    id,
                    &previous,
                    version_only.versions().as_slice(),
                )
            }
            CommittedSnapshotUpdate::Replace(replacement) => {
                SnapshotDeltaRecord::between(id, &previous, replacement.snapshot())
            }
        };
        match update.storage_strategy() {
            SnapshotStorageStrategy::SharedReplacement => {
                self.telemetry_mut()
                    .storage
                    .shared_snapshot_replacement_count += 1;
                self.telemetry_mut()
                    .storage
                    .structural_replace_batch_commit_count += 1;
            }
            SnapshotStorageStrategy::VersionOnlyDelta => {
                self.telemetry_mut()
                    .storage
                    .version_only_snapshot_update_count += 1;
                self.telemetry_mut().storage.stable_shape_batch_commit_count += 1;
                self.telemetry_mut().storage.snapshot_shape_reuse_count += 1;
            }
        }
        if !delta.changed() {
            return Ok(delta);
        }
        let next_snapshot = update.apply_to(&previous);
        let snapshot_id = self.insert_dependency_snapshot(next_snapshot.into_snapshot());
        self.set_dep_snapshot_id_direct(id, snapshot_id)?;
        self.record_branch_mutation_snapshot(
            id,
            DependencySnapshotStructuralDelta::from_snapshot_delta(delta),
        );
        self.record_graph_storage_pressure();
        Ok(delta)
    }

    pub(crate) fn apply_stable_shape_snapshot_batch_commit(
        &mut self,
        commit: StableShapeSnapshotBatchCommit,
    ) -> Result<(), SignalError> {
        if commit.is_empty() {
            return Ok(());
        }
        self.telemetry_mut().storage.snapshot_batch_size += commit.pending().len() as u64;
        self.telemetry_mut().storage.stable_shape_batch_commit_count += 1;

        for snapshot in commit.pending() {
            self.validate_handle(snapshot.node())?;
        }

        for snapshot in commit.pending() {
            if !snapshot.delta().changed() {
                continue;
            }
            self.telemetry_mut().storage.patch_application_breadth += 1;
            self.telemetry_mut()
                .storage
                .version_only_snapshot_update_count += 1;
            self.telemetry_mut().storage.snapshot_shape_reuse_count += 1;
            let previous = self.get_dep_snapshot(snapshot.node())?.clone();
            let next_snapshot = CommittedSnapshotUpdate::VersionOnly(snapshot.update().clone())
                .apply_to(&previous)
                .into_snapshot();
            let snapshot_id = self.insert_dependency_snapshot(next_snapshot);
            self.set_dep_snapshot_id_direct(snapshot.node(), snapshot_id)?;
            self.record_branch_mutation_snapshot(
                snapshot.node(),
                DependencySnapshotStructuralDelta::from_snapshot_delta(snapshot.delta()),
            );
        }

        self.record_graph_storage_pressure();
        Ok(())
    }

    pub(crate) fn apply_mixed_snapshot_batch_commit(
        &mut self,
        commit: MixedSnapshotBatchCommit,
    ) -> Result<(), SignalError> {
        if commit.is_empty() {
            return Ok(());
        }
        self.telemetry_mut().storage.snapshot_batch_size +=
            (commit.stable_shape().len() + commit.replacements().len()) as u64;
        self.telemetry_mut()
            .storage
            .structural_replace_batch_commit_count += 1;

        for snapshot in commit.stable_shape() {
            self.validate_handle(snapshot.node())?;
        }
        for snapshot in commit.replacements() {
            self.validate_handle(snapshot.node())?;
        }

        for snapshot in commit.stable_shape() {
            if !snapshot.delta().changed() {
                continue;
            }
            self.telemetry_mut().storage.patch_application_breadth += 1;
            self.telemetry_mut()
                .storage
                .version_only_snapshot_update_count += 1;
            self.telemetry_mut().storage.snapshot_shape_reuse_count += 1;
            let previous = self.get_dep_snapshot(snapshot.node())?.clone();
            let next_snapshot = CommittedSnapshotUpdate::VersionOnly(snapshot.update().clone())
                .apply_to(&previous)
                .into_snapshot();
            let snapshot_id = self.insert_dependency_snapshot(next_snapshot);
            self.set_dep_snapshot_id_direct(snapshot.node(), snapshot_id)?;
            self.record_branch_mutation_snapshot(
                snapshot.node(),
                DependencySnapshotStructuralDelta::from_snapshot_delta(snapshot.delta()),
            );
        }

        for snapshot in commit.replacements() {
            if !snapshot.delta().changed() {
                continue;
            }
            self.telemetry_mut().storage.patch_application_breadth += 1;
            self.telemetry_mut()
                .storage
                .shared_snapshot_replacement_count += 1;
            let next_snapshot = CommittedSnapshotUpdate::Replace(snapshot.update().clone())
                .apply_to(self.get_dep_snapshot(snapshot.node())?)
                .into_snapshot();
            let snapshot_id = self.insert_dependency_snapshot(next_snapshot);
            self.set_dep_snapshot_id_direct(snapshot.node(), snapshot_id)?;
            self.record_branch_mutation_snapshot(
                snapshot.node(),
                DependencySnapshotStructuralDelta::from_snapshot_delta(snapshot.delta()),
            );
        }

        self.record_graph_storage_pressure();
        Ok(())
    }

    pub(crate) fn apply_classified_snapshot_batch_commit(
        &mut self,
        commit: ClassifiedSnapshotBatchCommit,
    ) -> Result<(), SignalError> {
        let commit_start = RuntimeInstant::now();
        let result = match commit {
            ClassifiedSnapshotBatchCommit::StableShape(commit) => {
                self.apply_stable_shape_snapshot_batch_commit(commit)
            }
            ClassifiedSnapshotBatchCommit::Mixed(commit) => {
                self.apply_mixed_snapshot_batch_commit(commit)
            }
        };
        self.telemetry_mut().storage.snapshot_batch_commit_nanos +=
            commit_start.elapsed().as_nanos();
        result
    }

    #[allow(dead_code)]
    pub(crate) fn derive_dependency_snapshot_restore_batch(
        &self,
        target: &SignalGraph,
    ) -> Result<SnapshotBatchCommit, SignalError> {
        let mut entries = Vec::new();
        for index in 0..target.arena_capacity() {
            let Some(node) = target.live_node_id_at(index) else {
                continue;
            };
            if !self.is_alive(node) {
                continue;
            }
            let previous = self.get_dep_snapshot(node)?.clone();
            let next = target.get_dep_snapshot(node)?.clone();
            let (_, previous_snapshot_id) = self.node_dependency_ids(node)?;
            let mut shape_store = self.topology.dependency_snapshot_shapes.clone();
            let previous_shape_handle = previous.shape().intern(&mut shape_store);
            let (update, delta) = CommittedSnapshotUpdate::between(
                node,
                previous_snapshot_id,
                previous_shape_handle,
                &previous,
                next,
                &mut shape_store,
            );
            if delta.changed() {
                entries.push(crate::data::proof::PendingSnapshotCommit {
                    node,
                    update,
                    delta,
                });
            }
        }
        Ok(SnapshotBatchCommit::new(PendingSnapshotBatch::new(entries)))
    }

    pub fn is_alive(&self, id: NodeId) -> bool {
        let idx = id.index() as usize;
        if idx >= self.arena.nodes.len() {
            return false;
        }
        let slot = &self.arena.nodes[idx];
        slot.generation == id.generation() && slot.is_occupied()
    }

    pub fn active_node_count(&self) -> usize {
        self.arena.active_nodes as usize
    }

    pub fn arena_capacity(&self) -> usize {
        self.arena.nodes.len()
    }

    pub(crate) fn live_node_id_at(&self, index: usize) -> Option<NodeId> {
        let slot = self.arena.nodes.get(index)?;
        if !slot.is_occupied() {
            return None;
        }
        Some(NodeId::new(index as u32, slot.generation))
    }

    pub(crate) fn replace_entry(
        &mut self,
        id: NodeId,
        entry: NodeEntry,
    ) -> Result<(), SignalError> {
        let mut target = self.get_entry_mut(id)?;
        *target = entry;
        drop(target);
        self.record_branch_mutation_state(id);
        Ok(())
    }

    pub(crate) fn replace_entry_from_checkpoint_image(
        &mut self,
        id: NodeId,
        image: CheckpointNodeImage,
    ) -> Result<(), SignalError> {
        self.replace_entry(id, NodeEntry::from_checkpoint_image(image))
    }

    pub(crate) fn node_runtime_artifact_structural_state(
        &self,
        node: NodeId,
    ) -> Result<
        (
            Option<crate::diagnostics::lineage::LineageArtifactId>,
            Option<StableHashValue>,
            Option<ReuseBasis>,
        ),
        SignalError,
    > {
        let runtime = self.warm_ref(node)?.runtime_artifact_state.as_ref();
        Ok((
            runtime.and_then(|state| state.lineage_artifact_id().get()),
            runtime.map(crate::data::trace::RuntimeArtifactState::output_hash),
            runtime.map(|state| state.reuse_basis().clone_inner()),
        ))
    }

    pub(crate) fn apply_node_aspect_version(
        &mut self,
        node: NodeId,
        version: AspectVersion,
        changed_regions: &[ChangedRegion],
    ) -> Result<(), SignalError> {
        let has_partition_overrides = {
            let warm = self.warm_mut(node)?;
            warm.aspect_version_overrides
                .apply_evaluation(version, changed_regions);
            warm.aspect_version_overrides.has_overrides()
        };
        let hot = self.hot_mut(node)?;
        hot.aspect_version_header.set_global(version);
        hot.aspect_version_header
            .set_has_partition_overrides(has_partition_overrides);
        Ok(())
    }

    pub(crate) fn apply_node_artifact_write_delta(
        &mut self,
        node: NodeId,
        delta: crate::data::trace::ArtifactWriteDelta,
    ) -> Result<bool, SignalError> {
        self.warm_mut(node)?.runtime_artifact_state = delta.runtime;
        let retained_present = delta.retained.is_some();
        if retained_present {
            self.cold_mut(node)?.retained_artifact = delta.retained;
        } else if let Some(cold) = self.arena.cold[node.index() as usize].as_mut() {
            cold.retained_artifact = None;
        }
        self.trim_cold_if_empty(node);
        Ok(retained_present)
    }

    pub(crate) fn transition_node_clean(&mut self, node: NodeId) -> Result<(), SignalError> {
        let hot = self.hot_mut(node)?;
        hot.state = NodeState::Clean;
        hot.dirty_aspects = crate::data::aspect::AspectMask::EMPTY;
        hot.dirty_partition_scope_aspects = crate::data::aspect::AspectMask::EMPTY;
        self.warm_mut(node)?.dirty_partition_scope_payload.clear();
        Ok(())
    }

    pub(crate) fn transition_node_dirty(
        &mut self,
        node: NodeId,
        aspect: crate::data::aspect::Aspect,
        scopes: &[PartitionSubscription],
    ) -> Result<(), SignalError> {
        let hot = self.hot_ref(node)?;
        let was_clean = matches!(hot.state, NodeState::Clean);
        let already_dirty_for_aspect = hot
            .dirty_aspects
            .contains(crate::data::aspect::AspectMask::from_aspect(aspect));
        let has_scoped_payload = {
            let warm = self.warm_mut(node)?;
            merge_dirty_partition_scopes(warm, aspect, scopes, was_clean, already_dirty_for_aspect)
        };
        let hot = self.hot_mut(node)?;
        hot.state = NodeState::Dirty;
        hot.dirty_aspects.insert(aspect);
        if has_scoped_payload {
            hot.dirty_partition_scope_aspects.insert(aspect);
        } else {
            sync_dirty_partition_scope_flag(hot, aspect);
        }
        Ok(())
    }

    pub(crate) fn transition_node_maybe_stale(
        &mut self,
        node: NodeId,
        aspect: crate::data::aspect::Aspect,
    ) -> Result<(), SignalError> {
        let hot = self.hot_ref(node)?;
        let was_clean = matches!(hot.state, NodeState::Clean);
        let already_dirty_for_aspect = hot
            .dirty_aspects
            .contains(crate::data::aspect::AspectMask::from_aspect(aspect));
        let should_clear_scopes = was_clean || !already_dirty_for_aspect;
        if should_clear_scopes {
            self.warm_mut(node)?
                .dirty_partition_scope_payload
                .retain(|(candidate_aspect, _)| *candidate_aspect != aspect);
        }
        let hot = self.hot_mut(node)?;
        hot.state = NodeState::MaybeStale;
        hot.dirty_aspects.insert(aspect);
        if should_clear_scopes {
            sync_dirty_partition_scope_flag(hot, aspect);
        }
        Ok(())
    }

    pub(crate) fn set_node_state(
        &mut self,
        node: NodeId,
        state: NodeState,
    ) -> Result<(), SignalError> {
        self.hot_mut(node)?.state = state;
        Ok(())
    }

    pub fn causality_of(&self, node: NodeId) -> Result<Option<&CausalityMetadata>, SignalError> {
        Ok(self
            .cold_ref(node)?
            .and_then(|cold| cold.causality.as_ref()))
    }

    pub(crate) fn node_execution_trace_stamp(
        &self,
        node: NodeId,
    ) -> Result<Option<ExecutionTraceStamp>, SignalError> {
        Ok(self.cold_ref(node)?.and_then(|cold| cold.execution_trace))
    }

    pub(crate) fn node_retained_diagnostic_artifact(
        &self,
        node: NodeId,
    ) -> Result<Option<&RetainedDiagnosticArtifact>, SignalError> {
        crate::data::access_counters::note_retained_artifact_read();
        Ok(self
            .cold_ref(node)?
            .and_then(|cold| cold.retained_artifact.as_ref()))
    }

    pub(crate) fn node_cold_artifact_record(
        &self,
        node: NodeId,
    ) -> Result<Option<&ColdArtifactRecord>, SignalError> {
        Ok(self
            .cold_ref(node)?
            .and_then(|cold| cold.retained_artifact.as_ref()))
    }

    pub(crate) fn node_lineage_artifact_id(
        &self,
        node: NodeId,
    ) -> Result<Option<crate::diagnostics::lineage::LineageArtifactId>, SignalError> {
        Ok(self
            .warm_ref(node)?
            .runtime_artifact_state
            .as_ref()
            .and_then(|state| state.lineage_artifact_id().get()))
    }

    pub(crate) fn node_replay_projection(
        &self,
        node: NodeId,
    ) -> Result<NodeReplayProjection, SignalError> {
        let runtime_artifact_state = self.warm_ref(node)?.runtime_artifact_state.as_ref();
        let lineage_artifact_id =
            runtime_artifact_state.and_then(|state| state.lineage_artifact_id().get());
        let (persistent_correspondence_kind, composition_region_count) = runtime_artifact_state
            .and_then(|state| state.reuse_boundary_authority())
            .map(|authority| {
                (
                    authority.persistent_correspondence_kind(),
                    authority.composition_region_count(),
                )
            })
            .unwrap_or((None, 0));
        Ok(NodeReplayProjection {
            lineage_artifact_id,
            persistent_correspondence_kind,
            composition_region_count: (composition_region_count > 0)
                .then_some(composition_region_count),
        })
    }

    pub fn set_causality(
        &mut self,
        node: NodeId,
        causality: Option<CausalityMetadata>,
    ) -> Result<(), SignalError> {
        if causality.is_some() {
            self.cold_mut(node)?.causality = causality;
        } else if let Some(cold) = self.arena.cold[node.index() as usize].as_mut() {
            cold.causality = None;
        }
        self.trim_cold_if_empty(node);
        self.record_branch_mutation_causality(node);
        Ok(())
    }

    pub(crate) fn stamp_runtime_artifact_lineage_and_execution(
        &mut self,
        node: NodeId,
        artifact_id: crate::diagnostics::lineage::LineageArtifactId,
        execution_record_id: crate::logic::planner::ExecutionRecordId,
        semantic_segment_id: crate::logic::planner::SemanticSegmentId,
    ) -> Result<(), SignalError> {
        let Some(runtime) = self.warm_mut(node)?.runtime_artifact_state.as_mut() else {
            return Ok(());
        };
        runtime.set_lineage_artifact_id(Some(artifact_id));
        self.cold_mut(node)?.execution_trace = Some(ExecutionTraceStamp {
            execution_record_id: Some(execution_record_id.0),
            semantic_segment_id: Some(semantic_segment_id.0),
        });
        Ok(())
    }
}

fn merge_dirty_partition_scopes(
    warm: &mut NodeWarmData,
    changed_aspect: crate::data::aspect::Aspect,
    changed_scopes: &[PartitionSubscription],
    was_clean: bool,
    already_dirty_for_aspect: bool,
) -> bool {
    if changed_scopes.is_empty() {
        warm.dirty_partition_scope_payload
            .retain(|(candidate_aspect, _)| *candidate_aspect != changed_aspect);
        return false;
    }
    if !was_clean
        && already_dirty_for_aspect
        && warm
            .dirty_partition_scope_payload
            .iter()
            .find(|(candidate_aspect, _)| *candidate_aspect == changed_aspect)
            .is_none()
    {
        return false;
    }
    for scope in changed_scopes {
        if !warm
            .dirty_partition_scope_payload
            .iter()
            .any(|(candidate_aspect, candidate_scope)| {
                *candidate_aspect == changed_aspect && *candidate_scope == *scope
            })
        {
            warm.dirty_partition_scope_payload
                .push((changed_aspect, scope.clone()));
        }
    }
    true
}

fn sync_dirty_partition_scope_flag(hot: &mut NodeHotData, aspect: crate::data::aspect::Aspect) {
    hot.dirty_partition_scope_aspects = crate::data::aspect::AspectMask::from_bits(
        hot.dirty_partition_scope_aspects.bits()
            & !crate::data::aspect::AspectMask::from_aspect(aspect).bits(),
    );
}
