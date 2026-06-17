use crate::identity_authority::{QueryProjectionIdentity, QuerySubscriptionIdentityKind};

use super::admission_diagnostics::QuerySubscriptionAdmissionDiagnostics;
use super::evidence_projection::subscription_evidence_projection;

impl QuerySubscriptionAdmissionDiagnostics {
    pub fn diagnostics_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(self.diagnostics_identity())
    }
}
