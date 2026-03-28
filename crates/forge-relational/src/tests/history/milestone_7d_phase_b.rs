use crate::facade::history::BranchId;
use crate::facade::merge::{
    DeletionExecutionClass, LoweredMergeBlockedReason, LoweredRecordDenialKind,
    MergeExecutableClass, MergeExecutionCompilationError, MergeExecutionRequest,
    MergeIntent, MergeResolutionClass, TopologyExecutionClass,
};
use crate::facade::transactions::RecordRef;
use crate::tests::support::{
    create_branch_from_main, create_entity, delete_entity, delete_entity_on_branch,
    persisted_runtime_with_test_schema, update_entity, update_entity_on_branch,
};

#[test]
fn lowered_plan_preserves_source_deleted_target_live_block_reason() {
    let mut runtime = persisted_runtime_with_test_schema();
    let entity = create_entity(&mut runtime, "shared");
    create_branch_from_main(&mut runtime, "feature");
    delete_entity_on_branch(&mut runtime, entity, BranchId("feature".to_string()));

    let artifact = runtime
        .merge_access()
        .inspect_planning_scope(
            MergeExecutionRequest {
                target_branch: BranchId("main".to_string()),
                source_branch: BranchId("feature".to_string()),
                merge_intent: MergeIntent::ReconcileIntoTarget,
            }
            .into(),
        )
        .expect("planning artifact");

    let lowered = artifact
        .lowered_plan
        .records
        .iter()
        .find(|record| record.record == RecordRef::Entity(entity))
        .expect("lowered record");

    assert_eq!(
        lowered.blocked_reason,
        Some(LoweredMergeBlockedReason::SourceDeletedTargetLive)
    );
    assert_eq!(
        lowered.resolution_class,
        MergeResolutionClass::Deletion(DeletionExecutionClass::SourceDeletedTargetLive)
    );
    assert_eq!(lowered.executable_class, None);
    let lowered_index = artifact
        .lowered_plan
        .records
        .iter()
        .position(|record| record.record == RecordRef::Entity(entity))
        .expect("lowered record index");
    assert_eq!(
        artifact.digest_basis.lowered_plan.blocked_reasons[lowered_index],
        Some(LoweredMergeBlockedReason::SourceDeletedTargetLive)
    );
    assert_eq!(
        artifact.digest_basis.lowered_plan.resolution_classes[lowered_index],
        MergeResolutionClass::Deletion(DeletionExecutionClass::SourceDeletedTargetLive)
    );
    assert_eq!(
        artifact.digest_basis.lowered_plan.executable_classes[lowered_index],
        None
    );
    assert_eq!(
        artifact.digest_basis.lowered_plan.denial_bundle_kinds[lowered_index],
        Some(LoweredRecordDenialKind::BlockedSourceDeletedTargetLive)
    );
}

#[test]
fn lowered_plan_preserves_source_live_target_deleted_block_reason() {
    let mut runtime = persisted_runtime_with_test_schema();
    let entity = create_entity(&mut runtime, "shared");
    create_branch_from_main(&mut runtime, "feature");
    update_entity_on_branch(
        &mut runtime,
        entity,
        "shared",
        BranchId("feature".to_string()),
    );
    delete_entity(&mut runtime, entity);

    let artifact = runtime
        .merge_access()
        .inspect_planning_scope(
            MergeExecutionRequest {
                target_branch: BranchId("main".to_string()),
                source_branch: BranchId("feature".to_string()),
                merge_intent: MergeIntent::ReconcileIntoTarget,
            }
            .into(),
        )
        .expect("planning artifact");

    let lowered = artifact
        .lowered_plan
        .records
        .iter()
        .find(|record| record.record == RecordRef::Entity(entity))
        .expect("lowered record");

    assert_eq!(
        lowered.blocked_reason,
        Some(LoweredMergeBlockedReason::SourceLiveTargetDeleted)
    );
    assert_eq!(
        lowered.resolution_class,
        MergeResolutionClass::Deletion(DeletionExecutionClass::SourceLiveTargetDeleted)
    );
    assert_eq!(lowered.executable_class, None);
    let lowered_index = artifact
        .lowered_plan
        .records
        .iter()
        .position(|record| record.record == RecordRef::Entity(entity))
        .expect("lowered record index");
    assert_eq!(
        artifact.digest_basis.lowered_plan.blocked_reasons[lowered_index],
        Some(LoweredMergeBlockedReason::SourceLiveTargetDeleted)
    );
    assert_eq!(
        artifact.digest_basis.lowered_plan.resolution_classes[lowered_index],
        MergeResolutionClass::Deletion(DeletionExecutionClass::SourceLiveTargetDeleted)
    );
    assert_eq!(
        artifact.digest_basis.lowered_plan.executable_classes[lowered_index],
        None
    );
    assert_eq!(
        artifact.digest_basis.lowered_plan.denial_bundle_kinds[lowered_index],
        Some(LoweredRecordDenialKind::BlockedSourceLiveTargetDeleted)
    );
}

