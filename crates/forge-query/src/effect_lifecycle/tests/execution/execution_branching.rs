use forge_relational::facade::commit_strategies::{
    CommitStrategyId, CommitStrategyRegistration, IntentReconciliationStrategy,
};
use forge_relational::facade::history::BranchId;
use forge_relational::facade::runtime::{RelationalRuntime, RelationalRuntimeApi};
use forge_relational::facade::schema::{
    EntityKindRegistration, KindAspectDeclarations, RelationalSchemaRegistry, SchemaId,
    SchemaVersionId,
};
use forge_relational::facade::transactions::{
    CreateIntent, EntityMutationIntent, EntitySpec, MutationIntent, RecordRef, TransactionOptions,
    UpdateEntityFieldsIntent, WorkerIntentBatch,
};
use forge_relational::facade::{identity::KindId, identity::PartitionId, symbols::ClientKey};
use serde_json::json;

use crate::aspect_field_authoring::{
    aspect_key, entity_string_field_aspect, lifecycle_string_aspect,
    single_aspect_field_patch_from_external_json,
};
use crate::effect_lifecycle::{
    scope_admitted_effect_plan, EffectExecutionAuthority, EffectExecutionDenialKind,
};

use super::execution::{branch_snapshot_token, runtime_snapshot_token};
use super::support::{
    admitted_mutation_effect_for_entity_with_binding, runtime_workflow_binding_with_snapshot,
};

#[test]
fn lowered_mutation_execution_preserves_branch_scoped_authority_target() {
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
    let lowered = scope_admitted_effect_plan(admitted_mutation_effect_for_entity_with_binding(
        runtime_workflow_binding_with_snapshot(&branch_snapshot_token(&runtime, "branch-a")),
        entity_id,
        json!({ "name": "authority-plan" }),
    ))
    .lower()
    .expect("mutation should lower");

    let executed = lowered
        .execute_with(EffectExecutionAuthority::relational(&mut runtime))
        .expect("lowered mutation should execute");

    let commit = executed
        .as_mutation()
        .expect("mutation artifact should be present");
    assert_eq!(commit.outcome.commit.parents, vec![branch_head_before]);
    assert_ne!(commit.outcome.commit.parents, vec![main_head_before]);
    assert_eq!(
        runtime
            .history()
            .branch_head(&BranchId("main".to_string()))
            .expect("main head should remain present")
            .commit_id,
        main_head_before
    );
    assert_eq!(
        runtime
            .history()
            .branch_head(&BranchId("branch-a".to_string()))
            .expect("branch-a head should advance")
            .commit_id,
        commit.outcome.commit.commit_id
    );
    let snapshot = runtime.snapshots().snapshot();
    let read_view = runtime
        .read_truth()
        .read_snapshot(&snapshot)
        .expect("snapshot should read");
    let updated = read_view
        .entities()
        .iter()
        .find(|record| record.entity_id == entity_id)
        .expect("entity should still exist after execution");
    assert_entity_name(updated, "authority-plan");
}

#[test]
fn retained_lowered_mutation_denies_after_intervening_truth_change() {
    let mut runtime = relational_runtime_with_intent_strategy();
    let entity_id = create_entity(&mut runtime, "before", BranchId("main".to_string()));
    runtime
        .history_authority()
        .create_branch(
            BranchId("branch-a".to_string()),
            &BranchId("main".to_string()),
        )
        .expect("branch-a should be created");
    let lowered = scope_admitted_effect_plan(admitted_mutation_effect_for_entity_with_binding(
        runtime_workflow_binding_with_snapshot(&runtime_snapshot_token(&runtime)),
        entity_id,
        json!({ "name": "authority-plan" }),
    ))
    .lower()
    .expect("mutation should lower");

    let branch_head_before = runtime
        .history()
        .branch_head(&BranchId("branch-a".to_string()))
        .expect("branch-a head should exist")
        .commit_id;
    update_entity_name(
        &mut runtime,
        entity_id,
        "intervening",
        BranchId("branch-a".to_string()),
    );

    let denial = lowered
        .execute_with(EffectExecutionAuthority::relational(&mut runtime))
        .expect_err("retained lowered mutation should deny stale exact-basis replay");

    assert_eq!(
        denial.denial_kind(),
        EffectExecutionDenialKind::RelationalExactBasisStale
    );
    assert_eq!(denial.counters().execution_denied_count(), 1);
    assert_eq!(
        runtime
            .history()
            .branch_head(&BranchId("branch-a".to_string()))
            .expect("stale denial should preserve intervening branch head")
            .commit_id,
        runtime.history().latest_commit().unwrap().commit_id
    );
    assert_ne!(
        runtime
            .history()
            .branch_head(&BranchId("branch-a".to_string()))
            .expect("branch-a head should remain advanced by intervening commit")
            .commit_id,
        branch_head_before
    );
    let snapshot = runtime.snapshots().snapshot();
    let read_view = runtime
        .read_truth()
        .read_snapshot(&snapshot)
        .expect("snapshot should read");
    let updated = read_view
        .entities()
        .iter()
        .find(|record| record.entity_id == entity_id)
        .expect("entity should still exist after stale denial");
    assert_entity_name(updated, "intervening");
}

