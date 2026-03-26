use crate::data::dependency::{
    CommittedSnapshotUpdate, DependencySnapshot, DependencySnapshotShapeStore, SnapshotDeltaRecord,
    SnapshotStorageStrategy,
};
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::node::{NodeContract, NodeEntry, NodeEvaluationConfig, NodeState};
use crate::data::proof::{
    ClassifiedSnapshotBatchCommit, MixedSnapshotBatchCommit, PendingSnapshotBatch,
    SnapshotBatchCommit, StableShapeSnapshotBatchCommit,
};
use crate::data::trace::CausalityMetadata;
use std::time::Instant;

use super::super::node_builder::NodeBuilder;
use super::super::signal_graph::stale_error;
use super::super::signal_graph::{DependencySnapshotStructuralDelta, SignalGraph};

impl SignalGraph {
    #[doc(hidden)]
    pub fn create_node(&mut self) -> NodeId {
        let entry = NodeEntry::new();
        self.allocate_node(entry)
    }

    pub(crate) fn create_node_from_entry(&mut self, entry: NodeEntry) -> NodeId {
        self.allocate_node(entry)
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
        Ok(*self.get_entry(id)?.get_state())
    }

    pub fn get_entry(&self, id: NodeId) -> Result<&NodeEntry, SignalError> {
        self.validate_handle(id)?;
        let slot = &self.arena.nodes[id.index() as usize];
        slot.data
            .as_ref()
            .ok_or_else(|| stale_error(id, slot.generation))
    }

    pub fn get_entry_mut(&mut self, id: NodeId) -> Result<&mut NodeEntry, SignalError> {
        self.validate_handle(id)?;
        let slot = &mut self.arena.nodes[id.index() as usize];
        let generation = slot.generation;
        slot.data
            .as_mut()
            .ok_or_else(|| stale_error(id, generation))
    }

    pub fn get_contract(&self, id: NodeId) -> Result<&NodeContract, SignalError> {
        Ok(&self.get_entry(id)?.get_eval_config().contract)
    }

    pub(crate) fn get_dep_snapshot(&self, id: NodeId) -> Result<&DependencySnapshot, SignalError> {
        let entry = self.get_entry(id)?;
        Ok(self
            .topology
            .dependency_snapshots
            .get(entry.get_dep_snapshot_id()))
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
            self.validate_handle(snapshot.node)?;
        }

        for snapshot in commit.pending() {
            if !snapshot.delta.changed() {
                continue;
            }
            self.telemetry_mut().storage.patch_application_breadth += 1;
            self.telemetry_mut()
                .storage
                .version_only_snapshot_update_count += 1;
            self.telemetry_mut().storage.snapshot_shape_reuse_count += 1;
            let previous = self.get_dep_snapshot(snapshot.node)?.clone();
            let next_snapshot = CommittedSnapshotUpdate::VersionOnly(snapshot.update.clone())
                .apply_to(&previous)
                .into_snapshot();
            let snapshot_id = self.insert_dependency_snapshot(next_snapshot);
            self.get_entry_mut(snapshot.node)?
                .set_dep_snapshot_id(snapshot_id);
            self.record_branch_mutation_snapshot(
                snapshot.node,
                DependencySnapshotStructuralDelta::from_snapshot_delta(snapshot.delta),
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
            self.validate_handle(snapshot.node)?;
        }
        for snapshot in commit.replacements() {
            self.validate_handle(snapshot.node)?;
        }

        for snapshot in commit.stable_shape() {
            if !snapshot.delta.changed() {
                continue;
            }
            self.telemetry_mut().storage.patch_application_breadth += 1;
            self.telemetry_mut()
                .storage
                .version_only_snapshot_update_count += 1;
            self.telemetry_mut().storage.snapshot_shape_reuse_count += 1;
            let previous = self.get_dep_snapshot(snapshot.node)?.clone();
            let next_snapshot = CommittedSnapshotUpdate::VersionOnly(snapshot.update.clone())
                .apply_to(&previous)
                .into_snapshot();
            let snapshot_id = self.insert_dependency_snapshot(next_snapshot);
            self.get_entry_mut(snapshot.node)?
                .set_dep_snapshot_id(snapshot_id);
            self.record_branch_mutation_snapshot(
                snapshot.node,
                DependencySnapshotStructuralDelta::from_snapshot_delta(snapshot.delta),
            );
        }

        for snapshot in commit.replacements() {
            if !snapshot.delta.changed() {
                continue;
            }
            self.telemetry_mut().storage.patch_application_breadth += 1;
            self.telemetry_mut()
                .storage
                .shared_snapshot_replacement_count += 1;
            let next_snapshot = CommittedSnapshotUpdate::Replace(snapshot.update.clone())
                .apply_to(self.get_dep_snapshot(snapshot.node)?)
                .into_snapshot();
            let snapshot_id = self.insert_dependency_snapshot(next_snapshot);
            self.get_entry_mut(snapshot.node)?
                .set_dep_snapshot_id(snapshot_id);
            self.record_branch_mutation_snapshot(
                snapshot.node,
                DependencySnapshotStructuralDelta::from_snapshot_delta(snapshot.delta),
            );
        }

        self.record_graph_storage_pressure();
        Ok(())
    }

    pub(crate) fn apply_snapshot_batch_commit(
        &mut self,
        commit: SnapshotBatchCommit,
    ) -> Result<(), SignalError> {
        let commit_start = Instant::now();
        let result = match commit.classify() {
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
        let target = self.get_entry_mut(id)?;
        *target = entry;
        self.record_branch_mutation_state(id);
        Ok(())
    }

    pub fn causality_of(&self, node: NodeId) -> Result<Option<&CausalityMetadata>, SignalError> {
        Ok(self.get_entry(node)?.get_causality())
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
}
