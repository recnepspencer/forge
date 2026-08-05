use super::*;

#[derive(Clone, Copy)]
pub(super) enum CapabilityGrantPopulation {
    None,
    Current,
    CurrentAndFutureReplacement,
    CurrentWithSameResourceUnrelated(usize),
    ExactPairPopulation(usize),
    Composed(super::capability_seed::CapabilityCompositionScenario),
    Delegated { links: usize, unrelated: usize },
    Elevated(super::capability_elevation_seed::CapabilityElevationScenario),
}

pub(in crate::domain_computation::primary_graph) struct AuthorizationWorld {
    pub(in crate::domain_computation::primary_graph) application:
        crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<
            IdentityExecutionSchema,
        >,
    pub(in crate::domain_computation::primary_graph) binding: InstalledIdentityBinding,
    pub(in crate::domain_computation::primary_graph) invariant:
        crate::domain_computation::primary_graph::WorthQueryApplicationInvariantProjectionAuthority<
            IdentityExecutionSchema,
        >,
}

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
        bind_account(
            &mut bootstrap,
            "account-1",
            "open",
            "primary",
            Some("reviewed"),
        );
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

pub(in crate::domain_computation::primary_graph) fn installed_authorization_world(
    include_owner_relation: bool,
) -> AuthorizationWorld {
    installed_authorization_world_with_principal_count(
        owner_bindings(include_owner_relation),
        false,
        1,
        "primary",
        WorthQueryApplicationQueryResourceProfile::default(),
        CapabilityGrantPopulation::None,
    )
}

pub(in crate::domain_computation::primary_graph) fn installed_authorization_world_with_label(
    label: &str,
) -> AuthorizationWorld {
    installed_authorization_world_with_principal_count(
        &[("principal-0", "account-1"), ("principal-0", "account-2")],
        false,
        1,
        label,
        WorthQueryApplicationQueryResourceProfile::default(),
        CapabilityGrantPopulation::None,
    )
}

pub(in crate::domain_computation::primary_graph) fn installed_authorization_world_with_resource_profile(
    profile: WorthQueryApplicationQueryResourceProfile,
) -> AuthorizationWorld {
    installed_authorization_world_with_principal_count(
        &[("principal-0", "account-1"), ("principal-0", "account-2")],
        false,
        1,
        "primary",
        profile,
        CapabilityGrantPopulation::None,
    )
}

pub(in crate::domain_computation::primary_graph) fn installed_two_principal_authorization_world(
    include_owner_relation: bool,
) -> AuthorizationWorld {
    installed_authorization_world_with_principal_count(
        owner_bindings(include_owner_relation),
        false,
        2,
        "primary",
        WorthQueryApplicationQueryResourceProfile::default(),
        CapabilityGrantPopulation::None,
    )
}

pub(in crate::domain_computation::primary_graph) fn installed_blocked_authorization_world(
) -> AuthorizationWorld {
    installed_authorization_world_with_principal_count(
        &[("principal-0", "account-1"), ("principal-0", "account-2")],
        true,
        1,
        "primary",
        WorthQueryApplicationQueryResourceProfile::default(),
        CapabilityGrantPopulation::None,
    )
}

