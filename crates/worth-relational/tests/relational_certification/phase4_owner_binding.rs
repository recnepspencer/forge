use super::phase4_fork_evidence::{
    assert_denial_left_no_reference_residue, assert_oracle_matches, capture_reference_evidence,
    certified_supply_chain_world,
};
use super::world::supply_chain::SupplyChainScale;
use worth_relational::facade::branch::{
    RelationalBranchIdentityDenial, RelationalLegacyBranchBindingDenial,
};
use worth_relational::facade::history::BranchId;
use worth_relational::facade::merge::{MergeExecutionRequest, MergeIntent};
use worth_relational::facade::transactions::{
    ConflictClass, TransactionCommitError, WorkerIntentBatch,
};

#[test]
fn owner_issues_transaction_options_from_exact_main_identity() {
    let (mut world, expected) = certified_supply_chain_world(SupplyChainScale::court());
    assert_oracle_matches(&world, &expected);
    let identity = world.runtime.main_branch_identity();
    assert_eq!(identity.branch_id(), &BranchId("main".to_owned()));

    let options = world
        .runtime
        .transaction_options_for(&identity)
        .expect("the owner admits its exact main branch identity");

    let mut transaction = world.runtime.begin_transaction(options);
    transaction.push_batch(WorkerIntentBatch::new("owner-issued-options"));
    let result = transaction
        .commit()
        .expect("owner-issued options route through the ordinary commit authority");
    assert_eq!(result.commit.branch_id, BranchId("main".to_owned()));
    assert!(matches!(
        world
            .runtime
            .branch_identity(&BranchId("forged-target".to_owned())),
        Err(RelationalBranchIdentityDenial::UnknownBranch(branch))
            if branch == BranchId("forged-target".to_owned())
    ));
    assert_oracle_matches(&world, &expected);
}

#[test]
fn copied_identity_cannot_issue_options_in_a_forked_runtime() {
    let (world, expected) = certified_supply_chain_world(SupplyChainScale::court());
    let identity = world.runtime.main_branch_identity();
    let clone = world.runtime.fork();

    assert!(matches!(
        clone.transaction_options_for(&identity),
        Err(RelationalLegacyBranchBindingDenial::ForeignRuntime { .. })
    ));
    assert_oracle_matches(&world, &expected);
}

#[test]
fn owner_issued_transaction_options_cannot_cross_runtime() {
    let (world, expected) = certified_supply_chain_world(SupplyChainScale::court());
    let main_identity = world.runtime.main_branch_identity();
    let options = world
        .runtime
        .transaction_options_for(&main_identity)
        .expect("configured main branch must remain owner-admissible");
    let mut foreign = world.runtime.fork();
    let before = capture_reference_evidence(
        &mut foreign,
        &BranchId("main".to_owned()),
        &BranchId("foreign-target".to_owned()),
        world.commit.commit_id,
    );
    let mut transaction = foreign.begin_transaction(options);
    transaction.push_batch(WorkerIntentBatch::new("foreign-owner-options"));

    let error = transaction
        .commit()
        .expect_err("an owner-issued binding must not route into a cloned runtime");
    assert!(matches!(
        &error,
        TransactionCommitError::Conflict {
            error: conflict,
            ..
        } if matches!(conflict.class, ConflictClass::StaleValidationBasis { .. })
    ));
    assert!(error.detail().contains("another Relational runtime"));
    let after = capture_reference_evidence(
        &mut foreign,
        &BranchId("main".to_owned()),
        &BranchId("foreign-target".to_owned()),
        world.commit.commit_id,
    );
    assert_denial_left_no_reference_residue(&before, &after);
    assert_oracle_matches(&world, &expected);
}

#[test]
fn public_owner_binding_path_prepares_merge_without_test_adapter() {
    let (mut world, expected) = certified_supply_chain_world(SupplyChainScale::court());
    let (_, source_basis) = world
        .runtime
        .observe_fork_source(&BranchId("main".to_owned()))
        .expect("main has a fork source");
    world
        .runtime
        .fork_branch(BranchId("storm".to_owned()), source_basis)
        .expect("storm branch is installed by the owner");

    let raw = MergeExecutionRequest::new(
        BranchId("main".to_owned()),
        BranchId("storm".to_owned()),
        MergeIntent::ReconcileIntoTarget,
    );
    let owner_bound = world
        .runtime
        .bind_merge_execution_request(raw)
        .expect("the runtime owner binds both exact branch cells");
    world
        .runtime
        .prepare_merge_execution(owner_bound)
        .expect("the non-test production facade accepts only owner-bound merge requests");
    assert_oracle_matches(&world, &expected);
}

#[test]
fn unrelated_branch_progress_is_not_staled_by_main_branch_movement() {
    let (mut world, expected) = certified_supply_chain_world(SupplyChainScale::court());
    let (_, source_basis) = world
        .runtime
        .observe_fork_source(&BranchId("main".to_owned()))
        .expect("main has a fork source");
    world
        .runtime
        .fork_branch(BranchId("storm".to_owned()), source_basis)
        .expect("storm branch is installed by the owner");

    let storm_identity = world
        .runtime
        .branch_identity(&BranchId("storm".to_owned()))
        .expect("storm identity is owner-issued");
    let storm_options = world
        .runtime
        .transaction_options_for(&storm_identity)
        .expect("storm options are owner-issued");
    let candidate = world
        .runtime
        .begin_transaction(storm_options)
        .validate()
        .expect("storm candidate validates against its own branch reference");

    let mut main_transaction = world.runtime.begin_transaction({
        let identity = world.runtime.main_branch_identity();
        world
            .runtime
            .transaction_options_for(&identity)
            .expect("configured main branch must remain owner-admissible")
    });
    main_transaction.push_batch(WorkerIntentBatch::new("advance-main-only"));
    main_transaction
        .commit()
        .expect("main branch movement succeeds independently");

    let committed = world
        .runtime
        .commit_validated_mutation(candidate)
        .expect("main branch movement must not stale an unrelated storm basis");
    assert_eq!(committed.commit.branch_id, BranchId("storm".to_owned()));
    assert_oracle_matches(&world, &expected);
}

