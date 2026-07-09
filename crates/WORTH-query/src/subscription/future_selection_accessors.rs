use crate::identity_authority::{QueryProjectionIdentity, QuerySubscriptionIdentityKind};

use super::evidence_projection::subscription_evidence_projection;
use super::future_selection::QuerySubscriptionFutureSelection;

impl QuerySubscriptionFutureSelection {
    pub fn future_selection_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(self.projection_identity())
    }
}
