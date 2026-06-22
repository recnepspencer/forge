use crate::identity_authority::{QueryProjectionIdentity, QuerySubscriptionIdentityKind};

use super::evidence_projection::subscription_evidence_projection;
use super::signal_strategy::QuerySubscriptionSignalStrategyRequest;

impl QuerySubscriptionSignalStrategyRequest {
    pub fn signal_strategy_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(self.evidence_identity())
    }
}
