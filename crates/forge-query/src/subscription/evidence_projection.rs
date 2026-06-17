use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::identity_authority::{
    project_query_subscription_evidence, QueryProjectionIdentity, QuerySubscriptionIdentityKind,
};

pub(crate) fn subscription_evidence_projection(
    identity: &ForgeQueryEvidenceIdentity,
) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
    project_query_subscription_evidence(identity)
}
