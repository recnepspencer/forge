use std::sync::Arc;

use crate::facade::history::BranchId;
use crate::facade::identity::KindId;
use crate::facade::merge::{
    AspectMergePolicyDeclaration, AspectMergePolicyKind, MergeExecutionError,
    MergeExecutionPreparationError, MergeExecutionRequest, MergeIntent,
};
use crate::facade::runtime::RelationalRuntimeApi;
use crate::facade::schema::{
    AspectKey, EntityKindRegistration, KindAspectDeclarations, RelationIntegrityDeclarations,
    RelationKindRegistration, RelationalSchemaRegistry, SchemaId, SchemaVersionId,
};
use crate::merge::data::{
    BoundExecutableMergeRecordPlan, ExecutableAspectPlan, MaterializedAspectValuePayload,
    MergeExecutionCompilationError, MergeValueMaterialization,
};
use crate::schema::data::RelationPayloadClass;
use crate::symbols::data::InternedString;
use crate::tests::support::{
    create_branch_from_main, create_entity, create_entity_outcome_on_branch, entity_payload_aspect,
    persisted_runtime_with_test_schema, update_entity, update_entity_on_branch,
    CascadeDeletePolicy, CrossContextPolicy,
};

#[test]
fn prepare_merge_execution_admits_fully_ready_source_only_addition() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    create_entity_outcome_on_branch(&mut runtime, "feature-only", BranchId("feature".to_string()));

    let request = MergeExecutionRequest {
        target_branch: BranchId("main".to_string()),
        source_branch: BranchId("feature".to_string()),
        merge_intent: MergeIntent::ReconcileIntoTarget,
    };

    let prepared = runtime
        .merge_access()
        .prepare_merge_execution(request.clone())
        .expect("merge execution should prepare");

    assert_eq!(prepared.request(), &request);
    assert!(prepared.artifact().lowered_plan.fully_execution_ready);
    assert_eq!(prepared.artifact().lowered_plan.blocked_count, 0);
    assert_eq!(prepared.artifact().lowered_plan.rejected_count, 0);
    assert!(prepared
        .execution_ready_plan()
        .lowered_records
        .iter()
        .all(|record| matches!(
            record.record_decision,
            crate::merge::data::LoweredRecordDecision::Execute(_)
        )));
}

#[test]
fn prepare_merge_execution_rejects_blocked_merge_plans() {
    let mut runtime = persisted_runtime_with_test_schema();
    let shared = create_entity(&mut runtime, "shared");
    create_branch_from_main(&mut runtime, "feature");
    update_entity(&mut runtime, shared, "shared-main");
    update_entity_on_branch(
        &mut runtime,
        shared,
        "shared-feature",
        BranchId("feature".to_string()),
    );

    let error = runtime
        .merge_access()
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect_err("blocked merge plan must not prepare");

    match error {
        MergeExecutionPreparationError::NotExecutionReady(report) => {
            assert_eq!(report.blocked_count, 1);
            assert_eq!(report.rejected_count, 0);
            assert_eq!(report.denied_records.len(), 1);
            assert_eq!(
                report.denied_records[0].decision,
                crate::merge::data::LoweredRecordDecisionKind::Block
            );
        }
        other => panic!("expected readiness failure, got {other:?}"),
    }
}

#[test]
fn prepare_merge_execution_rejects_rejected_merge_plans() {
    let registry = RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: KindAspectDeclarations::new(vec![entity_payload_aspect("name", "name")])
                .with_merge_policy_declarations(vec![AspectMergePolicyDeclaration {
                    aspect_key: AspectKey(InternedString::Raw("name".to_string())),
                    policy: AspectMergePolicyKind::FailOnConflict,
                }]),
        })
        .and_then(|registry| {
            registry.register_relation_kind(RelationKindRegistration {
                kind_id: KindId(2),
                kind_name: "test.relation".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                payload_class: RelationPayloadClass::PayloadBearingRelation,
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                aspect_declarations: KindAspectDeclarations::default(),
                relation_integrity: RelationIntegrityDeclarations::default(),
            })
        })
        .unwrap();
    let mut runtime = RelationalRuntimeApi::builder().schema_registry(registry).build();
    let shared = create_entity(&mut runtime, "shared");
    create_branch_from_main(&mut runtime, "feature");
    update_entity(&mut runtime, shared, "shared-main");
    update_entity_on_branch(
        &mut runtime,
        shared,
        "shared-feature",
        BranchId("feature".to_string()),
    );

    let error = runtime
        .merge_access()
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect_err("rejected merge plan must not prepare");

    match error {
        MergeExecutionPreparationError::NotExecutionReady(report) => {
            assert_eq!(report.blocked_count, 0);
            assert_eq!(report.rejected_count, 1);
            assert_eq!(report.denied_records.len(), 1);
            assert_eq!(
                report.denied_records[0].decision,
                crate::merge::data::LoweredRecordDecisionKind::Reject
            );
        }
        other => panic!("expected readiness failure, got {other:?}"),
    }
}

