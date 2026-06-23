use crate::identity_authority::{QueryProjectionIdentity, QuerySubscriptionIdentityKind};

use super::super::evidence_projection::subscription_evidence_projection;
use super::error::{
    QuerySubscriptionRuntimeCertificationCounters, QuerySubscriptionRuntimeCertificationError,
};

impl QuerySubscriptionRuntimeCertificationCounters {
    pub fn counter_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.counter_identity())
    }
}

impl QuerySubscriptionRuntimeCertificationError {
    pub fn failure_identity(&self) -> &crate::evidence_identity::ForgeQueryEvidenceIdentity {
        &self.failure_identity
    }

    pub fn failure_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.failure_identity)
    }
}
