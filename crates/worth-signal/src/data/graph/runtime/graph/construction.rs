use std::collections::{BTreeMap, BTreeSet};

use crate::data::bitset::DenseBitset;
use crate::data::graph::compaction::CompactionState;
use crate::schema::data::SignalSchemaRegistry;

use super::{EdgeTopology, NodeArena, RuntimeObservation, SignalGraph, TraversalResources};

impl Clone for SignalGraph {
    fn clone(&self) -> Self {
        let instance_id = super::next_signal_graph_instance_id();
        let mut cause_sets = self.cause_sets.clone();
        cause_sets.readmit_graph_instance(instance_id);
        Self {
            lifecycle_token: Default::default(),
            instance_id,
            arena: self.arena.clone(),
            topology: self.topology.clone(),
            cause_sets,
            cause_readmission_required: self.cause_readmission_required,
            traversal: self.traversal.clone(),
            observation: self.observation.clone(),
            schema_registry: self.schema_registry.clone(),
            aspect_lowering_owner: None,
            conditional_dependency_versions: self.conditional_dependency_versions.clone(),
            authorization_policy_identities: self.authorization_policy_identities.clone(),
            invalidation_readiness_epoch: 0,
            invalidation_performed_counters: Default::default(),
            pending_repeated_invalidation_admissions: BTreeMap::new(),
        }
    }
}

impl Default for SignalGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl SignalGraph {
    pub(super) const PARALLELISM_NODE_THRESHOLD: usize = 1_000;
    pub(super) const GC_PRESSURE_TOMBSTONE_RATIO: f32 = 0.30;

    pub fn new() -> Self {
        Self {
            lifecycle_token: Default::default(),
            instance_id: super::next_signal_graph_instance_id(),
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
            cause_sets: Default::default(),
            cause_readmission_required: false,
            traversal: TraversalResources::default(),
            observation: RuntimeObservation::default(),
            schema_registry: SignalSchemaRegistry::default(),
            aspect_lowering_owner: None,
            conditional_dependency_versions: BTreeMap::new(),
            authorization_policy_identities: BTreeSet::new(),
            invalidation_readiness_epoch: 0,
            invalidation_performed_counters: Default::default(),
            pending_repeated_invalidation_admissions: BTreeMap::new(),
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
}
