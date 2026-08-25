use super::{
    admitted_program, authenticated_principal, idempotency, installed_authorization_world,
    live_scope, resolved_account,
};
use crate::domain_computation::primary_graph::WorthQueryApplicationCommitOutcome;
use worth_runtime_bridge::facade::TruthBranchHeadSource;

#[test]
fn application_commit_returns_exact_repair_authority_after_performed_append_fault() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let account = resolved_account(&world, "open", &request);
    let program = admitted_program(
        &world,
        &principal,
        &account,
        &request,
        "performed-before-durable-fault",
    );
    world.application.fail_next_durable_append_for_test();

    let WorthQueryApplicationCommitOutcome::SettlementDeferred(deferred) = world
        .application
        .compare_and_commit_application(program, idempotency(91, 91))
    else {
        panic!("performed application publication must retain unresolved settlement evidence");
    };
    assert_eq!(
        deferred.next_action(),
        crate::domain_computation::primary_graph::WorthQueryApplicationSettlementNextAction::RecoverDeferredApplicationSettlement
    );
    let settlement = deferred.settlement().clone();
    let foreign_world = installed_authorization_world(true);
    assert!(matches!(
        foreign_world
            .application
            .recover_deferred_application_settlement(&deferred),
        Err(crate::domain_computation::primary_graph::WorthQueryApplicationSettlementRecoveryError::Durability(
            worth_relational::facade::publication::DeferredPublicationSettlementError::ForeignRuntime {
                ..
            }
        ))
    ));
    let repaired = world
        .application
        .recover_deferred_application_settlement(&deferred)
        .expect("installed application owner completes typed settlement recovery");
    let repeated = world
        .application
        .recover_deferred_application_settlement(&deferred)
        .expect("installed application settlement recovery is idempotent");
    assert_eq!(repaired.commit_id, settlement.commit().commit_id);
    assert_eq!(repeated, repaired);

    let next_account = resolved_account(&world, "performed-before-durable-fault", &request);
    assert_eq!(next_account.entity_id(), account.entity_id());
    let later_program = admitted_program(
        &world,
        &principal,
        &next_account,
        &request,
        "performed-after-settlement-repair",
    );
    assert!(matches!(
        world
            .application
            .compare_and_commit_application(later_program, idempotency(92, 92)),
        WorthQueryApplicationCommitOutcome::Committed(_)
    ));
}

#[test]
fn settlement_authority_survives_index_reconstruction_after_append_fault() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let account = resolved_account(&world, "open", &request);
    let program = admitted_program(
        &world,
        &principal,
        &account,
        &request,
        "combined-publication-fault",
    );
    let retry = admitted_program(
        &world,
        &principal,
        &account,
        &request,
        "combined-publication-fault",
    );
    let truth_before = world
        .application
        .primary_provider
        .graph
        .current_truth_snapshot(
            &crate::domain_computation::primary_graph::primary_truth_branch_identity(),
        )
        .expect("application publication installs an initial Bridge head");
    world.application.fail_next_durable_append_for_test();
    world.faults.fail_next_index_publication();

    let WorthQueryApplicationCommitOutcome::SettlementDeferred(deferred) = world
        .application
        .compare_and_commit_application(program, idempotency(93, 93))
    else {
        panic!("index reconstruction must not erase exact settlement authority");
    };
    assert!(deferred.publication_failure_detail().is_some());
    assert_eq!(
        deferred.next_action(),
        crate::domain_computation::primary_graph::WorthQueryApplicationSettlementNextAction::RecoverDeferredApplicationSettlement
    );
    assert_eq!(
        world
            .application
            .primary_provider
            .graph
            .current_truth_snapshot(
                &crate::domain_computation::primary_graph::primary_truth_branch_identity(),
            ),
        Some(truth_before.clone())
    );
    world
        .application
        .recover_deferred_application_settlement(&deferred)
        .expect("application owner completes the preserved settlement and Query publication");
    assert_ne!(
        world
            .application
            .primary_provider
            .graph
            .current_truth_snapshot(
                &crate::domain_computation::primary_graph::primary_truth_branch_identity(),
            ),
        Some(truth_before)
    );
    assert!(matches!(
        world
            .application
            .compare_and_commit_application(retry, idempotency(93, 93)),
        WorthQueryApplicationCommitOutcome::AlreadyCommitted(_)
    ));
    let _committed = resolved_account(&world, "combined-publication-fault", &request);
}

#[test]
fn settlement_recovery_preserves_a_later_legal_application_head() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let account = resolved_account(&world, "open", &request);
    let performed = admitted_program(
        &world,
        &principal,
        &account,
        &request,
        "performed-before-intervening-commit",
    );
    let performed_retry = admitted_program(
        &world,
        &principal,
        &account,
        &request,
        "performed-before-intervening-commit",
    );
    world.application.fail_next_durable_append_for_test();
    world.faults.fail_next_index_publication();
    let WorthQueryApplicationCommitOutcome::SettlementDeferred(deferred) = world
        .application
        .compare_and_commit_application(performed, idempotency(94, 94))
    else {
        panic!("performed application commit must retain exact settlement recovery");
    };

    world
        .application
        .primary_provider
        .graph
        .with_runtime_mut(|runtime| {
            runtime.repair_deferred_publication_settlement(deferred.settlement())
        })
        .expect("adversarial setup repairs only the durable settlement");
    world
        .application
        .primary_provider
        .graph
        .with_runtime_mut(|runtime| {
            world
                .application
                .primary_provider
                .graph
                .ensure_primary_indexes_current_for_branch(runtime, deferred.branch())
        })
        .expect("adversarial setup reconstructs the former half-published state");
    let unrelated = resolved_account(&world, "unrelated", &request);
    let intervening = admitted_program(
        &world,
        &principal,
        &unrelated,
        &request,
        "intervening-application-commit",
    );

    let intervening_outcome = world
        .application
        .compare_and_commit_application(intervening, idempotency(95, 95));
    let intervening_receipt = match intervening_outcome {
        WorthQueryApplicationCommitOutcome::Committed(receipt) => receipt,
        outcome => panic!("intervening application attempt did not commit: {outcome:?}"),
    };

    world
        .application
        .recover_deferred_application_settlement(&deferred)
        .expect("serialized recovery accepts the performed commit in current ancestry");
    let bridge_head = world
        .application
        .primary_provider
        .graph
        .relational_bridge_source()
        .load_branch_head_patch(
            &crate::domain_computation::primary_graph::primary_truth_branch_identity(),
        )
        .expect("recovery leaves the later application commit bound as Bridge head");
    assert_eq!(
        bridge_head.commit_identity(),
        &worth_runtime_bridge::facade::TruthCommitIdentity::from_relational_commit_id(
            intervening_receipt.commit_id().0,
        ),
        "earlier settlement recovery must not rewind a later legal Bridge head"
    );
    assert!(matches!(
        world
            .application
            .compare_and_commit_application(performed_retry, idempotency(94, 94)),
        WorthQueryApplicationCommitOutcome::AlreadyCommitted(_)
    ));
    let recovered = resolved_account(&world, "performed-before-intervening-commit", &request);
    assert_eq!(recovered.entity_id(), account.entity_id());
    let _latest = resolved_account(&world, "intervening-application-commit", &request);
}
