use worth_foundational::facade::{
    admit_foundational_authority_identity, project_foundational_identity,
};
use worth_proof::{AuthorityMarker, AuthorityWitness};

use crate::WorthQueryEvidenceIdentity;

use super::authority::query_subscription_authority;
use super::categories::QueryProjectionIdentity;
use super::kinds::QuerySubscriptionIdentityKind;

pub(crate) fn project_query_subscription_evidence(
    identity: &WorthQueryEvidenceIdentity,
) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
    project_query_evidence_with_authority(
        identity,
        query_subscription_authority(),
        query_subscription_authority(),
    )
}

fn project_query_evidence_with_authority<Authority, Kind>(
    identity: &WorthQueryEvidenceIdentity,
    admit_authority: AuthorityWitness<Authority>,
    project_authority: AuthorityWitness<Authority>,
) -> QueryProjectionIdentity<String, Kind>
where
    Authority: AuthorityMarker,
    Kind: worth_foundational::facade::FoundationalIdentityKind,
{
    let authority_identity =
        admit_foundational_authority_identity(identity.clone(), admit_authority);
    project_foundational_identity(
        &authority_identity,
        identity.reporting_projection().to_string(),
        project_authority,
    )
}
