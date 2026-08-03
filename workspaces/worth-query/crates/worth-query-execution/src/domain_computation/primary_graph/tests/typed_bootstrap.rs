use worth_query_declaration::facade::authentication::WorthQueryPrincipalMappingStatus;
use worth_query_installation::facade::{
    WorthQueryInstallationAdmissionProfile, WorthQueryInstallationGeneration,
    WorthQueryPortableDomainIdentity, WorthQueryPortableDomainPackage,
};

use crate::domain_computation::execution_runtime::WorthQueryExecutionRuntimeInstaller;
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationPrincipalKey, WorthQueryPrimaryGraphInstallationDenialKind,
};

use super::fixture::{
    external_identity, installed_world_with_policy_fact, IdentityBinding, IdentityExecutionSchema,
};

#[test]
fn typed_policy_facts_publish_atomically_with_principal_identity() {
    let world = installed_world_with_policy_fact(
        &[("alice", WorthQueryPrincipalMappingStatus::Enabled)],
        true,
    );

    assert_eq!(world.publication.principal_binding_count(), 1);
    assert_eq!(world.publication.policy_entity_count(), 1);
    assert_eq!(world.publication.policy_relation_count(), 1);
}

#[test]
fn duplicate_typed_principal_identity_denies_without_poisoning_bootstrap() {
    let declaration = IdentityExecutionSchema::declaration().unwrap();
    let package = WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        "identity_execution_test",
        1,
        0,
    ))
    .application_schema(declaration.clone())
    .validate()
    .unwrap();
    let admitted = WorthQueryInstallationAdmissionProfile::new("support", "configuration")
        .admit(package)
        .unwrap();
    let installation = WorthQueryExecutionRuntimeInstaller::new()
        .install(WorthQueryInstallationGeneration::initial(), [admitted])
        .unwrap();
    let (mut runtime, authority) = installation.into_parts();
    let schema = runtime
        .installed_packages()
        .bind_application_schema(declaration)
        .unwrap();
    let binding = schema
        .principal_binding(IdentityBinding::reference())
        .unwrap();
    let mut bootstrap = authority.prepare_primary_graph(&runtime, &schema).unwrap();

    bootstrap
        .bind_principal(
            &binding,
            WorthQueryApplicationPrincipalKey::new("principal-alice").unwrap(),
            7_u64,
            external_identity("alice"),
            WorthQueryPrincipalMappingStatus::Enabled,
        )
        .unwrap();
    let denial = bootstrap
        .bind_principal(
            &binding,
            WorthQueryApplicationPrincipalKey::new("principal-bob").unwrap(),
            7_u64,
            external_identity("bob"),
            WorthQueryPrincipalMappingStatus::Enabled,
        )
        .unwrap_err();
    assert_eq!(
        denial.kind(),
        WorthQueryPrimaryGraphInstallationDenialKind::DuplicatePrincipalIdentity
    );

    bootstrap
        .bind_principal(
            &binding,
            WorthQueryApplicationPrincipalKey::new("principal-bob").unwrap(),
            8_u64,
            external_identity("bob"),
            WorthQueryPrincipalMappingStatus::Enabled,
        )
        .expect("a denied duplicate identity must not reserve unrelated seed keys");
    let publication = bootstrap.publish(&mut runtime, &authority).unwrap();
    assert_eq!(publication.principal_binding_count(), 2);
}
