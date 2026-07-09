use std::sync::Arc;

use crate::policy::BridgeRoutePlanningPolicy;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct RouteScope {
    route_planning_policy: Option<BridgeRoutePlanningPolicy>,
    route_planning_policy_digest: Option<Arc<str>>,
}

impl RouteScope {
    pub(crate) fn begin() -> Self {
        Self {
            route_planning_policy: None,
            route_planning_policy_digest: None,
        }
    }

    pub(crate) fn with_route_planning_policy(
        mut self,
        route_planning_policy: BridgeRoutePlanningPolicy,
    ) -> Self {
        self.route_planning_policy_digest =
            Some(Arc::from(route_planning_policy.digest().to_owned()));
        self.route_planning_policy = Some(route_planning_policy);
        self
    }

    pub(crate) fn with_route_planning_policy_digest(
        mut self,
        route_planning_policy_digest: impl Into<Arc<str>>,
    ) -> Self {
        self.route_planning_policy = None;
        self.route_planning_policy_digest = Some(route_planning_policy_digest.into());
        self
    }

    pub(crate) fn route_planning_policy(&self) -> Option<&BridgeRoutePlanningPolicy> {
        self.route_planning_policy.as_ref()
    }

    pub(crate) fn route_planning_policy_digest(&self) -> Option<&str> {
        self.route_planning_policy_digest.as_deref()
    }
}
