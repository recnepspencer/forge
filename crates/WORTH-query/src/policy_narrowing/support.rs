use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PolicyNarrowingSurface {
    AuthorizedProjection,
    MaskedInfluenceValidation,
    RelationshipProofDescriptorAdmission,
    NarrowedPolicyQueryArtifact,
    PolicyAwareExecution,
    PolicyAwareLive,
    PolicyAwareHistoricalDiff,
    PolicyAwareDelivery,
    StoreBackedDurability,
}

impl PolicyNarrowingSurface {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AuthorizedProjection => "authorized_projection",
            Self::MaskedInfluenceValidation => "masked_influence_validation",
            Self::RelationshipProofDescriptorAdmission => "relationship_proof_descriptor_admission",
            Self::NarrowedPolicyQueryArtifact => "narrowed_policy_query_artifact",
            Self::PolicyAwareExecution => "policy_aware_execution",
            Self::PolicyAwareLive => "policy_aware_live",
            Self::PolicyAwareHistoricalDiff => "policy_aware_historical_diff",
            Self::PolicyAwareDelivery => "policy_aware_delivery",
            Self::StoreBackedDurability => "store_backed_durability",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PolicyNarrowingSupportStatus {
    Verified,
    Deferred,
    BlockedOnWORTHStore,
}

impl PolicyNarrowingSupportStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Deferred => "deferred",
            Self::BlockedOnWORTHStore => "blocked_on_worth_store",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyNarrowingSupportProfile {
    surfaces: Vec<(PolicyNarrowingSurface, PolicyNarrowingSupportStatus)>,
    profile_digest: String,
}

impl PolicyNarrowingSupportProfile {
    pub(crate) fn new(
        surfaces: Vec<(PolicyNarrowingSurface, PolicyNarrowingSupportStatus)>,
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

    pub fn surfaces(&self) -> &[(PolicyNarrowingSurface, PolicyNarrowingSupportStatus)] {
        &self.surfaces
    }

    pub fn profile_digest(&self) -> &str {
        &self.profile_digest
    }
}

pub fn runtime_backed_policy_narrowing_support_profile() -> PolicyNarrowingSupportProfile {
    PolicyNarrowingSupportProfile::new(vec![
        (
            PolicyNarrowingSurface::AuthorizedProjection,
            PolicyNarrowingSupportStatus::Verified,
        ),
        (
            PolicyNarrowingSurface::MaskedInfluenceValidation,
            PolicyNarrowingSupportStatus::Verified,
        ),
        (
            PolicyNarrowingSurface::RelationshipProofDescriptorAdmission,
            PolicyNarrowingSupportStatus::Verified,
        ),
        (
            PolicyNarrowingSurface::NarrowedPolicyQueryArtifact,
            PolicyNarrowingSupportStatus::Verified,
        ),
        (
            PolicyNarrowingSurface::PolicyAwareExecution,
            PolicyNarrowingSupportStatus::Verified,
        ),
        (
            PolicyNarrowingSurface::PolicyAwareLive,
            PolicyNarrowingSupportStatus::Deferred,
        ),
        (
            PolicyNarrowingSurface::PolicyAwareHistoricalDiff,
            PolicyNarrowingSupportStatus::Deferred,
        ),
        (
            PolicyNarrowingSurface::PolicyAwareDelivery,
            PolicyNarrowingSupportStatus::Deferred,
        ),
        (
            PolicyNarrowingSurface::StoreBackedDurability,
            PolicyNarrowingSupportStatus::BlockedOnWORTHStore,
        ),
    ])
}
