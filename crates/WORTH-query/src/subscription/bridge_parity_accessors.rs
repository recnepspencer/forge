use crate::evidence_identity::WorthQueryEvidenceIdentity;
use crate::identity_authority::{QueryProjectionIdentity, QuerySubscriptionIdentityKind};

use super::bridge_parity::QuerySubscriptionBridgeParityFailure;
use super::evidence_projection::subscription_evidence_projection;

impl QuerySubscriptionBridgeParityFailure {
    pub fn source_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.source_identity
    }

    pub fn source_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(self.source_identity())
    }
}
