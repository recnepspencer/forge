use crate::identity_authority::{QueryProjectionIdentity, QuerySubscriptionIdentityKind};

use super::equivalence::QuerySubscriptionEquivalenceBasis;
use super::evidence_projection::subscription_evidence_projection;

impl QuerySubscriptionEquivalenceBasis {
    pub fn equivalence_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(self.evidence_identity())
    }
}
