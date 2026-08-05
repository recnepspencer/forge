use std::sync::Arc;

use worth_relational::facade::authorization::{
    RelationalAuthorizationObservationCounters, RelationalAuthorizationObservationEvidence,
};
use worth_runtime_bridge::facade::BridgeAuthorizationRuntime;

use super::{add_counters, observation_is_current};

#[derive(Clone)]
pub(in crate::domain_computation::authorization) struct WorthQueryDelegationActivationDecisionFact {
    session_identity:
        crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity,
    narrowing: Arc<RelationalAuthorizationObservationEvidence>,
}

impl WorthQueryDelegationActivationDecisionFact {
    pub(in crate::domain_computation::authorization) fn new(
        session_identity: crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity,
        narrowing: RelationalAuthorizationObservationEvidence,
    ) -> Self {
        Self {
            session_identity,
            narrowing: Arc::new(narrowing),
        }
    }

    pub(super) fn belongs_to_session(
        &self,
        session: crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity,
    ) -> bool {
        self.session_identity == session
    }

    pub(super) fn retained_for_session(
        &self,
        session_identity: crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity,
    ) -> Self {
        Self {
            session_identity,
            narrowing: Arc::clone(&self.narrowing),
        }
    }

    pub(super) fn add_relational_counters(
        &self,
        counters: &mut RelationalAuthorizationObservationCounters,
    ) {
        add_counters(counters, self.narrowing.counters());
    }

    pub(super) fn signal_dependency_count(&self) -> usize {
        0
    }

    pub(super) fn bridge_is_retained(&self, bridge: &BridgeAuthorizationRuntime) -> bool {
        let _ = bridge;
        true
    }

    pub(super) fn remains_current_in(
        &self,
        runtime: &worth_relational::facade::runtime::RelationalRuntime,
        snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
        bridge: &BridgeAuthorizationRuntime,
    ) -> bool {
        let _ = bridge;
        observation_is_current(runtime, snapshot, &self.narrowing)
    }

    pub(super) fn remains_equal_in(
        &self,
        runtime: &worth_relational::facade::runtime::RelationalRuntime,
        snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    ) -> bool {
        observation_is_current(runtime, snapshot, &self.narrowing)
    }
}