#[test]
fn runtime_prepare_merge_execution_matches_merge_access_surface() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    create_entity_outcome_on_branch(&mut runtime, "feature-only", BranchId("feature".to_string()));
    let request = MergeExecutionRequest {
        target_branch: BranchId("main".to_string()),
        source_branch: BranchId("feature".to_string()),
        merge_intent: MergeIntent::ReconcileIntoTarget,
    };

    let via_runtime = runtime
        .prepare_merge_execution(request.clone())
        .expect("runtime merge prepare");
    let via_access = runtime
        .merge_access()
        .prepare_merge_execution(request.clone())
        .expect("merge access prepare");

    assert_eq!(via_runtime.request(), &request);
    assert_eq!(via_runtime.artifact(), via_access.artifact());
}

#[test]
fn verify_prepared_merge_execution_accepts_fresh_prepared_merge() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    create_entity_outcome_on_branch(&mut runtime, "feature-only", BranchId("feature".to_string()));

    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared merge");

    runtime
        .merge_access()
        .verify_prepared_merge_execution(&prepared)
        .expect("fresh prepared merge should verify");
}

#[test]
fn verify_prepared_merge_execution_rejects_runtime_instance_mismatch() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    create_entity_outcome_on_branch(&mut runtime, "feature-only", BranchId("feature".to_string()));

    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared merge");
    let forked = runtime.fork();

    match forked.merge_access().verify_prepared_merge_execution(&prepared) {
        Err(MergeExecutionError::RuntimeInstanceMismatch { .. }) => {}
        other => panic!("expected runtime instance mismatch, got {other:?}"),
    }
}

#[test]
fn verify_prepared_merge_execution_rejects_target_head_drift() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    create_entity_outcome_on_branch(&mut runtime, "feature-only", BranchId("feature".to_string()));

    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared merge");

    create_entity(&mut runtime, "main-advance");

    match runtime.merge_access().verify_prepared_merge_execution(&prepared) {
        Err(MergeExecutionError::StaleBranchHead { branch, .. }) => {
            assert_eq!(branch, BranchId("main".to_string()));
        }
        other => panic!("expected target stale-head rejection, got {other:?}"),
    }
}

#[test]
fn verify_prepared_merge_execution_rejects_source_head_drift() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    create_entity_outcome_on_branch(&mut runtime, "feature-only", BranchId("feature".to_string()));

    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared merge");

    create_entity_outcome_on_branch(&mut runtime, "feature-advance", BranchId("feature".to_string()));

    match runtime.merge_access().verify_prepared_merge_execution(&prepared) {
        Err(MergeExecutionError::StaleBranchHead { branch, .. }) => {
            assert_eq!(branch, BranchId("feature".to_string()));
        }
        other => panic!("expected source stale-head rejection, got {other:?}"),
    }
}

#[test]
fn verify_prepared_merge_execution_rejects_schema_semantic_drift() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    create_entity_outcome_on_branch(&mut runtime, "feature-only", BranchId("feature".to_string()));

    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared merge");

    let drifted_registry = RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(2),
            aspect_declarations: KindAspectDeclarations::new(vec![
                entity_payload_aspect("name", "name"),
                entity_payload_aspect("status", "status"),
            ]),
        })
        .and_then(|registry| {
            registry.register_relation_kind(RelationKindRegistration {
                kind_id: KindId(2),
                kind_name: "test.relation".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(2),
                payload_class: RelationPayloadClass::PayloadBearingRelation,
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                aspect_declarations: KindAspectDeclarations::default(),
                relation_integrity: RelationIntegrityDeclarations::default(),
            })
        })
        .unwrap();
    runtime.config.schema.registry = drifted_registry;

    match runtime.merge_access().verify_prepared_merge_execution(&prepared) {
        Err(MergeExecutionError::SchemaSemanticDrift { .. }) => {}
        other => panic!("expected schema drift rejection, got {other:?}"),
    }
}

#[test]
fn verify_prepared_merge_execution_rejects_merge_base_drift() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    create_entity_outcome_on_branch(&mut runtime, "feature-only", BranchId("feature".to_string()));

    let mut prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared merge");
    prepared.authority_binding_mut_for_test().merge_base_commit_id =
        crate::facade::history::CommitId(999_999);

    match runtime.merge_access().verify_prepared_merge_execution(&prepared) {
        Err(MergeExecutionError::MergeBaseDrift { .. }) => {}
        other => panic!("expected merge-base drift rejection, got {other:?}"),
    }
}