#[test]
fn lowered_plan_preserves_deleted_vs_modified_block_reason() {
    let mut runtime = persisted_runtime_with_test_schema();
    let entity = create_entity(&mut runtime, "shared");
    create_branch_from_main(&mut runtime, "feature");
    update_entity(&mut runtime, entity, "main-modified");
    delete_entity_on_branch(&mut runtime, entity, BranchId("feature".to_string()));

    let artifact = runtime
        .merge_access()
        .inspect_planning_scope(
            MergeExecutionRequest {
                target_branch: BranchId("main".to_string()),
                source_branch: BranchId("feature".to_string()),
                merge_intent: MergeIntent::ReconcileIntoTarget,
            }
            .into(),
        )
        .expect("planning artifact");

    let lowered = artifact
        .lowered_plan
        .records
        .iter()
        .find(|record| record.record == RecordRef::Entity(entity))
        .expect("lowered record");

    assert_eq!(
        lowered.blocked_reason,
        Some(LoweredMergeBlockedReason::DeletedVsModified)
    );
    assert_eq!(
        lowered.resolution_class,
        MergeResolutionClass::Deletion(DeletionExecutionClass::DeletedVsModified)
    );
    assert_eq!(lowered.executable_class, None);
    let lowered_index = artifact
        .lowered_plan
        .records
        .iter()
        .position(|record| record.record == RecordRef::Entity(entity))
        .expect("lowered record index");
    assert_eq!(
        artifact.digest_basis.lowered_plan.blocked_reasons[lowered_index],
        Some(LoweredMergeBlockedReason::DeletedVsModified)
    );
    assert_eq!(
        artifact.digest_basis.lowered_plan.resolution_classes[lowered_index],
        MergeResolutionClass::Deletion(DeletionExecutionClass::DeletedVsModified)
    );
    assert_eq!(
        artifact.digest_basis.lowered_plan.executable_classes[lowered_index],
        None
    );
    assert_eq!(
        artifact.digest_basis.lowered_plan.denial_bundle_kinds[lowered_index],
        Some(LoweredRecordDenialKind::BlockedDeletedVsModified)
    );
}

#[test]
fn admitted_source_addition_carries_executable_class() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    let feature_only =
        crate::tests::support::create_entity_outcome_on_branch(
            &mut runtime,
            "feature-only",
            BranchId("feature".to_string()),
        );
    let entity = crate::tests::support::changed_entities(&feature_only)[0];

    let artifact = runtime
        .merge_access()
        .inspect_planning_scope(
            MergeExecutionRequest {
                target_branch: BranchId("main".to_string()),
                source_branch: BranchId("feature".to_string()),
                merge_intent: MergeIntent::ReconcileIntoTarget,
            }
            .into(),
        )
        .expect("planning artifact");

    let lowered = artifact
        .lowered_plan
        .records
        .iter()
        .find(|record| record.record == RecordRef::Entity(entity))
        .expect("lowered record");

    assert_eq!(lowered.resolution_class, MergeResolutionClass::SourceOnlyAddition);
    assert_eq!(
        lowered.executable_class,
        Some(MergeExecutableClass::AdoptSourceRecord)
    );
}

#[test]
fn compile_rejects_corrupted_non_executable_resolution_class() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    crate::tests::support::create_entity_outcome_on_branch(
        &mut runtime,
        "feature-only",
        BranchId("feature".to_string()),
    );

    let mut prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared merge");

    let execution_ready = prepared.execution_ready_plan_mut_for_test();
    let lowered = std::sync::Arc::make_mut(&mut execution_ready.lowered_records);
    lowered[0].resolution_class =
        MergeResolutionClass::Topology(TopologyExecutionClass::RelationEndpointDivergence);
    lowered[0].executable_class = None;

    match runtime
        .merge_access()
        .compile_execution_ready_merge_plan_for_test(execution_ready)
    {
        Err(MergeExecutionCompilationError::MissingExecutableClass { .. }) => {}
        other => panic!("expected missing executable class rejection, got {other:?}"),
    }
}
