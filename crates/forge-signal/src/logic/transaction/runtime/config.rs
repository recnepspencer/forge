use std::collections::BTreeMap;

use crate::data::comparator::VersionComparatorPolicy;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::{NodeContract, NodeEvaluationConfig};
use crate::data::node_meta::NodeMetaStore;
use crate::data::output::{
    ComputationFamily, ComputationKey, NodeEvaluationResult, StructuralMemoKey,
};
use crate::data::tier::TierPolicy;
use crate::data::tier_policy_table::TierPolicyTable;

use super::super::key_registry::{RuntimeKeyRegistry, RuntimeStringId};

#[derive(Debug, Clone)]
pub struct SignalRuntimeConfig<T: Copy + Ord> {
    node_meta: NodeMetaStore<T>,
    tier_policies: TierPolicyTable<T>,
    fallback_comparator: VersionComparatorPolicy,
    pub(super) key_registry: RuntimeKeyRegistry,
    computations: BTreeMap<RuntimeStringId, ComputationRegistration<T>>,
    keyed_nodes: BTreeMap<(RuntimeStringId, RuntimeStringId), NodeId>,
    memo_cache: BTreeMap<(RuntimeStringId, RuntimeStringId, RuntimeStringId), NodeEvaluationResult>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ComputationRegistration<T: Copy + Ord> {
    contract: NodeContract,
    tier: T,
    comparator: VersionComparatorPolicy,
}

impl<T: Copy + Ord> Default for SignalRuntimeConfig<T> {
    fn default() -> Self {
        Self {
            node_meta: NodeMetaStore::default(),
            tier_policies: TierPolicyTable::default(),
            fallback_comparator: VersionComparatorPolicy::Exact,
            key_registry: RuntimeKeyRegistry::default(),
            computations: BTreeMap::new(),
            keyed_nodes: BTreeMap::new(),
            memo_cache: BTreeMap::new(),
        }
    }
}

impl<T: Copy + Ord> SignalRuntimeConfig<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub(super) fn sync_graph_capacity(&mut self, graph: &SignalGraph) {
        self.node_meta.ensure_capacity(graph.arena_capacity());
    }

    pub(super) fn prune_stale_node_meta(&mut self, graph: &SignalGraph) {
        self.node_meta.prune_slots(|index, generation| {
            graph
                .live_node_id_at(index)
                .is_some_and(|node| node.generation() == generation)
        });
    }

    pub fn set_node_tier(&mut self, graph: &SignalGraph, node: NodeId, tier: T) {
        self.sync_graph_capacity(graph);
        self.node_meta.set_tier(node, tier);
    }

    pub fn set_tier_policy(&mut self, policy: TierPolicy<T>) {
        self.tier_policies.set(policy);
    }

    pub fn set_fallback_comparator(&mut self, policy: VersionComparatorPolicy) {
        self.fallback_comparator = policy;
    }

    pub fn node_meta(&self) -> &NodeMetaStore<T> {
        &self.node_meta
    }

    pub fn tier_policies(&self) -> &TierPolicyTable<T> {
        &self.tier_policies
    }

    pub fn fallback_comparator(&self) -> &VersionComparatorPolicy {
        &self.fallback_comparator
    }

    pub fn define_computation(
        &mut self,
        family: impl Into<ComputationFamily>,
        contract: NodeContract,
        tier: T,
        comparator: VersionComparatorPolicy,
    ) -> Result<ComputationFamily, crate::data::error::SignalError> {
        let family = family.into();
        let family_id = self.key_registry.intern_family(&family);
        let registration = ComputationRegistration {
            contract,
            tier,
            comparator,
        };
        if let Some(existing) = self.computations.get(&family_id) {
            if existing != &registration {
                return Err(crate::data::error::SignalError::invalid_input(format!(
                    "computation family '{family:?}' already defined with a different spec",
                )));
            }
            return Ok(family);
        }
        self.computations.insert(family_id, registration);
        Ok(family)
    }

    pub(super) fn resolve_defined_node(
        &mut self,
        graph: &mut SignalGraph,
        family: &ComputationFamily,
        key: impl Into<ComputationKey>,
    ) -> NodeId {
        let key = key.into();
        let registry_key = (
            self.key_registry.intern_family(family),
            self.key_registry.intern_key(&key),
        );
        if let Some(node) = self.keyed_nodes.get(&registry_key).copied() {
            return node;
        }
        let node = self.create_keyed_node(graph, registry_key.0);
        self.sync_graph_capacity(graph);
        self.keyed_nodes.insert(registry_key, node);
        node
    }

    pub(super) fn resolve_defined_node_with_created(
        &mut self,
        graph: &mut SignalGraph,
        family: &ComputationFamily,
        key: impl Into<ComputationKey>,
    ) -> (NodeId, bool) {
        let key = key.into();
        let registry_key = (
            self.key_registry.intern_family(family),
            self.key_registry.intern_key(&key),
        );
        if let Some(node) = self.keyed_nodes.get(&registry_key).copied() {
            return (node, false);
        }
        let node = self.create_keyed_node(graph, registry_key.0);
        self.sync_graph_capacity(graph);
        self.keyed_nodes.insert(registry_key, node);
        (node, true)
    }

    fn create_keyed_node(&mut self, graph: &mut SignalGraph, family_id: RuntimeStringId) -> NodeId {
        let node = match self.computations.get(&family_id) {
            Some(registration) => {
                let mut config = NodeEvaluationConfig::default();
                config.contract = registration.contract.clone();
                config.comparator = Some(registration.comparator.clone());
                graph.create_node_with_config(config)
            }
            None => graph.node().build(),
        };
        self.sync_graph_capacity(graph);
        if let Some(registration) = self.computations.get(&family_id) {
            self.node_meta.set_tier(node, registration.tier);
        }
        node
    }

    pub(super) fn lookup_memoized_result(
        &self,
        family: &ComputationFamily,
        key: &ComputationKey,
        memo_key: &StructuralMemoKey,
    ) -> Option<NodeEvaluationResult> {
        let family_id = self.key_registry.family_lookup.get(family).copied()?;
        let key_id = self.key_registry.key_lookup.get(key).copied()?;
        let memo_key_id = self.key_registry.memo_key_lookup.get(memo_key).copied()?;
        self.memo_cache
            .get(&(family_id, key_id, memo_key_id))
            .cloned()
    }

    pub(super) fn store_memoized_result(
        &mut self,
        family: &ComputationFamily,
        key: &ComputationKey,
        memo_key: &StructuralMemoKey,
        result: NodeEvaluationResult,
    ) {
        let family_id = self.key_registry.intern_family(family);
        let key_id = self.key_registry.intern_key(key);
        let memo_key_id = self.key_registry.intern_memo_key(memo_key);
        self.memo_cache
            .insert((family_id, key_id, memo_key_id), result);
    }

    #[cfg(test)]
    pub(crate) fn test_registry_counts(&self) -> (usize, usize, usize, usize, usize) {
        (
            self.key_registry.families.len(),
            self.key_registry.keys.len(),
            self.key_registry.memo_keys.len(),
            self.keyed_nodes.len(),
            self.memo_cache.len(),
        )
    }
}