#[test]
fn verify_prepared_merge_execution_does_not_increment_planning_counters() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    create_entity_outcome_on_branch(&mut runtime, "feature-only", BranchId("feature".to_string()));

    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared merge");
    let before = runtime.performance_access().counters();

    runtime
        .merge_access()
        .verify_prepared_merge_execution(&prepared)
        .expect("verification should succeed");

    let after = runtime.performance_access().counters();
    assert_eq!(before.merge_planning_requests, after.merge_planning_requests);
    assert_eq!(
        before.merge_planning_schema_kinds_snapshotted,
        after.merge_planning_schema_kinds_snapshotted
    );
    assert_eq!(
        before.merge_planning_elapsed_nanos,
        after.merge_planning_elapsed_nanos
    );
    assert_eq!(
        after.merge_execution_verification_requests,
        before.merge_execution_verification_requests + 1
    );
    assert_eq!(
        after.merge_execution_branch_head_checks,
        before.merge_execution_branch_head_checks + 2
    );
    assert_eq!(
        after.merge_execution_merge_base_checks,
        before.merge_execution_merge_base_checks + 1
    );
    assert_eq!(
        after.merge_execution_compiled_plan_digest_checks,
        before.merge_execution_compiled_plan_digest_checks + 1
    );
    assert!(
        after.merge_execution_schema_kinds_snapshotted
            >= before.merge_execution_schema_kinds_snapshotted
    );
}

#[test]
fn verify_prepared_merge_execution_reports_verification_counters_without_planning_work() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    create_entity_outcome_on_branch(&mut runtime, "feature-only", BranchId("feature".to_string()));

    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared merge");
    runtime.performance_access().reset_counters();

    runtime
        .merge_access()
        .verify_prepared_merge_execution(&prepared)
        .expect("verification should succeed");

    let counters = runtime.performance_access().counters();
    assert_eq!(counters.merge_planning_requests, 0);
    assert_eq!(counters.merge_planning_schema_kinds_snapshotted, 0);
    assert_eq!(counters.merge_planning_elapsed_nanos, 0);
    assert_eq!(counters.merge_execution_verification_requests, 1);
    assert_eq!(counters.merge_execution_branch_head_checks, 2);
    assert_eq!(counters.merge_execution_merge_base_checks, 1);
    assert_eq!(counters.merge_execution_compiled_plan_digest_checks, 1);
    assert!(counters.merge_execution_schema_kinds_snapshotted > 0);
}

#[test]
fn prepare_merge_execution_compiles_source_addition_record_plan() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    create_entity_outcome_on_branch(&mut runtime, "feature-only", BranchId("feature".to_string()));

    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared merge");

    assert_eq!(prepared.bound_executable_plan().record_plans.len(), 1);
    match &prepared.bound_executable_plan().record_plans[0] {
        BoundExecutableMergeRecordPlan::AdoptSource(plan) => {
            match &plan.source_visible_snapshot {
                crate::merge::data::VisibleMergeRecordSnapshot::Entity(entity) => {
                    assert_eq!(
                        entity.payload.as_json().and_then(|json| json.get("name")),
                        Some(&serde_json::Value::String("feature-only".to_string()))
                    );
                }
                other => panic!("expected entity source snapshot, got {other:?}"),
            }
        }
        other => panic!("expected adopt-source record plan, got {other:?}"),
    }
}

#[test]
fn prepare_merge_execution_compiles_exact_shared_record_plan() {
    let registry = RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: KindAspectDeclarations::new(vec![entity_payload_aspect("name", "name")]),
        })
        .and_then(|registry| {
            registry.register_relation_kind(RelationKindRegistration {
                kind_id: KindId(2),
                kind_name: "test.relation".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                payload_class: RelationPayloadClass::PayloadBearingRelation,
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                aspect_declarations: KindAspectDeclarations::default(),
                relation_integrity: RelationIntegrityDeclarations::default(),
            })
        })
        .unwrap();
    let mut runtime = RelationalRuntimeApi::builder().schema_registry(registry).build();
    let shared = create_entity(&mut runtime, "shared");
    create_branch_from_main(&mut runtime, "feature");
    update_entity(&mut runtime, shared, "same");
    update_entity_on_branch(&mut runtime, shared, "same", BranchId("feature".to_string()));

    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared merge");

    assert_eq!(prepared.bound_executable_plan().record_plans.len(), 1);
    match &prepared.bound_executable_plan().record_plans[0] {
        BoundExecutableMergeRecordPlan::PreserveShared(plan) => {
            assert!(!plan.equality_witness.witness_digest.is_empty());
            assert_eq!(plan.provenance.classification, crate::merge::data::MergeConflictClass::ExactSharedTruth);
            assert!(!plan.aspect_plan.is_empty());
            match &plan.aspect_plan[0] {
                ExecutableAspectPlan::PreserveSharedValue { shared_value, .. } => {
                    assert_eq!(
                        shared_value.policy,
                        MergeValueMaterialization::EqualityWitnessDigest
                    );
                    match &shared_value.payload {
                        MaterializedAspectValuePayload::EqualityWitnessDigest(digest) => {
                            assert!(!digest.is_empty());
                        }
                        other => panic!("expected equality witness payload, got {other:?}"),
                    }
                }
                other => panic!("expected preserve-shared aspect plan, got {other:?}"),
            }
        }
        other => panic!("expected preserve-shared record plan, got {other:?}"),
    }
}

