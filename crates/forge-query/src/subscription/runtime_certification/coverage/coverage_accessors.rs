use super::matrix::{CertifiedFamilyCoverageHandle, QuerySubscriptionFamilyCoverageMatrix};
use super::variations::{
    QuerySubscriptionBasisVariationSet, QuerySubscriptionLifecycleClassVariationSet,
    QuerySubscriptionPolicyVariationSet, QuerySubscriptionRelationshipProofVariationSet,
    QuerySubscriptionTenantVariationSet, QuerySubscriptionViewShapeVariationSet,
};
use crate::identity_authority::{QueryProjectionIdentity, QuerySubscriptionIdentityKind};
use crate::subscription::evidence_projection::subscription_evidence_projection;

macro_rules! variation_projection {
    ($ty:ty) => {
        impl $ty {
            pub fn variation_projection(
                &self,
            ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
                subscription_evidence_projection(self.variation_identity())
            }
        }
    };
}

variation_projection!(QuerySubscriptionBasisVariationSet);
variation_projection!(QuerySubscriptionPolicyVariationSet);
variation_projection!(QuerySubscriptionTenantVariationSet);
variation_projection!(QuerySubscriptionRelationshipProofVariationSet);
variation_projection!(QuerySubscriptionViewShapeVariationSet);
variation_projection!(QuerySubscriptionLifecycleClassVariationSet);

impl QuerySubscriptionFamilyCoverageMatrix {
    pub fn family_coverage_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(self.family_coverage_identity())
    }
}

impl CertifiedFamilyCoverageHandle {
    pub fn family_coverage_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(self.family_coverage_identity())
    }
}
