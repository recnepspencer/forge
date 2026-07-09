use crate::identity_authority::{QueryProjectionIdentity, QuerySubscriptionIdentityKind};

use super::basis_request::QuerySubscriptionBasisBindingRequest;
use super::evidence_projection::subscription_evidence_projection;

impl QuerySubscriptionBasisBindingRequest {
    pub fn basis_binding_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(self.evidence_identity())
    }

    pub fn source_declaration_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(self.source_declaration_identity())
    }
}
