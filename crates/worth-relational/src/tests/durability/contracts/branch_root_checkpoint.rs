use super::*;

#[test]
fn checkpoint_recovery_rejects_missing_exact_branch_root_artifact() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "root-artifact-required");
    runtime
        .durability_authority()
        .checkpoint()
        .expect("checkpoint captures the live main root");
    let mut plan = runtime
        .durability()
        .recovery_plan(RecoveryVerificationMode::NormalRecoveryVerification);
    plan.checkpoint
        .as_mut()
        .expect("checkpoint is selected")
        .branch_roots
        .clear();

    let mut recovered = persisted_runtime_with_test_schema();
    let error = recovered
        .durability_recovery()
        .recover(plan)
        .expect_err("global partition images cannot substitute for a branch root");

    assert_eq!(error.class, RecoveryFailureClass::CorruptCheckpoint);
    assert!(error.detail.contains("missing branch-root image"));
    assert_eq!(recovered.history().immutable_commit_count(), 0);
}

#[test]
fn checkpoint_recovery_rejects_root_artifact_without_commit_envelope() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "root-envelope-binding");
    runtime
        .durability_authority()
        .checkpoint()
        .expect("checkpoint captures the live main root");
    let mut plan = runtime
        .durability()
        .recovery_plan(RecoveryVerificationMode::NormalRecoveryVerification);
    let checkpoint = plan.checkpoint.as_mut().expect("checkpoint is selected");
    checkpoint.branch_roots[0].commit_id = crate::history::data::CommitId(u64::MAX);

    let mut recovered = persisted_runtime_with_test_schema();
    let error = recovered
        .durability_recovery()
        .recover(plan)
        .expect_err("a root image cannot select an unrelated or missing envelope");

    assert_eq!(error.class, RecoveryFailureClass::CorruptCheckpoint);
    assert!(error.detail.contains("names missing commit envelope"));
    assert_eq!(recovered.history().immutable_commit_count(), 0);
}

#[test]
fn tail_recovery_resolves_the_checkpoint_target_root_not_the_fork_source_head() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "shared-seed");
    let storm = create_branch_from_main(&mut runtime, "storm");
    runtime
        .durability_authority()
        .checkpoint()
        .expect("checkpoint retains the shared pre-divergence root");

    create_entity_outcome(&mut runtime, "main-tail-only");
    create_entity_outcome_on_branch(&mut runtime, "storm-tail-only", storm.clone());
    let plan = runtime
        .durability()
        .recovery_plan(RecoveryVerificationMode::NormalRecoveryVerification);

    let mut recovered = persisted_runtime_with_test_schema();
    recovered
        .durability_recovery()
        .recover(plan)
        .expect("divergent tail commits recover from their exact pre-commit roots");

    let main_count = current_branch_entity_count(&mut recovered, &BranchId("main".to_owned()));
    let storm_count = current_branch_entity_count(&mut recovered, &storm);
    assert_eq!(main_count, 2, "main contains seed plus its own tail write");
    assert_eq!(
        storm_count, 2,
        "storm excludes the source's later tail write"
    );
}

#[test]
fn checkpoint_recovery_rejects_duplicate_global_partition_images() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "duplicate-global-partition");
    runtime.durability_authority().checkpoint().unwrap();
    let mut plan = runtime
        .durability()
        .recovery_plan(RecoveryVerificationMode::NormalRecoveryVerification);
    let checkpoint = plan.checkpoint.as_mut().expect("selected checkpoint");
    checkpoint
        .partition_images
        .push(checkpoint.partition_images[0].clone());

    let mut recovered = persisted_runtime_with_test_schema();
    let error = recovered.durability_recovery().recover(plan).unwrap_err();

    assert_eq!(error.class, RecoveryFailureClass::CorruptCheckpoint);
    assert!(error.detail.contains("duplicate partition image"));
    assert_eq!(recovered.history().immutable_commit_count(), 0);
}

