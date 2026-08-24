use super::reference_attempt_evidence::{
    assert_denial_left_no_reference_residue, capture_reference_evidence,
};
use super::world::supply_chain::{
    assert_oracle_matches, certified_supply_chain_world, SupplyChainScale,
};
use worth_relational::facade::branch::{
    RelationalBranchBasisDenial, RelationalBranchIdentityDenial,
};
use worth_relational::facade::history::BranchId;
use worth_relational::facade::merge::{MergeExecutionRequest, MergeIntent};
use worth_relational::facade::mvcc::RelationalBranchTransactionAdmissionDenial;
use worth_relational::facade::transactions::{
    ConflictClass, TransactionCommitError, WorkerIntentBatch,
};

#[test]
fn owner_issues_transaction_context_from_exact_main_identity() {
    let (mut world, expected) = certified_supply_chain_world(SupplyChainScale::court());
    assert_oracle_matches(&world, &expected);
    let identity = world.runtime.main_branch_identity();
    assert_eq!(identity.branch_id(), &BranchId("main".to_owned()));

    let context = world
        .runtime
        .admit_branch_basis(&identity)
        .expect("the owner admits its exact main branch identity");

    let mut transaction = {
        let transaction_validation_input = context;
        world
            .runtime
            .begin_branch_transaction(
                &transaction_validation_input,
                worth_relational::facade::mvcc::RelationalTransactionIntent::ordinary(),
            )
            .expect("owner-admitted transaction context")
    };
    transaction.push_batch(WorkerIntentBatch::new("owner-issued-options"));
    let result = transaction
        .commit(&mut world.runtime)
        .expect("owner-issued context routes through the ordinary commit authority");
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
fn copied_identity_cannot_issue_context_in_a_forked_runtime() {
    let (world, expected) = certified_supply_chain_world(SupplyChainScale::court());
    let identity = world.runtime.main_branch_identity();
    let clone = world.runtime.fork();

    assert!(matches!(
        clone.admit_branch_basis(&identity),
        Err(RelationalBranchBasisDenial::ForeignRuntime { .. })
    ));
    assert_oracle_matches(&world, &expected);
}

#[test]
fn owner_issued_transaction_basis_is_denied_before_foreign_transaction_creation() {
    let (world, expected) = certified_supply_chain_world(SupplyChainScale::court());
    let main_identity = world.runtime.main_branch_identity();
    let context = world
        .runtime
        .admit_branch_basis(&main_identity)
        .expect("configured main branch must remain owner-admissible");
    let mut foreign = world.runtime.fork();
    let before = capture_reference_evidence(
        &mut foreign,
        &BranchId("main".to_owned()),
        &BranchId("foreign-target".to_owned()),
        world.commit.commit_id,
    );
    let error = match foreign.begin_branch_transaction(
        &context,
        worth_relational::facade::mvcc::RelationalTransactionIntent::ordinary(),
    ) {
        Ok(_) => panic!("a foreign basis must be denied before transaction creation"),
        Err(error) => error,
    };
    assert!(matches!(
        error,
        RelationalBranchTransactionAdmissionDenial::ForeignRuntime { .. }
    ));
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
        .admit_branch_basis(&storm_identity)
        .expect("storm options are owner-issued");
    let candidate = {
        let transaction_validation_input = storm_options;
        world
            .runtime
            .begin_branch_transaction(
                &transaction_validation_input,
                worth_relational::facade::mvcc::RelationalTransactionIntent::ordinary(),
            )
            .expect("owner-admitted transaction context")
    }
    .validate(&mut world.runtime)
    .expect("storm candidate validates against its own branch reference");

    let mut main_transaction = {
        let transaction_validation_input = {
            let identity = world.runtime.main_branch_identity();
            world
                .runtime
                .admit_branch_basis(&identity)
                .expect("configured main branch must remain owner-admissible")
        };
        world
            .runtime
            .begin_branch_transaction(
                &transaction_validation_input,
                worth_relational::facade::mvcc::RelationalTransactionIntent::ordinary(),
            )
            .expect("owner-admitted transaction context")
    };
    main_transaction.push_batch(WorkerIntentBatch::new("advance-main-only"));
    main_transaction
        .commit(&mut world.runtime)
        .expect("main branch movement succeeds independently");

    let committed = world
        .runtime
        .commit_validated_proposal(candidate)
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
        .admit_branch_basis(&main_identity)
        .expect("main movement uses an owner-issued binding");
    let mut advance = {
        let transaction_validation_input = options;
        world
            .runtime
            .begin_branch_transaction(
                &transaction_validation_input,
                worth_relational::facade::mvcc::RelationalTransactionIntent::ordinary(),
            )
            .expect("owner-admitted transaction context")
    };
    advance.push_batch(WorkerIntentBatch::new("advance-source-after-fork"));
    advance
        .commit(&mut world.runtime)
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
        .admit_branch_basis(&identity)
        .expect("candidate receives owner-issued branch proof");
    let candidate = {
        let transaction_validation_input = candidate_options;
        world
            .runtime
            .begin_branch_transaction(
                &transaction_validation_input,
                worth_relational::facade::mvcc::RelationalTransactionIntent::ordinary(),
            )
            .expect("owner-admitted transaction context")
    }
    .validate(&mut world.runtime)
    .expect("the candidate validates against the current branch cell");
    let before_advance = world
        .runtime
        .branch_reference_state(&BranchId("main".to_owned()))
        .expect("main reference state exists before the advancing commit");

    let advance_options = world
        .runtime
        .admit_branch_basis(&identity)
        .expect("the advancing transaction receives a fresh branch proof");
    let mut advance = {
        let transaction_validation_input = advance_options;
        world
            .runtime
            .begin_branch_transaction(
                &transaction_validation_input,
                worth_relational::facade::mvcc::RelationalTransactionIntent::ordinary(),
            )
            .expect("owner-admitted transaction context")
    };
    advance.push_batch(WorkerIntentBatch::new("advance-branch-version"));
    advance
        .commit(&mut world.runtime)
        .expect("the branch truth version advances");
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
        .commit_validated_proposal(candidate)
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
