use crate::facade::storage::RecordLifecycleState;
use crate::tests::support::*;

#[test]
fn relation_integrity_commit_boundary_rejects_endpoint_delete_with_live_relations() {
    let mut runtime = endpoint_deletion_runtime(
        crate::schema::data::EndpointDeletionIntegrityMode::RejectDeleteWithLiveRelations,
        CascadeDeletePolicy::RetainDanglingForAudit,
    );
    let (source, _target, _relation) =
        create_endpoint_deletion_relation_fixture(&mut runtime, "live");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("delete-source").push(MutationIntent::Entity(
            EntityMutationIntent::Delete(DeleteEntityIntent { entity_id: source }),
        )),
    );

    let error = txn.commit().unwrap_err();
    match error {
        TransactionCommitError::Conflict { error, .. } => {
            assert_eq!(
                error.code(),
                DiagnosticCode::RelationEndpointDeletionIntegrityViolation
            );
            match error.class {
                crate::transactions::data::ConflictClass::InvariantViolation {
                    fields:
                        crate::validation::data::InvariantViolationFields::RelationEndpointDeletionIntegrity {
                            contract_id,
                            relation_kind_id,
                            entity_id,
                            mode,
                            ..
                        },
                    ..
                } => {
                    assert_eq!(contract_id.as_str(), "endpoint_delete");
                    assert_eq!(relation_kind_id, KindId(2));
                    assert_eq!(entity_id, source);
                    assert_eq!(
                        mode,
                        crate::schema::data::EndpointDeletionIntegrityMode::RejectDeleteWithLiveRelations
                    );
                }
                other => {
                    panic!("expected typed endpoint deletion invariant conflict, got {other:?}")
                }
            }
        }
        other => panic!("expected conflict, got {:?}", other),
    }
}

#[test]
fn relation_integrity_commit_boundary_rejects_replace_when_retained_relation_keeps_live_endpoint_dependency(
) {
    let mut runtime = endpoint_deletion_runtime(
        crate::schema::data::EndpointDeletionIntegrityMode::RejectDeleteWithLiveRelations,
        CascadeDeletePolicy::RetainDanglingForAudit,
    );
    let (source, _target, _relation) =
        create_endpoint_deletion_relation_fixture(&mut runtime, "live");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("replace-source").push(MutationIntent::Entity(
            EntityMutationIntent::Replace(ReplaceEntityIntent {
                entity_id: source,
                replacement: crate::transactions::data::EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(1),
                    client_key: crate::symbols::data::ClientKey::raw("source-replacement"),
                    fields: crate::tests::support::single_string_aspect_field_patch(
                        crate::tests::support::aspect_key("name"),
                        crate::tests::support::field_key("name"),
                        "source-replacement",
                    ),
                },
            }),
        )),
    );

    let error = txn.commit().unwrap_err();
    match error {
        TransactionCommitError::Conflict { error, .. } => {
            assert_eq!(
                error.code(),
                DiagnosticCode::RelationEndpointDeletionIntegrityViolation
            );
        }
        other => panic!("expected conflict, got {:?}", other),
    }
}

#[test]
fn relation_integrity_commit_boundary_requires_relation_deletion_in_same_commit_under_retain_policy(
) {
    let mut runtime = endpoint_deletion_runtime(
        crate::schema::data::EndpointDeletionIntegrityMode::RequireRelationDeletionInSameCommit,
        CascadeDeletePolicy::RetainDanglingForAudit,
    );
    let (source, _target, _relation) =
        create_endpoint_deletion_relation_fixture(&mut runtime, "live");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("delete-source").push(MutationIntent::Entity(
            EntityMutationIntent::Delete(DeleteEntityIntent { entity_id: source }),
        )),
    );

    let error = txn.commit().unwrap_err();
    match error {
        TransactionCommitError::Conflict { error, .. } => {
            assert_eq!(
                error.code(),
                DiagnosticCode::RelationEndpointDeletionIntegrityViolation
            );
            assert!(error
                .detail()
                .contains("requires deleting dependent relations in the same commit"));
        }
        other => panic!("expected conflict, got {:?}", other),
    }
}

#[test]
fn relation_integrity_commit_boundary_allows_relation_deletion_in_same_commit_under_cascade_policy()
{
    let mut runtime = endpoint_deletion_runtime(
        crate::schema::data::EndpointDeletionIntegrityMode::RequireRelationDeletionInSameCommit,
        CascadeDeletePolicy::CascadeDeleteRelations,
    );
    let (source, _target, relation) =
        create_endpoint_deletion_relation_fixture(&mut runtime, "live");

    let deleted = delete_entity(&mut runtime, source);
    let read = runtime
        .read_truth()
        .read_snapshot(&deleted.snapshot)
        .unwrap();

    assert!(read.get_relation(relation).is_none());
}

