use std::sync::Arc;

use worth_relational::facade::authorization::{
    RelationalAuthorizationObservationCounters, RelationalAuthorizationObservationEvidence,
    RelationalAuthorizationObservationFreshness,
};
use worth_runtime_bridge::facade::{
    BridgeAuthorizationDecisionEvidence, BridgeAuthorizationRuntime,
};

use super::{
    WorthQueryCapabilityCommitBasis, WorthQueryOperationAdmissionIdentity,
    WorthQueryRetainedCapabilityAuthorization,
};
mod principal_currentness;
pub(in crate::domain_computation) use principal_currentness::WorthQueryPrincipalCurrentnessDependency;

#[derive(Clone)]
pub(in crate::domain_computation) struct WorthQueryAuthorizationDecisionFact {
    pub(in crate::domain_computation) session_identity:
        crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity,
    pub(in crate::domain_computation) relational: Arc<RelationalAuthorizationObservationEvidence>,
    pub(in crate::domain_computation) bridge: Arc<BridgeAuthorizationDecisionEvidence>,
}

impl WorthQueryAuthorizationDecisionFact {
    pub(in crate::domain_computation) fn new(
        session_identity: crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity,
        relational: RelationalAuthorizationObservationEvidence,
        bridge: BridgeAuthorizationDecisionEvidence,
    ) -> Self {
        Self {
            session_identity,
            relational: Arc::new(relational),
            bridge: Arc::new(bridge),
        }
    }

    pub(in crate::domain_computation) const fn session_identity(
        &self,
    ) -> crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity {
        self.session_identity
    }

    pub(super) fn remains_current_in(
        &self,
        runtime: &worth_relational::facade::runtime::RelationalRuntime,
        snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
        bridge: &BridgeAuthorizationRuntime,
    ) -> bool {
        bridge.retains(&self.bridge)
            && self.bridge.is_allowed()
            && self.bridge.dependency_identity() == self.relational.observation_identity().bytes()
            && runtime.compare_authorization_observation(&self.relational, snapshot.clone())
                == RelationalAuthorizationObservationFreshness::Fresh
    }
}

pub(in crate::domain_computation) enum WorthQueryRetainedAuthorizationDecisionFacts {
    Principal(WorthQueryPrincipalCurrentnessDependency),
    Abilities {
        principal: WorthQueryPrincipalCurrentnessDependency,
        decisions: Vec<WorthQueryAuthorizationDecisionFact>,
    },
    Capability(WorthQueryRetainedCapabilityAuthorization),
}

impl WorthQueryRetainedAuthorizationDecisionFacts {
    pub(in crate::domain_computation) const fn principal(
        principal: WorthQueryPrincipalCurrentnessDependency,
    ) -> Self {
        Self::Principal(principal)
    }

    pub(in crate::domain_computation) fn abilities(
        principal: WorthQueryPrincipalCurrentnessDependency,
        decisions: Vec<WorthQueryAuthorizationDecisionFact>,
    ) -> Self {
        Self::Abilities {
            principal,
            decisions,
        }
    }

    pub(super) const fn capability(
        authorization: WorthQueryRetainedCapabilityAuthorization,
    ) -> Self {
        Self::Capability(authorization)
    }

    pub(super) const fn capability_authorization(
        &self,
    ) -> Option<&WorthQueryRetainedCapabilityAuthorization> {
        match self {
            Self::Capability(authorization) => Some(authorization),
            Self::Principal(_) | Self::Abilities { .. } => None,
        }
    }

    pub(super) const fn capability_authorization_mut(
        &mut self,
    ) -> Option<&mut WorthQueryRetainedCapabilityAuthorization> {
        match self {
            Self::Capability(authorization) => Some(authorization),
            Self::Principal(_) | Self::Abilities { .. } => None,
        }
    }

