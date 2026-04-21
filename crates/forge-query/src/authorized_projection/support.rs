use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AuthorizedProjectionSurface {
    AuthorizedProjectionArtifact,
    MaskedInfluenceValidation,
    ImmutableMaskBoundary,
    OptimizerSafeProjection,
    PolicyAwareExecution,
}

impl AuthorizedProjectionSurface {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AuthorizedProjectionArtifact => "authorized_projection_artifact",
            Self::MaskedInfluenceValidation => "masked_influence_validation",
            Self::ImmutableMaskBoundary => "immutable_mask_boundary",
            Self::OptimizerSafeProjection => "optimizer_safe_projection",
            Self::PolicyAwareExecution => "policy_aware_execution",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AuthorizedProjectionSupportStatus {
    Verified,
    Deferred,
}

impl AuthorizedProjectionSupportStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Deferred => "deferred",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorizedProjectionSupportProfile {
    surfaces: Vec<(
        AuthorizedProjectionSurface,
        AuthorizedProjectionSupportStatus,
    )>,
    profile_digest: String,
}

impl AuthorizedProjectionSupportProfile {
    pub(crate) fn new(
        surfaces: Vec<(
            AuthorizedProjectionSurface,
            AuthorizedProjectionSupportStatus,
        )>,
    ) -> Self {
        let profile_digest = hash_parts(
            &surfaces
                .iter()
                .map(|(surface, status)| format!("{}:{}", surface.as_str(), status.as_str()))
                .collect::<Vec<_>>(),
        );
        Self {
            surfaces,
            profile_digest,
        }
    }

    pub fn surfaces(
        &self,
    ) -> &[(
        AuthorizedProjectionSurface,
        AuthorizedProjectionSupportStatus,
    )] {
        &self.surfaces
    }

    pub fn profile_digest(&self) -> &str {
        &self.profile_digest
    }
}

pub fn runtime_backed_authorized_projection_support_profile() -> AuthorizedProjectionSupportProfile
{
    AuthorizedProjectionSupportProfile::new(vec![
        (
            AuthorizedProjectionSurface::AuthorizedProjectionArtifact,
            AuthorizedProjectionSupportStatus::Verified,
        ),
        (
            AuthorizedProjectionSurface::MaskedInfluenceValidation,
            AuthorizedProjectionSupportStatus::Verified,
        ),
        (
            AuthorizedProjectionSurface::ImmutableMaskBoundary,
            AuthorizedProjectionSupportStatus::Verified,
        ),
        (
            AuthorizedProjectionSurface::OptimizerSafeProjection,
            AuthorizedProjectionSupportStatus::Verified,
        ),
        (
            AuthorizedProjectionSurface::PolicyAwareExecution,
            AuthorizedProjectionSupportStatus::Deferred,
        ),
    ])
}
