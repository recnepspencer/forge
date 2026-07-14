use super::*;

#[test]
fn handle_bound_contribution_matches_internal_oracle_and_retains_installed_witnesses() {
    let runtime = installed_runtime();
    let handle = runtime.domain(InstalledDomain).unwrap();
    let contributions = handle.contributions(&runtime).unwrap();
    let declaration = contribution_declaration("installed-authority");
    let target = contributions.intent_target(&declaration);
    let materialized = contributions
        .for_intent_target(target.clone())
        .unwrap()
        .advises("accepts")
        .because("the installed domain accepts this declaration")
        .materialize()
        .unwrap();
    let requested = WorthQueryAdmissionContributionAuthoring::advisory(
        "WORTH.tests.installed-domain.accepts",
        "the installed domain accepts this declaration",
    )
    .bind_to_installed_target(target.clone());
    let worth_proof::TransitionOutcome::Success(eligible) =
        evaluate_requested_domain_capability_contribution(requested)
    else {
        panic!("installed contribution oracle must be eligible")
    };
    let worth_proof::TransitionOutcome::Success(admitted) =
        admit_eligible_domain_capability_contribution(eligible)
    else {
        panic!("installed contribution oracle must be admitted")
    };
    let worth_proof::TransitionOutcome::Success(ready) =
        prepare_admitted_domain_capability_contribution_for_materialization(admitted, target)
    else {
        panic!("installed contribution oracle must be ready")
    };
    let oracle = materialize_canonical_admission_artifact(ready);
    assert_eq!(
        materialized.materialization_identity(),
        oracle.materialization_identity()
    );
    let authority = materialized.installed_authority().unwrap();
    assert_eq!(
        authority.authority_identity(),
        handle.authority().authority_identity()
    );
    assert_eq!(authority.package_identity(), handle.package_identity());
    assert_eq!(
        materialized.installed_world_identity(),
        Some(handle.authority().world_identity())
    );
}

#[test]
fn contribution_authority_denies_foreign_targets_and_uninstalled_categories_first() {
    let left = installed_runtime();
    let right = installed_runtime();
    let left_handle = left.domain(InstalledDomain).unwrap();
    let right_handle = right.domain(InstalledDomain).unwrap();
    let declaration = contribution_declaration("foreign-target");
    let foreign_target = left_handle
        .contributions(&left)
        .unwrap()
        .intent_target(&declaration);
    let denial = right_handle
        .contributions(&right)
        .unwrap()
        .for_intent_target(foreign_target)
        .unwrap_err();
    assert_eq!(
        denial.kind(),
        WorthQueryDomainHandleDenialKind::ForeignRuntime
    );
    let policy_denial = left_handle
        .contributions(&left)
        .unwrap()
        .for_intent(&declaration)
        .supports_capability("")
        .because("authority is checked before semantic content")
        .try_materialize();
    assert_eq!(
        policy_denial.denial().unwrap().kind(),
        WorthQueryDomainCapabilityProgressionDenialKind::ContributionCategoryNotInstalled
    );
}

#[test]
fn contribution_target_mismatch_requires_rebind_without_a_ready_successor() {
    let runtime = installed_runtime();
    let handle = runtime.domain(InstalledDomain).unwrap();
    let contributions = handle.contributions(&runtime).unwrap();
    let bound_target = contributions.intent_target(&contribution_declaration("bound"));
    let current_target = contributions.intent_target(&contribution_declaration("current"));
    let requested = WorthQueryAdmissionContributionAuthoring::advisory(
        "WORTH.tests.installed-domain.rebind",
        "target identity is exact",
    )
    .bind_to_installed_target(bound_target);
    let worth_proof::TransitionOutcome::Success(eligible) =
        evaluate_requested_domain_capability_contribution(requested)
    else {
        panic!("fixture must be eligible")
    };
    let worth_proof::TransitionOutcome::Success(admitted) =
        admit_eligible_domain_capability_contribution(eligible)
    else {
        panic!("fixture must be admitted")
    };
    assert!(matches!(
        prepare_admitted_domain_capability_contribution_for_materialization(
            admitted,
            current_target
        ),
        worth_proof::TransitionOutcome::RebindRequired(_)
    ));
}

fn contribution_declaration(name: &str) -> WorthQueryIntentDeclaration {
    WorthQueryIntentDeclaration::strategy_commit(
        name,
        "installed-domain-test",
        "1.0",
        "installed-domain-test.v1",
        WorthQueryIntentInput::null(),
    )
}
