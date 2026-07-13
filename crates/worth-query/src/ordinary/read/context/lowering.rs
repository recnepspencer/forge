use crate::authorized_projection::{PolicyInfluenceSet, PolicyMaskSnapshot};
use crate::ordinary::read::{WorthQueryDeclaredReadIntent, WorthQueryReadPlanningAuthority};
use crate::policy_basis::{
    admit_policy_tenant_context_for_query_identity, AdmittedPolicyTenantContext,
    PolicyAdmissionDisposition, PolicyExecutionModeRequest,
};
use crate::policy_narrowing::{
    narrow_policy_query, NarrowedPolicyQueryArtifact, PolicyNarrowingFailureClass,
};
use crate::relationship_proof::{
    admit_relationship_proofs_for_query_identity, RelationshipProofAdmission,
    RelationshipProofDescriptorSet,
};
use crate::runtime::{
    admit_graph_read_access_authority, WorthQueryGraphReadAccessAuthorityContext,
    WorthQueryGraphReadAccessAuthorityRequest,
};

use super::{
    WorthQueryCurrentPolicyTenantReadContext, WorthQueryReadContextAdmissionCounters,
    WorthQueryReadContextDeclaration, WorthQueryReadContextDenial, WorthQueryReadContextKind,
    WorthQueryReadContextReceipt, WorthQueryReadRelationshipProofs,
};

pub(crate) struct WorthQueryAdmittedReadContext {
    authority: WorthQueryGraphReadAccessAuthorityContext,
    planning_authority: WorthQueryReadPlanningAuthority,
    receipt: WorthQueryReadContextReceipt,
}

impl WorthQueryAdmittedReadContext {
    pub(crate) fn into_parts(
        self,
    ) -> (
        WorthQueryGraphReadAccessAuthorityContext,
        WorthQueryReadPlanningAuthority,
        WorthQueryReadContextReceipt,
    ) {
        (self.authority, self.planning_authority, self.receipt)
    }
}

struct AdmittedContextParts {
    authority_request: WorthQueryGraphReadAccessAuthorityRequest,
    policy_tenant_digest: Option<String>,
    policy_narrowing_digest: Option<String>,
    relationship_proof_digest: Option<String>,
    planning_authority: WorthQueryReadPlanningAuthority,
}

pub(crate) fn admit_read_context_declaration(
    intent: &WorthQueryDeclaredReadIntent,
    declaration: WorthQueryReadContextDeclaration,
) -> Result<WorthQueryAdmittedReadContext, WorthQueryReadContextDenial> {
    let context_kind = declaration.kind();
    let canonical_query_digest = intent.canonical_query_digest();
    let mut counters = WorthQueryReadContextAdmissionCounters::begin();

    if intent.requires_relationship_proof() && !context_kind.carries_relationship_proofs() {
        return Err(WorthQueryReadContextDenial::missing_relationship_proof(
            counters,
        ));
    }

    let parts = match declaration {
        WorthQueryReadContextDeclaration::Current(_) => canonical_context_parts(),
        WorthQueryReadContextDeclaration::CurrentPolicyTenant(context) => {
            admit_policy_context(intent, context, None, &mut counters)?
        }
        WorthQueryReadContextDeclaration::CurrentPolicyTenantRelationship(context) => {
            admit_policy_context(
                intent,
                context.policy_tenant,
                Some(context.relationship_proofs),
                &mut counters,
            )?
        }
    };

    counters.record_graph_authority_admission_attempt();
    let authority = admit_graph_read_access_authority(parts.authority_request)
        .map_err(|denial| WorthQueryReadContextDenial::graph_authority(denial, counters.clone()))?;
    counters.record_graph_authority_admitted();
    let receipt = WorthQueryReadContextReceipt::new(
        context_kind,
        canonical_query_digest.as_str().to_string(),
        parts.policy_tenant_digest,
        parts.policy_narrowing_digest,
        parts.relationship_proof_digest,
        authority.receipt().digest().to_string(),
        counters,
    );
    Ok(WorthQueryAdmittedReadContext {
        authority,
        planning_authority: parts.planning_authority,
        receipt,
    })
}

impl WorthQueryReadContextKind {
    fn carries_relationship_proofs(self) -> bool {
        matches!(self, Self::CurrentPolicyTenantRelationship)
    }
}

fn canonical_context_parts() -> AdmittedContextParts {
    AdmittedContextParts {
        authority_request: WorthQueryGraphReadAccessAuthorityRequest::current_head(),
        policy_tenant_digest: None,
        policy_narrowing_digest: None,
        relationship_proof_digest: None,
        planning_authority: WorthQueryReadPlanningAuthority::canonical(None),
    }
}

