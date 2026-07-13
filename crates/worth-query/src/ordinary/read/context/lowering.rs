use crate::ordinary::read::WorthQueryDeclaredReadIntent;
use crate::policy_basis::{
    admit_policy_tenant_context_for_query_identity, AdmittedPolicyTenantContext,
    PolicyExecutionModeRequest,
};
use crate::relationship_proof::{
    admit_relationship_proofs_for_query_identity, RelationshipProofAdmission,
};
use crate::runtime::{
    admit_graph_read_access_authority, WorthQueryGraphReadAccessAuthorityContext,
    WorthQueryGraphReadAccessAuthorityRequest,
};

use super::{
    WorthQueryCurrentPolicyTenantReadContext, WorthQueryCurrentRelationshipReadContext,
    WorthQueryReadContextAdmissionCounters, WorthQueryReadContextDeclaration,
    WorthQueryReadContextDenial, WorthQueryReadContextReceipt,
};

pub(crate) struct WorthQueryAdmittedReadContext {
    authority: WorthQueryGraphReadAccessAuthorityContext,
    relationship_proof: Option<RelationshipProofAdmission>,
    receipt: WorthQueryReadContextReceipt,
}

impl WorthQueryAdmittedReadContext {
    pub(crate) fn into_parts(
        self,
    ) -> (
        WorthQueryGraphReadAccessAuthorityContext,
        Option<RelationshipProofAdmission>,
        WorthQueryReadContextReceipt,
    ) {
        (self.authority, self.relationship_proof, self.receipt)
    }
}

pub(crate) fn admit_read_context_declaration(
    intent: &WorthQueryDeclaredReadIntent,
    declaration: WorthQueryReadContextDeclaration,
) -> Result<WorthQueryAdmittedReadContext, WorthQueryReadContextDenial> {
    let context_kind = declaration.kind();
    let canonical_query_digest = intent.canonical_query_digest();
    let mut counters = WorthQueryReadContextAdmissionCounters::begin();

    if intent.requires_relationship_proof()
        && context_kind != super::WorthQueryReadContextKind::CurrentPolicyTenantRelationship
    {
        return Err(WorthQueryReadContextDenial::missing_relationship_proof(
            counters,
        ));
    }

    let (authority_request, policy_tenant_digest, relationship_proof, relationship_proof_digest) =
        match declaration {
            WorthQueryReadContextDeclaration::Current(_) => (
                WorthQueryGraphReadAccessAuthorityRequest::current_head(),
                None,
                None,
                None,
            ),
            WorthQueryReadContextDeclaration::CurrentPolicyTenant(context) => {
                let admitted_policy_tenant =
                    admit_policy_tenant_context(canonical_query_digest, context, &mut counters)?;
                let policy_tenant_digest = admitted_policy_tenant
                    .bundle()
                    .digest()
                    .as_str()
                    .to_string();
                (
                    WorthQueryGraphReadAccessAuthorityRequest::current_head()
                        .with_policy_tenant(admitted_policy_tenant),
                    Some(policy_tenant_digest),
                    None,
                    None,
                )
            }
            WorthQueryReadContextDeclaration::CurrentPolicyTenantRelationship(context) => {
                admit_policy_tenant_relationship_context(
                    canonical_query_digest,
                    context,
                    &mut counters,
                )?
            }
        };

    counters.record_graph_authority_admission_attempt();
    let authority = admit_graph_read_access_authority(authority_request)
        .map_err(|denial| WorthQueryReadContextDenial::graph_authority(denial, counters.clone()))?;
    counters.record_graph_authority_admitted();
    let receipt = WorthQueryReadContextReceipt::new(
        context_kind,
        canonical_query_digest.as_str().to_string(),
        policy_tenant_digest,
        relationship_proof_digest,
        authority.receipt().digest().to_string(),
        counters,
    );
    Ok(WorthQueryAdmittedReadContext {
        authority,
        relationship_proof,
        receipt,
    })
}

fn admit_policy_tenant_context(
    canonical_query_digest: &crate::identity::CanonicalQueryDigest,
    context: WorthQueryCurrentPolicyTenantReadContext,
    counters: &mut WorthQueryReadContextAdmissionCounters,
) -> Result<AdmittedPolicyTenantContext, WorthQueryReadContextDenial> {
    counters.record_policy_tenant_admission_attempt();
    let admitted = admit_policy_tenant_context_for_query_identity(
        canonical_query_digest,
        context.policy,
        context.tenant,
        context.branch,
        context.schema,
        PolicyExecutionModeRequest::CurrentRead,
    )
    .map_err(|denial| WorthQueryReadContextDenial::policy_tenant(denial, counters.clone()))?;
    counters.record_policy_tenant_admitted();
    if admitted.bundle().admission_disposition()
        != crate::policy_basis::PolicyAdmissionDisposition::AdmittedUnchanged
    {
        return Err(
            WorthQueryReadContextDenial::policy_narrowing_context_required(
                admitted.bundle().admission_disposition(),
                counters.clone(),
            ),
        );
    }
    Ok(admitted)
}

fn admit_policy_tenant_relationship_context(
    canonical_query_digest: &crate::identity::CanonicalQueryDigest,
    context: WorthQueryCurrentRelationshipReadContext,
    counters: &mut WorthQueryReadContextAdmissionCounters,
) -> Result<
    (
        WorthQueryGraphReadAccessAuthorityRequest,
        Option<String>,
        Option<RelationshipProofAdmission>,
        Option<String>,
    ),
    WorthQueryReadContextDenial,
> {
    let admitted_policy_tenant =
        admit_policy_tenant_context(canonical_query_digest, context.policy_tenant, counters)?;
    let policy_tenant_digest = admitted_policy_tenant
        .bundle()
        .digest()
        .as_str()
        .to_string();
    let relationship_proof = admit_relationship_proof(
        canonical_query_digest,
        &admitted_policy_tenant,
        context.relationship_proofs,
        counters,
    )?;
    let relationship_proof_digest = relationship_proof.identity().as_str().to_string();
    let authority_request = WorthQueryGraphReadAccessAuthorityRequest::current_head()
        .with_policy_tenant(admitted_policy_tenant)
        .with_relationship_proofs(relationship_proof.clone());
    Ok((
        authority_request,
        Some(policy_tenant_digest),
        Some(relationship_proof),
        Some(relationship_proof_digest),
    ))
}

fn admit_relationship_proof(
    canonical_query_digest: &crate::identity::CanonicalQueryDigest,
    admitted_policy_tenant: &AdmittedPolicyTenantContext,
    relationship_proofs: super::WorthQueryReadRelationshipProofs,
    counters: &mut WorthQueryReadContextAdmissionCounters,
) -> Result<RelationshipProofAdmission, WorthQueryReadContextDenial> {
    counters.record_relationship_proof_admission_attempt();
    let relationship_proofs = relationship_proofs.lower(
        admitted_policy_tenant.bundle().policy_digest(),
        admitted_policy_tenant.bundle().tenant_schema_basis_digest(),
    );
    let admitted = admit_relationship_proofs_for_query_identity(
        canonical_query_digest,
        admitted_policy_tenant,
        &relationship_proofs,
    )
    .map(|(admission, _)| admission)
    .map_err(|denial| WorthQueryReadContextDenial::relationship_proof(denial, counters.clone()))?;
    counters.record_relationship_proof_admitted();
    Ok(admitted)
}
