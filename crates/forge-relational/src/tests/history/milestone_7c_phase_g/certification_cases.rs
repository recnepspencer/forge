use crate::facade::history::BranchId;
use crate::facade::identity::{KindId, PartitionId};
use crate::facade::merge::{MergeExecutionRequest, MergeIntent};
use crate::facade::transactions::{
    CreateIntent, MutationIntent, TransactionOptions, WorkerIntentBatch,
};
use crate::tests::support::{
    create_branch_from_main, create_entity, create_entity_outcome_on_branch,
    persisted_runtime_with_test_schema, read_entity_field, read_entity_name,
    unique_test_store_path, update_entity, update_entity_on_branch,
};

use super::artifacts::MergeExecutionCertificationArtifacts;
use super::recovery_certification::certify_merge_execution_with_recovery;
use super::schema_fixtures::{persisted_runtime_with_registry, prefer_richer_registry};

pub(super) fn certify_exact_shared_merge_execution() -> MergeExecutionCertificationArtifacts {
    let mut runtime = persisted_runtime_with_test_schema();
    let shared = create_entity(&mut runtime, "shared");
    create_branch_from_main(&mut runtime, "feature");
    update_entity(&mut runtime, shared, "same");
    update_entity_on_branch(
        &mut runtime,
        shared,
        "same",
        BranchId("feature".to_string()),
    );

    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared exact-shared merge");
    let merge = runtime
        .execute_prepared_merge(prepared)
        .expect("executed exact-shared merge");

    assert_eq!(merge.structural_summary.preserved_shared_record_count, 1);
    assert_eq!(merge.structural_summary.emitted_mutation_intent_count, 0);

    certify_merge_execution_with_recovery(&mut runtime, &merge, persisted_runtime_with_test_schema)
}

pub(super) fn certify_source_only_addition_merge_execution() -> MergeExecutionCertificationArtifacts
{
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    create_entity_outcome_on_branch(
        &mut runtime,
        "feature-only",
        BranchId("feature".to_string()),
    );

    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared source-only merge");
    let merge = runtime
        .execute_prepared_merge(prepared)
        .expect("executed source-only merge");

    assert_eq!(merge.structural_summary.adopted_source_record_count, 1);
    assert_eq!(merge.structural_summary.emitted_entity_create_count, 1);

    certify_merge_execution_with_recovery(&mut runtime, &merge, persisted_runtime_with_test_schema)
}

pub(super) fn certify_prefer_richer_merge_execution() -> MergeExecutionCertificationArtifacts {
    let store_path = unique_test_store_path("forge-relational-7c-phase-g");
    let registry = prefer_richer_registry();
    let mut runtime = persisted_runtime_with_registry(registry.clone(), store_path.clone());

    let main_entity = create_entity(&mut runtime, "shared-name");
    create_branch_from_main(&mut runtime, "feature");

    let mut feature_txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(BranchId("feature".to_string())),
        ..TransactionOptions::default()
    });
    feature_txn.push_batch(
        WorkerIntentBatch::new("feature-seed").push(MutationIntent::Create(CreateIntent::Entity(
            crate::transactions::data::EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: crate::symbols::data::ClientKey::raw("feature-shared"),
                fields: crate::tests::support::string_aspect_field_patch([
                    (
                        crate::tests::support::aspect_key("name"),
                        crate::tests::support::field_key("name"),
                        "shared-name",
                    ),
                    (
                        crate::tests::support::aspect_key("status"),
                        crate::tests::support::field_key("status"),
                        "active",
                    ),
                ]),
            },
        ))),
    );
    feature_txn.commit().expect("feature branch seed");

    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared prefer-richer merge");
    let merge = runtime
        .execute_prepared_merge(prepared)
        .expect("executed prefer-richer merge");

    assert_eq!(merge.structural_summary.reconciled_record_count, 1);
    assert_eq!(merge.structural_summary.emitted_entity_update_count, 1);
    let current = runtime
        .read_truth()
        .read_snapshot(&merge.commit.snapshot)
        .expect("current merge snapshot");
    let current_record = current
        .get_entity(main_entity)
        .expect("merged target entity remains visible");
    assert_eq!(read_entity_name(current_record), Some("shared-name".into()));
    assert_eq!(
        read_entity_field(current_record, crate::tests::support::field_key("status")),
        Some("active".into())
    );

    certify_merge_execution_with_recovery(&mut runtime, &merge, move || {
        persisted_runtime_with_registry(registry.clone(), store_path.clone())
    })
}
