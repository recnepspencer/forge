use crate::identity_authority::{QueryProjectionIdentity, QuerySubscriptionIdentityKind};

use super::declaration::QuerySubscriptionDeclarationArtifact;
use super::evidence_projection::subscription_evidence_projection;

impl QuerySubscriptionDeclarationArtifact {
    pub fn declaration_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(self.declaration_identity())
    }

    pub fn equivalence_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(self.equivalence_identity())
    }
}
