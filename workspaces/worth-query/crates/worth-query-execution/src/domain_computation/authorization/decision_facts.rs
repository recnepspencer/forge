use std::sync::Arc;

use worth_relational::facade::authorization::{
    RelationalAuthorizationObservationCounters, RelationalAuthorizationObservationEvidence,
    RelationalAuthorizationObservationFreshness,
};
use worth_runtime_bridge::facade::{
    BridgeAuthorizationDecisionEvidence, BridgeAuthorizationRuntime,
};

use crate::domain_computation::primary_graph::{
    freshness::WorthQueryPrincipalFreshnessEvidence,
    schema_layout::WorthQueryPrimaryPrincipalBindingLayout, WorthQueryAuthenticatedPrincipal,
};

pub(in crate::domain_computation) struct WorthQueryAuthorizationDecisionFact {
    pub(in crate::domain_computation) relational:
        RelationalAuthorizationObservationEvidence,
    pub(in crate::domain_computation) bridge: BridgeAuthorizationDecisionEvidence,
}

#[derive(Clone)]
pub(in crate::domain_computation) struct WorthQueryPrincipalCurrentnessDependency {
    binding: Arc<str>,
    layout: WorthQueryPrimaryPrincipalBindingLayout,
    freshness: WorthQueryPrincipalFreshnessEvidence,
}

impl WorthQueryPrincipalCurrentnessDependency {
    pub(in crate::domain_computation) fn capture<
        Schema,
        Principal,
        PrincipalIdentity,
    >(
        principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
        layout: &WorthQueryPrimaryPrincipalBindingLayout,
    ) -> Self {
        Self {
            binding: Arc::from(principal.binding()),
            layout: layout.clone(),
            freshness: principal.freshness().clone(),
        }
    }

    pub(in crate::domain_computation) fn remains_current_in(
        &self,
        runtime: &worth_relational::facade::runtime::RelationalRuntime,
        snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    ) -> bool {
        self.freshness
            .remains_current_in(runtime, snapshot, &self.layout, &self.binding)
    }
}

pub(in crate::domain_computation) struct WorthQueryRetainedAuthorizationDecisionFacts
{
    principal: WorthQueryPrincipalCurrentnessDependency,
    policy: Vec<WorthQueryAuthorizationDecisionFact>,
}

impl WorthQueryRetainedAuthorizationDecisionFacts {
    pub(in crate::domain_computation) fn new(
        principal: WorthQueryPrincipalCurrentnessDependency,
        policy: Vec<WorthQueryAuthorizationDecisionFact>,
    ) -> Self {
        Self { principal, policy }
    }

    pub(super) fn policy_count(&self) -> usize {
        self.policy.len()
    }

    pub(in crate::domain_computation) fn exact_fact_count(&self) -> usize {
        1usize.saturating_add(self.policy.len())
    }

    pub(super) fn principal_remains_current_in(
        &self,
        runtime: &worth_relational::facade::runtime::RelationalRuntime,
        snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    ) -> bool {
        self.principal.remains_current_in(runtime, snapshot)
    }

    pub(super) fn replace_single_policy(
        &mut self,
        fact: WorthQueryAuthorizationDecisionFact,
    ) -> Result<(), ()> {
        let [current] = self.policy.as_mut_slice() else {
            return Err(());
        };
        *current = fact;
        Ok(())
    }

    pub(super) fn relational_counters(&self) -> RelationalAuthorizationObservationCounters {
        self.policy.iter().fold(
            RelationalAuthorizationObservationCounters::default(),
            |mut total, fact| {
                let counters = fact.relational.counters();
                total.paths_evaluated += counters.paths_evaluated;
                total.adjacency_lists_read += counters.adjacency_lists_read;
                total.adjacency_edges_inspected += counters.adjacency_edges_inspected;
                total.relation_records_inspected += counters.relation_records_inspected;
                total.entity_records_inspected += counters.entity_records_inspected;
                total.predicate_fields_inspected += counters.predicate_fields_inspected;
                total.maximum_frontier_width = total
                    .maximum_frontier_width
                    .max(counters.maximum_frontier_width);
                total.reconstructive_graph_scans += counters.reconstructive_graph_scans;
                total.reconstructive_relation_records_scanned +=
                    counters.reconstructive_relation_records_scanned;
                total
            },
        )
    }

    pub(super) fn signal_dependency_count(&self) -> usize {
        self.policy
            .iter()
            .map(|fact| {
                let counters = fact.bridge.counters();
                counters.entities_depended_on
                    + counters.relations_depended_on
                    + counters.adjacency_lists_depended_on
                    + counters.fields_depended_on
            })
            .sum()
    }

    pub(super) fn bridge_is_retained(&self, bridge: &BridgeAuthorizationRuntime) -> bool {
        self.policy.iter().all(|fact| {
            bridge.retains(&fact.bridge)
                && fact.bridge.is_allowed()
                && fact.bridge.dependency_identity()
                    == fact.relational.observation_identity().bytes()
        })
    }

    pub(in crate::domain_computation) fn remains_current_in(
        &self,
        runtime: &worth_relational::facade::runtime::RelationalRuntime,
        snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
        bridge: &BridgeAuthorizationRuntime,
    ) -> bool {
        self.principal.remains_current_in(runtime, snapshot)
            && self.bridge_is_retained(bridge)
            && self.policy.iter().all(|fact| {
                runtime.compare_authorization_observation(&fact.relational, snapshot.clone())
                    == RelationalAuthorizationObservationFreshness::Fresh
            })
    }

    pub(in crate::domain_computation) fn into_parts(
        self,
    ) -> (
        WorthQueryPrincipalCurrentnessDependency,
        Vec<WorthQueryAuthorizationDecisionFact>,
    ) {
        (self.principal, self.policy)
    }
}
