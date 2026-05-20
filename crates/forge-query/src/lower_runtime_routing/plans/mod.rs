use crate::identity::hash_parts;

use super::ForgeQueryLowerRuntimeCapabilityEligibility;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeRoutePlan {
    eligibility: ForgeQueryLowerRuntimeCapabilityEligibility,
    route_subject: String,
    route_digest: String,
}

impl ForgeQueryLowerRuntimeRoutePlan {
    pub(crate) fn new(
        eligibility: ForgeQueryLowerRuntimeCapabilityEligibility,
        route_subject: impl Into<String>,
    ) -> Self {
        let route_subject = route_subject.into();
        let route_digest = hash_parts(&[
            "lower_runtime_route_plan_v1".to_string(),
            format!("eligibility:{}", eligibility.eligibility_digest()),
            format!("route_subject:{route_subject}"),
        ]);
        Self {
            eligibility,
            route_subject,
            route_digest,
        }
    }

    pub fn eligibility(&self) -> &ForgeQueryLowerRuntimeCapabilityEligibility {
        &self.eligibility
    }

    pub fn route_subject(&self) -> &str {
        &self.route_subject
    }

    pub fn route_digest(&self) -> &str {
        &self.route_digest
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lower_runtime_routing::{
        ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeCapabilityRequest,
        ForgeQueryLowerRuntimeRouteKind, ForgeQueryLowerRuntimeSeamKey,
    };

    #[test]
    fn route_plan_digest_reuses_eligibility_digest() {
        let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
            ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
            ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
            ForgeQueryLowerRuntimeAuthorityOwner::Query,
            "write-authority",
            "subject-1",
        );
        let eligibility =
            ForgeQueryLowerRuntimeCapabilityEligibility::admitted(request, "detail-1");
        let plan = ForgeQueryLowerRuntimeRoutePlan::new(eligibility, "mutation-write");

        assert_eq!(plan.route_subject(), "mutation-write");
        assert!(!plan.route_digest().is_empty());
    }
}
