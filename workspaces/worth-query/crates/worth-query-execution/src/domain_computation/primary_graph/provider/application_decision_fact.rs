use std::sync::Arc;

use crate::domain_computation::authorization::WorthQueryPrincipalCurrentnessDependency;

#[derive(Clone)]
pub(in crate::domain_computation::primary_graph) enum WorthQueryPrimaryGraphApplicationDecisionFact
{
    Application(super::super::application_attempt::WorthQueryApplicationObservedFact),
    Principal(WorthQueryPrincipalCurrentnessDependency),
    Authorization {
        session: crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity,
        locator: Arc<str>,
        decision: crate::domain_computation::authorization::WorthQueryAuthorizationDecisionFact,
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
        dependency: crate::domain_computation::authorization::WorthQueryAuthorizationDecisionFact,
    ) -> Self {
        let session = dependency.session_identity();
        let locator = format!("application-authorization:{requirement_ordinal}");
        let fact = Self::Authorization {
            session,
            locator: Arc::from(locator),
            decision: dependency,
        };
        debug_assert_eq!(fact.session_identity(), Some(session));
        fact
    }

    pub(super) fn session_identity(
        &self,
    ) -> Option<crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity>
    {
        match self {
            Self::Principal(dependency) => Some(dependency.session_identity()),
            Self::Authorization { session, .. } => Some(*session),
            Self::Application(_) => None,
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
            Self::Authorization { decision, .. } => decision.remains_equal_in(runtime, snapshot),
        }
    }
}
