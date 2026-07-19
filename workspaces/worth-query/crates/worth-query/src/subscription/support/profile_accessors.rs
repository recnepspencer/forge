use crate::identity_authority::{QueryProjectionIdentity, QuerySubscriptionIdentityKind};

use super::super::evidence_projection::subscription_evidence_projection;
use super::profile::QuerySubscriptionSupportProfile;

impl QuerySubscriptionSupportProfile {
    pub fn source_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(self.source_identity())
    }

    pub fn profile_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(self.profile_identity())
    }
}
