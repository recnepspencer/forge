use crate::identity::hash_parts;

use super::ForgeQueryLowerRuntimeCapabilityRequest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeCapabilityEligibility {
    request: ForgeQueryLowerRuntimeCapabilityRequest,
    posture: ForgeQueryLowerRuntimeCapabilityPosture,
    posture_detail_digest: String,
    eligibility_digest: String,
}

impl ForgeQueryLowerRuntimeCapabilityEligibility {
    pub(crate) fn admitted(
        request: ForgeQueryLowerRuntimeCapabilityRequest,
        posture_detail_digest: impl Into<String>,
    ) -> Self {
        Self::new(
            request,
            ForgeQueryLowerRuntimeCapabilityPosture::Admitted,
            posture_detail_digest,
        )
    }

    pub(crate) fn new(
        request: ForgeQueryLowerRuntimeCapabilityRequest,
        posture: ForgeQueryLowerRuntimeCapabilityPosture,
        posture_detail_digest: impl Into<String>,
    ) -> Self {
        let posture_detail_digest = posture_detail_digest.into();
        let eligibility_digest = hash_parts(&[
            "lower_runtime_capability_eligibility_v1".to_string(),
            format!("request:{}", request.request_digest()),
            format!("posture:{}", posture.as_str()),
            format!("detail:{posture_detail_digest}"),
        ]);
        Self {
            request,
            posture,
            posture_detail_digest,
            eligibility_digest,
        }
    }

    pub fn request(&self) -> &ForgeQueryLowerRuntimeCapabilityRequest {
        &self.request
    }

    pub fn posture(&self) -> ForgeQueryLowerRuntimeCapabilityPosture {
        self.posture
    }

    pub fn posture_detail_digest(&self) -> &str {
        &self.posture_detail_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.eligibility_digest
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
        ForgeQueryLowerRuntimeSeamKey,
    };

    #[test]
    fn eligibility_digest_binds_request_posture_and_detail() {
        let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
            ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
            ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
            ForgeQueryLowerRuntimeAuthorityOwner::Query,
            "write-authority",
            "subject-1",
        );

        let eligibility =
            ForgeQueryLowerRuntimeCapabilityEligibility::admitted(request, "detail-1");

        assert_eq!(eligibility.posture().as_str(), "admitted");
        assert!(!eligibility.eligibility_digest().is_empty());
    }
}
