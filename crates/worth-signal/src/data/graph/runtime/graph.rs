//! Arena-based signal graph with dependency storage.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicU64, Ordering};

use serde::{Deserialize, Serialize};

use crate::data::aspect::SignalAspectLoweringOwner;
use crate::data::bitset::DenseBitset;
use crate::data::error::SignalError;
use crate::data::graph::storage::invalidation_causes::CanonicalCauseSetStore;
use crate::data::handle::NodeId;
use crate::data::node::{NodeColdData, NodeHotData, NodeWarmData};
use crate::schema::data::SignalSchemaRegistry;

use super::super::compaction::CompactionState;
use super::super::storage::Slot;

mod branch_mutations;
mod capabilities;
#[cfg(test)]
mod cause_sets_tests;
mod checkpoint;
mod construction;
mod counter_access;
#[cfg(test)]
mod direct_invalidation_basis_tests;
mod observation_state;
mod reconstruction_counters;
mod scratch_lease;
mod topology_state;
mod traversal_state;

pub(crate) use crate::logic::invalidation::causality::PreparedDirectCauseAdmission;
pub(crate) use branch_mutations::{BranchMutationNodeImage, BranchMutationRecord};
pub use branch_mutations::{
    BranchStructuralDelta, DependencySnapshotStructuralDelta, DependencyTopologyDelta,
    RuntimeArtifactStructuralDelta,
};
pub(crate) use observation_state::RuntimeObservation;
pub(crate) use reconstruction_counters::ReconstructionCounters;
pub(crate) use topology_state::EdgeTopology;
pub(crate) use traversal_state::TraversalResources;

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
    #[serde(
        default,
        serialize_with = "crate::data::graph::storage::invalidation_causes::serialize_canonical_cause_sets"
    )]
    pub(crate) cause_sets: CanonicalCauseSetStore,
    #[serde(skip, default)]
    pub(crate) cause_readmission_required: bool,
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

static NEXT_SIGNAL_GRAPH_INSTANCE_ID: AtomicU64 = AtomicU64::new(1);

fn next_signal_graph_instance_id() -> u64 {
    NEXT_SIGNAL_GRAPH_INSTANCE_ID.fetch_add(1, Ordering::Relaxed)
}

pub(in crate::data::graph) fn stale_error(id: NodeId, expected_generation: u32) -> SignalError {
    SignalError::stale_handle(id, expected_generation)
}