#[test]
fn relation_integrity_commit_boundary_allows_relation_retirement_when_policy_retains_for_audit() {
    let mut runtime = endpoint_deletion_runtime(
        crate::schema::data::EndpointDeletionIntegrityMode::RequireRelationRetirement,
        CascadeDeletePolicy::RetainDanglingForAudit,
    );
    let (source, _target, relation) =
        create_endpoint_deletion_relation_fixture(&mut runtime, "live");

    let deleted = delete_entity(&mut runtime, source);
    let read = runtime
        .read_truth()
        .read_snapshot(&deleted.snapshot)
        .unwrap();
    let relation = read.get_relation(relation).unwrap();

    assert_eq!(
        relation.lifecycle,
        RecordLifecycleState::RetainedDanglingForAudit
    );
}

#[test]
fn relation_integrity_commit_boundary_rejects_relation_retirement_under_cascade_policy() {
    let mut runtime = endpoint_deletion_runtime(
        crate::schema::data::EndpointDeletionIntegrityMode::RequireRelationRetirement,
        CascadeDeletePolicy::CascadeDeleteRelations,
    );
    let (source, _target, _relation) =
        create_endpoint_deletion_relation_fixture(&mut runtime, "live");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("delete-source").push(MutationIntent::Entity(
            EntityMutationIntent::Delete(DeleteEntityIntent { entity_id: source }),
        )),
    );

    let error = txn.commit().unwrap_err();
    match error {
        TransactionCommitError::Conflict { error, .. } => {
            assert_eq!(
                error.code(),
                DiagnosticCode::RelationEndpointDeletionIntegrityViolation
            );
            assert!(error
                .detail()
                .contains("requires audit-retained relation retirement"));
        }
        other => panic!("expected conflict, got {:?}", other),
    }
}

#[test]
fn relation_integrity_commit_boundary_allows_opposite_endpoint_delete_after_relation_retirement() {
    let mut runtime = endpoint_deletion_runtime(
        crate::schema::data::EndpointDeletionIntegrityMode::RequireRelationRetirement,
        CascadeDeletePolicy::RetainDanglingForAudit,
    );
    let (source, target, relation) =
        create_endpoint_deletion_relation_fixture(&mut runtime, "live");

    delete_entity(&mut runtime, source);
    let deleted_target = delete_entity(&mut runtime, target);
    let read = runtime
        .read_truth()
        .read_snapshot(&deleted_target.snapshot)
        .unwrap();
    let relation = read.get_relation(relation).unwrap();

    assert_eq!(
        relation.lifecycle,
        RecordLifecycleState::RetainedDanglingForAudit
    );
}

#[test]
fn relation_integrity_endpoint_deletion_history_stays_branch_local_under_divergence() {
    let mut runtime = endpoint_deletion_runtime(
        crate::schema::data::EndpointDeletionIntegrityMode::RequireRelationRetirement,
        CascadeDeletePolicy::RetainDanglingForAudit,
    );
    let (source, target, relation) =
        create_endpoint_deletion_relation_fixture(&mut runtime, "live");

    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();

    let main_delete = delete_entity(&mut runtime, source);
    let _feature_update = update_entity_on_branch(
        &mut runtime,
        target,
        "feature-target",
        BranchId("feature".to_string()),
    );

    let main_digest = relation_aspect_history_digest_on_branch(
        &runtime,
        &BranchId("main".to_string()),
        relation,
        None,
    );
    let feature_digest = relation_aspect_history_digest_on_branch(
        &runtime,
        &BranchId("feature".to_string()),
        relation,
        None,
    );
    let main_head_version = runtime
        .history()
        .branch_head(&BranchId("main".to_string()))
        .unwrap()
        .version_id;
    let feature_head_version = runtime
        .history()
        .branch_head(&BranchId("feature".to_string()))
        .unwrap()
        .version_id;
    let main_inspection = runtime.inspect_what_happened().inspect_historical_record(
        &BranchId("main".to_string()),
        main_head_version,
        RecordRef::Relation(relation),
        crate::facade::inspection::HistoricalInspectionMode::RetainedOnly,
    );
    let feature_inspection = runtime.inspect_what_happened().inspect_historical_record(
        &BranchId("feature".to_string()),
        feature_head_version,
        RecordRef::Relation(relation),
        crate::facade::inspection::HistoricalInspectionMode::RetainedOnly,
    );

    assert_eq!(main_delete.version_id, main_head_version);
    assert_eq!(main_digest.entry_count, 2);
    assert!(feature_digest.entry_count < main_digest.entry_count);
    assert_eq!(
        main_inspection
            .aspect_history_observation
            .as_ref()
            .map(|observation| observation.query_result.trace.branch_id.clone()),
        Some(BranchId("main".to_string()))
    );
    assert_eq!(
        feature_inspection
            .aspect_history_observation
            .as_ref()
            .map(|observation| observation.query_result.trace.branch_id.clone()),
        Some(BranchId("feature".to_string()))
    );
    let main_read = runtime
        .read_truth()
        .read_snapshot(&main_delete.snapshot)
        .unwrap();
    assert_eq!(
        main_read.get_relation(relation).unwrap().lifecycle,
        RecordLifecycleState::RetainedDanglingForAudit
    );
}
