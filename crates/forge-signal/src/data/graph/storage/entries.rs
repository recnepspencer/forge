use crate::data::dependency::{
    CommittedSnapshotUpdate, DependencySnapshot, DependencySnapshotShapeStore, SnapshotDeltaRecord,
    SnapshotStorageStrategy,
};
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::node::{
    CheckpointNodeImage, NodeColdData, NodeContract, NodeEntry, NodeEvaluationConfig, NodeHotData,
    NodeState, NodeWarmData,
};
use crate::data::output::PartitionSubscription;
use crate::data::proof::{
    ClassifiedSnapshotBatchCommit, MixedSnapshotBatchCommit, PendingSnapshotBatch,
    SnapshotBatchCommit, StableShapeSnapshotBatchCommit,
};
use crate::data::reuse::ReuseBasis;
use crate::data::trace::{
    CausalityMetadata, ColdArtifactRecord, ExecutionTraceStamp, RetainedDiagnosticArtifact,
    RuntimeArtifactFinalizeImage, RuntimeArtifactHot, RuntimeArtifactWarm,
};
use crate::data::{aspect::AspectVersion, core_profile::StableHashValue, output::ChangedRegion};
use std::ops::{Deref, DerefMut};
use std::time::Instant;

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

    pub(crate) fn create_node_from_checkpoint_image(&mut self, image: CheckpointNodeImage) -> NodeId {
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
        Ok(self
            .arena
            .hot
            .get(id.index() as usize)
            .and_then(Option::as_ref)
            .map(|hot| hot.state)
            .ok_or_else(|| stale_error(id, id.generation()))?)
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
            .hot_ref(id)?
            .dirty_partition_scopes
            .iter()
            .map(|(_, scope)| scope.clone())
            .collect())
    }

    pub(crate) fn node_dirty_partition_scopes_present(
        &self,
        id: NodeId,
    ) -> Result<bool, SignalError> {
        Ok(!self.hot_ref(id)?.dirty_partition_scopes.is_empty())
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

    pub(crate) fn node_runtime_artifact_state_present(&self, id: NodeId) -> Result<bool, SignalError> {
        Ok(self.warm_ref(id)?.runtime_artifact_state.is_some())
    }

    pub(crate) fn node_checkpoint_image(
        &self,
        id: NodeId,
    ) -> Result<CheckpointNodeImage, SignalError> {
        Ok(self.materialize_entry(id)?.to_checkpoint_image())
    }

    pub(crate) fn node_condition(
        &self,
        id: NodeId,
    ) -> Result<crate::data::node::EvaluationCondition, SignalError> {
        Ok(self.warm_ref(id)?.eval_config.condition.clone())
    }

    pub(crate) fn node_aspect_version(
        &self,
        id: NodeId,
    ) -> Result<crate::data::aspect::AspectVersion, SignalError> {
        Ok(self.hot_ref(id)?.aspect_versions.global())
    }

    pub(crate) fn node_partitioned_aspect_version(
        &self,
        id: NodeId,
        scope: &PartitionSubscription,
    ) -> Result<AspectVersion, SignalError> {
        Ok(self.hot_ref(id)?.aspect_versions.scoped(scope))
    }

    pub(crate) fn node_version_for_scope(
        &self,
        id: NodeId,
        aspect: crate::data::aspect::Aspect,
        scope: Option<&PartitionSubscription>,
    ) -> Result<u64, SignalError> {
        Ok(self.hot_ref(id)?.aspect_versions.version_for_scope(aspect, scope))
    }

    pub(crate) fn get_entry(&self, id: NodeId) -> Result<MaterializedEntryRef, SignalError> {
        Ok(MaterializedEntryRef(self.materialize_entry(id)?))
    }

    pub(crate) fn get_entry_mut(&mut self, id: NodeId) -> Result<MaterializedEntryGuard<'_>, SignalError> {
        let entry = self.materialize_entry(id)?;
        Ok(MaterializedEntryGuard { graph: self, id, entry })
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
        let previous_snapshot_id = self.get_entry(id)?.get_dep_snapshot_id();
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
        self.get_entry_mut(id)?.set_dep_snapshot_id(snapshot_id);
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
        self.get_entry_mut(id)?.set_dep_snapshot_id(snapshot_id);
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
            self.get_entry_mut(snapshot.node())?
                .set_dep_snapshot_id(snapshot_id);
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
            self.get_entry_mut(snapshot.node())?
                .set_dep_snapshot_id(snapshot_id);
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
            self.get_entry_mut(snapshot.node())?
                .set_dep_snapshot_id(snapshot_id);
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
        let commit_start = Instant::now();
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
            let previous_snapshot_id = self.get_entry(node)?.get_dep_snapshot_id();
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
        let runtime = self.node_runtime_artifact_state(node)?;
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
        self.get_entry_mut(node)?
            .apply_aspect_version(version, changed_regions);
        Ok(())
    }

    pub(crate) fn apply_node_artifact_write_delta(
        &mut self,
        node: NodeId,
        delta: crate::data::trace::ArtifactWriteDelta,
    ) -> Result<bool, SignalError> {
        let mut entry = self.get_entry_mut(node)?;
        entry.apply_artifact_write_delta(delta);
        Ok(entry.cold_artifact_record().is_some())
    }

    pub(crate) fn transition_node_clean(&mut self, node: NodeId) -> Result<(), SignalError> {
        self.get_entry_mut(node)?.transition_clean();
        Ok(())
    }

    pub(crate) fn transition_node_dirty(
        &mut self,
        node: NodeId,
        aspect: crate::data::aspect::Aspect,
        scopes: &[PartitionSubscription],
    ) -> Result<(), SignalError> {
        self.get_entry_mut(node)?.transition_dirty(aspect, scopes);
        Ok(())
    }

    pub(crate) fn transition_node_maybe_stale(
        &mut self,
        node: NodeId,
        aspect: crate::data::aspect::Aspect,
    ) -> Result<(), SignalError> {
        self.get_entry_mut(node)?.transition_maybe_stale(aspect);
        Ok(())
    }

    pub(crate) fn set_node_state(
        &mut self,
        node: NodeId,
        state: NodeState,
    ) -> Result<(), SignalError> {
        self.get_entry_mut(node)?.set_state(state);
        Ok(())
    }

    pub fn causality_of(&self, node: NodeId) -> Result<Option<&CausalityMetadata>, SignalError> {
        Ok(self.cold_ref(node)?.and_then(|cold| cold.causality.as_ref()))
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
        crate::data::access_counters::note_retained_artifact_read();
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

    pub(crate) fn node_reuse_boundary_authority(
        &self,
        node: NodeId,
    ) -> Result<Option<&crate::data::reuse::ReuseBoundaryAuthority>, SignalError> {
        Ok(self
            .warm_ref(node)?
            .runtime_artifact_state
            .as_ref()
            .and_then(|state| state.reuse_boundary_authority()))
    }

    pub fn set_causality(
        &mut self,
        node: NodeId,
        causality: Option<CausalityMetadata>,
    ) -> Result<(), SignalError> {
        self.get_entry_mut(node)?.set_causality(causality);
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
        let mut entry = self.get_entry_mut(node)?;
        let Some(runtime) = entry.runtime_artifact_state_mut() else {
            return Ok(());
        };
        runtime.set_lineage_artifact_id(Some(artifact_id));
        entry.set_execution_trace_stamp(Some(ExecutionTraceStamp {
            execution_record_id: Some(execution_record_id.0),
            semantic_segment_id: Some(semantic_segment_id.0),
        }));
        Ok(())
    }
}
