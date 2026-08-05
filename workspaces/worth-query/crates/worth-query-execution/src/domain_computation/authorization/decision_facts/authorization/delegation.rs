use std::sync::Arc;

use worth_relational::facade::authorization::{
    RelationalAuthorizationObservationCounters, RelationalAuthorizationObservationEvidence,
};
use worth_runtime_bridge::facade::BridgeAuthorizationRuntime;

use super::{add_counters, observation_is_current, WorthQueryAuthorizationDecisionFact};

#[derive(Clone)]
pub(in crate::domain_computation::authorization) enum WorthQueryDelegationDecisionFact {
    Root {
        discovery: Arc<RelationalAuthorizationObservationEvidence>,
    },
    Delegated {
        grantor: worth_relational::facade::identity::EntityId,
        parent_grant: worth_relational::facade::identity::EntityId,
        discovery: Arc<RelationalAuthorizationObservationEvidence>,
        transition: Arc<RelationalAuthorizationObservationEvidence>,
        parent: Arc<WorthQueryAuthorizationDecisionFact>,
    },
}

impl WorthQueryDelegationDecisionFact {
    pub(in crate::domain_computation::authorization) fn root(
        discovery: RelationalAuthorizationObservationEvidence,
    ) -> Self {
        Self::Root {
            discovery: Arc::new(discovery),
        }
    }

    pub(in crate::domain_computation::authorization) fn delegated(
        grantor: worth_relational::facade::identity::EntityId,
        parent_grant: worth_relational::facade::identity::EntityId,
        discovery: RelationalAuthorizationObservationEvidence,
        transition: RelationalAuthorizationObservationEvidence,
        parent: WorthQueryAuthorizationDecisionFact,
    ) -> Self {
        Self::Delegated {
            grantor,
            parent_grant,
            discovery: Arc::new(discovery),
            transition: Arc::new(transition),
            parent: Arc::new(parent),
        }
    }

    pub(super) fn belongs_to_session(
        &self,
        session: crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity,
    ) -> bool {
        match self {
            Self::Root { .. } => true,
            Self::Delegated { parent, .. } => parent.session_identity() == session,
        }
    }

    pub(super) fn add_relational_counters(
        &self,
        counters: &mut RelationalAuthorizationObservationCounters,
    ) {
        match self {
            Self::Root { discovery } => add_counters(counters, discovery.counters()),
            Self::Delegated {
                discovery,
                transition,
                parent,
                ..
            } => {
                add_counters(counters, discovery.counters());
                add_counters(counters, transition.counters());
                add_counters(counters, parent.relational_counters());
            }
        }
    }

    pub(super) fn signal_dependency_count(&self) -> usize {
        match self {
            Self::Root { .. } => 0,
            Self::Delegated { parent, .. } => parent.signal_dependency_count(),
        }
    }

    pub(super) fn bridge_is_retained(&self, bridge: &BridgeAuthorizationRuntime) -> bool {
        match self {
            Self::Root { .. } => true,
            Self::Delegated { parent, .. } => parent.bridge_is_retained(bridge),
        }
    }

    pub(super) fn remains_current_in(
        &self,
        runtime: &worth_relational::facade::runtime::RelationalRuntime,
        snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
        bridge: &BridgeAuthorizationRuntime,
    ) -> bool {
        self.remains_equal_in(runtime, snapshot)
            && match self {
                Self::Root { .. } => true,
                Self::Delegated { parent, .. } => {
                    parent.remains_current_in(runtime, snapshot, bridge)
                }
            }
    }

    pub(super) fn remains_equal_in(
        &self,
        runtime: &worth_relational::facade::runtime::RelationalRuntime,
        snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    ) -> bool {
        match self {
            Self::Root { discovery } => observation_is_current(runtime, snapshot, discovery),
            Self::Delegated {
                discovery,
                transition,
                parent,
                ..
            } => {
                observation_is_current(runtime, snapshot, discovery)
                    && observation_is_current(runtime, snapshot, transition)
                    && parent.remains_equal_in(runtime, snapshot)
            }
        }
    }

    pub(super) fn has_same_lineage(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Root { .. }, Self::Root { .. }) => true,
            (
                Self::Delegated {
                    grantor: left_grantor,
                    parent_grant: left_parent,
                    parent: left,
                    ..
                },
                Self::Delegated {
                    grantor: right_grantor,
                    parent_grant: right_parent,
                    parent: right,
                    ..
                },
            ) => {
                left_grantor == right_grantor
                    && left_parent == right_parent
                    && left.has_same_lineage(right)
            }
            (Self::Root { .. }, Self::Delegated { .. })
            | (Self::Delegated { .. }, Self::Root { .. }) => false,
        }
    }
}