pub(super) fn installed_authorization_world_with_principal_count(
    owner_bindings: &[(&str, &str)],
    include_blocked_relation: bool,
    principal_count: usize,
    primary_label: &str,
    resources: WorthQueryApplicationQueryResourceProfile,
    capability_grants: CapabilityGrantPopulation,
) -> AuthorizationWorld {
    let declaration = IdentityExecutionSchema::declaration().unwrap();
    let admitted = WorthQueryInstallationAdmissionProfile::new("support", "configuration")
        .admit(portable_package(declaration.clone()))
        .unwrap();
    let installation = WorthQueryExecutionRuntimeInstaller::new()
        .application_query_resources(resources)
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
    if principal_count >= 2 {
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
    if principal_count >= 3 {
        bootstrap
            .bind_principal(
                &binding,
                WorthQueryApplicationPrincipalKey::new("principal-2").unwrap(),
                3_u64,
                external_identity("carol"),
                WorthQueryPrincipalMappingStatus::Enabled,
            )
            .unwrap();
    }
    assert!(
        principal_count <= 3,
        "fixture supports at most three principals"
    );
    bind_account(
        &mut bootstrap,
        "account-1",
        "open",
        primary_label,
        Some("reviewed"),
    );
    let secondary_status = match capability_grants {
        CapabilityGrantPopulation::Elevated(
            super::capability_elevation_seed::CapabilityElevationScenario::DistinctCommandResource,
        ) => "open",
        _ => "unrelated",
    };
    bind_account(
        &mut bootstrap,
        "account-2",
        secondary_status,
        "secondary",
        None,
    );
    bind_activity(&mut bootstrap, "activity-primary", 11);
    bind_activity(&mut bootstrap, "activity-secondary", 22);
    bootstrap
        .bind_relation(WorthQueryApplicationRelationSeed::new(
            AccountPrimaryActivity::reference(),
            "primary-activity-1",
            WorthQueryApplicationEntityKey::new("account-1").unwrap(),
            WorthQueryApplicationEntityKey::new("activity-primary").unwrap(),
        ))
        .unwrap();
    bootstrap
        .bind_relation(WorthQueryApplicationRelationSeed::new(
            AccountSecondaryActivity::reference(),
            "secondary-activity-1",
            WorthQueryApplicationEntityKey::new("account-1").unwrap(),
            WorthQueryApplicationEntityKey::new("activity-secondary").unwrap(),
        ))
        .unwrap();
    for (relation, activity) in [
        ("all-activity-2", "activity-secondary"),
        ("all-activity-1", "activity-primary"),
    ] {
        bootstrap
            .bind_relation(WorthQueryApplicationRelationSeed::new(
                AccountAllActivity::reference(),
                relation,
                WorthQueryApplicationEntityKey::new("account-1").unwrap(),
                WorthQueryApplicationEntityKey::new(activity).unwrap(),
            ))
            .unwrap();
    }
    for (relation, activity) in [
        ("reverse-activity-1", "activity-primary"),
        ("reverse-activity-2", "activity-secondary"),
    ] {
        bootstrap
            .bind_relation(WorthQueryApplicationRelationSeed::new(
                ActivityAccount::reference(),
                relation,
                WorthQueryApplicationEntityKey::new(activity).unwrap(),
                WorthQueryApplicationEntityKey::new("account-1").unwrap(),
            ))
            .unwrap();
    }
    for (ordinal, (principal, account)) in owner_bindings.iter().enumerate() {
        bootstrap
            .bind_relation(WorthQueryApplicationRelationSeed::new(
                AccountOwner::reference(),
                format!("owner-{}", ordinal + 1),
                WorthQueryApplicationEntityKey::new(*principal).unwrap(),
                WorthQueryApplicationEntityKey::new(*account).unwrap(),
            ))
            .unwrap();
    }
    if include_blocked_relation {
        bootstrap
            .bind_relation(WorthQueryApplicationRelationSeed::new(
                AccountBlocked::reference(),
                "blocked-1",
                WorthQueryApplicationEntityKey::new("principal-0").unwrap(),
                WorthQueryApplicationEntityKey::new("account-1").unwrap(),
            ))
            .unwrap();
    }
    match capability_grants {
        CapabilityGrantPopulation::None => {}
        CapabilityGrantPopulation::Current => super::capability_seed::bind_grant(&mut bootstrap),
        CapabilityGrantPopulation::CurrentAndFutureReplacement => {
            super::capability_seed::bind_grant(&mut bootstrap);
            super::capability_seed::bind_future_replacement_grant(&mut bootstrap);
        }
        CapabilityGrantPopulation::CurrentWithSameResourceUnrelated(unrelated) => {
            super::capability_population_seed::bind_same_resource_unrelated_grants(
                &mut bootstrap,
                unrelated,
            )
        }
        CapabilityGrantPopulation::ExactPairPopulation(count) => {
            super::capability_population_seed::bind_exact_pair_grants(&mut bootstrap, count)
        }
        CapabilityGrantPopulation::Composed(scenario) => {
            super::capability_seed::bind_composed_grant(&mut bootstrap, scenario)
        }
        CapabilityGrantPopulation::Delegated { links, unrelated } => {
            super::capability_seed::bind_delegated_grants(&mut bootstrap, links, unrelated)
        }
        CapabilityGrantPopulation::Elevated(scenario) => {
            super::capability_elevation_seed::bind_elevated_capability(&mut bootstrap, scenario)
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

fn owner_bindings(include: bool) -> &'static [(&'static str, &'static str)] {
    if include {
        &[("principal-0", "account-1"), ("principal-0", "account-2")]
    } else {
        &[]
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
    note: Option<&str>,
) {
    let mut seed = WorthQueryApplicationEntitySeed::new(
        Account::reference(),
        WorthQueryApplicationEntityKey::new(key).unwrap(),
    )
    .field(AccountIdentity::reference(), key.to_owned())
    .field(AccountStatus::reference(), status.to_string())
    .field(AccountLabel::reference(), label.to_string());
    if let Some(note) = note {
        seed = seed.field(AccountNote::reference(), note.to_string());
    }
    bootstrap.bind_entity(seed).unwrap();
}

fn bind_activity(
    bootstrap: &mut crate::domain_computation::primary_graph::WorthQueryPrimaryGraphBootstrap<
        IdentityExecutionSchema,
    >,
    key: &str,
    sequence: u64,
) {
    bootstrap
        .bind_entity(
            WorthQueryApplicationEntitySeed::new(
                Activity::reference(),
                WorthQueryApplicationEntityKey::new(key).unwrap(),
            )
            .field(ActivityIdentity::reference(), key.to_owned())
            .field(ActivitySequence::reference(), sequence),
        )
        .unwrap();
}
