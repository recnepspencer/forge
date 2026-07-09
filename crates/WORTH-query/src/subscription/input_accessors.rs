use crate::identity_authority::{QueryProjectionIdentity, QuerySubscriptionIdentityKind};

use super::evidence_projection::subscription_evidence_projection;
use super::input::LiveQueryAdmissionArtifact;

impl LiveQueryAdmissionArtifact {
    pub fn query_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(self.query_identity())
    }

    pub fn plan_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(self.plan_identity())
    }

    pub fn collection_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(self.collection_identity())
    }

    pub fn relevance_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(self.relevance_identity())
    }

    pub fn delivery_intent_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(self.delivery_intent_identity())
    }

    pub fn policy_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(self.policy_context_identity())
    }

    pub fn tenant_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(self.tenant_context_identity())
    }

    pub fn relationship_proof_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(self.relationship_proof_context_identity())
    }
}
