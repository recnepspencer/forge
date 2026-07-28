mod authentication;
mod runtime_world;
mod schema;

use std::time::Duration;

use worth_query_declaration::facade::authentication::WorthQueryPrincipalMappingStatus;
use worth_query_execution::facade::primary_graph::{
    WorthQueryApplicationPrincipalKey, WorthQueryPrincipalResolutionMode,
};

use self::authentication::{authenticate, external_identity, live_scope};
use self::runtime_world::{
    host_relational_runtime, mixed_basis_relational_runtime, CommittingWriteAuthority,
};
use self::schema::{
    primary_graph_domain_package, IdentityBinding, PrimaryGraphCompositionSchema, Principal,
};
use super::support::{
    complete_backend_from_parts_builder, custom_backend_without_primary_graph_transfer_builder,
    insert_command, test_string_aspect_value,
};

#[test]
fn ordinary_write_and_principal_resolution_share_one_configured_relational_graph() {
    let subject = "dynamic-user-7f643b";
    let mut runtime = complete_backend_from_parts_builder()
        .domain_package(primary_graph_domain_package())
        .expect("primary graph domain package should admit")
        .relational_runtime(host_relational_runtime())
        .write_authority(CommittingWriteAuthority)
        .application_primary_graph::<PrimaryGraphCompositionSchema, _>(move |graph| {
            graph.bind_principal(
                IdentityBinding::reference(),
                WorthQueryApplicationPrincipalKey::<PrimaryGraphCompositionSchema, Principal>::new(
                    "principal-dynamic-user-7f643b",
                )
                .expect("typed principal key should admit"),
                7_u64,
                external_identity(subject),
                WorthQueryPrincipalMappingStatus::Enabled,
            )
        })
        .expect("one typed primary graph should configure")
        .build_backend_from_parts()
        .build()
        .expect("the configured runtime should publish");

    let publication = runtime
        .primary_graph_publication()
        .expect("primary graph publication evidence should be retained");
    assert_eq!(publication.principal_binding_count(), 1);
    assert_eq!(publication.identity_index_count(), 1);

    runtime
        .write(insert_command(
            "Task",
            [(
                "identity.id",
                test_string_aspect_value("ordinary-shared-root-write"),
            )],
        ))
        .expect("ordinary Query write should commit through the shared graph");

    let declaration =
        PrimaryGraphCompositionSchema::declaration().expect("typed schema should declare");
    let installed = runtime
        .installed_application_schema(declaration)
        .expect("runtime should bind the exact installed schema");
    let binding = installed
        .principal_binding(IdentityBinding::reference())
        .expect("typed principal binding should be installed");
    let scope = live_scope();
    let admitted = authenticate(&installed, subject, Duration::from_secs(60), &scope);
    let principal = runtime
        .resolve_authenticated_principal(
            &binding,
            admitted,
            &scope,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .expect("post-write identity index should resolve the enabled principal");

    assert_eq!(principal.external_identity().subject(), subject);
    assert_eq!(*principal.principal_identity(), 7);
    assert_eq!(principal.examined_candidate_count(), 1);
    runtime
        .validate_authenticated_principal(&principal, &scope)
        .expect("resolved principal should remain fresh");
}

#[test]
fn mixed_host_schema_basis_is_rejected_before_primary_graph_publication() {
    let result = complete_backend_from_parts_builder()
        .domain_package(primary_graph_domain_package())
        .expect("primary graph domain package should admit")
        .relational_runtime(mixed_basis_relational_runtime())
        .application_primary_graph::<PrimaryGraphCompositionSchema, _>(|graph| {
            graph.bind_principal(
                IdentityBinding::reference(),
                WorthQueryApplicationPrincipalKey::<PrimaryGraphCompositionSchema, Principal>::new(
                    "principal-hostile-basis",
                )
                .expect("typed principal key should admit"),
                1_u64,
                external_identity("hostile-basis-subject"),
                WorthQueryPrincipalMappingStatus::Enabled,
            )
        })
        .expect("typed primary graph contribution should configure")
        .build_backend_from_parts()
        .build();
    let error = match result {
        Ok(_) => panic!("mixed Relational schema authority must not publish a primary graph"),
        Err(error) => error,
    };

    match error {
        crate::runtime::WorthQueryRuntimeError::InvariantRegistration { stage, message } => {
            assert_eq!(stage, "primary_graph_bootstrap_preparation");
            assert!(message.contains("RelationalSchemaRejected"));
            assert!(message.contains("mixed schema basis"));
        }
        other => panic!("unexpected mixed-basis denial: {other:?}"),
    }
}

#[test]
fn custom_backend_must_explicitly_implement_primary_graph_transfer() {
    let result = custom_backend_without_primary_graph_transfer_builder()
        .domain_package(primary_graph_domain_package())
        .expect("primary graph domain package should admit")
        .application_primary_graph::<PrimaryGraphCompositionSchema, _>(|graph| {
            graph.bind_principal(
                IdentityBinding::reference(),
                WorthQueryApplicationPrincipalKey::<PrimaryGraphCompositionSchema, Principal>::new(
                    "principal-custom-backend",
                )
                .expect("typed principal key should admit"),
                1_u64,
                external_identity("custom-backend-subject"),
                WorthQueryPrincipalMappingStatus::Enabled,
            )
        })
        .expect("typed primary graph contribution should configure")
        .build();
    let error = match result {
        Ok(_) => panic!("an implicit custom-backend primary graph must not publish"),
        Err(error) => error,
    };

    match error {
        crate::runtime::WorthQueryRuntimeError::Workspace(denial) => {
            assert!(denial
                .to_string()
                .contains("cannot surrender an unpublished primary graph runtime"));
        }
        other => panic!("unexpected custom-backend transfer denial: {other:?}"),
    }
}
