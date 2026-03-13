//! Arena-based signal graph with dependency storage.

use serde::{Deserialize, Serialize};

use crate::data::bitset::DenseBitset;
use crate::data::dependency::DependencySnapshotStore;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::node::NodeEntry;
use crate::data::output::PartitionInterner;
use crate::data::telemetry::RuntimeTelemetry;
use crate::diagnostics::state::DiagnosticsState;
use crate::diagnostics::DiagnosticsProfile;

use super::super::compaction::CompactionState;
use super::super::storage::Slot;
use super::super::{DependencyEdgeStore, SubscriberEdgeStore};
use super::scratch::{GraphScratch, ScratchLeaseKind, TraversalScratch};
use super::strategy::{EvaluationStrategy, GcPressure, ObservationLevel, ParallelismHint};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct NodeArena {
    pub(in crate::data::graph) nodes: Vec<Slot>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct RuntimeObservation {
    #[serde(skip, default)]
    pub(in crate::data::graph) telemetry: RuntimeTelemetry,
    #[serde(default)]
    pub(in crate::data::graph) partition_interner: PartitionInterner,
    #[serde(skip, default)]
    pub(in crate::data::graph) diagnostics: DiagnosticsState,
}

/// The reactive signal graph.
///
/// An arena of `NodeEntry` values with graph-owned dependency, subscriber,
/// and snapshot storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalGraph {
    pub(in crate::data::graph) arena: NodeArena,
    pub(in crate::data::graph) topology: EdgeTopology,
    pub(in crate::data::graph) traversal: TraversalResources,
    pub(in crate::data::graph) observation: RuntimeObservation,
}

const NODE_ARENA_RESERVE_CHUNK: usize = 1024;

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
            arena: NodeArena {
                nodes: Vec::new(),
                free_list: Vec::new(),
                free_slots: DenseBitset::default(),
                active_nodes: 0,
                compaction: CompactionState::default(),
            },
            topology: EdgeTopology::default(),
            traversal: TraversalResources::default(),
            observation: RuntimeObservation::default(),
        }
    }

    pub fn with_gc_threshold(gc_threshold: u32) -> Self {
        let mut graph = Self::new();
        graph.arena.compaction = CompactionState::new(gc_threshold);
        graph
    }

    pub(crate) fn clone_stateful(&self) -> Self {
        self.clone()
    }

    pub fn observe(&self) -> super::observer::GraphObserver<'_> {
        super::observer::GraphObserver::new(self)
    }

    pub fn derive_evaluation_strategy(&self) -> EvaluationStrategy {
        let active_nodes = self.active_node_count();
        let tombstone_ratio = self.tombstone_ratio();
        let diagnostics_profile = self.observation.diagnostics.profile();
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

    fn tombstone_ratio(&self) -> f32 {
        let active_nodes = self.active_node_count();
        let total = active_nodes + self.arena.compaction.tombstone_count as usize;
        if total == 0 {
            0.0
        } else {
            self.arena.compaction.tombstone_count as f32 / total as f32
        }
    }

    fn observation_level_for_profile(profile: DiagnosticsProfile) -> ObservationLevel {
        match profile {
            DiagnosticsProfile::Operational => ObservationLevel::Minimal,
            DiagnosticsProfile::Development | DiagnosticsProfile::Forensic => {
                ObservationLevel::Full
            }
        }
    }

    pub(in crate::data::graph) fn allocate_node(&mut self, entry: NodeEntry) -> NodeId {
        while let Some(index) = self.arena.free_list.pop() {
            if index as usize >= self.arena.nodes.len() {
                continue;
            }
            self.arena.free_slots.clear(index as usize);
            let slot = &mut self.arena.nodes[index as usize];
            if slot.is_retired() {
                continue;
            }
            let generation = slot.occupy(entry);
            self.arena.active_nodes += 1;
            return NodeId::new(index, generation);
        }

        let index = self.arena.nodes.len() as u32;
        if self.arena.nodes.len() == self.arena.nodes.capacity() {
            self.arena.nodes.reserve(NODE_ARENA_RESERVE_CHUNK);
        }
        let mut slot = Slot::vacant();
        let generation = slot.occupy(entry);
        self.arena.nodes.push(slot);
        self.arena.active_nodes += 1;
        NodeId::new(index, generation)
    }

    pub(crate) fn rollback_created_nodes(&mut self, created_nodes: &[NodeId]) {
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
                slot.data
                    .as_ref()
                    .map(|_| NodeId::new(index as u32, slot.generation))
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
