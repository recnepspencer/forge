use std::collections::{BTreeMap, BTreeSet};

use crate::data::bitset::DenseBitset;
use crate::data::dependency::{
    CommittedSnapshotUpdate, DependencySnapshot, DependencySnapshotShapeStore,
    DependencySnapshotStore,
};
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::node::NodeEntry;
use crate::data::output::PartitionInterner;
use crate::data::proof::{PendingSnapshotBatch, PendingSnapshotCommit, SnapshotBatchCommit};
use crate::data::telemetry::RuntimeTelemetry;
use crate::schema::data::SignalSchemaRegistry;
use crate::state::{
    SignalCheckpointArena, SignalCheckpointAuthority, SignalCheckpointSlot,
    SignalCheckpointTopology,
};

use crate::data::graph::compaction::CompactionState;
use crate::data::graph::storage::Slot;

use super::{
    EdgeTopology, NodeArena, ReconstructionCounters, RuntimeObservation, SignalGraph,
    TraversalResources,
};

impl SignalGraph {
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
            instance_id: super::next_signal_graph_instance_id(),
            arena: NodeArena {
                nodes: authority
                    .arena
                    .slots
                    .iter()
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
        slot.node.as_ref()?;
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
}
