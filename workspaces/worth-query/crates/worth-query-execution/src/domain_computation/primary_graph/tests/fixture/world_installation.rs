use super::*;

pub(in crate::domain_computation::primary_graph::tests) fn installed_world(
    rows: &[(&str, WorthQueryPrincipalMappingStatus)],
) -> IdentityWorld {
    installed_world_with_policy_fact(rows, false)
}

pub(in crate::domain_computation::primary_graph::tests) fn installed_world_with_policy_fact(
    rows: &[(&str, WorthQueryPrincipalMappingStatus)],
    include_policy_fact: bool,
) -> IdentityWorld {
    let declaration = IdentityExecutionSchema::declaration().unwrap();
    let package = portable_package(declaration.clone());
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
    for (ordinal, (subject, status)) in rows.iter().enumerate() {
        bootstrap
            .bind_principal(
                &binding,
                WorthQueryApplicationPrincipalKey::new(format!("principal-{ordinal}")).unwrap(),
                u64::try_from(ordinal + 1).unwrap(),
                external_identity(subject),
                *status,
            )
            .unwrap();
    }
    if include_policy_fact {
        bind_account(&mut bootstrap, "account-1", "open", "primary");
        bootstrap
            .bind_relation(WorthQueryApplicationRelationSeed::new(
                AccountOwner::reference(),
                "owner-1",
                WorthQueryApplicationEntityKey::new("principal-0").unwrap(),
                WorthQueryApplicationEntityKey::new("account-1").unwrap(),
            ))
            .unwrap();
    }
    let publication = bootstrap.publish(&mut runtime, &authority).unwrap();
    IdentityWorld {
        runtime,
        schema,
        binding,
        publication,
    }
}

pub(in crate::domain_computation::primary_graph::tests) fn installed_authorization_world(
    include_owner_relation: bool,
) -> AuthorizationWorld {
    installed_authorization_world_with_principal_count(include_owner_relation, 1)
}

pub(in crate::domain_computation::primary_graph::tests) fn installed_two_principal_authorization_world(
    include_owner_relation: bool,
) -> AuthorizationWorld {
    installed_authorization_world_with_principal_count(include_owner_relation, 2)
}

fn installed_authorization_world_with_principal_count(
    include_owner_relation: bool,
    principal_count: usize,
) -> AuthorizationWorld {
    let declaration = IdentityExecutionSchema::declaration().unwrap();
    let admitted = WorthQueryInstallationAdmissionProfile::new("support", "configuration")
        .admit(portable_package(declaration.clone()))
        .unwrap();
    let installation = WorthQueryExecutionRuntimeInstaller::new()
        .install(WorthQueryInstallationGeneration::initial(), [admitted])
        .unwrap();
    let (runtime, authority) = installation.into_parts();
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
            WorthQueryApplicationPrincipalKey::new("principal-0").unwrap(),
            1_u64,
            external_identity("alice"),
            WorthQueryPrincipalMappingStatus::Enabled,
        )
        .unwrap();
    if principal_count == 2 {
        bootstrap
            .bind_principal(
                &binding,
                WorthQueryApplicationPrincipalKey::new("principal-1").unwrap(),
                2_u64,
                external_identity("bob"),
                WorthQueryPrincipalMappingStatus::Enabled,
            )
            .unwrap();
    }
    bind_account(&mut bootstrap, "account-1", "open", "primary");
    bind_account(&mut bootstrap, "account-2", "unrelated", "secondary");
    if include_owner_relation {
        for (relation, account) in [("owner-1", "account-1"), ("owner-2", "account-2")] {
            bootstrap
                .bind_relation(WorthQueryApplicationRelationSeed::new(
                    AccountOwner::reference(),
                    relation,
                    WorthQueryApplicationEntityKey::new("principal-0").unwrap(),
                    WorthQueryApplicationEntityKey::new(account).unwrap(),
                ))
                .unwrap();
        }
    }
    let invariant = bootstrap.retain_invariant_projection_authority();
    let application = bootstrap
        .publish_application_runtime(runtime, authority, schema)
        .unwrap();
    AuthorizationWorld {
        application,
        binding,
        invariant,
    }
}

fn portable_package(
    declaration: worth_query_declaration::facade::application_schema::ApplicationSchemaDeclaration<
        IdentityExecutionSchema,
    >,
) -> WorthQueryValidatedPortableDomainPackage {
    WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        "identity_execution_test",
        1,
        0,
    ))
    .application_schema(declaration)
    .validate()
    .unwrap()
}

fn bind_account(
    bootstrap: &mut crate::domain_computation::primary_graph::WorthQueryPrimaryGraphBootstrap<
        IdentityExecutionSchema,
    >,
    key: &str,
    status: &str,
    label: &str,
) {
    bootstrap
        .bind_entity(
            WorthQueryApplicationEntitySeed::new(
                Account::reference(),
                WorthQueryApplicationEntityKey::new(key).unwrap(),
            )
            .field(AccountStatus::reference(), status.to_string())
            .field(AccountLabel::reference(), label.to_string()),
        )
        .unwrap();
}
