//! Arena-based signal graph with dependency storage.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::data::aspect::SignalAspectLoweringOwner;
use crate::data::bitset::DenseBitset;
use crate::data::core_profile::StableHashValue;
use crate::data::dependency::{
    CommittedSnapshotUpdate, DependencyEdge, DependencySnapshot, SnapshotDeltaRecord,
};
use crate::data::dependency::{DependencySnapshotShapeStore, DependencySnapshotStore};
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::node::{NodeColdData, NodeEntry, NodeHotData, NodeWarmData};
use crate::data::output::PartitionInterner;
use crate::data::proof::{PendingSnapshotBatch, PendingSnapshotCommit, SnapshotBatchCommit};
use crate::data::reuse::ReuseBasis;
use crate::data::telemetry::RuntimeTelemetry;
use crate::diagnostics::lineage::LineageArtifactId;
use crate::diagnostics::state::DiagnosticsState;
use crate::diagnostics::DiagnosticsLevel;
use crate::schema::data::SignalSchemaRegistry;
use crate::state::{
    SignalCheckpointArena, SignalCheckpointAuthority, SignalCheckpointSlot,
    SignalCheckpointTopology,
};

use super::super::compaction::CompactionState;
use super::super::storage::Slot;
use super::super::{DependencyEdgeStore, SubscriberEdgeStore};
use super::scratch::{GraphScratch, ScratchLeaseKind, TraversalScratch};
use super::strategy::{EvaluationStrategy, GcPressure, ObservationLevel, ParallelismHint};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct NodeArena {
    pub(in crate::data::graph) nodes: Vec<Slot>,
    pub(in crate::data::graph) hot: Vec<Option<NodeHotData>>,
    pub(in crate::data::graph) warm: Vec<NodeWarmData>,
    pub(in crate::data::graph) cold: Vec<Option<Box<NodeColdData>>>,
    pub(in crate::data::graph) free_list: Vec<u32>,
    #[serde(skip, default)]
    pub(in crate::data::graph) free_slots: DenseBitset,
    pub(in crate::data::graph) active_nodes: u32,
    #[serde(default)]
    pub(in crate::data::graph) compaction: CompactionState,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct EdgeTopology {
    #[serde(default)]
    pub(in crate::data::graph) dependency_snapshots: DependencySnapshotStore,
    #[serde(default)]
    pub(in crate::data::graph) dependency_snapshot_shapes: DependencySnapshotShapeStore,
    #[serde(default)]
    pub(in crate::data::graph) dependency_edges: DependencyEdgeStore,
    #[serde(default)]
    pub(in crate::data::graph) subscriber_edges: SubscriberEdgeStore,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct TraversalResources {
    #[serde(skip, default)]
    pub(in crate::data::graph) scratch: TraversalScratch,
    #[serde(skip, default)]
    pub(in crate::data::graph) scratch_lease: Option<ScratchLeaseKind>,
    #[serde(skip, default)]
    pub(in crate::data::graph) suppression_marks: DenseBitset,
    #[serde(skip, default)]
    pub(in crate::data::graph) topology_node_buffer: Vec<NodeId>,
    #[cfg_attr(not(test), allow(dead_code))]
    #[serde(skip, default)]
    pub(in crate::data::graph) topology_dependency_buffer: Vec<DependencyEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct RuntimeObservation {
    #[serde(skip, default)]
    pub(in crate::data::graph) telemetry: RuntimeTelemetry,
    #[serde(skip, default)]
    pub(in crate::data::graph) reconstruction_counters: ReconstructionCounters,
    #[serde(default)]
    pub(in crate::data::graph) partition_interner: PartitionInterner,
    #[serde(default)]
    pub(in crate::data::graph) branch_mutation_view: BTreeMap<NodeId, BranchMutationRecord>,
    #[serde(default)]
    pub(in crate::data::graph) branch_mutation_records: BTreeMap<NodeId, BranchMutationRecord>,
    #[serde(skip, default)]
    pub(in crate::data::graph) diagnostics: DiagnosticsState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub(crate) struct BranchMutationRecord {
    pub introduced: bool,
    pub state_changed: bool,
    pub dependencies_changed: bool,
    pub dependency_snapshot_changed: bool,
    pub runtime_artifact_changed: bool,
    pub retained_artifact_changed: bool,
    pub causality_changed: bool,
    #[serde(default)]
    pub structural_deltas: Vec<BranchStructuralDelta>,
}

impl BranchMutationRecord {
    pub(crate) fn merge_relevant(&self) -> bool {
        self.introduced
            || self.state_changed
            || self.dependencies_changed
            || self.dependency_snapshot_changed
    }

    fn mark_introduced(&mut self) {
        self.introduced = true;
        self.state_changed = true;
        self.structural_deltas
            .push(BranchStructuralDelta::NodeIntroduced);
    }

    fn mark_state_changed(&mut self) {
        self.state_changed = true;
        self.structural_deltas
            .push(BranchStructuralDelta::NodeStateChanged);
    }

    fn mark_dependencies_changed(&mut self, delta: DependencyTopologyDelta) {
        self.dependencies_changed = true;
        if let Some(BranchStructuralDelta::DependencyTopologyChanged(existing)) = self
            .structural_deltas
            .iter_mut()
            .find(|delta| matches!(delta, BranchStructuralDelta::DependencyTopologyChanged(_)))
        {
            merge_dependency_topology_delta(existing, delta);
        } else {
            self.structural_deltas
                .push(BranchStructuralDelta::DependencyTopologyChanged(delta));
        }
    }

    fn mark_dependency_snapshot_changed(&mut self, delta: DependencySnapshotStructuralDelta) {
        self.dependency_snapshot_changed = true;
        self.structural_deltas
            .push(BranchStructuralDelta::DependencySnapshotChanged(delta));
    }

    fn mark_runtime_artifact_changed(&mut self, delta: RuntimeArtifactStructuralDelta) {
        self.runtime_artifact_changed = true;
        self.structural_deltas
            .push(BranchStructuralDelta::RuntimeArtifactChanged(delta));
    }

    fn mark_retained_artifact_changed(&mut self) {
        self.retained_artifact_changed = true;
        self.structural_deltas
            .push(BranchStructuralDelta::RetainedArtifactChanged);
    }

    fn mark_causality_changed(&mut self) {
        self.causality_changed = true;
        self.structural_deltas
            .push(BranchStructuralDelta::CausalityChanged);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchStructuralDelta {
    NodeIntroduced,
    NodeStateChanged,
    DependencyTopologyChanged(DependencyTopologyDelta),
    DependencySnapshotChanged(DependencySnapshotStructuralDelta),
    RuntimeArtifactChanged(RuntimeArtifactStructuralDelta),
    RetainedArtifactChanged,
    CausalityChanged,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DependencyTopologyDelta {
    pub added_edges: Vec<DependencyEdge>,
    pub removed_edges: Vec<DependencyEdge>,
}

fn merge_dependency_topology_delta(
    existing: &mut DependencyTopologyDelta,
    delta: DependencyTopologyDelta,
) {
    for added in delta.added_edges {
        if let Some(index) = existing
            .removed_edges
            .iter()
            .position(|edge| edge == &added)
        {
            existing.removed_edges.remove(index);
        } else if !existing.added_edges.iter().any(|edge| edge == &added) {
            existing.added_edges.push(added);
        }
    }

    for removed in delta.removed_edges {
        if let Some(index) = existing
            .added_edges
            .iter()
            .position(|edge| edge == &removed)
        {
            existing.added_edges.remove(index);
        } else if !existing.removed_edges.iter().any(|edge| edge == &removed) {
            existing.removed_edges.push(removed);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencySnapshotStructuralDelta {
    pub previous_entry_count: u32,
    pub next_entry_count: u32,
    pub changed_entry_count: u32,
}

impl DependencySnapshotStructuralDelta {
    pub(crate) fn from_snapshot_delta(delta: SnapshotDeltaRecord) -> Self {
        Self {
            previous_entry_count: delta.previous_entry_count,
            next_entry_count: delta.next_entry_count,
            changed_entry_count: delta.changed_entry_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeArtifactStructuralDelta {
    pub previous_artifact_id: Option<LineageArtifactId>,
    pub next_artifact_id: Option<LineageArtifactId>,
    pub previous_output_hash: Option<StableHashValue>,
    pub next_output_hash: Option<StableHashValue>,
    pub previous_reuse_basis: Option<ReuseBasis>,
    pub next_reuse_basis: Option<ReuseBasis>,
}

#[derive(Debug, Default)]
pub(crate) struct ReconstructionCounters {
    hot_path_artifact_reconstruction_count: Arc<AtomicU64>,
    explicit_cold_materialization_request_count: Arc<AtomicU64>,
    retained_forensic_read_count: Arc<AtomicU64>,
    cold_explanation_reconstruction_count: Arc<AtomicU64>,
    cold_provenance_reconstruction_count: Arc<AtomicU64>,
    retained_artifact_read_count: Arc<AtomicU64>,
    reconstructed_artifact_read_count: Arc<AtomicU64>,
    denied_reconstruction_by_budget_count: Arc<AtomicU64>,
    denied_reconstruction_by_tier_count: Arc<AtomicU64>,
    denied_reconstruction_explanation_api_count: Arc<AtomicU64>,
    denied_reconstruction_provenance_api_count: Arc<AtomicU64>,
}

impl ReconstructionCounters {
    pub(crate) fn record_hot_path_artifact_reconstruction(&self) {
        self.hot_path_artifact_reconstruction_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn hot_path_artifact_reconstruction_count(&self) -> u64 {
        self.hot_path_artifact_reconstruction_count
            .load(Ordering::Relaxed)
    }

    pub(crate) fn record_explicit_cold_materialization_request(&self) {
        self.explicit_cold_materialization_request_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn explicit_cold_materialization_request_count(&self) -> u64 {
        self.explicit_cold_materialization_request_count
            .load(Ordering::Relaxed)
    }

    pub(crate) fn record_retained_forensic_read(&self) {
        self.retained_forensic_read_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn retained_forensic_read_count(&self) -> u64 {
        self.retained_forensic_read_count.load(Ordering::Relaxed)
    }

    pub(crate) fn record_cold_explanation_reconstruction(&self) {
        self.cold_explanation_reconstruction_count
            .fetch_add(1, Ordering::Relaxed);
        self.reconstructed_artifact_read_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn cold_explanation_reconstruction_count(&self) -> u64 {
        self.cold_explanation_reconstruction_count
            .load(Ordering::Relaxed)
    }

    pub(crate) fn record_cold_provenance_reconstruction(&self) {
        self.cold_provenance_reconstruction_count
            .fetch_add(1, Ordering::Relaxed);
        self.reconstructed_artifact_read_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn cold_provenance_reconstruction_count(&self) -> u64 {
        self.cold_provenance_reconstruction_count
            .load(Ordering::Relaxed)
    }

    pub(crate) fn reconstructed_artifact_read_count(&self) -> u64 {
        self.reconstructed_artifact_read_count
            .load(Ordering::Relaxed)
    }

    pub(crate) fn record_retained_artifact_read(&self) {
        self.retained_artifact_read_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn retained_artifact_read_count(&self) -> u64 {
        self.retained_artifact_read_count.load(Ordering::Relaxed)
    }

    pub(crate) fn record_denied_reconstruction_by_budget(&self, explanation_api: bool) {
        self.denied_reconstruction_by_budget_count
            .fetch_add(1, Ordering::Relaxed);
        if explanation_api {
            self.denied_reconstruction_explanation_api_count
                .fetch_add(1, Ordering::Relaxed);
        } else {
            self.denied_reconstruction_provenance_api_count
                .fetch_add(1, Ordering::Relaxed);
        }
    }

    pub(crate) fn denied_reconstruction_by_budget_count(&self) -> u64 {
        self.denied_reconstruction_by_budget_count
            .load(Ordering::Relaxed)
    }

    pub(crate) fn record_denied_reconstruction_by_tier(&self, explanation_api: bool) {
        self.denied_reconstruction_by_tier_count
            .fetch_add(1, Ordering::Relaxed);
        if explanation_api {
            self.denied_reconstruction_explanation_api_count
                .fetch_add(1, Ordering::Relaxed);
        } else {
            self.denied_reconstruction_provenance_api_count
                .fetch_add(1, Ordering::Relaxed);
        }
    }

    pub(crate) fn denied_reconstruction_by_tier_count(&self) -> u64 {
        self.denied_reconstruction_by_tier_count
            .load(Ordering::Relaxed)
    }

    pub(crate) fn denied_reconstruction_explanation_api_count(&self) -> u64 {
        self.denied_reconstruction_explanation_api_count
            .load(Ordering::Relaxed)
    }

    pub(crate) fn denied_reconstruction_provenance_api_count(&self) -> u64 {
        self.denied_reconstruction_provenance_api_count
            .load(Ordering::Relaxed)
    }
}

impl Clone for ReconstructionCounters {
    fn clone(&self) -> Self {
        Self {
            hot_path_artifact_reconstruction_count: Arc::new(AtomicU64::new(
                self.hot_path_artifact_reconstruction_count(),
            )),
            explicit_cold_materialization_request_count: Arc::new(AtomicU64::new(
                self.explicit_cold_materialization_request_count(),
            )),
            retained_forensic_read_count: Arc::new(AtomicU64::new(
                self.retained_forensic_read_count(),
            )),
            cold_explanation_reconstruction_count: Arc::new(AtomicU64::new(
                self.cold_explanation_reconstruction_count(),
            )),
            cold_provenance_reconstruction_count: Arc::new(AtomicU64::new(
                self.cold_provenance_reconstruction_count(),
            )),
            retained_artifact_read_count: Arc::new(AtomicU64::new(
                self.retained_artifact_read_count(),
            )),
            reconstructed_artifact_read_count: Arc::new(AtomicU64::new(
                self.reconstructed_artifact_read_count(),
            )),
            denied_reconstruction_by_budget_count: Arc::new(AtomicU64::new(
                self.denied_reconstruction_by_budget_count(),
            )),
            denied_reconstruction_by_tier_count: Arc::new(AtomicU64::new(
                self.denied_reconstruction_by_tier_count(),
            )),
            denied_reconstruction_explanation_api_count: Arc::new(AtomicU64::new(
                self.denied_reconstruction_explanation_api_count(),
            )),
            denied_reconstruction_provenance_api_count: Arc::new(AtomicU64::new(
                self.denied_reconstruction_provenance_api_count(),
            )),
        }
    }
}

/// The reactive signal graph.
///
/// An arena of `NodeEntry` values with graph-owned dependency, subscriber,
/// and snapshot storage.
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalGraph {
    #[serde(skip, default = "next_signal_graph_instance_id")]
    pub(in crate::data::graph) instance_id: u64,
    pub(in crate::data::graph) arena: NodeArena,
    pub(in crate::data::graph) topology: EdgeTopology,
    pub(in crate::data::graph) traversal: TraversalResources,
    pub(in crate::data::graph) observation: RuntimeObservation,
    #[serde(skip, default)]
    pub(in crate::data::graph) schema_registry: SignalSchemaRegistry,
    #[serde(skip, default)]
    pub(crate) aspect_lowering_owner: Option<SignalAspectLoweringOwner>,
    #[serde(skip, default)]
    pub(crate) conditional_dependency_versions: BTreeMap<NodeId, Vec<u64>>,
    #[serde(skip, default)]
    pub(crate) authorization_policy_identities: BTreeSet<[u8; 32]>,
}

const NODE_ARENA_RESERVE_CHUNK: usize = 1024;
static NEXT_SIGNAL_GRAPH_INSTANCE_ID: AtomicU64 = AtomicU64::new(1);

fn next_signal_graph_instance_id() -> u64 {
    NEXT_SIGNAL_GRAPH_INSTANCE_ID.fetch_add(1, Ordering::Relaxed)
}

impl Clone for SignalGraph {
    fn clone(&self) -> Self {
        Self {
            instance_id: next_signal_graph_instance_id(),
            arena: self.arena.clone(),
            topology: self.topology.clone(),
            traversal: self.traversal.clone(),
            observation: self.observation.clone(),
            schema_registry: self.schema_registry.clone(),
            aspect_lowering_owner: None,
            conditional_dependency_versions: self.conditional_dependency_versions.clone(),
            authorization_policy_identities: self.authorization_policy_identities.clone(),
        }
    }
}

impl Default for SignalGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl SignalGraph {
    const PARALLELISM_NODE_THRESHOLD: usize = 1_000;
    const GC_PRESSURE_TOMBSTONE_RATIO: f32 = 0.30;

    pub fn new() -> Self {
        Self {
            instance_id: next_signal_graph_instance_id(),
            arena: NodeArena {
                nodes: Vec::new(),
                hot: Vec::new(),
                warm: Vec::new(),
                cold: Vec::new(),
                free_list: Vec::new(),
                free_slots: DenseBitset::default(),
                active_nodes: 0,
                compaction: CompactionState::default(),
            },
            topology: EdgeTopology::default(),
            traversal: TraversalResources::default(),
            observation: RuntimeObservation::default(),
            schema_registry: SignalSchemaRegistry::default(),
            aspect_lowering_owner: None,
            conditional_dependency_versions: BTreeMap::new(),
            authorization_policy_identities: BTreeSet::new(),
        }
    }

    pub(crate) const fn runtime_instance_id(&self) -> u64 {
        self.instance_id
    }

    pub fn with_gc_threshold(gc_threshold: u32) -> Self {
        let mut graph = Self::new();
        graph.arena.compaction = CompactionState::new(gc_threshold);
        graph
    }

    pub(crate) fn clone_stateful(&self) -> Self {
        self.clone()
    }

    pub fn with_schema_registry(mut self, schema_registry: SignalSchemaRegistry) -> Self {
        self.schema_registry = schema_registry;
        self
    }

    pub fn set_schema_registry(&mut self, schema_registry: SignalSchemaRegistry) {
        self.schema_registry = schema_registry;
    }

    pub fn schema_registry(&self) -> &SignalSchemaRegistry {
        &self.schema_registry
    }

    pub(crate) fn capture_checkpoint_authority(&self) -> SignalCheckpointAuthority {
        let mut graph = self.clone_stateful();
        for index in 0..graph.arena.nodes.len() {
            if !graph.arena.nodes[index].is_occupied() {
                continue;
            }
            if let Some(hot) = graph.arena.hot[index].as_mut() {
                hot.dep_snapshot_id = crate::data::dependency::DependencySnapshotId::EMPTY;
            }
            graph.arena.cold[index] = None;
        }
        graph.observation.telemetry = RuntimeTelemetry::default();
        graph.observation.reconstruction_counters = ReconstructionCounters::default();
        graph.observation.branch_mutation_view.clear();
        graph.observation.branch_mutation_records.clear();
        graph.topology.dependency_snapshots = DependencySnapshotStore::default();
        graph.topology.dependency_snapshot_shapes = DependencySnapshotShapeStore::default();
        graph.observation.diagnostics = self.observation.diagnostics.authority_carrier_clone();
        SignalCheckpointAuthority {
            arena: SignalCheckpointArena {
                slots: graph
                    .arena
                    .nodes
                    .iter()
                    .enumerate()
                    .map(|(index, slot)| SignalCheckpointSlot {
                        node: slot.is_occupied().then(|| {
                            NodeEntry::from_storage_parts(
                                graph.arena.hot[index]
                                    .clone()
                                    .expect("occupied slot must retain hot lane"),
                                graph.arena.warm[index].clone(),
                                graph.arena.cold[index].clone(),
                            )
                            .to_checkpoint_image()
                        }),
                        generation: slot.generation,
                        retired: slot.retired,
                    })
                    .collect(),
                free_list: graph.arena.free_list,
                active_nodes: graph.arena.active_nodes,
            },
            topology: SignalCheckpointTopology {
                dependency_edges: graph.topology.dependency_edges,
                subscriber_edges: graph.topology.subscriber_edges,
            },
            diagnostics: graph.observation.diagnostics,
        }
    }

    pub(crate) fn restore_from_checkpoint_authority(authority: &SignalCheckpointAuthority) -> Self {
        let mut free_slots = DenseBitset::default();
        for index in &authority.arena.free_list {
            free_slots.mark(*index as usize);
        }
        Self {
            instance_id: next_signal_graph_instance_id(),
            arena: NodeArena {
                nodes: authority
                    .arena
                    .slots
                    .iter()
                    .cloned()
                    .map(|slot| Slot {
                        generation: slot.generation,
                        retired: slot.retired,
                        occupied: slot.node.is_some(),
                    })
                    .collect(),
                hot: authority
                    .arena
                    .slots
                    .iter()
                    .cloned()
                    .map(|slot| {
                        slot.node.map(|image| {
                            let (hot, _, _) =
                                NodeEntry::from_checkpoint_image(image).into_storage_parts();
                            hot
                        })
                    })
                    .collect(),
                warm: authority
                    .arena
                    .slots
                    .iter()
                    .cloned()
                    .map(|slot| {
                        slot.node
                            .map(|image| {
                                let (_, warm, _) =
                                    NodeEntry::from_checkpoint_image(image).into_storage_parts();
                                warm
                            })
                            .unwrap_or_default()
                    })
                    .collect(),
                cold: authority
                    .arena
                    .slots
                    .iter()
                    .cloned()
                    .map(|slot| {
                        slot.node.map(|image| {
                            let (_, _, cold) =
                                NodeEntry::from_checkpoint_image(image).into_storage_parts();
                            cold
                        })
                    })
                    .map(|cold| cold.unwrap_or(None))
                    .collect(),
                free_list: authority.arena.free_list.clone(),
                free_slots,
                active_nodes: authority.arena.active_nodes,
                compaction: CompactionState::default(),
            },
            topology: EdgeTopology {
                dependency_snapshots: DependencySnapshotStore::default(),
                dependency_snapshot_shapes: DependencySnapshotShapeStore::default(),
                dependency_edges: authority.topology.dependency_edges.clone(),
                subscriber_edges: authority.topology.subscriber_edges.clone(),
            },
            traversal: TraversalResources::default(),
            observation: RuntimeObservation {
                telemetry: RuntimeTelemetry::default(),
                reconstruction_counters: ReconstructionCounters::default(),
                partition_interner: PartitionInterner::default(),
                branch_mutation_view: BTreeMap::new(),
                branch_mutation_records: BTreeMap::new(),
                diagnostics: authority.diagnostics.clone(),
            },
            schema_registry: SignalSchemaRegistry::default(),
            aspect_lowering_owner: None,
            conditional_dependency_versions: BTreeMap::new(),
            authorization_policy_identities: BTreeSet::new(),
        }
    }

    pub(crate) fn checkpoint_authority_arena_capacity(
        authority: &SignalCheckpointAuthority,
    ) -> usize {
        authority.arena.slots.len()
    }

    pub(crate) fn checkpoint_authority_live_node_id_at(
        authority: &SignalCheckpointAuthority,
        index: usize,
    ) -> Option<NodeId> {
        let slot = authority.arena.slots.get(index)?;
        if slot.node.is_none() {
            return None;
        }
        Some(NodeId::new(index as u32, slot.generation))
    }

    pub(crate) fn capture_checkpoint_dependency_snapshot_batch(&self) -> SnapshotBatchCommit {
        let entries = self
            .live_node_ids()
            .into_iter()
            .filter_map(|node| {
                let snapshot = self
                    .get_dep_snapshot(node)
                    .expect("live node must have readable dependency snapshot")
                    .clone();
                (!snapshot.entries().is_empty()).then_some((node, snapshot))
            })
            .collect::<Vec<_>>();
        SnapshotBatchCommit::from_pairs(entries)
    }

    pub(crate) fn derive_dependency_snapshot_restore_batch_from_checkpoint_batch(
        &self,
        authority: &SignalCheckpointAuthority,
        checkpoint_batch: &SnapshotBatchCommit,
    ) -> Result<SnapshotBatchCommit, SignalError> {
        let target_snapshots = checkpoint_batch
            .pending()
            .as_slice()
            .iter()
            .map(|entry| {
                let target = entry
                    .update
                    .clone()
                    .apply_to(&DependencySnapshot::empty())
                    .into_snapshot();
                (entry.node, target)
            })
            .collect::<BTreeMap<_, _>>();
        let mut entries = Vec::new();
        for index in 0..Self::checkpoint_authority_arena_capacity(authority) {
            let Some(node) = Self::checkpoint_authority_live_node_id_at(authority, index) else {
                continue;
            };
            if !self.is_alive(node) {
                continue;
            }
            let previous = self.get_dep_snapshot(node)?.clone();
            let next = target_snapshots
                .get(&node)
                .cloned()
                .unwrap_or_else(DependencySnapshot::empty);
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
                entries.push(PendingSnapshotCommit {
                    node,
                    update,
                    delta,
                });
            }
        }
        Ok(SnapshotBatchCommit::new(PendingSnapshotBatch::new(entries)))
    }

    pub(crate) fn node_allocator_state(&self) -> u32 {
        self.arena.nodes.len() as u32
    }

    pub(crate) fn synchronize_node_allocator(&mut self, next_node_index: u32) {
        if self.arena.nodes.len() as u32 >= next_node_index {
            return;
        }
        let missing = next_node_index as usize - self.arena.nodes.len();
        self.arena.nodes.reserve(missing);
        for _ in 0..missing {
            self.arena.nodes.push(Slot::retired_placeholder());
            self.arena.hot.push(None);
            self.arena.warm.push(NodeWarmData::default());
            self.arena.cold.push(None);
        }
    }

    #[cfg(test)]
    pub(crate) fn reserve_node_capacity(&mut self, additional: usize) {
        self.arena.nodes.reserve(additional);
        self.arena.hot.reserve(additional);
        self.arena.warm.reserve(additional);
        self.arena.cold.reserve(additional);
    }

    fn record_branch_mutation(
        &mut self,
        node: NodeId,
        mut update: impl FnMut(&mut BranchMutationRecord),
    ) {
        update(
            self.observation
                .branch_mutation_view
                .entry(node)
                .or_default(),
        );
        update(
            self.observation
                .branch_mutation_records
                .entry(node)
                .or_default(),
        );
    }

    pub(crate) fn record_branch_mutation_introduced(&mut self, node: NodeId) {
        self.record_branch_mutation(node, BranchMutationRecord::mark_introduced);
    }

    pub(crate) fn record_branch_mutation_state(&mut self, node: NodeId) {
        self.record_branch_mutation(node, BranchMutationRecord::mark_state_changed);
    }

    pub(crate) fn record_branch_mutation_dependencies(
        &mut self,
        node: NodeId,
        delta: DependencyTopologyDelta,
    ) {
        self.record_branch_mutation(node, |record| {
            record.mark_dependencies_changed(delta.clone())
        });
    }

    pub(crate) fn record_branch_mutation_snapshot(
        &mut self,
        node: NodeId,
        delta: DependencySnapshotStructuralDelta,
    ) {
        self.record_branch_mutation(node, |record| {
            record.mark_dependency_snapshot_changed(delta.clone())
        });
    }

    pub(crate) fn record_branch_mutation_runtime_artifact(
        &mut self,
        node: NodeId,
        delta: RuntimeArtifactStructuralDelta,
    ) {
        self.record_branch_mutation(node, |record| {
            record.mark_runtime_artifact_changed(delta.clone())
        });
    }

    pub(crate) fn record_branch_mutation_retained_artifact(&mut self, node: NodeId) {
        self.record_branch_mutation(node, BranchMutationRecord::mark_retained_artifact_changed);
    }

    pub(crate) fn record_branch_mutation_causality(&mut self, node: NodeId) {
        self.record_branch_mutation(node, BranchMutationRecord::mark_causality_changed);
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn branch_mutation_records(&self) -> Vec<(NodeId, BranchMutationRecord)> {
        self.observation
            .branch_mutation_view
            .iter()
            .map(|(node, record)| (*node, record.clone()))
            .collect()
    }

    pub(crate) fn pending_branch_mutation_records(&self) -> Vec<(NodeId, BranchMutationRecord)> {
        self.observation
            .branch_mutation_records
            .iter()
            .map(|(node, record)| (*node, record.clone()))
            .collect()
    }

    pub(crate) fn clear_branch_mutation_nodes(&mut self) {
        self.observation.branch_mutation_records.clear();
    }

    pub fn observe(&self) -> super::observer::GraphObserver<'_> {
        super::observer::GraphObserver::new(self)
    }

    pub fn derive_evaluation_strategy(&self) -> EvaluationStrategy {
        let active_nodes = self.active_node_count();
        let tombstone_ratio = self.tombstone_ratio();
        let diagnostics_profile = self.observation.diagnostics.tier();
        EvaluationStrategy {
            parallelism: if active_nodes >= Self::PARALLELISM_NODE_THRESHOLD {
                ParallelismHint::Preferred
            } else {
                ParallelismHint::Serial
            },
            gc_pressure: if tombstone_ratio >= Self::GC_PRESSURE_TOMBSTONE_RATIO
                || self
                    .arena
                    .should_run_compaction_epoch(&self.topology, active_nodes)
            {
                GcPressure::CompactAfterEvaluation
            } else {
                GcPressure::Deferred
            },
            observation_level: Self::observation_level_for_profile(diagnostics_profile),
        }
    }

    pub(crate) fn as_parts_mut(
        &mut self,
    ) -> (
        &mut NodeArena,
        &mut EdgeTopology,
        &mut TraversalResources,
        &mut RuntimeObservation,
    ) {
        (
            &mut self.arena,
            &mut self.topology,
            &mut self.traversal,
            &mut self.observation,
        )
    }

    pub(crate) fn acquire_scratch(
        &mut self,
        kind: ScratchLeaseKind,
    ) -> Result<TraversalScratch, SignalError> {
        let (_, _, traversal, observation) = self.as_parts_mut();
        if let Some(active) = traversal.scratch_lease {
            observation.telemetry.storage.scratch_reentry_error_count += 1;
            return Err(SignalError::scratch_reentry(active, kind));
        }
        traversal.scratch_lease = Some(kind);
        Ok(std::mem::take(&mut traversal.scratch))
    }

    pub(crate) fn restore_scratch(
        &mut self,
        kind: ScratchLeaseKind,
        scratch: TraversalScratch,
    ) -> Result<(), SignalError> {
        let (_, _, traversal, _) = self.as_parts_mut();
        match traversal.scratch_lease {
            Some(active) if active == kind => {
                traversal.scratch = scratch;
                traversal.scratch_lease = None;
                Ok(())
            }
            Some(active) => Err(SignalError::scratch_mismatch(active, kind)),
            None => Err(SignalError::internal(format!(
                "signal scratch restore called without active lease for {kind:?}"
            ))),
        }
    }

    pub(crate) fn with_scratch<R, E>(
        &mut self,
        kind: ScratchLeaseKind,
        f: impl FnOnce(&mut SignalGraph, &mut GraphScratch<'_>) -> Result<R, E>,
    ) -> Result<R, E>
    where
        E: From<SignalError>,
    {
        let mut scratch = self.acquire_scratch(kind)?;
        let mut graph_scratch = GraphScratch::new(&mut scratch);
        let result = f(self, &mut graph_scratch);
        self.restore_scratch(kind, scratch)?;
        result
    }

    pub(crate) fn record_hot_path_artifact_reconstruction(&self) {
        self.observation
            .reconstruction_counters
            .record_hot_path_artifact_reconstruction();
    }

    pub(crate) fn hot_path_artifact_reconstruction_count(&self) -> u64 {
        self.observation
            .reconstruction_counters
            .hot_path_artifact_reconstruction_count()
    }

    pub(crate) fn record_explicit_cold_materialization_request(&self) {
        self.observation
            .reconstruction_counters
            .record_explicit_cold_materialization_request();
    }

    pub(crate) fn explicit_cold_materialization_request_count(&self) -> u64 {
        self.observation
            .reconstruction_counters
            .explicit_cold_materialization_request_count()
    }

    pub(crate) fn record_retained_forensic_read(&self) {
        self.observation
            .reconstruction_counters
            .record_retained_forensic_read();
    }

    pub(crate) fn retained_forensic_read_count(&self) -> u64 {
        self.observation
            .reconstruction_counters
            .retained_forensic_read_count()
    }

    pub(crate) fn record_cold_explanation_reconstruction(&self) {
        self.observation
            .reconstruction_counters
            .record_cold_explanation_reconstruction();
    }

    pub(crate) fn cold_explanation_reconstruction_count(&self) -> u64 {
        self.observation
            .reconstruction_counters
            .cold_explanation_reconstruction_count()
    }

    pub(crate) fn record_cold_provenance_reconstruction(&self) {
        self.observation
            .reconstruction_counters
            .record_cold_provenance_reconstruction();
    }

    pub(crate) fn cold_provenance_reconstruction_count(&self) -> u64 {
        self.observation
            .reconstruction_counters
            .cold_provenance_reconstruction_count()
    }

    pub(crate) fn record_retained_artifact_read(&self) {
        self.observation
            .reconstruction_counters
            .record_retained_artifact_read();
    }

    pub(crate) fn retained_artifact_read_count(&self) -> u64 {
        self.observation
            .reconstruction_counters
            .retained_artifact_read_count()
    }

    pub(crate) fn reconstructed_artifact_read_count(&self) -> u64 {
        self.observation
            .reconstruction_counters
            .reconstructed_artifact_read_count()
    }

    pub(crate) fn record_denied_reconstruction_by_budget(&self, explanation_api: bool) {
        self.observation
            .reconstruction_counters
            .record_denied_reconstruction_by_budget(explanation_api);
    }

    pub(crate) fn denied_reconstruction_by_budget_count(&self) -> u64 {
        self.observation
            .reconstruction_counters
            .denied_reconstruction_by_budget_count()
    }

    pub(crate) fn record_denied_reconstruction_by_tier(&self, explanation_api: bool) {
        self.observation
            .reconstruction_counters
            .record_denied_reconstruction_by_tier(explanation_api);
    }

    pub(crate) fn denied_reconstruction_by_tier_count(&self) -> u64 {
        self.observation
            .reconstruction_counters
            .denied_reconstruction_by_tier_count()
    }

    pub(crate) fn denied_reconstruction_explanation_api_count(&self) -> u64 {
        self.observation
            .reconstruction_counters
            .denied_reconstruction_explanation_api_count()
    }

    pub(crate) fn denied_reconstruction_provenance_api_count(&self) -> u64 {
        self.observation
            .reconstruction_counters
            .denied_reconstruction_provenance_api_count()
    }

    fn tombstone_ratio(&self) -> f32 {
        let active_nodes = self.active_node_count();
        let total = active_nodes + self.arena.compaction.tombstone_count as usize;
        if total == 0 {
            0.0
        } else {
            self.arena.compaction.tombstone_count as f32 / total as f32
        }
    }

    fn observation_level_for_profile(profile: DiagnosticsLevel) -> ObservationLevel {
        match profile {
            DiagnosticsLevel::Operational => ObservationLevel::Minimal,
            DiagnosticsLevel::Development | DiagnosticsLevel::Forensic => ObservationLevel::Full,
        }
    }

    pub(in crate::data::graph) fn allocate_node(&mut self, entry: NodeEntry) -> NodeId {
        let (hot, warm, cold) = entry.into_storage_parts();
        while let Some(index) = self.arena.free_list.pop() {
            if index as usize >= self.arena.nodes.len() {
                continue;
            }
            self.arena.free_slots.clear(index as usize);
            let slot = &mut self.arena.nodes[index as usize];
            if slot.is_retired() {
                continue;
            }
            self.arena.hot[index as usize] = Some(hot.clone());
            self.arena.warm[index as usize] = warm.clone();
            self.arena.cold[index as usize] = cold.clone();
            let generation = slot.occupy();
            self.arena.active_nodes += 1;
            let node = NodeId::new(index, generation);
            self.record_branch_mutation_introduced(node);
            return node;
        }

        let index = self.arena.nodes.len() as u32;
        if self.arena.nodes.len() == self.arena.nodes.capacity() {
            self.arena.nodes.reserve(NODE_ARENA_RESERVE_CHUNK);
            self.arena.hot.reserve(NODE_ARENA_RESERVE_CHUNK);
            self.arena.warm.reserve(NODE_ARENA_RESERVE_CHUNK);
            self.arena.cold.reserve(NODE_ARENA_RESERVE_CHUNK);
        }
        let mut slot = Slot::vacant();
        let generation = slot.occupy();
        self.arena.nodes.push(slot);
        self.arena.hot.push(Some(hot));
        self.arena.warm.push(warm);
        self.arena.cold.push(cold);
        self.arena.active_nodes += 1;
        let node = NodeId::new(index, generation);
        self.record_branch_mutation_introduced(node);
        node
    }

    pub(crate) fn rollback_created_nodes(&mut self, created_nodes: &[NodeId]) {
        for node in created_nodes {
            self.conditional_dependency_versions.remove(node);
        }
        let mut indices = created_nodes
            .iter()
            .map(|node| node.index() as usize)
            .collect::<Vec<_>>();
        indices.sort_unstable();
        indices.dedup();
        self.observation
            .telemetry
            .storage
            .rolled_back_created_node_count += indices.len() as u64;

        for index in indices.iter().rev().copied() {
            let Some(slot) = self.arena.nodes.get_mut(index) else {
                continue;
            };
            if slot.is_occupied() {
                slot.vacate();
                self.arena.hot[index] = None;
                self.arena.warm[index] = NodeWarmData::default();
                self.arena.cold[index] = None;
                self.arena.active_nodes = self.arena.active_nodes.saturating_sub(1);
            }
            if !slot.is_retired() && !self.arena.free_slots.contains(index) {
                self.arena.free_list.push(index as u32);
                self.arena.free_slots.mark(index);
            }
        }

        while self
            .arena
            .nodes
            .last()
            .is_some_and(|slot| !slot.is_occupied())
        {
            self.arena.free_slots.clear(self.arena.nodes.len() - 1);
            self.arena.nodes.pop();
            self.arena.hot.pop();
            self.arena.warm.pop();
            self.arena.cold.pop();
        }
        self.arena
            .free_list
            .retain(|index| (*index as usize) < self.arena.nodes.len());
    }

    pub(in crate::data::graph) fn validate_handle(&self, id: NodeId) -> Result<(), SignalError> {
        let idx = id.index() as usize;
        if idx >= self.arena.nodes.len() {
            return Err(stale_error(id, id.generation()));
        }
        let slot = &self.arena.nodes[idx];
        if slot.generation != id.generation() || !slot.is_occupied() {
            return Err(stale_error(id, slot.generation));
        }
        Ok(())
    }

    pub(crate) fn live_node_ids(&self) -> Vec<NodeId> {
        self.arena
            .nodes
            .iter()
            .enumerate()
            .filter_map(|(index, slot)| {
                slot.is_occupied()
                    .then_some(NodeId::new(index as u32, slot.generation))
            })
            .collect()
    }

    #[cfg(test)]
    pub(crate) fn storage_counts(&self) -> ((usize, usize), (usize, usize), usize) {
        (
            self.topology.dependency_edges.storage_counts(),
            self.topology.subscriber_edges.storage_counts(),
            self.topology.dependency_snapshots.snapshot_count(),
        )
    }

    #[cfg(test)]
    pub(crate) fn free_list_snapshot(&self) -> Vec<u32> {
        self.arena.free_list.clone()
    }

    #[cfg(test)]
    pub(crate) fn force_slot_generation_for_test(
        &mut self,
        index: u32,
        generation: u32,
    ) -> Result<(), SignalError> {
        let slot = self
            .arena
            .nodes
            .get_mut(index as usize)
            .ok_or_else(|| SignalError::invalid_input(format!("unknown slot `{index}`")))?;
        slot.generation = generation;
        slot.retired = false;
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn is_slot_retired_for_test(&self, index: u32) -> Result<bool, SignalError> {
        let slot = self
            .arena
            .nodes
            .get(index as usize)
            .ok_or_else(|| SignalError::invalid_input(format!("unknown slot `{index}`")))?;
        Ok(slot.is_retired())
    }
}

impl NodeArena {
    pub(crate) fn len(&self) -> usize {
        self.nodes.len()
    }
}

impl RuntimeObservation {
    pub(crate) fn telemetry_mut(&mut self) -> &mut RuntimeTelemetry {
        &mut self.telemetry
    }

    pub(crate) fn partition_interner_mut(&mut self) -> &mut PartitionInterner {
        &mut self.partition_interner
    }
}

pub(in crate::data::graph) fn stale_error(id: NodeId, expected_generation: u32) -> SignalError {
    SignalError::stale_handle(id, expected_generation)
}
