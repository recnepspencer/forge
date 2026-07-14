use worth_query::facade::runtime::{TenantBasisEpoch, TenantBindingSnapshot, TenantResolutionClass};

fn main() {
    let _ = TenantBindingSnapshot {
        tenant_identity: "tenant".to_string(),
        truth_branch_identity: Some("branch".to_string()),
        schema_basis_identity: Some("schema".to_string()),
        resolution_class: TenantResolutionClass::DirectBinding,
        epoch: TenantBasisEpoch::Synthetic(1),
        ambiguous: false,
        hidden_filter: false,
        digest: "digest".to_string(),
    };
}