fn admit_policy_context(
    intent: &WorthQueryDeclaredReadIntent,
    context: WorthQueryCurrentPolicyTenantReadContext,
    relationship_proofs: Option<WorthQueryReadRelationshipProofs>,
    counters: &mut WorthQueryReadContextAdmissionCounters,
) -> Result<AdmittedContextParts, WorthQueryReadContextDenial> {
    let admitted =
        admit_policy_tenant_authority(intent.canonical_query_digest(), context, counters)?;
    if admitted.bundle().admission_disposition() == PolicyAdmissionDisposition::AdmittedUnchanged {
        return match relationship_proofs {
            Some(proofs) => {
                let relationship_proof = admit_relationship_proof(
                    intent.canonical_query_digest(),
                    &admitted,
                    proofs,
                    counters,
                )?;
                context_parts_with_relationship(admitted, relationship_proof)
            }
            None => {
                let policy_tenant_digest = admitted.bundle().digest().as_str().to_string();
                Ok(AdmittedContextParts {
                    authority_request: WorthQueryGraphReadAccessAuthorityRequest::current_head()
                        .with_policy_tenant(admitted),
                    policy_tenant_digest: Some(policy_tenant_digest),
                    policy_narrowing_digest: None,
                    relationship_proof_digest: None,
                    planning_authority: WorthQueryReadPlanningAuthority::canonical(None),
                })
            }
        };
    }
    let descriptors = relationship_proofs
        .map(|proofs| {
            proofs.lower(
                admitted.bundle().policy_digest(),
                admitted.bundle().tenant_schema_basis_digest(),
            )
        })
        .unwrap_or_else(RelationshipProofDescriptorSet::none);
    let narrowed = narrow_policy_context(intent, admitted.clone(), descriptors, counters)?;
    let relationship_proof = narrowed.relationship_proof().clone();
    let policy_tenant_digest = admitted.bundle().digest().as_str().to_string();
    let policy_narrowing_digest = narrowed.digest().to_string();
    let relationship_proof_digest = relationship_proof.identity().as_str().to_string();
    Ok(AdmittedContextParts {
        authority_request: WorthQueryGraphReadAccessAuthorityRequest::current_head()
            .with_policy_tenant(admitted)
            .with_relationship_proofs(relationship_proof),
        policy_tenant_digest: Some(policy_tenant_digest),
        policy_narrowing_digest: Some(policy_narrowing_digest),
        relationship_proof_digest: Some(relationship_proof_digest),
        planning_authority: WorthQueryReadPlanningAuthority::policy_narrowed(narrowed),
    })
}

fn narrow_policy_context(
    intent: &WorthQueryDeclaredReadIntent,
    admitted: AdmittedPolicyTenantContext,
    descriptors: RelationshipProofDescriptorSet,
    counters: &mut WorthQueryReadContextAdmissionCounters,
) -> Result<NarrowedPolicyQueryArtifact, WorthQueryReadContextDenial> {
    counters.record_policy_narrowing_attempt();
    let mask = PolicyMaskSnapshot::from_admitted_policy(&admitted)
        .expect("policy admission guarantees projection authority for narrowing dispositions");
    let influence = PolicyInfluenceSet::none();
    match narrow_policy_query(intent.canonical(), admitted, mask, influence, descriptors) {
        Ok(narrowed) => {
            counters.record_policy_narrowing_admitted();
            counters.record_relationship_proof_admission_attempt();
            counters.record_relationship_proof_admitted();
            Ok(narrowed)
        }
        Err(error) => {
            if matches!(
                error.failure_class(),
                PolicyNarrowingFailureClass::RelationshipProofDenied(_)
            ) {
                counters.record_relationship_proof_admission_attempt();
            }
            Err(WorthQueryReadContextDenial::policy_narrowing(
                error,
                counters.clone(),
            ))
        }
    }
}

fn admit_policy_tenant_authority(
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
    Ok(admitted)
}

fn context_parts_with_relationship(
    admitted: AdmittedPolicyTenantContext,
    relationship_proof: RelationshipProofAdmission,
) -> Result<AdmittedContextParts, WorthQueryReadContextDenial> {
    let policy_tenant_digest = admitted.bundle().digest().as_str().to_string();
    let relationship_proof_digest = relationship_proof.identity().as_str().to_string();
    Ok(AdmittedContextParts {
        authority_request: WorthQueryGraphReadAccessAuthorityRequest::current_head()
            .with_policy_tenant(admitted)
            .with_relationship_proofs(relationship_proof.clone()),
        policy_tenant_digest: Some(policy_tenant_digest),
        policy_narrowing_digest: None,
        relationship_proof_digest: Some(relationship_proof_digest),
        planning_authority: WorthQueryReadPlanningAuthority::canonical(Some(relationship_proof)),
    })
}

fn admit_relationship_proof(
    canonical_query_digest: &crate::identity::CanonicalQueryDigest,
    admitted_policy_tenant: &AdmittedPolicyTenantContext,
    relationship_proofs: WorthQueryReadRelationshipProofs,
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
