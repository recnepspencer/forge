use crate::layout_declarations;
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_security::{
    admitted_store_managed_root_security_scope_for_layout_partition_test,
    admitted_tenant_artifact_security_scope_for_layout_partition_test,
    admitted_tenant_page_export_prepared_scope_for_layout_partition_test,
    admitted_tenant_page_security_scope_for_layout_partition_test,
    admitted_tenant_page_without_authenticity_for_layout_partition_test,
};

use super::{
    ArtifactFamilyDenial, ArtifactKeyScopePartition, ArtifactTenantScopePartition, AuthorityRole,
    DerivedAccuracyClass,
};

#[test]
fn phase_three_role_accuracy_and_scope_are_typed() {
    let facade = layout_declarations();
    let admitted_scope = admitted_store_managed_root_security_scope_for_layout_partition_test();
    let declaration = facade
        .declaration(DurableArtifactFamilyId::PhysicalRootManifest)
        .unwrap();
    let classification = facade.classify_family(declaration);
    let role = facade.declare_authority_role(classification);
    let accuracy = facade.declare_derived_accuracy_class(role);
    let scoped = facade
        .require_scope_partition(accuracy, admitted_scope.witnesses())
        .unwrap();

    assert_eq!(role.role(), AuthorityRole::PhysicalDiscoveryAuthority);
    assert_eq!(accuracy.accuracy(), DerivedAccuracyClass::Exact);
    assert_eq!(
        scoped.tenant_partition(),
        ArtifactTenantScopePartition::Single(
            admitted_scope.witnesses().tenant_scope().tenant_scope()
        )
    );
    assert_eq!(
        scoped.key_partition(),
        ArtifactKeyScopePartition::Single(admitted_scope.witnesses().key_scope().key_scope())
    );
    assert_eq!(
        scoped.security_boundary().authenticity_requirement(),
        admitted_scope
            .witnesses()
            .authenticity_scope()
            .requirement()
    );
    assert_eq!(
        scoped.security_boundary().custody_posture(),
        admitted_scope.witnesses().custody_scope().custody_posture()
    );
}

#[test]
fn phase_three_accuracy_claims_deny_inexact_families() {
    let facade = layout_declarations();
    let exact_family = facade
        .declaration(DurableArtifactFamilyId::DedupeIndex)
        .unwrap();
    let heuristic_family = facade
        .declaration(DurableArtifactFamilyId::MaintenanceSnapshot)
        .unwrap();

    let exact_accuracy = facade.declare_derived_accuracy_class(
        facade.declare_authority_role(facade.classify_family(exact_family)),
    );
    let heuristic_accuracy = facade.declare_derived_accuracy_class(
        facade.declare_authority_role(facade.classify_family(heuristic_family)),
    );

    assert_eq!(exact_accuracy.accuracy(), DerivedAccuracyClass::Exact);
    assert!(facade.require_exact_accuracy_claim(exact_accuracy).is_ok());
    assert_eq!(
        heuristic_accuracy.accuracy(),
        DerivedAccuracyClass::Heuristic
    );
    assert_eq!(
        facade.require_exact_accuracy_claim(heuristic_accuracy),
        Err(ArtifactFamilyDenial::InexactFamilyCannotSatisfyExactClaim)
    );
}

#[test]
fn phase_three_scope_partition_denies_wrong_admitted_scope() {
    let facade = layout_declarations();
    let root_scope = admitted_store_managed_root_security_scope_for_layout_partition_test();
    let page_scope = admitted_tenant_page_security_scope_for_layout_partition_test();
    let artifact_scope = admitted_tenant_artifact_security_scope_for_layout_partition_test();
    let unauthenticated_page_scope =
        admitted_tenant_page_without_authenticity_for_layout_partition_test();
    let export_prepared_page_scope =
        admitted_tenant_page_export_prepared_scope_for_layout_partition_test();
    let page_family = facade
        .declaration(DurableArtifactFamilyId::PhysicalPage)
        .unwrap();
    let page_accuracy = facade.declare_derived_accuracy_class(
        facade.declare_authority_role(facade.classify_family(page_family)),
    );

    assert!(facade
        .require_scope_partition(page_accuracy, page_scope.witnesses())
        .is_ok());
    assert_eq!(
        facade.require_scope_partition(page_accuracy, root_scope.witnesses()),
        Err(ArtifactFamilyDenial::CrossTenantScopePartitionDenied)
    );
    assert_eq!(
        facade.require_scope_partition(page_accuracy, artifact_scope.witnesses()),
        Err(ArtifactFamilyDenial::CrossKeyScopePartitionDenied)
    );
    assert_eq!(
        facade.require_scope_partition(page_accuracy, unauthenticated_page_scope.witnesses()),
        Err(ArtifactFamilyDenial::AuthenticityBoundaryDenied)
    );
    assert_eq!(
        facade.require_scope_partition(page_accuracy, export_prepared_page_scope.witnesses()),
        Err(ArtifactFamilyDenial::CustodyBoundaryDenied)
    );
}
