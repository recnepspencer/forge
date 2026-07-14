use worth_store_contracts::DurableArtifactFamilyId;
use worth_store_layout_indexes::declarations::layout_declarations;
use worth_store_security::{
    StoreAdmittedSecurityScope, StoreAuthenticityRequirement, StoreCustodyPosture, StoreKeyScope,
    StoreTenantScope,
};
use worth_store_test_support::{admit_security_scope_fixture, SecurityScopeFixtureAuthority};

pub(super) fn admit_family(
    family: DurableArtifactFamilyId,
    security: &StoreAdmittedSecurityScope,
) -> worth_store_layout_indexes::AdmittedPhysicalArtifactFamily {
    let declaration = layout_declarations().declaration(family).unwrap();
    layout_declarations()
        .admit_physical_artifact_family(declaration, security.witnesses())
        .unwrap()
}

pub(super) fn admit_key_domain(
    family: worth_store_layout_indexes::AdmittedPhysicalArtifactFamily,
    security: &StoreAdmittedSecurityScope,
) -> worth_store_layout_indexes::AdmittedPhysicalKeyDomain {
    layout_declarations()
        .admit_physical_key_domain(family, security.witnesses())
        .unwrap()
}

pub(super) fn security_scope(
    authority: SecurityScopeFixtureAuthority,
    key: StoreKeyScope,
    tenant: StoreTenantScope,
    authenticity: StoreAuthenticityRequirement,
    custody: StoreCustodyPosture,
) -> StoreAdmittedSecurityScope {
    admit_security_scope_fixture(authority, key, tenant, authenticity, custody)
}