    pub(super) fn policy_count(&self) -> usize {
        match self {
            Self::Principal(_) => 0,
            Self::Abilities { decisions, .. } => decisions.len(),
            Self::Capability(_) => 1,
        }
    }

    pub(in crate::domain_computation) fn exact_fact_count(&self) -> usize {
        1usize.saturating_add(self.policy_count())
    }

    pub(in crate::domain_computation) fn belongs_to_session(
        &self,
        session: crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity,
    ) -> bool {
        principal(self).session_identity() == session
            && decisions(self).all(|fact| fact.session_identity() == session)
    }

    pub(super) fn relational_counters(&self) -> RelationalAuthorizationObservationCounters {
        decisions(self).fold(
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
        decisions(self)
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
        decisions(self).all(|fact| {
            bridge.retains(&fact.bridge)
                && fact.bridge.is_allowed()
                && fact.bridge.dependency_identity()
                    == fact.relational.observation_identity().bytes()
        })
    }

    pub(in crate::domain_computation) fn validate_currentness_in(
        &self,
        runtime: &worth_relational::facade::runtime::RelationalRuntime,
        snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
        bridge: &BridgeAuthorizationRuntime,
    ) -> Result<(), super::WorthQueryOperationAuthorizationDenialKind> {
        if !self.principal_remains_current_in(runtime, snapshot) {
            return Err(super::WorthQueryOperationAuthorizationDenialKind::StalePrincipal);
        }
        self.decisions_remain_current_in(runtime, snapshot, bridge)
            .then_some(())
            .ok_or(super::WorthQueryOperationAuthorizationDenialKind::StaleAuthorization)
    }

    pub(super) fn principal_remains_current_in(
        &self,
        runtime: &worth_relational::facade::runtime::RelationalRuntime,
        snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    ) -> bool {
        principal(self).remains_current_in(runtime, snapshot)
    }

    pub(super) fn decisions_remain_current_in(
        &self,
        runtime: &worth_relational::facade::runtime::RelationalRuntime,
        snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
        bridge: &BridgeAuthorizationRuntime,
    ) -> bool {
        decisions(self).all(|fact| fact.remains_current_in(runtime, snapshot, bridge))
    }

    pub(in crate::domain_computation) fn into_provider_parts(
        self,
        admission_identity: WorthQueryOperationAdmissionIdentity,
    ) -> (
        WorthQueryProviderAuthorizationDecisionFacts,
        WorthQueryCommitAuthorizationBasis,
    ) {
        match self {
            Self::Principal(principal) => {
                let commit = WorthQueryObservedCommitBasis::new(principal.clone(), Vec::new());
                (
                    WorthQueryProviderAuthorizationDecisionFacts::new(principal, Vec::new()),
                    WorthQueryCommitAuthorizationBasis::Observed {
                        admission_identity,
                        authorization: commit,
                    },
                )
            }
            Self::Abilities {
                principal,
                decisions,
            } => {
                let commit =
                    WorthQueryObservedCommitBasis::new(principal.clone(), decisions.clone());
                (
                    WorthQueryProviderAuthorizationDecisionFacts::new(principal, decisions),
                    WorthQueryCommitAuthorizationBasis::Observed {
                        admission_identity,
                        authorization: commit,
                    },
                )
            }
            Self::Capability(authorization) => {
                let (principal, decision, commit) = authorization.into_parts();
                (
                    WorthQueryProviderAuthorizationDecisionFacts::new(principal, vec![decision]),
                    WorthQueryCommitAuthorizationBasis::Capability {
                        admission_identity,
                        authorization: commit,
                    },
                )
            }
        }
    }
}

fn principal(
    facts: &WorthQueryRetainedAuthorizationDecisionFacts,
) -> &WorthQueryPrincipalCurrentnessDependency {
    match facts {
        WorthQueryRetainedAuthorizationDecisionFacts::Principal(principal)
        | WorthQueryRetainedAuthorizationDecisionFacts::Abilities { principal, .. } => principal,
        WorthQueryRetainedAuthorizationDecisionFacts::Capability(authorization) => {
            authorization.principal()
        }
    }
}

fn decisions(
    facts: &WorthQueryRetainedAuthorizationDecisionFacts,
) -> impl Iterator<Item = &WorthQueryAuthorizationDecisionFact> {
    let slice = match facts {
        WorthQueryRetainedAuthorizationDecisionFacts::Principal(_) => &[][..],
        WorthQueryRetainedAuthorizationDecisionFacts::Abilities { decisions, .. } => decisions,
        WorthQueryRetainedAuthorizationDecisionFacts::Capability(authorization) => {
            std::slice::from_ref(authorization.decision())
        }
    };
    slice.iter()
}

pub(in crate::domain_computation) struct WorthQueryProviderAuthorizationDecisionFacts {
    principal: WorthQueryPrincipalCurrentnessDependency,
    decisions: Vec<WorthQueryAuthorizationDecisionFact>,
}

impl WorthQueryProviderAuthorizationDecisionFacts {
    fn new(
        principal: WorthQueryPrincipalCurrentnessDependency,
        decisions: Vec<WorthQueryAuthorizationDecisionFact>,
    ) -> Self {
        Self {
            principal,
            decisions,
        }
    }

