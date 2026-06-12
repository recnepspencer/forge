use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::ForgeQueryLowerRuntimeCapabilityEligibility;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeRouteSubjectIdentity {
    evidence_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryLowerRuntimeRouteSubjectIdentity {
    pub(crate) fn from_evidence_identity(
        route_family: impl AsRef<str>,
        subject_identity: &ForgeQueryEvidenceIdentity,
    ) -> Self {
        Self {
            evidence_identity: ForgeQueryEvidenceIdentity::compose(
                ForgeQueryEvidenceScope::LowerRuntimeRouteSubject,
            )
            .field_shape(ForgeQueryEvidenceTag::new("route_family"), route_family)
            .field_evidence_identity(ForgeQueryEvidenceTag::new("subject"), subject_identity)
            .seal(),
        }
    }

    pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.evidence_identity
    }

    pub fn as_str(&self) -> &str {
        self.evidence_identity.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeRoutePlan {
    eligibility: ForgeQueryLowerRuntimeCapabilityEligibility,
    route_subject: ForgeQueryLowerRuntimeRouteSubjectIdentity,
    route_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryLowerRuntimeRoutePlan {
    pub(crate) fn new(
        eligibility: ForgeQueryLowerRuntimeCapabilityEligibility,
        route_subject: ForgeQueryLowerRuntimeRouteSubjectIdentity,
    ) -> Self {
        let route_identity =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeRoutePlan)
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("eligibility"),
                    eligibility.eligibility_identity(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("route_subject"),
                    route_subject.evidence_identity(),
                )
                .seal();
        Self {
            eligibility,
            route_subject,
            route_identity,
        }
    }

    pub fn eligibility(&self) -> &ForgeQueryLowerRuntimeCapabilityEligibility {
        &self.eligibility
    }

    pub fn route_subject(&self) -> &ForgeQueryLowerRuntimeRouteSubjectIdentity {
        &self.route_subject
    }

    pub fn route_digest(&self) -> &str {
        self.route_identity.as_str()
    }

    pub fn route_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.route_identity
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lower_runtime_routing::{
        ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeCapabilityRequest,
        ForgeQueryLowerRuntimeRouteKind, ForgeQueryLowerRuntimeSeamKey,
        ForgeQueryLowerRuntimeSubjectIdentity,
    };

    #[test]
    fn route_plan_digest_reuses_eligibility_digest() {
        let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
            ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
            ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
            ForgeQueryLowerRuntimeAuthorityOwner::Query,
            "write-authority",
            ForgeQueryLowerRuntimeSubjectIdentity::compose("test-subject")
                .field_identity(ForgeQueryEvidenceTag::new("test_subject"), "subject-1")
                .seal(),
        );
        let detail_identity = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
        )
        .field_identity(ForgeQueryEvidenceTag::new("test_detail"), "detail-1")
        .seal();
        let eligibility =
            ForgeQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
                request,
                &detail_identity,
            );
        let plan = ForgeQueryLowerRuntimeRoutePlan::new(
            eligibility,
            ForgeQueryLowerRuntimeRouteSubjectIdentity::from_evidence_identity(
                "test-route",
                &detail_identity,
            ),
        );

        assert_eq!(
            plan.route_subject().evidence_identity().scope(),
            ForgeQueryEvidenceScope::LowerRuntimeRouteSubject
        );
        assert!(!plan.route_digest().is_empty());
    }
}
