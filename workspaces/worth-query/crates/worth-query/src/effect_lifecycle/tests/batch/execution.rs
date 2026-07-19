use worth_relational::facade::history::BranchId;

use crate::aspect_field_authoring::aspect_key;
use crate::effect_lifecycle::{
    effect_batch, EffectAuthoringBasis, EffectBatchExecutionDenialKind, EffectExecutionAuthority,
    EffectExecutionDenialKind,
};

use super::super::execution_support::{
    branch_snapshot_identity, create_entity, relational_runtime_with_intent_strategy,
    runtime_snapshot_identity, update_entity_name,
};
use super::super::support::{
    branch_mutation_basis, native_name_patch, raw_mutation_effect_with_binding,
    runtime_workflow_binding_for_branch, runtime_workflow_binding_with_snapshot,
};

#[test]
fn mutation_batch_executes_through_one_batch_native_lowered_plan() {
    let mut runtime = relational_runtime_with_intent_strategy();
    let left = create_entity(&mut runtime, "left", BranchId("main".to_string()));
    let right = create_entity(&mut runtime, "right", BranchId("main".to_string()));
    runtime
        .history_authority()
        .create_branch(
            BranchId("branch-a".to_string()),
            &BranchId("main".to_string()),
        )
        .expect("branch-a should be created");
    let binding = runtime_workflow_binding_with_snapshot(runtime_snapshot_identity(&runtime));

    let executed = effect_batch()
        .using_basis(EffectAuthoringBasis::from(branch_mutation_basis()))
        .push(raw_mutation_effect_with_binding(
            binding.clone(),
            left,
            native_name_patch("left-batched"),
        ))
        .push(raw_mutation_effect_with_binding(
            binding,
            right,
            native_name_patch("right-batched"),
        ))
        .admit()
        .expect("batch should admit")
        .lower()
        .expect("batch should lower")
        .execute_with(EffectExecutionAuthority::relational(&mut runtime))
        .expect("batch should execute");

    assert_eq!(executed.components().len(), 2);
    assert_eq!(executed.counters().batch_lowering_count(), 1);
    assert_eq!(executed.counters().effect_execution_width(), 1);
    assert_eq!(
        executed
            .aggregate_mutation()
            .expect("batch should retain one aggregate mutation commit")
            .outcome
            .changed_records
            .len(),
        2
    );
    let aggregate_commit_id = executed
        .aggregate_mutation()
        .expect("batch should retain one aggregate mutation commit")
        .outcome
        .commit
        .commit_id;
    assert!(executed.components().iter().all(|component| {
        component
            .as_mutation()
            .is_some_and(|commit| commit.outcome.commit.commit_id == aggregate_commit_id)
    }));

    let snapshot = runtime.snapshots().snapshot();
    let read_view = runtime
        .read_truth()
        .read_snapshot(&snapshot)
        .expect("snapshot should read");
    let names = read_view
        .entities()
        .iter()
        .filter_map(entity_name)
        .collect::<Vec<_>>();
    assert!(names.contains(&"left-batched".to_string()));
    assert!(names.contains(&"right-batched".to_string()));
}

#[test]
fn mutation_batch_preserves_branch_scoped_authority_target() {
    let mut runtime = relational_runtime_with_intent_strategy();
    let entity_id = create_entity(&mut runtime, "before", BranchId("main".to_string()));
    runtime
        .history_authority()
        .create_branch(
            BranchId("branch-a".to_string()),
            &BranchId("main".to_string()),
        )
        .expect("branch-a should be created");
    let main_head_before = update_entity_name(
        &mut runtime,
        entity_id,
        "main-diverged",
        BranchId("main".to_string()),
    );
    let branch_head_before = runtime
        .history()
        .branch_head(&BranchId("branch-a".to_string()))
        .expect("branch-a head should exist")
        .commit_id;

    let executed = effect_batch()
        .using_basis(EffectAuthoringBasis::from(branch_mutation_basis()))
        .push(raw_mutation_effect_with_binding(
            runtime_workflow_binding_for_branch(
                branch_snapshot_identity(&runtime, "branch-a"),
                "branch-a",
            ),
            entity_id,
            native_name_patch("branch-batched"),
        ))
        .admit()
        .expect("batch should admit")
        .lower()
        .expect("batch should lower")
        .execute_with(EffectExecutionAuthority::relational(&mut runtime))
        .expect("batch should execute");

    let aggregate = executed
        .aggregate_mutation()
        .expect("batch should retain one aggregate mutation commit");
    assert_eq!(aggregate.outcome.commit.parents, vec![branch_head_before]);
    assert_ne!(aggregate.outcome.commit.parents, vec![main_head_before]);
}

