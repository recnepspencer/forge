use std::sync::Arc;

use worth_relational::facade::authorization::{
    RelationalAuthorizationObservationEvidence, RelationalAuthorizationObservationFreshness,
};
use worth_runtime_bridge::facade::BridgeAuthorizationDecisionEvidence;

use super::super::authorization::WorthQueryPrincipalCurrentnessDependency;

#[derive(Clone)]
pub(in crate::domain_computation::primary_graph) enum WorthQueryPrimaryGraphApplicationDecisionFact
{
    Application(super::super::application_attempt::WorthQueryApplicationObservedFact),
    Principal(WorthQueryPrincipalCurrentnessDependency),
    Authorization {
        locator: Arc<str>,
        observation: Arc<RelationalAuthorizationObservationEvidence>,
        bridge: Arc<BridgeAuthorizationDecisionEvidence>,
    },
}

impl WorthQueryPrimaryGraphApplicationDecisionFact {
    pub(in crate::domain_computation::primary_graph) const fn application(
        fact: super::super::application_attempt::WorthQueryApplicationObservedFact,
    ) -> Self {
        Self::Application(fact)
    }

    pub(in crate::domain_computation::primary_graph) const fn principal(
        dependency: WorthQueryPrincipalCurrentnessDependency,
    ) -> Self {
        Self::Principal(dependency)
    }

    pub(in crate::domain_computation::primary_graph) fn authorization(
        requirement_ordinal: usize,
        dependency: super::super::authorization::WorthQueryAuthorizationDecisionFact,
    ) -> Self {
        let super::super::authorization::WorthQueryAuthorizationDecisionFact {
            relational: observation,
            bridge,
        } = dependency;
        let locator = format!("application-authorization:{requirement_ordinal}");
        Self::Authorization {
            locator: Arc::from(locator),
            observation: Arc::new(observation),
            bridge: Arc::new(bridge),
        }
    }

    pub(in crate::domain_computation::primary_graph) fn locator_identity(&self) -> String {
        match self {
            Self::Application(fact) => fact.locator_identity(),
            Self::Principal(_) => "application-principal-currentness".to_string(),
            Self::Authorization { locator, .. } => locator.to_string(),
        }
    }

    pub(super) fn remains_equal_in(
        &self,
        runtime: &worth_relational::facade::runtime::RelationalRuntime,
        snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    ) -> bool {
        match self {
            Self::Application(fact) => fact.remains_equal_in(runtime, snapshot),
            Self::Principal(dependency) => dependency.remains_current_in(runtime, snapshot),
            Self::Authorization {
                observation,
                bridge,
                ..
            } => {
                bridge.is_allowed()
                    && bridge.dependency_identity() == observation.observation_identity().bytes()
                    && runtime.compare_authorization_observation(observation, snapshot.clone())
                        == RelationalAuthorizationObservationFreshness::Fresh
            }
        }
    }
}
