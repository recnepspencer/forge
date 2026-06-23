#![allow(dead_code)]

use forge_foundational::facade::{
    admit_foundational_authority_identity, project_foundational_identity,
};
use forge_proof::{AuthorityMarker, AuthorityWitness};

use crate::ForgeQueryEvidenceIdentity;

use super::authority::{
    query_causal_inspection_authority, query_evidence_authority, query_feeder_authority,
    query_intent_authority, query_receipt_admission_authority, query_subscription_authority,
    query_workflow_authority,
};
use super::categories::QueryProjectionIdentity;
use super::kinds::{
    QueryCausalInspectionIdentityKind, QueryEvidenceIdentityKind, QueryFeederIdentityKind,
    QueryIntentIdentityKind, QueryReceiptIdentityKind, QuerySubscriptionIdentityKind,
    QueryWorkflowIdentityKind,
};

pub(crate) fn project_query_subscription_evidence(
    identity: &ForgeQueryEvidenceIdentity,
) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
    project_query_evidence_with_authority(
        identity,
        query_subscription_authority(),
        query_subscription_authority(),
    )
}

pub(crate) fn project_query_evidence_identity(
    identity: &ForgeQueryEvidenceIdentity,
) -> QueryProjectionIdentity<String, QueryEvidenceIdentityKind> {
    project_query_evidence_with_authority(
        identity,
        query_evidence_authority(),
        query_evidence_authority(),
    )
}

pub(crate) fn project_query_feeder_evidence(
    identity: &ForgeQueryEvidenceIdentity,
) -> QueryProjectionIdentity<String, QueryFeederIdentityKind> {
    project_query_evidence_with_authority(
        identity,
        query_feeder_authority(),
        query_feeder_authority(),
    )
}

pub(crate) fn project_query_receipt_evidence(
    identity: &ForgeQueryEvidenceIdentity,
) -> QueryProjectionIdentity<String, QueryReceiptIdentityKind> {
    project_query_evidence_with_authority(
        identity,
        query_receipt_admission_authority(),
        query_receipt_admission_authority(),
    )
}

pub(crate) fn project_query_intent_evidence(
    identity: &ForgeQueryEvidenceIdentity,
) -> QueryProjectionIdentity<String, QueryIntentIdentityKind> {
    project_query_evidence_with_authority(
        identity,
        query_intent_authority(),
        query_intent_authority(),
    )
}

pub(crate) fn project_query_workflow_evidence(
    identity: &ForgeQueryEvidenceIdentity,
) -> QueryProjectionIdentity<String, QueryWorkflowIdentityKind> {
    project_query_evidence_with_authority(
        identity,
        query_workflow_authority(),
        query_workflow_authority(),
    )
}

pub(crate) fn project_query_causal_inspection_evidence(
    identity: &ForgeQueryEvidenceIdentity,
) -> QueryProjectionIdentity<String, QueryCausalInspectionIdentityKind> {
    project_query_evidence_with_authority(
        identity,
        query_causal_inspection_authority(),
        query_causal_inspection_authority(),
    )
}

fn project_query_evidence_with_authority<Authority, Kind>(
    identity: &ForgeQueryEvidenceIdentity,
    admit_authority: AuthorityWitness<Authority>,
    project_authority: AuthorityWitness<Authority>,
) -> QueryProjectionIdentity<String, Kind>
where
    Authority: AuthorityMarker,
    Kind: forge_foundational::facade::FoundationalIdentityKind,
{
    let authority_identity =
        admit_foundational_authority_identity(identity.clone(), admit_authority);
    project_foundational_identity(
        &authority_identity,
        identity.reporting_projection().to_string(),
        project_authority,
    )
}