#[test]
fn source_movement_cannot_mutate_a_forked_target_reference() {
    let (mut world, expected) = certified_supply_chain_world(SupplyChainScale::court());
    let (_, source_basis) = world
        .runtime
        .observe_fork_source(&BranchId("main".to_owned()))
        .expect("main exposes an exact source basis");
    world
        .runtime
        .fork_branch(BranchId("storm".to_owned()), source_basis)
        .expect("storm fork succeeds");

    let before = capture_reference_evidence(
        &mut world.runtime,
        &BranchId("main".to_owned()),
        &BranchId("storm".to_owned()),
        world.commit.commit_id,
    );
    let target_state_before = before
        .target_state
        .clone()
        .expect("fork target has an exact reference state");
    let target_identity_before = before
        .target_identity
        .clone()
        .expect("fork target has an owner identity");

    let main_identity = world.runtime.main_branch_identity();
    let options = world
        .runtime
        .transaction_options_for(&main_identity)
        .expect("main movement uses an owner-issued binding");
    let mut advance = world.runtime.begin_transaction(options);
    advance.push_batch(WorkerIntentBatch::new("advance-source-after-fork"));
    advance
        .commit()
        .expect("source movement succeeds after the fork");

    let after = capture_reference_evidence(
        &mut world.runtime,
        &BranchId("main".to_owned()),
        &BranchId("storm".to_owned()),
        world.commit.commit_id,
    );
    assert_ne!(after.source_state, before.source_state);
    assert_eq!(after.target_state, Some(target_state_before));
    assert_eq!(
        after.target_identity,
        Ok(target_identity_before),
        "source movement must not replace the fork target identity"
    );
    assert_eq!(after.artifact_identity, before.artifact_identity);
    assert_eq!(after.artifact_parents, before.artifact_parents);
    assert_oracle_matches(&world, &expected);
}

#[test]
fn unknown_branch_cannot_issue_an_owner_binding() {
    let (mut world, expected) = certified_supply_chain_world(SupplyChainScale::court());
    let before = capture_reference_evidence(
        &mut world.runtime,
        &BranchId("main".to_owned()),
        &BranchId("ghost".to_owned()),
        world.commit.commit_id,
    );

    assert!(matches!(
        world.runtime.branch_identity(&BranchId("ghost".to_owned())),
        Err(RelationalBranchIdentityDenial::UnknownBranch(branch))
            if branch == BranchId("ghost".to_owned())
    ));
    let after = capture_reference_evidence(
        &mut world.runtime,
        &BranchId("main".to_owned()),
        &BranchId("ghost".to_owned()),
        world.commit.commit_id,
    );
    assert_denial_left_no_reference_residue(&before, &after);
    assert_oracle_matches(&world, &expected);
}

#[test]
fn validation_commit_race_is_closed_by_branch_local_truth_version() {
    let (mut world, expected) = certified_supply_chain_world(SupplyChainScale::court());
    let identity = world.runtime.main_branch_identity();
    let candidate_options = world
        .runtime
        .transaction_options_for(&identity)
        .expect("candidate receives owner-issued branch proof");
    let candidate = world
        .runtime
        .begin_transaction(candidate_options)
        .validate()
        .expect("the candidate validates against the current branch cell");
    let before_advance = world
        .runtime
        .branch_reference_state(&BranchId("main".to_owned()))
        .expect("main reference state exists before the advancing commit");

    let advance_options = world
        .runtime
        .transaction_options_for(&identity)
        .expect("the advancing transaction receives a fresh branch proof");
    let mut advance = world.runtime.begin_transaction(advance_options);
    advance.push_batch(WorkerIntentBatch::new("advance-branch-version"));
    advance.commit().expect("the branch truth version advances");
    let after_advance = world
        .runtime
        .branch_reference_state(&BranchId("main".to_owned()))
        .expect("main reference state exists after the advancing commit");
    assert_eq!(
        after_advance.observation().generation().get(),
        before_advance.observation().generation().get() + 1
    );
    assert_eq!(
        after_advance.truth_version().as_u64(),
        before_advance.truth_version().as_u64() + 1
    );
    let before_denial = capture_reference_evidence(
        &mut world.runtime,
        &BranchId("main".to_owned()),
        &BranchId("stale-target".to_owned()),
        world.commit.commit_id,
    );

    let denied = world
        .runtime
        .commit_validated_mutation(candidate)
        .expect_err("the old branch-local version must no longer be current");
    assert!(matches!(
        &denied,
        TransactionCommitError::Conflict {
            error: conflict,
            ..
        } if matches!(conflict.class, ConflictClass::StaleValidationBasis { .. })
    ));
    let after_denial = capture_reference_evidence(
        &mut world.runtime,
        &BranchId("main".to_owned()),
        &BranchId("stale-target".to_owned()),
        world.commit.commit_id,
    );
    assert_denial_left_no_reference_residue(&before_denial, &after_denial);
    assert_oracle_matches(&world, &expected);
}
