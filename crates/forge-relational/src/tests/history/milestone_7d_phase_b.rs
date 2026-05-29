use crate::facade::history::BranchId;
use crate::facade::merge::{
    DeletionExecutionClass, LoweredMergeBlockedReason, LoweredRecordDenialKind,
    MergeExecutableClass, MergeExecutionCompilationError, MergeExecutionRequest, MergeIntent,
    MergeManualResolutionClass, MergePolicyDecisionBoundary, MergePolicyOwnershipSurface,
    MergePolicyRejectClass, MergeResolutionClass, TopologyExecutionClass,
};
use crate::facade::runtime::RelationalRuntimeApi;
use crate::facade::transactions::RecordRef;
use crate::merge::data::AspectMergePolicyDeclaration;
use crate::publication::patch::data::AspectKey;
use crate::schema::data::{
    EntityKindRegistration, KindAspectDeclarations, RelationKindRegistration, SchemaId,
    SchemaVersionId,
};
use crate::tests::support::{
    create_branch_from_main, create_entity, delete_entity, delete_entity_on_branch,
    entity_field_aspect, persisted_runtime_with_test_schema, update_entity,
    update_entity_on_branch,
};
use crate::{
    config::data::{CascadeDeletePolicy, CrossContextPolicy},
    facade::identity::KindId,
    facade::merge::{
        AspectMergePolicyKind, IdentityBasisDeclaration, IdentityBasisKind, IdentityBasisScope,
    },
    schema::data::RelationalSchemaRegistry,
};

fn runtime_with_name_merge_policy(
    merge_policy: AspectMergePolicyKind,
) -> crate::facade::runtime::RelationalRuntime {
    let name_key = AspectKey::new("name").unwrap();
    let registry = RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: KindAspectDeclarations::new(vec![entity_field_aspect(
                name_key.as_str(),
                "name",
            )])
            .with_identity_declarations(vec![IdentityBasisDeclaration {
                scope: IdentityBasisScope::AspectKey(name_key.clone()),
                basis: IdentityBasisKind::DeclaredKeySet(vec![name_key.clone()].into()),
            }])
            .with_merge_policy_declarations(vec![AspectMergePolicyDeclaration {
                aspect_key: name_key,
                policy: merge_policy,
            }]),
        })
        .and_then(|registry: RelationalSchemaRegistry| {
            registry.register_relation_kind(RelationKindRegistration {
                kind_id: KindId(2),
                kind_name: "test.relation".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                aspect_declarations: KindAspectDeclarations::default(),
                relation_integrity: crate::schema::data::RelationIntegrityDeclarations::default(),
            })
        })
        .expect("schema registry");
    RelationalRuntimeApi::builder()
        .schema_registry(registry)
        .build()
}

#[test]
fn lowered_plan_preserves_source_deleted_target_live_block_reason() {
    let mut runtime = persisted_runtime_with_test_schema();
    let entity = create_entity(&mut runtime, "shared");
    create_branch_from_main(&mut runtime, "feature");
    delete_entity_on_branch(&mut runtime, entity, BranchId("feature".to_string()));

    let artifact = runtime
        .merge()
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
        .merge()
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
        .merge()
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
fn lowered_plan_carries_explicit_manual_resolution_policy_boundary_for_generic_denial() {
    let mut runtime = persisted_runtime_with_test_schema();
    let entity = create_entity(&mut runtime, "shared");
    create_branch_from_main(&mut runtime, "feature");
    delete_entity_on_branch(&mut runtime, entity, BranchId("feature".to_string()));

    let artifact = runtime
        .merge()
        .inspect_planning_scope(
            MergeExecutionRequest {
                target_branch: BranchId("main".to_string()),
                source_branch: BranchId("feature".to_string()),
                merge_intent: MergeIntent::ReconcileIntoTarget,
            }
            .into(),
        )
        .expect("planning artifact");

    let lowered_index = artifact
        .lowered_plan
        .records
        .iter()
        .position(|record| record.record == RecordRef::Entity(entity))
        .expect("lowered record index");
    let lowered = &artifact.lowered_plan.records[lowered_index];

    assert_eq!(
        lowered.policy_proof_boundary.ownership_surface,
        MergePolicyOwnershipSurface::RuntimeOnly
    );
    assert_eq!(
        lowered.policy_proof_boundary.decision_boundary,
        MergePolicyDecisionBoundary::RequiresManualResolution {
            class: MergeManualResolutionClass::GenericRuntimeConflict,
        }
    );
    assert_eq!(
        artifact.digest_basis.policy.proof_boundaries[lowered_index],
        lowered.policy_proof_boundary
    );
}

#[test]
fn lowered_plan_carries_explicit_hard_reject_policy_boundary_for_fail_on_conflict() {
    let mut runtime = runtime_with_name_merge_policy(AspectMergePolicyKind::FailOnConflict);
    let entity = create_entity(&mut runtime, "shared");
    create_branch_from_main(&mut runtime, "feature");
    update_entity(&mut runtime, entity, "main-name");
    update_entity_on_branch(
        &mut runtime,
        entity,
        "feature-name",
        BranchId("feature".to_string()),
    );

    let artifact = runtime
        .merge()
        .inspect_planning_scope(
            MergeExecutionRequest {
                target_branch: BranchId("main".to_string()),
                source_branch: BranchId("feature".to_string()),
                merge_intent: MergeIntent::ReconcileIntoTarget,
            }
            .into(),
        )
        .expect("planning artifact");

    let lowered_index = artifact
        .lowered_plan
        .records
        .iter()
        .position(|record| record.record == RecordRef::Entity(entity))
        .expect("lowered record index");
    let lowered = &artifact.lowered_plan.records[lowered_index];

    assert_eq!(
        lowered.policy_proof_boundary.ownership_surface,
        MergePolicyOwnershipSurface::RuntimeOnly
    );
    assert_eq!(
        lowered.policy_proof_boundary.decision_boundary,
        MergePolicyDecisionBoundary::Reject {
            class: MergePolicyRejectClass::BuiltInFailOnConflict,
        }
    );
    assert_eq!(
        artifact.digest_basis.policy.proof_boundaries[lowered_index],
        lowered.policy_proof_boundary
    );
    assert!(matches!(
        lowered.record_decision,
        crate::facade::merge::LoweredRecordDecision::Reject(_)
    ));
}

#[test]
fn admitted_source_addition_carries_executable_class() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    let feature_only = crate::tests::support::create_entity_outcome_on_branch(
        &mut runtime,
        "feature-only",
        BranchId("feature".to_string()),
    );
    let entity = crate::tests::support::changed_entities(&feature_only)[0];

    let artifact = runtime
        .merge()
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
        lowered.resolution_class,
        MergeResolutionClass::SourceOnlyAddition
    );
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
        MergeResolutionClass::Topology(TopologyExecutionClass::TopologyRegionConflict);
    lowered[0].executable_class = None;

    match runtime
        .merge()
        .compile_execution_ready_merge_plan_for_test(execution_ready)
    {
        Err(MergeExecutionCompilationError::MissingExecutableClass { .. }) => {}
        other => panic!("expected missing executable class rejection, got {other:?}"),
    }
}