#[test]
fn retained_lowered_batch_denies_after_intervening_truth_change() {
    let mut runtime = relational_runtime_with_intent_strategy();
    let entity_id = create_entity(&mut runtime, "before", BranchId("main".to_string()));
    runtime
        .history_authority()
        .create_branch(
            BranchId("branch-a".to_string()),
            &BranchId("main".to_string()),
        )
        .expect("branch-a should be created");
    let lowered = effect_batch()
        .using_basis(EffectAuthoringBasis::from(branch_mutation_basis()))
        .push(raw_mutation_effect_with_binding(
            runtime_workflow_binding_for_branch(
                branch_snapshot_identity(&runtime, "branch-a"),
                "branch-a",
            ),
            entity_id,
            native_name_patch("stale-batch"),
        ))
        .admit()
        .expect("batch should admit")
        .lower()
        .expect("batch should lower");

    let intervening_commit_id = update_entity_name(
        &mut runtime,
        entity_id,
        "intervening",
        BranchId("branch-a".to_string()),
    );

    let denial = lowered
        .execute_with(EffectExecutionAuthority::relational(&mut runtime))
        .expect_err("retained lowered batch should deny stale exact-basis replay");

    assert_eq!(
        denial.kind(),
        &EffectBatchExecutionDenialKind::AggregateExecutionDenied(
            EffectExecutionDenialKind::RelationalExactBasisStale,
        )
    );
    assert_eq!(
        runtime
            .history()
            .branch_head(&BranchId("branch-a".to_string()))
            .expect("intervening branch head should remain authoritative")
            .commit_id,
        intervening_commit_id
    );
}

#[test]
fn lowered_branch_batch_does_not_deny_when_only_another_branch_moves() {
    let mut runtime = relational_runtime_with_intent_strategy();
    let entity_id = create_entity(&mut runtime, "before", BranchId("main".to_string()));
    runtime
        .history_authority()
        .create_branch(
            BranchId("branch-a".to_string()),
            &BranchId("main".to_string()),
        )
        .expect("branch-a should be created");
    let lowered = effect_batch()
        .using_basis(EffectAuthoringBasis::from(branch_mutation_basis()))
        .push(raw_mutation_effect_with_binding(
            runtime_workflow_binding_for_branch(
                branch_snapshot_identity(&runtime, "branch-a"),
                "branch-a",
            ),
            entity_id,
            native_name_patch("branch-batched"),
        ))
        .admit()
        .expect("batch should admit")
        .lower()
        .expect("batch should lower");

    let main_head_before = update_entity_name(
        &mut runtime,
        entity_id,
        "main-only-intervening",
        BranchId("main".to_string()),
    );

    let executed = lowered
        .execute_with(EffectExecutionAuthority::relational(&mut runtime))
        .expect("unrelated main-branch movement should not stale-deny branch-a batch");

    let aggregate = executed
        .aggregate_mutation()
        .expect("batch should retain one aggregate mutation commit");
    assert_ne!(aggregate.outcome.commit.commit_id, main_head_before);
    assert_eq!(aggregate.outcome.commit.parents.len(), 1);
}

fn entity_name(record: &worth_relational::facade::runtime::EntityReadRecord) -> Option<String> {
    let state = record.authoritative_aspect_state.as_ref()?;
    let value = state.get(&aspect_key("name").ok()?)?;
    match value.view() {
        worth_foundational::facade::ContractValidatedAspectValueView::Scalar(
            worth_foundational::facade::AspectValue::String(
                worth_foundational::facade::InternedString::Raw(actual),
            ),
        ) => Some(actual.clone()),
        _ => None,
    }
}