#[test]
fn prepare_merge_execution_compiles_reconcile_record_plan() {
    let name_key = AspectKey(InternedString::Raw("name".to_string()));
    let registry = RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: KindAspectDeclarations::new(vec![entity_payload_aspect("name", "name")])
                .with_identity_declarations(vec![crate::facade::merge::IdentityBasisDeclaration {
                    scope: crate::facade::merge::IdentityBasisScope::AspectKey(name_key.clone()),
                    basis: crate::facade::merge::IdentityBasisKind::DeclaredKeySet(
                        Arc::from([name_key.clone()]),
                    ),
                }])
                .with_merge_policy_declarations(vec![AspectMergePolicyDeclaration {
                    aspect_key: name_key.clone(),
                    policy: AspectMergePolicyKind::PreferRicher,
                }]),
        })
        .and_then(|registry| {
            registry.register_relation_kind(RelationKindRegistration {
                kind_id: KindId(2),
                kind_name: "test.relation".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                payload_class: RelationPayloadClass::PayloadBearingRelation,
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                aspect_declarations: KindAspectDeclarations::default(),
                relation_integrity: RelationIntegrityDeclarations::default(),
            })
        })
        .unwrap();
    let mut runtime = RelationalRuntimeApi::builder().schema_registry(registry).build();
    create_entity(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    create_entity(&mut runtime, "shared-name");
    create_entity_outcome_on_branch(&mut runtime, "shared-name", BranchId("feature".to_string()));

    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared merge");

    assert_eq!(prepared.bound_executable_plan().record_plans.len(), 1);
    match &prepared.bound_executable_plan().record_plans[0] {
        BoundExecutableMergeRecordPlan::Reconcile(plan) => {
            assert_eq!(plan.identity_basis.source_record, plan.source_record);
            assert_eq!(plan.identity_basis.target_record, plan.target_record);
            assert!(!plan.aspect_plan.is_empty());
            match &plan.aspect_plan[0] {
                ExecutableAspectPlan::ReconcileValue {
                    source_value,
                    target_value,
                    base_value,
                    ..
                } => {
                    let source_value = source_value.as_ref().expect("source value");
                    let target_value = target_value.as_ref().expect("target value");
                    let base_value = base_value.as_ref().expect("base value");
                    assert_eq!(source_value.policy, MergeValueMaterialization::SnapshotPinnedRead);
                    assert_eq!(target_value.policy, MergeValueMaterialization::SnapshotPinnedRead);
                    assert_eq!(base_value.policy, MergeValueMaterialization::SnapshotPinnedRead);
                    match &source_value.payload {
                        MaterializedAspectValuePayload::VisibleAspectReference { .. } => {}
                        other => panic!("expected source aspect reference, got {other:?}"),
                    }
                    match &base_value.payload {
                        MaterializedAspectValuePayload::VisibleAspectReference { .. } => {}
                        other => panic!("expected base aspect reference, got {other:?}"),
                    }
                }
                other => panic!("expected reconcile aspect plan, got {other:?}"),
            }
        }
        other => panic!("expected reconcile record plan, got {other:?}"),
    }
}

#[test]
fn compile_execution_ready_merge_plan_rejects_missing_source_record() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    create_entity_outcome_on_branch(&mut runtime, "feature-only", BranchId("feature".to_string()));

    let mut prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared merge");
    prepared.execution_ready_plan_mut_for_test().source_records = Arc::from([]);

    match runtime
        .merge_access()
        .compile_execution_ready_merge_plan_for_test(prepared.execution_ready_plan())
    {
        Err(MergeExecutionCompilationError::MissingSourceRecord { .. }) => {}
        other => panic!("expected missing source record compilation failure, got {other:?}"),
    }
}

#[test]
fn verify_prepared_merge_execution_rejects_corrupted_compiled_plan() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    create_entity_outcome_on_branch(&mut runtime, "feature-only", BranchId("feature".to_string()));

    let mut prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared merge");
    prepared.bound_executable_plan_mut_for_test().record_plans = Arc::from([]);

    match runtime.merge_access().verify_prepared_merge_execution(&prepared) {
        Err(MergeExecutionError::Compilation(
            MergeExecutionCompilationError::PreparedAuthorityBindingMismatch { .. },
        )) => {}
        other => panic!("expected compilation rejection during verify, got {other:?}"),
    }
}
