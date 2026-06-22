use forge_query::facade::foundation::{
    AspectFieldSelector, AuthoredResultShapeField, CanonicalQueryBundle, DetailQueryBuilder,
    DetailResultShapeBuilder, GuidedAuthoringPath, RootEntityKey,
};
use forge_query::facade::policy::{
    admit_policy_tenant_context, AdmittedPolicyTenantContext, BranchAccessGrant, PolicyCostPosture,
    PolicyEpoch, PolicyExecutionModeRequest, PolicyRuleSnapshot, PolicyWorkBudget,
};
use forge_query::facade::runtime::{
    ForgeQueryGraphReadPolicyTenantAuthorityRequest, ForgeQuerySessionLabel, SchemaVariantSnapshot,
    TenantBasisEpoch, TenantBindingSnapshot,
};

pub fn canonical_query() -> CanonicalQueryBundle {
    let query = DetailQueryBuilder::new(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .build()
        .unwrap();
    let result_shape = DetailResultShapeBuilder::new()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .build()
        .unwrap();
    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}

pub fn admitted_policy_tenant(
    canonical: &CanonicalQueryBundle,
    tenant_label: &str,
) -> AdmittedPolicyTenantContext {
    admitted_policy_tenant_for_mode(
        canonical,
        tenant_label,
        PolicyExecutionModeRequest::CurrentRead,
    )
}

pub fn admitted_policy_tenant_for_mode(
    canonical: &CanonicalQueryBundle,
    tenant_label: &str,
    execution_mode: PolicyExecutionModeRequest,
) -> AdmittedPolicyTenantContext {
    let policy = phase_fourteen_policy(true);
    let tenant = phase_fourteen_tenant(tenant_label);
    let branch = BranchAccessGrant::synthetic_granted("branch-a", &policy);
    let schema = phase_fourteen_schema(tenant_label);
    admit_policy_tenant_context(
        canonical.query(),
        policy,
        tenant,
        branch,
        schema,
        execution_mode,
    )
    .expect("policy/tenant context should admit")
}

pub fn policy_tenant_authority_request(
    canonical: &CanonicalQueryBundle,
    tenant_label: &str,
    admits_query_family: bool,
) -> ForgeQueryGraphReadPolicyTenantAuthorityRequest {
    let policy = phase_fourteen_policy(admits_query_family);
    let tenant = phase_fourteen_tenant(tenant_label);
    let branch = BranchAccessGrant::synthetic_granted("branch-a", &policy);
    let schema = phase_fourteen_schema(tenant_label);
    ForgeQueryGraphReadPolicyTenantAuthorityRequest::current_read(
        canonical.query(),
        policy,
        tenant,
        branch,
        schema,
    )
}

pub fn session_label(name: &str) -> ForgeQuerySessionLabel {
    ForgeQuerySessionLabel::scoped_strs("graph-read-access-phase-fourteen", [name])
        .expect("session label should admit")
}

fn phase_fourteen_policy(admits_query_family: bool) -> PolicyRuleSnapshot {
    PolicyRuleSnapshot::synthetic_authority_with_budget(
        "phase-fourteen-runtime-policy",
        "rules-v1",
        PolicyEpoch::Synthetic(7),
        admits_query_family,
        true,
        true,
        PolicyCostPosture::BoundedRelationshipProof,
        Some(PolicyWorkBudget::bounded(1, 1, 1)),
    )
}

fn phase_fourteen_tenant(tenant_label: &str) -> TenantBindingSnapshot {
    TenantBindingSnapshot::synthetic_direct(
        tenant_label,
        "branch-a",
        "schema-a",
        TenantBasisEpoch::Synthetic(3),
    )
}

fn phase_fourteen_schema(tenant_label: &str) -> SchemaVariantSnapshot {
    SchemaVariantSnapshot::synthetic_authority(tenant_label, "schema-a", "compatible")
}