#[test]
fn checkpoint_recovery_rejects_duplicate_branch_root_partition_images() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "duplicate-root-partition");
    runtime.durability_authority().checkpoint().unwrap();
    let mut plan = runtime
        .durability()
        .recovery_plan(RecoveryVerificationMode::NormalRecoveryVerification);
    let root = &mut plan.checkpoint.as_mut().unwrap().branch_roots[0];
    root.partition_images.push(root.partition_images[0].clone());

    let mut recovered = persisted_runtime_with_test_schema();
    let error = recovered.durability_recovery().recover(plan).unwrap_err();

    assert_eq!(error.class, RecoveryFailureClass::CorruptCheckpoint);
    assert!(error.detail.contains("duplicate partition image"));
    assert_eq!(recovered.history().immutable_commit_count(), 0);
}

#[test]
fn checkpoint_recovery_rejects_foreign_branch_target_substitution() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "lineage-seed");
    let feature = create_branch_from_main(&mut runtime, "feature");
    create_entity_outcome_on_branch(&mut runtime, "feature-only", feature.clone());
    create_entity_outcome(&mut runtime, "main-only");
    runtime.durability_authority().checkpoint().unwrap();
    let mut plan = runtime
        .durability()
        .recovery_plan(RecoveryVerificationMode::NormalRecoveryVerification);
    let checkpoint = plan.checkpoint.as_mut().unwrap();
    let foreign_target = checkpoint
        .branch_cells
        .iter()
        .find(|cell| cell.branch_id == feature)
        .unwrap()
        .observation
        .target()
        .clone();
    let main = checkpoint
        .branch_cells
        .iter_mut()
        .find(|cell| cell.branch_id.0 == "main")
        .unwrap();
    replace_checkpoint_target(main, foreign_target);

    let mut recovered = persisted_runtime_with_test_schema();
    let error = recovered.durability_recovery().recover(plan).unwrap_err();

    assert_eq!(error.class, RecoveryFailureClass::CorruptCheckpoint);
    assert!(error.detail.contains("foreign branch stream"));
    assert_eq!(recovered.history().immutable_commit_count(), 0);
}

#[test]
fn tail_recovery_rejects_foreign_branch_target_substitution() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "tail-lineage-seed");
    let feature = create_branch_from_main(&mut runtime, "feature");
    create_entity_outcome_on_branch(&mut runtime, "feature-before-checkpoint", feature.clone());
    runtime.durability_authority().checkpoint().unwrap();
    let tail_commit = create_entity_outcome(&mut runtime, "main-tail");
    let mut plan = runtime
        .durability()
        .recovery_plan(RecoveryVerificationMode::NormalRecoveryVerification);
    let foreign_target = plan
        .checkpoint
        .as_ref()
        .unwrap()
        .branch_cells
        .iter()
        .find(|cell| cell.branch_id == feature)
        .unwrap()
        .observation
        .target()
        .clone();
    let tail_cell = plan
        .tail_log
        .iter_mut()
        .map(|commit| commit.envelope_mut_for_test())
        .find(|envelope| envelope.commit.commit_id == tail_commit.commit.commit_id)
        .unwrap()
        .branch_cell_checkpoint
        .as_mut()
        .unwrap();
    replace_checkpoint_target(tail_cell, foreign_target);

    let mut recovered = persisted_runtime_with_test_schema();
    let error = recovered.durability_recovery().recover(plan).unwrap_err();

    assert_eq!(error.class, RecoveryFailureClass::CorruptCheckpoint);
    assert!(error.detail.contains("foreign branch stream"));
    assert_eq!(recovered.history().immutable_commit_count(), 0);
}

fn replace_checkpoint_target(
    checkpoint: &mut crate::branch::RelationalBranchCellCheckpoint,
    target: worth_foundational::FoundationalBranchTarget<crate::branch::RelationalBranchTarget>,
) {
    checkpoint.observation = crate::branch::relational_branch_observation(
        checkpoint.runtime_instance_id,
        &checkpoint.branch_id.0,
        target,
        checkpoint.observation.generation(),
    )
    .unwrap();
}

fn current_branch_entity_count(runtime: &RelationalRuntime, branch_id: &BranchId) -> usize {
    let identity = runtime
        .branch_identity(branch_id)
        .expect("recovered branch identity exists");
    let snapshot = crate::tests::support::snapshot_for_owner_identity(runtime, &identity);
    runtime
        .read_truth()
        .read_snapshot(&snapshot)
        .expect("recovered branch snapshot is readable")
        .entities()
        .len()
}
