use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::ForgeQueryLowerRuntimeCapabilityRequest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeCapabilityEligibility {
    request: ForgeQueryLowerRuntimeCapabilityRequest,
    posture: ForgeQueryLowerRuntimeCapabilityPosture,
    posture_detail_identity: ForgeQueryEvidenceIdentity,
    eligibility_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryLowerRuntimeCapabilityEligibility {
    pub(crate) fn admitted_with_evidence_identity(
        request: ForgeQueryLowerRuntimeCapabilityRequest,
        posture_detail_identity: &ForgeQueryEvidenceIdentity,
    ) -> Self {
        Self::new_with_evidence_identity(
            request,
            ForgeQueryLowerRuntimeCapabilityPosture::Admitted,
            posture_detail_identity,
        )
    }

    pub(crate) fn new_with_evidence_identity(
        request: ForgeQueryLowerRuntimeCapabilityRequest,
        posture: ForgeQueryLowerRuntimeCapabilityPosture,
        posture_detail_identity: &ForgeQueryEvidenceIdentity,
    ) -> Self {
        let eligibility_identity = Self::digest_builder(&request, posture)
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("detail"),
                posture_detail_identity,
            )
            .seal();
        Self {
            request,
            posture,
            posture_detail_identity: posture_detail_identity.clone(),
            eligibility_identity,
        }
    }

    fn digest_builder(
        request: &ForgeQueryLowerRuntimeCapabilityRequest,
        posture: ForgeQueryLowerRuntimeCapabilityPosture,
    ) -> crate::evidence_identity::ForgeQueryEvidenceIdentityEncoder {
        forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeCapabilityEligibility)
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("request"),
                request.request_identity(),
            )
            .field_shape(ForgeQueryEvidenceTag::new("posture"), posture.as_str())
    }

    pub fn request(&self) -> &ForgeQueryLowerRuntimeCapabilityRequest {
        &self.request
    }

    pub fn posture(&self) -> ForgeQueryLowerRuntimeCapabilityPosture {
        self.posture
    }

    pub fn posture_detail_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.posture_detail_identity
    }

    pub fn eligibility_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.eligibility_identity
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryLowerRuntimeCapabilityPosture {
    Admitted,
    Deferred,
    Unsupported,
    Forbidden,
}

impl ForgeQueryLowerRuntimeCapabilityPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Deferred => "deferred",
            Self::Unsupported => "unsupported",
            Self::Forbidden => "forbidden",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lower_runtime_routing::{
        ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeRouteKind,
        ForgeQueryLowerRuntimeSeamKey, ForgeQueryLowerRuntimeSubjectIdentity,
    };

    #[test]
    fn eligibility_digest_binds_request_posture_and_detail() {
        let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
            ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
            ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
            ForgeQueryLowerRuntimeAuthorityOwner::Query,
            "write-authority",
            ForgeQueryLowerRuntimeSubjectIdentity::compose("test-subject")
                .field_value(ForgeQueryEvidenceTag::new("test_subject"), "subject-1")
                .seal(),
        );

        let detail_identity = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
        )
        .field_value(ForgeQueryEvidenceTag::new("test_detail"), "detail-1")
        .seal();
        let eligibility =
            ForgeQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
                request,
                &detail_identity,
            );

        assert_eq!(eligibility.posture().as_str(), "admitted");
        let eligibility_identity = eligibility.eligibility_identity();
        assert!(!eligibility_identity.reporting_projection().is_empty());
    }
}
