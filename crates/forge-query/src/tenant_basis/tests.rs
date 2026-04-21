use super::{
    SchemaVariantSnapshot, TenantBasisEpoch, TenantBindingSnapshot, TenantResolutionClass,
};

#[test]
fn synthetic_tenant_authority_artifacts_are_stable_and_specific() {
    let direct = TenantBindingSnapshot::synthetic_direct(
        "tenant-a",
        "branch-a",
        "schema-a",
        TenantBasisEpoch::Synthetic(1),
    );
    let cached = TenantBindingSnapshot::synthetic_cached(
        "tenant-a",
        "branch-a",
        "schema-a",
        TenantBasisEpoch::Synthetic(1),
    );
    let schema = SchemaVariantSnapshot::synthetic_authority("tenant-a", "schema-a", "compatible");

    assert_eq!(direct.tenant_identity(), "tenant-a");
    assert_eq!(direct.truth_branch_identity(), Some("branch-a"));
    assert_eq!(direct.schema_basis_identity(), Some("schema-a"));
    assert_eq!(
        direct.resolution_class(),
        TenantResolutionClass::DirectBinding
    );
    assert_eq!(
        cached.resolution_class(),
        TenantResolutionClass::CachedBinding
    );
    assert_ne!(direct.digest(), cached.digest());
    assert_eq!(schema.tenant_identity(), "tenant-a");
    assert!(!schema.global_fallback());
    assert!(!schema.digest().is_empty());
}
