use std::sync::Arc;

use worth_relational::facade::authorization::{
    RelationalAuthorizationObservationEvidence, RelationalAuthorizationObservationFreshness,
};
use worth_runtime_bridge::facade::{
    BridgeAuthorizationDecisionEvidence, BridgeAuthorizationRuntime,
};

use super::super::{
    freshness::WorthQueryPrincipalFreshnessEvidence,
    schema_layout::WorthQueryPrimaryPrincipalBindingLayout, WorthQueryAuthenticatedPrincipal,
};

pub(in crate::domain_computation::primary_graph) struct WorthQueryAuthorizationDecisionFact {
    pub(in crate::domain_computation::primary_graph) relational:
        RelationalAuthorizationObservationEvidence,
    pub(in crate::domain_computation::primary_graph) bridge: BridgeAuthorizationDecisionEvidence,
}

#[derive(Clone)]
pub(in crate::domain_computation::primary_graph) struct WorthQueryPrincipalCurrentnessDependency {
    binding: Arc<str>,
    layout: WorthQueryPrimaryPrincipalBindingLayout,
    freshness: WorthQueryPrincipalFreshnessEvidence,
}

impl WorthQueryPrincipalCurrentnessDependency {
    pub(super) fn capture<Schema, Principal, PrincipalIdentity>(
        principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
        layout: &WorthQueryPrimaryPrincipalBindingLayout,
    ) -> Self {
        Self {
            binding: Arc::from(principal.binding()),
            layout: layout.clone(),
            freshness: principal.freshness().clone(),
        }
    }

    pub(in crate::domain_computation::primary_graph) fn remains_current_in(
        &self,
        runtime: &worth_relational::facade::runtime::RelationalRuntime,
        snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    ) -> bool {
        self.freshness
            .remains_current_in(runtime, snapshot, &self.layout, &self.binding)
    }
}

pub(in crate::domain_computation::primary_graph) struct WorthQueryRetainedAuthorizationDecisionFacts
{
    principal: WorthQueryPrincipalCurrentnessDependency,
    policy: Vec<WorthQueryAuthorizationDecisionFact>,
}

impl WorthQueryRetainedAuthorizationDecisionFacts {
    pub(super) fn new(
        principal: WorthQueryPrincipalCurrentnessDependency,
        policy: Vec<WorthQueryAuthorizationDecisionFact>,
    ) -> Self {
        Self { principal, policy }
    }

    pub(super) fn policy(&self) -> &[WorthQueryAuthorizationDecisionFact] {
        &self.policy
    }

    pub(super) fn policy_count(&self) -> usize {
        self.policy.len()
    }

    pub(super) fn exact_fact_count(&self) -> usize {
        1usize.saturating_add(self.policy.len())
    }

    pub(super) fn bridge_is_retained(&self, bridge: &BridgeAuthorizationRuntime) -> bool {
        self.policy.iter().all(|fact| {
            bridge.retains(&fact.bridge)
                && fact.bridge.is_allowed()
                && fact.bridge.dependency_identity()
                    == fact.relational.observation_identity().bytes()
        })
    }

    pub(in crate::domain_computation::primary_graph) fn remains_current_in(
        &self,
        runtime: &worth_relational::facade::runtime::RelationalRuntime,
        snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
        bridge: &BridgeAuthorizationRuntime,
    ) -> bool {
        self.principal.remains_current_in(runtime, snapshot)
            && self.bridge_is_retained(bridge)
            && self.policy.iter().all(|fact| {
                runtime.compare_authorization_observation(
                    &fact.relational,
                    snapshot.clone(),
                ) == RelationalAuthorizationObservationFreshness::Fresh
            })
    }

    pub(in crate::domain_computation::primary_graph) fn into_parts(
        self,
    ) -> (
        WorthQueryPrincipalCurrentnessDependency,
        Vec<WorthQueryAuthorizationDecisionFact>,
    ) {
        (self.principal, self.policy)
    }
}
