use super::*;
use worth_relational::facade::{
    history::BranchId,
    runtime::{RelationalRuntime, RelationalRuntimeConfig},
};

#[path = "world_installation/seeding.rs"]
mod seeding;

use seeding::{bind_account, bind_activity, owner_bindings, portable_package};

#[derive(Clone, Copy)]
enum CapabilityGrantPopulation {
    None,
    Current,
    CurrentAndFutureReplacement,
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
        None,
    )
}

pub(in crate::domain_computation::primary_graph) fn installed_authorization_world_on_branch(
    include_owner_relation: bool,
    branch: &str,
) -> AuthorizationWorld {
    installed_authorization_world_with_principal_count(
        owner_bindings(include_owner_relation),
        false,
        1,
        "primary",
        WorthQueryApplicationQueryResourceProfile::default(),
        CapabilityGrantPopulation::None,
        Some(branch),
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
        None,
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
        None,
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
        None,
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
        None,
    )
}

pub(in crate::domain_computation::primary_graph) fn installed_capability_authorization_world(
) -> AuthorizationWorld {
    installed_authorization_world_with_principal_count(
        &[],
        false,
        1,
        "primary",
        WorthQueryApplicationQueryResourceProfile::default(),
        CapabilityGrantPopulation::Current,
        None,
    )
}

pub(in crate::domain_computation::primary_graph) fn installed_capability_authorization_world_on_branch(
    branch: &str,
) -> AuthorizationWorld {
    installed_authorization_world_with_principal_count(
        &[],
        false,
        1,
        "primary",
        WorthQueryApplicationQueryResourceProfile::default(),
        CapabilityGrantPopulation::Current,
        Some(branch),
    )
}

pub(in crate::domain_computation::primary_graph) fn installed_capability_world_with_label(
    label: &str,
) -> AuthorizationWorld {
    installed_authorization_world_with_principal_count(
        &[],
        false,
        1,
        label,
        WorthQueryApplicationQueryResourceProfile::default(),
        CapabilityGrantPopulation::Current,
        None,
    )
}

pub(in crate::domain_computation::primary_graph) fn installed_capability_live_world(
) -> AuthorizationWorld {
    installed_authorization_world_with_principal_count(
        &[("principal-1", "account-1")],
        false,
        2,
        "primary",
        WorthQueryApplicationQueryResourceProfile::default(),
        CapabilityGrantPopulation::Current,
        None,
    )
}

pub(in crate::domain_computation::primary_graph) fn installed_capability_replacement_world(
) -> AuthorizationWorld {
    installed_authorization_world_with_principal_count(
        &[],
        false,
        1,
        "primary",
        WorthQueryApplicationQueryResourceProfile::default(),
        CapabilityGrantPopulation::CurrentAndFutureReplacement,
        None,
    )
}

fn installed_authorization_world_with_principal_count(
    owner_bindings: &[(&str, &str)],
    include_blocked_relation: bool,
    principal_count: usize,
    primary_label: &str,
    resources: WorthQueryApplicationQueryResourceProfile,
    capability_grants: CapabilityGrantPopulation,
    relational_branch: Option<&str>,
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
    let mut bootstrap = if let Some(branch) = relational_branch {
        let mut config = RelationalRuntimeConfig::default();
        config.history.main_branch = BranchId(branch.to_owned());
        authority
            .prepare_primary_graph_with_relational_runtime(
                &runtime,
                &schema,
                RelationalRuntime::new(config),
            )
            .unwrap()
    } else {
        authority.prepare_primary_graph(&runtime, &schema).unwrap()
    };
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
    bind_account(&mut bootstrap, "account-1", "open", primary_label);
    bind_account(&mut bootstrap, "account-2", "unrelated", "secondary");
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
    if !matches!(capability_grants, CapabilityGrantPopulation::None) {
        super::capability_seed::bind_grant(&mut bootstrap);
    }
    if matches!(
        capability_grants,
        CapabilityGrantPopulation::CurrentAndFutureReplacement
    ) {
        super::capability_seed::bind_future_replacement_grant(&mut bootstrap);
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
