use forge_query::facade::{
    admit_policy_tenant_context, BranchAccessGrant, PolicyEpoch, PolicyExecutionModeRequest,
    PolicyRuleSnapshot, SchemaVariantSnapshot, TenantBasisEpoch, TenantBindingSnapshot,
};

fn main() {
    let policy =
        PolicyRuleSnapshot::synthetic_authority("policy", "rules", PolicyEpoch::Synthetic(1));
    let tenant = TenantBindingSnapshot::synthetic_direct(
        "tenant",
        "branch",
        "schema",
        TenantBasisEpoch::Synthetic(1),
    );
    let branch = BranchAccessGrant::synthetic_granted("branch", &policy);
    let schema = SchemaVariantSnapshot::synthetic_authority("tenant", "schema", "compatible");

    let _ = admit_policy_tenant_context(
        "raw-query-digest",
        policy,
        tenant,
        branch,
        schema,
        PolicyExecutionModeRequest::CurrentRead,
    );
}