    pub(in crate::domain_computation) fn into_parts(
        self,
    ) -> (
        WorthQueryPrincipalCurrentnessDependency,
        Vec<WorthQueryAuthorizationDecisionFact>,
    ) {
        (self.principal, self.decisions)
    }
}

pub(in crate::domain_computation) enum WorthQueryCommitAuthorizationBasis {
    Observed {
        admission_identity: WorthQueryOperationAdmissionIdentity,
        authorization: WorthQueryObservedCommitBasis,
    },
    Capability {
        admission_identity: WorthQueryOperationAdmissionIdentity,
        authorization: WorthQueryCapabilityCommitBasis,
    },
}

impl WorthQueryCommitAuthorizationBasis {
    pub(super) const fn admission_identity(&self) -> WorthQueryOperationAdmissionIdentity {
        match self {
            Self::Observed {
                admission_identity, ..
            }
            | Self::Capability {
                admission_identity, ..
            } => *admission_identity,
        }
    }

    pub(super) fn belongs_to_session(
        &self,
        session: crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity,
    ) -> bool {
        match self {
            Self::Observed { authorization, .. } => authorization.belongs_to_session(session),
            Self::Capability { authorization, .. } => authorization.belongs_to_session(session),
        }
    }
}

pub(in crate::domain_computation) struct WorthQueryObservedCommitBasis {
    principal: WorthQueryPrincipalCurrentnessDependency,
    decisions: Vec<WorthQueryAuthorizationDecisionFact>,
}

impl WorthQueryObservedCommitBasis {
    fn new(
        principal: WorthQueryPrincipalCurrentnessDependency,
        decisions: Vec<WorthQueryAuthorizationDecisionFact>,
    ) -> Self {
        Self {
            principal,
            decisions,
        }
    }

    fn belongs_to_session(
        &self,
        session: crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity,
    ) -> bool {
        self.principal.session_identity() == session
            && self
                .decisions
                .iter()
                .all(|fact| fact.session_identity() == session)
    }

    pub(super) fn principal_remains_current_in(
        &self,
        runtime: &worth_relational::facade::runtime::RelationalRuntime,
        snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    ) -> bool {
        self.principal.remains_current_in(runtime, snapshot)
    }

    pub(super) fn decisions_remain_current_in(
        &self,
        runtime: &worth_relational::facade::runtime::RelationalRuntime,
        snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
        bridge: &BridgeAuthorizationRuntime,
    ) -> bool {
        self.decisions
            .iter()
            .all(|fact| fact.remains_current_in(runtime, snapshot, bridge))
    }
}