#[test]
fn lowered_branch_mutation_does_not_deny_when_only_another_branch_moves() {
    let mut runtime = relational_runtime_with_intent_strategy();
    let entity_id = create_entity(&mut runtime, "before", BranchId("main".to_string()));
    runtime
        .history_authority()
        .create_branch(
            BranchId("branch-a".to_string()),
            &BranchId("main".to_string()),
        )
        .expect("branch-a should be created");
    let lowered = scope_admitted_effect_plan(admitted_mutation_effect_for_entity_with_binding(
        runtime_workflow_binding_with_snapshot(&runtime_snapshot_token(&runtime)),
        entity_id,
        json!({ "name": "authority-plan" }),
    ))
    .lower()
    .expect("mutation should lower");

    let main_head_before = update_entity_name(
        &mut runtime,
        entity_id,
        "main-only-intervening",
        BranchId("main".to_string()),
    );

    let executed = lowered
        .execute_with(EffectExecutionAuthority::relational(&mut runtime))
        .expect("unrelated main-branch movement should not stale-deny branch-a mutation");

    let commit = executed
        .as_mutation()
        .expect("mutation artifact should be present");
    assert_ne!(commit.outcome.commit.commit_id, main_head_before);
    assert_eq!(commit.outcome.commit.parents.len(), 1);
    assert_eq!(
        runtime
            .history()
            .branch_head(&BranchId("main".to_string()))
            .expect("main branch should retain intervening head")
            .commit_id,
        main_head_before
    );
}

fn relational_runtime_with_intent_strategy() -> RelationalRuntime {
    let descriptor = IntentReconciliationStrategy::descriptor(CommitStrategyId(311));
    RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .commit_strategy(
            CommitStrategyRegistration::new(descriptor.clone()).expect("strategy registration"),
        )
        .commit_strategy_executor(IntentReconciliationStrategy::execution_registration(
            &descriptor,
        ))
        .build()
}

fn create_entity(
    runtime: &mut RelationalRuntime,
    name: &str,
    branch: BranchId,
) -> forge_relational::facade::identity::EntityId {
    let mut txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch),
        ..TransactionOptions::default()
    });
    txn.push_batch(
        WorkerIntentBatch::new(format!("create-{name}")).push(MutationIntent::Create(
            CreateIntent::Entity(EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: ClientKey::raw(name),
                fields: single_aspect_field_patch_from_external_json("name", "name", json!(name))
                    .expect("entity name aspect patch"),
            }),
        )),
    );
    let outcome = txn.commit().expect("seed commit should succeed");
    outcome
        .changed_records
        .iter()
        .find_map(|record| match record {
            RecordRef::Entity(entity_id) => Some(*entity_id),
            RecordRef::Relation(_) => None,
        })
        .expect("seed commit should touch one entity")
}

fn update_entity_name(
    runtime: &mut RelationalRuntime,
    entity_id: forge_relational::facade::identity::EntityId,
    name: &str,
    branch: BranchId,
) -> forge_relational::facade::history::CommitId {
    let mut txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch),
        ..TransactionOptions::default()
    });
    txn.push_batch(
        WorkerIntentBatch::new(format!("update-{name}")).push(MutationIntent::Entity(
            EntityMutationIntent::UpdateFields(UpdateEntityFieldsIntent {
                entity_id,
                fields: single_aspect_field_patch_from_external_json("name", "name", json!(name))
                    .expect("entity name aspect patch"),
            }),
        )),
    );
    txn.commit()
        .expect("intervening update should succeed")
        .outcome
        .commit
        .commit_id
}

fn test_schema_registry() -> RelationalSchemaRegistry {
    RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: KindAspectDeclarations::new(vec![
                entity_string_field_aspect("name", "name").expect("name aspect"),
                lifecycle_string_aspect("lifecycle").expect("lifecycle aspect"),
            ]),
        })
        .expect("test entity kind should register")
}

fn assert_entity_name(
    record: &forge_relational::facade::runtime::EntityReadRecord,
    expected_name: &str,
) {
    let state = record
        .authoritative_aspect_state
        .as_ref()
        .expect("entity should carry authoritative aspect state");
    let value = state
        .get(&aspect_key("name").expect("valid aspect key"))
        .expect("name aspect should be present");
    match value.view() {
        forge_foundational::facade::ContractValidatedAspectValueView::Scalar(
            forge_foundational::facade::AspectValue::String(
                forge_foundational::facade::InternedString::Raw(actual),
            ),
        ) => assert_eq!(actual, expected_name),
        other => panic!("expected scalar string name aspect, got {other:?}"),
    }
}
