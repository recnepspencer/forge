use serde::Serialize;
use serde_json::json;

use crate::commit_strategies::data::{
    CommitStrategyExecutionRegistration, CommitStrategyExecutor, CommitStrategyRegistration,
    StrategyExecutionResult, StrategyExecutorFailure, StrategyExecutorFailureClass,
    StrategyObservationContext,
};
use crate::commit_strategies::strategies::{
    AspectFieldReconciliationInput, AspectFieldReconciliationStrategy,
    EntityReplacementReconciliationInput, EntityReplacementReconciliationStrategy,
    IntentReconciliationInput, IntentReconciliationStrategy, ReplicaConvergenceInput,
    ReplicaConvergenceStrategy,
};
use crate::facade::commit_strategies::RawStrategyCommitRequest;
use crate::facade::config::RelationalRuntimeProfile;
use crate::facade::durability::{DurabilityMode, DurableStoreLayout};
use crate::facade::history::BranchId;
use crate::facade::merge::{MergeIntent, MergePlanningRequest};
use crate::facade::replay::{
    RelationalReplayRequest, ReplayExecutionMode, ReplayMismatchClass, ReplayObservableSurface,
    ReplayVerificationMode,
};
use crate::facade::runtime::{RelationalRuntime, RelationalRuntimeApi};
use crate::facade::transactions::TransactionOptions;
use crate::tests::support::{
    certification_digest, changed_entities, checkpoint_and_recover_with, create_branch_from_main,
    create_entity, entity_payload_aspect, lifecycle_aspect, read_entity_name,
    unique_test_store_path, AspectSchemaFixture,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct StrategyCertificationBundle {
    main_commit_strategy_digest: String,
    feature_commit_strategy_digest: String,
    replacement_commit_strategy_digest: String,
    merge_conflict_digest: String,
    merge_lowered_plan_digest: String,
    aspect_overlap_merge_conflict_digest: String,
    aspect_overlap_merge_lowered_plan_digest: String,
    aspect_disjoint_merge_conflict_digest: String,
    aspect_disjoint_merge_lowered_plan_digest: String,
    controller_sequence_merge_conflict_digest: String,
    controller_sequence_merge_lowered_plan_digest: String,
    main_replay_digest: String,
    feature_replay_digest: String,
    controller_sequence_noop_digest: String,
    replacement_replay_digest: String,
    replacement_lineage_digest: String,
    missing_executor_replay_digest: String,
    failing_executor_replay_digest: String,
    branch_heads_digest: String,
    visible_truth_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct ReplacementCertificationBundle {
    replacement_commit_strategy_digest: String,
    replacement_replay_digest: String,
    replacement_lineage_digest: String,
}

#[derive(Debug, Clone, Copy)]
struct DeterministicFailureExecutor;

impl CommitStrategyExecutor for DeterministicFailureExecutor {
    fn execute(
        &self,
        _request: &crate::commit_strategies::data::CanonicalStrategyCommitRequest,
        _observation: &StrategyObservationContext<'_>,
    ) -> Result<StrategyExecutionResult, StrategyExecutorFailure> {
        Err(StrategyExecutorFailure::new(
            StrategyExecutorFailureClass::DomainRejection,
            "milestone-8.5 hostile deterministic executor rejection",
        ))
    }
}

fn persisted_strategy_runtime(root_path: std::path::PathBuf) -> RelationalRuntime {
    let intent_descriptor = IntentReconciliationStrategy::descriptor(
        crate::facade::commit_strategies::CommitStrategyId(801),
    );
    let replica_descriptor = ReplicaConvergenceStrategy::descriptor(
        crate::facade::commit_strategies::CommitStrategyId(802),
    );
    let aspect_descriptor = AspectFieldReconciliationStrategy::descriptor(
        crate::facade::commit_strategies::CommitStrategyId(803),
    );
    let replacement_descriptor = EntityReplacementReconciliationStrategy::descriptor(
        crate::facade::commit_strategies::CommitStrategyId(804),
    );
    RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(strategy_schema_registry())
        .durability_mode(DurabilityMode::PersistedSegmentedLocalFs)
        .durable_store_layout(DurableStoreLayout {
            root_path,
            segment_commit_capacity: 2,
        })
        .commit_strategy(
            crate::facade::commit_strategies::CommitStrategyRegistration::new(
                intent_descriptor.clone(),
            )
            .expect("intent registration"),
        )
        .commit_strategy_executor(IntentReconciliationStrategy::execution_registration(
            &intent_descriptor,
        ))
        .commit_strategy(
            crate::facade::commit_strategies::CommitStrategyRegistration::new(
                replica_descriptor.clone(),
            )
            .expect("replica registration"),
        )
        .commit_strategy_executor(ReplicaConvergenceStrategy::execution_registration(
            &replica_descriptor,
        ))
        .commit_strategy(
            crate::facade::commit_strategies::CommitStrategyRegistration::new(
                aspect_descriptor.clone(),
            )
            .expect("aspect registration"),
        )
        .commit_strategy_executor(AspectFieldReconciliationStrategy::execution_registration(
            &aspect_descriptor,
        ))
        .commit_strategy(
            crate::facade::commit_strategies::CommitStrategyRegistration::new(
                replacement_descriptor.clone(),
            )
            .expect("replacement registration"),
        )
        .commit_strategy_executor(
            EntityReplacementReconciliationStrategy::execution_registration(
                &replacement_descriptor,
            ),
        )
        .build()
}

fn persisted_replacement_strategy_runtime(root_path: std::path::PathBuf) -> RelationalRuntime {
    let replacement_descriptor = EntityReplacementReconciliationStrategy::descriptor(
        crate::facade::commit_strategies::CommitStrategyId(804),
    );
    RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(strategy_schema_registry())
        .durability_mode(DurabilityMode::PersistedSegmentedLocalFs)
        .durable_store_layout(DurableStoreLayout {
            root_path,
            segment_commit_capacity: 2,
        })
        .commit_strategy(
            crate::facade::commit_strategies::CommitStrategyRegistration::new(
                replacement_descriptor.clone(),
            )
            .expect("replacement registration"),
        )
        .commit_strategy_executor(
            EntityReplacementReconciliationStrategy::execution_registration(
                &replacement_descriptor,
            ),
        )
        .build()
}

fn persisted_strategy_runtime_without_executors(
    root_path: std::path::PathBuf,
) -> RelationalRuntime {
    let intent_descriptor = IntentReconciliationStrategy::descriptor(
        crate::facade::commit_strategies::CommitStrategyId(801),
    );
    let replica_descriptor = ReplicaConvergenceStrategy::descriptor(
        crate::facade::commit_strategies::CommitStrategyId(802),
    );
    let aspect_descriptor = AspectFieldReconciliationStrategy::descriptor(
        crate::facade::commit_strategies::CommitStrategyId(803),
    );
    let replacement_descriptor = EntityReplacementReconciliationStrategy::descriptor(
        crate::facade::commit_strategies::CommitStrategyId(804),
    );
    RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(strategy_schema_registry())
        .durability_mode(DurabilityMode::PersistedSegmentedLocalFs)
        .durable_store_layout(DurableStoreLayout {
            root_path,
            segment_commit_capacity: 2,
        })
        .commit_strategy(
            CommitStrategyRegistration::new(intent_descriptor).expect("intent registration"),
        )
        .commit_strategy(
            CommitStrategyRegistration::new(replica_descriptor).expect("replica registration"),
        )
        .commit_strategy(
            CommitStrategyRegistration::new(aspect_descriptor).expect("aspect registration"),
        )
        .commit_strategy(
            CommitStrategyRegistration::new(replacement_descriptor)
                .expect("replacement registration"),
        )
        .build()
}

fn persisted_strategy_runtime_with_failing_intent_executor(
    root_path: std::path::PathBuf,
) -> RelationalRuntime {
    let intent_descriptor = IntentReconciliationStrategy::descriptor(
        crate::facade::commit_strategies::CommitStrategyId(801),
    );
    let replica_descriptor = ReplicaConvergenceStrategy::descriptor(
        crate::facade::commit_strategies::CommitStrategyId(802),
    );
    let aspect_descriptor = AspectFieldReconciliationStrategy::descriptor(
        crate::facade::commit_strategies::CommitStrategyId(803),
    );
    let replacement_descriptor = EntityReplacementReconciliationStrategy::descriptor(
        crate::facade::commit_strategies::CommitStrategyId(804),
    );
    RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(strategy_schema_registry())
        .durability_mode(DurabilityMode::PersistedSegmentedLocalFs)
        .durable_store_layout(DurableStoreLayout {
            root_path,
            segment_commit_capacity: 2,
        })
        .commit_strategy(
            CommitStrategyRegistration::new(intent_descriptor.clone())
                .expect("intent registration"),
        )
        .commit_strategy_executor(CommitStrategyExecutionRegistration::new(
            &intent_descriptor,
            DeterministicFailureExecutor,
        ))
        .commit_strategy(
            CommitStrategyRegistration::new(replica_descriptor.clone())
                .expect("replica registration"),
        )
        .commit_strategy_executor(ReplicaConvergenceStrategy::execution_registration(
            &replica_descriptor,
        ))
        .commit_strategy(
            CommitStrategyRegistration::new(aspect_descriptor.clone())
                .expect("aspect registration"),
        )
        .commit_strategy_executor(AspectFieldReconciliationStrategy::execution_registration(
            &aspect_descriptor,
        ))
        .commit_strategy(
            CommitStrategyRegistration::new(replacement_descriptor.clone())
                .expect("replacement registration"),
        )
        .commit_strategy_executor(
            EntityReplacementReconciliationStrategy::execution_registration(
                &replacement_descriptor,
            ),
        )
        .build()
}

fn strategy_schema_registry() -> crate::schema::data::RelationalSchemaRegistry {
    AspectSchemaFixture {
        entity_aspects: vec![
            entity_payload_aspect("name", "name"),
            entity_payload_aspect("replicas", "replicas"),
            lifecycle_aspect(),
        ],
        ..AspectSchemaFixture::default()
    }
    .build_registry()
}

fn execute_strategy_commit(
    runtime: &mut RelationalRuntime,
    strategy_name: &str,
    input: serde_json::Value,
    target_branch: Option<BranchId>,
) -> crate::facade::transactions::CommitResult {
    let request = runtime
        .commit_strategies()
        .canonicalize_request(&RawStrategyCommitRequest::new(
            crate::facade::commit_strategies::CommitStrategySemanticName::new(strategy_name),
            serde_json::to_vec(&input).expect("serialize strategy input"),
            crate::facade::commit_strategies::StrategyCallerProvenance {
                request_origin: crate::facade::commit_strategies::StrategyRequestOrigin::Test,
                actor_identity: None,
                correlation_id: None,
            },
        ))
        .expect("canonical strategy request");
    let snapshot = runtime.visibility_authority().snapshot();
    let execution = runtime
        .commit_strategies()
        .execute(&request, &snapshot)
        .expect("strategy execution");
    let mut authority = runtime.commit_strategies_authority();
    let lowered = authority
        .lower_execution(
            &request,
            &execution,
            TransactionOptions {
                target_branch,
                ..TransactionOptions::default()
            },
        )
        .expect("lowered strategy plan");
    let validated = authority
        .validate_lowered_plan(lowered)
        .expect("validated strategy plan");
    authority
        .execute_validated_commit(validated)
        .expect("strategy commit")
}

fn full_replay_digest(replay: &crate::facade::replay::RelationalReplayOutcome) -> String {
    certification_digest(&(
        &replay.reconstructed_commit_closure,
        &replay.snapshot_version,
        &replay.lineage_authority_basis,
        &replay.compared_surfaces,
        &replay.mismatches,
        &replay.failure,
    ))
}

fn run_strategy_merge_certification() -> StrategyCertificationBundle {
    let root_path = unique_test_store_path("forge-relational-strategy-cert");
    let recovered_root = root_path.clone();
    let mut runtime = persisted_strategy_runtime(root_path);
    let entity = create_entity(&mut runtime, "service");
    let feature_branch = create_branch_from_main(&mut runtime, "strategy-feature");
    let aspect_overlap_branch = create_branch_from_main(&mut runtime, "aspect-overlap-feature");
    let aspect_disjoint_branch = create_branch_from_main(&mut runtime, "aspect-disjoint-feature");

    let main_commit = execute_strategy_commit(
        &mut runtime,
        IntentReconciliationStrategy::DEFAULT_SEMANTIC_NAME,
        serde_json::to_value(IntentReconciliationInput {
            entity_id: entity,
            desired_payload: json!({"name":"service-main","replicas":1}),
        })
        .expect("intent input value"),
        None,
    );
    let feature_commit = execute_strategy_commit(
        &mut runtime,
        ReplicaConvergenceStrategy::DEFAULT_SEMANTIC_NAME,
        serde_json::to_value(ReplicaConvergenceInput {
            entity_id: entity,
            desired_replicas: 7,
        })
        .expect("replica input value"),
        Some(feature_branch.clone()),
    );
    let planning = runtime
        .merge_access()
        .inspect_planning_scope(MergePlanningRequest::new(
            BranchId("main".to_string()),
            feature_branch.clone(),
            MergeIntent::ReconcileIntoTarget,
        ))
        .expect("merge planning");
    let classification = planning
        .conflict_classification
        .classifications
        .iter()
        .find(|classification| {
            classification.record == crate::facade::transactions::RecordRef::Entity(entity)
        })
        .expect("strategy conflict classification");
    assert_eq!(
        classification.class,
        crate::merge::data::MergeConflictClass::StrategyIntentConflict
    );
    let policy_record = planning
        .policy_resolution
        .records
        .iter()
        .find(|record| record.record == crate::facade::transactions::RecordRef::Entity(entity))
        .expect("strategy policy record");
    assert_eq!(
        policy_record.proof_boundary.decision_boundary,
        crate::merge::data::MergePolicyDecisionBoundary::RequiresManualResolution {
            class: crate::merge::data::MergeManualResolutionClass::StrategyIntentConflict,
        }
    );

    let aspect_overlap_entity = create_entity(&mut runtime, "aspect-overlap");
    let _aspect_overlap_main_commit = execute_strategy_commit(
        &mut runtime,
        AspectFieldReconciliationStrategy::DEFAULT_SEMANTIC_NAME,
        serde_json::to_value(AspectFieldReconciliationInput {
            entity_id: aspect_overlap_entity,
            field_name: "name".to_string(),
            desired_value: json!("aspect-main"),
        })
        .expect("aspect overlap main input value"),
        None,
    );
    let _aspect_overlap_feature_commit = execute_strategy_commit(
        &mut runtime,
        AspectFieldReconciliationStrategy::DEFAULT_SEMANTIC_NAME,
        serde_json::to_value(AspectFieldReconciliationInput {
            entity_id: aspect_overlap_entity,
            field_name: "name".to_string(),
            desired_value: json!("aspect-feature"),
        })
        .expect("aspect overlap feature input value"),
        Some(aspect_overlap_branch.clone()),
    );
    let aspect_overlap_planning = runtime
        .merge_access()
        .inspect_planning_scope(MergePlanningRequest::new(
            BranchId("main".to_string()),
            aspect_overlap_branch.clone(),
            MergeIntent::ReconcileIntoTarget,
        ))
        .expect("aspect overlap merge planning");
    let aspect_overlap_classification = aspect_overlap_planning
        .conflict_classification
        .classifications
        .iter()
        .find(|classification| {
            classification.record
                == crate::facade::transactions::RecordRef::Entity(aspect_overlap_entity)
        })
        .expect("aspect overlap classification");
    assert_eq!(
        aspect_overlap_classification.class,
        crate::merge::data::MergeConflictClass::StrategyIntentConflict
    );

    let aspect_disjoint_entity = create_entity(&mut runtime, "aspect-disjoint");
    let _aspect_disjoint_main_commit = execute_strategy_commit(
        &mut runtime,
        AspectFieldReconciliationStrategy::DEFAULT_SEMANTIC_NAME,
        serde_json::to_value(AspectFieldReconciliationInput {
            entity_id: aspect_disjoint_entity,
            field_name: "name".to_string(),
            desired_value: json!("disjoint-main"),
        })
        .expect("aspect disjoint main input value"),
        None,
    );
    let _aspect_disjoint_feature_commit = execute_strategy_commit(
        &mut runtime,
        ReplicaConvergenceStrategy::DEFAULT_SEMANTIC_NAME,
        serde_json::to_value(ReplicaConvergenceInput {
            entity_id: aspect_disjoint_entity,
            desired_replicas: 9,
        })
        .expect("aspect disjoint feature input value"),
        Some(aspect_disjoint_branch.clone()),
    );
    let aspect_disjoint_planning = runtime
        .merge_access()
        .inspect_planning_scope(MergePlanningRequest::new(
            BranchId("main".to_string()),
            aspect_disjoint_branch.clone(),
            MergeIntent::ReconcileIntoTarget,
        ))
        .expect("aspect disjoint merge planning");
    let aspect_disjoint_classification = aspect_disjoint_planning
        .conflict_classification
        .classifications
        .iter()
        .find(|classification| {
            classification.record
                == crate::facade::transactions::RecordRef::Entity(aspect_disjoint_entity)
        })
        .expect("aspect disjoint classification");
    assert_ne!(
        aspect_disjoint_classification.class,
        crate::merge::data::MergeConflictClass::StrategyIntentConflict
    );
    assert!(
        aspect_disjoint_classification.strategy_evidence.is_none(),
        "disjoint aspect-vs-replica intent should not synthesize strategy conflict evidence: {aspect_disjoint_classification:?}"
    );

    let controller_sequence_entity = create_entity(&mut runtime, "controller-sequence");
    let controller_sequence_branch =
        create_branch_from_main(&mut runtime, "controller-sequence-feature");
    let _controller_initial_intent = execute_strategy_commit(
        &mut runtime,
        IntentReconciliationStrategy::DEFAULT_SEMANTIC_NAME,
        serde_json::to_value(IntentReconciliationInput {
            entity_id: controller_sequence_entity,
            desired_payload: json!({"name":"controller-main","replicas":2}),
        })
        .expect("controller initial intent input"),
        None,
    );
    let _controller_feature_converge = execute_strategy_commit(
        &mut runtime,
        ReplicaConvergenceStrategy::DEFAULT_SEMANTIC_NAME,
        serde_json::to_value(ReplicaConvergenceInput {
            entity_id: controller_sequence_entity,
            desired_replicas: 7,
        })
        .expect("controller feature converge input"),
        Some(controller_sequence_branch.clone()),
    );
    let _controller_narrowed_intent = execute_strategy_commit(
        &mut runtime,
        IntentReconciliationStrategy::DEFAULT_SEMANTIC_NAME,
        serde_json::to_value(IntentReconciliationInput {
            entity_id: controller_sequence_entity,
            desired_payload: json!({"name":"controller-renamed"}),
        })
        .expect("controller narrowed intent input"),
        None,
    );
    let controller_feature_idempotent_commit = execute_strategy_commit(
        &mut runtime,
        ReplicaConvergenceStrategy::DEFAULT_SEMANTIC_NAME,
        serde_json::to_value(ReplicaConvergenceInput {
            entity_id: controller_sequence_entity,
            desired_replicas: 7,
        })
        .expect("controller idempotent converge input"),
        Some(controller_sequence_branch.clone()),
    );
    assert_eq!(
        controller_feature_idempotent_commit
            .change_summary()
            .expect("controller idempotent change summary")
            .changed_record_count,
        0
    );
    let controller_sequence_planning = runtime
        .merge_access()
        .inspect_planning_scope(MergePlanningRequest::new(
            BranchId("main".to_string()),
            controller_sequence_branch.clone(),
            MergeIntent::ReconcileIntoTarget,
        ))
        .expect("controller sequence merge planning");
    let controller_sequence_classification = controller_sequence_planning
        .conflict_classification
        .classifications
        .iter()
        .find(|classification| {
            classification.record
                == crate::facade::transactions::RecordRef::Entity(controller_sequence_entity)
        })
        .expect("controller sequence classification");
    assert_ne!(
        controller_sequence_classification.class,
        crate::merge::data::MergeConflictClass::StrategyIntentConflict
    );
    assert!(
        controller_sequence_classification.class
            == crate::merge::data::MergeConflictClass::ExactSharedTruth,
        "narrowed controller intent should become explicit benign shared truth: {controller_sequence_classification:?}"
    );

    let planning = runtime
        .merge_access()
        .inspect_planning_scope(MergePlanningRequest::new(
            BranchId("main".to_string()),
            feature_branch.clone(),
            MergeIntent::ReconcileIntoTarget,
        ))
        .expect("final merge planning");
    let aspect_overlap_planning = runtime
        .merge_access()
        .inspect_planning_scope(MergePlanningRequest::new(
            BranchId("main".to_string()),
            aspect_overlap_branch.clone(),
            MergeIntent::ReconcileIntoTarget,
        ))
        .expect("final aspect overlap merge planning");
    let aspect_disjoint_planning = runtime
        .merge_access()
        .inspect_planning_scope(MergePlanningRequest::new(
            BranchId("main".to_string()),
            aspect_disjoint_branch.clone(),
            MergeIntent::ReconcileIntoTarget,
        ))
        .expect("final aspect disjoint merge planning");
    let replacement_certification = run_replacement_strategy_certification();

    let main_replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: main_commit.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::AuditRecoveryVerification,
        });
    let feature_replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: feature_commit.commit.commit_id,
            branch_id: feature_branch.clone(),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::AuditRecoveryVerification,
        });
    assert!(
        main_replay.failure.is_none(),
        "main replay failed: {main_replay:?}"
    );
    assert!(main_replay
        .compared_surfaces
        .contains(&ReplayObservableSurface::Strategy));
    assert!(feature_replay
        .compared_surfaces
        .contains(&ReplayObservableSurface::Strategy));
    let current = runtime
        .visibility_reads()
        .read_version(runtime.current_version_id());
    let visible_truth_digest = certification_digest(&(
        read_entity_name(current.get_entity(entity).expect("entity visible")).map(str::to_string),
        runtime
            .history_access()
            .branch_head(&BranchId("main".to_string()))
            .cloned(),
        runtime
            .history_access()
            .branch_head(&feature_branch)
            .cloned(),
    ));

    let mut live_bundle = StrategyCertificationBundle {
        main_commit_strategy_digest: certification_digest(
            main_commit
                .publication
                .strategy_artifacts
                .as_ref()
                .expect("main strategy artifacts"),
        ),
        feature_commit_strategy_digest: certification_digest(
            feature_commit
                .publication
                .strategy_artifacts
                .as_ref()
                .expect("feature strategy artifacts"),
        ),
        replacement_commit_strategy_digest: replacement_certification
            .replacement_commit_strategy_digest
            .clone(),
        merge_conflict_digest: certification_digest(&planning.digest_basis.conflict),
        merge_lowered_plan_digest: certification_digest(&planning.digest_basis.lowered_plan),
        aspect_overlap_merge_conflict_digest: certification_digest(
            &aspect_overlap_planning.digest_basis.conflict,
        ),
        aspect_overlap_merge_lowered_plan_digest: certification_digest(
            &aspect_overlap_planning.digest_basis.lowered_plan,
        ),
        aspect_disjoint_merge_conflict_digest: certification_digest(
            &aspect_disjoint_planning.digest_basis.conflict,
        ),
        aspect_disjoint_merge_lowered_plan_digest: certification_digest(
            &aspect_disjoint_planning.digest_basis.lowered_plan,
        ),
        controller_sequence_merge_conflict_digest: certification_digest(
            &controller_sequence_planning.digest_basis.conflict,
        ),
        controller_sequence_merge_lowered_plan_digest: certification_digest(
            &controller_sequence_planning.digest_basis.lowered_plan,
        ),
        main_replay_digest: full_replay_digest(&main_replay),
        feature_replay_digest: full_replay_digest(&feature_replay),
        controller_sequence_noop_digest: certification_digest(&(
            certification_digest(
                controller_feature_idempotent_commit
                    .publication
                    .strategy_artifacts
                    .as_ref()
                    .expect("controller idempotent strategy artifacts"),
            ),
            controller_feature_idempotent_commit
                .change_summary()
                .expect("controller idempotent change summary")
                .changed_record_count,
            controller_feature_idempotent_commit
                .publication_summary()
                .expect("controller idempotent publication summary")
                .patch_record_count,
        )),
        replacement_replay_digest: replacement_certification.replacement_replay_digest.clone(),
        replacement_lineage_digest: replacement_certification.replacement_lineage_digest.clone(),
        missing_executor_replay_digest: String::new(),
        failing_executor_replay_digest: String::new(),
        branch_heads_digest: certification_digest(&(
            runtime
                .history_access()
                .branch_head(&BranchId("main".to_string()))
                .cloned(),
            runtime
                .history_access()
                .branch_head(&feature_branch)
                .cloned(),
        )),
        visible_truth_digest,
    };

    let recovery_plan = runtime.durability_access().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::AuditRecoveryVerification,
    );

    let mut missing_executor_plan = recovery_plan.clone();
    missing_executor_plan.commit_strategy_executors = Default::default();
    let mut missing_executor_runtime =
        persisted_strategy_runtime_without_executors(recovered_root.clone());
    missing_executor_runtime
        .durability_authority()
        .recover(missing_executor_plan)
        .expect("recover without executors");
    let missing_executor_replay =
        missing_executor_runtime
            .replay_authority()
            .replay_commit(RelationalReplayRequest {
                commit_id: main_commit.commit.commit_id,
                branch_id: BranchId("main".to_string()),
                execution_mode: ReplayExecutionMode::SerialDeterministic,
                verification_mode: ReplayVerificationMode::AuditRecoveryVerification,
            });
    assert!(missing_executor_replay.mismatches.iter().any(|mismatch| {
        mismatch.class == ReplayMismatchClass::StrategyExecutorUnavailable
            && mismatch.surface == ReplayObservableSurface::Strategy
    }));

    let mut failing_executor_runtime =
        persisted_strategy_runtime_with_failing_intent_executor(recovered_root.clone());
    let mut failing_executor_plan = recovery_plan.clone();
    failing_executor_plan.commit_strategy_executors = failing_executor_runtime
        .commit_strategy_executor_registry()
        .clone();
    failing_executor_runtime
        .durability_authority()
        .recover(failing_executor_plan)
        .expect("recover with failing intent executor");
    let failing_executor_replay =
        failing_executor_runtime
            .replay_authority()
            .replay_commit(RelationalReplayRequest {
                commit_id: main_commit.commit.commit_id,
                branch_id: BranchId("main".to_string()),
                execution_mode: ReplayExecutionMode::SerialDeterministic,
                verification_mode: ReplayVerificationMode::AuditRecoveryVerification,
            });
    assert!(failing_executor_replay.mismatches.iter().any(|mismatch| {
        mismatch.class == ReplayMismatchClass::StrategyExecutionFailure
            && mismatch.surface == ReplayObservableSurface::Strategy
    }));
    live_bundle.missing_executor_replay_digest = full_replay_digest(&missing_executor_replay);
    live_bundle.failing_executor_replay_digest = full_replay_digest(&failing_executor_replay);

    let (_recovery, mut recovered) =
        checkpoint_and_recover_with(&mut runtime, || persisted_strategy_runtime(recovered_root));

    let recovered_planning = recovered
        .merge_access()
        .inspect_planning_scope(MergePlanningRequest::new(
            BranchId("main".to_string()),
            feature_branch.clone(),
            MergeIntent::ReconcileIntoTarget,
        ))
        .expect("recovered merge planning");
    let recovered_main_replay =
        recovered
            .replay_authority()
            .replay_commit(RelationalReplayRequest {
                commit_id: main_commit.commit.commit_id,
                branch_id: BranchId("main".to_string()),
                execution_mode: ReplayExecutionMode::SerialDeterministic,
                verification_mode: ReplayVerificationMode::AuditRecoveryVerification,
            });
    let recovered_feature_replay =
        recovered
            .replay_authority()
            .replay_commit(RelationalReplayRequest {
                commit_id: feature_commit.commit.commit_id,
                branch_id: feature_branch.clone(),
                execution_mode: ReplayExecutionMode::SerialDeterministic,
                verification_mode: ReplayVerificationMode::AuditRecoveryVerification,
            });
    assert!(
        recovered_main_replay.failure.is_none(),
        "recovered main replay failed: {recovered_main_replay:?}"
    );
    assert!(
        recovered_feature_replay.failure.is_none(),
        "recovered feature replay failed: {recovered_feature_replay:?}"
    );
    assert!(recovered_main_replay
        .compared_surfaces
        .contains(&ReplayObservableSurface::Strategy));
    assert!(recovered_feature_replay
        .compared_surfaces
        .contains(&ReplayObservableSurface::Strategy));
    let recovered_aspect_overlap_planning = recovered
        .merge_access()
        .inspect_planning_scope(MergePlanningRequest::new(
            BranchId("main".to_string()),
            aspect_overlap_branch.clone(),
            MergeIntent::ReconcileIntoTarget,
        ))
        .expect("recovered aspect overlap planning");
    let recovered_aspect_disjoint_planning = recovered
        .merge_access()
        .inspect_planning_scope(MergePlanningRequest::new(
            BranchId("main".to_string()),
            aspect_disjoint_branch.clone(),
            MergeIntent::ReconcileIntoTarget,
        ))
        .expect("recovered aspect disjoint planning");
    let recovered_controller_sequence_planning = recovered
        .merge_access()
        .inspect_planning_scope(MergePlanningRequest::new(
            BranchId("main".to_string()),
            controller_sequence_branch.clone(),
            MergeIntent::ReconcileIntoTarget,
        ))
        .expect("recovered controller sequence planning");
    let recovered_main_envelope = recovered
        .replay_access()
        .canonical_commit_envelope(main_commit.commit.commit_id)
        .cloned()
        .expect("recovered main envelope");
    let recovered_feature_envelope = recovered
        .replay_access()
        .canonical_commit_envelope(feature_commit.commit.commit_id)
        .cloned()
        .expect("recovered feature envelope");
    let recovered_current = recovered
        .visibility_reads()
        .read_version(recovered.current_version_id());
    let recovered_bundle = StrategyCertificationBundle {
        main_commit_strategy_digest: certification_digest(
            recovered_main_envelope
                .strategy_artifacts
                .as_ref()
                .expect("recovered main strategy artifacts"),
        ),
        feature_commit_strategy_digest: certification_digest(
            recovered_feature_envelope
                .strategy_artifacts
                .as_ref()
                .expect("recovered feature strategy artifacts"),
        ),
        replacement_commit_strategy_digest: replacement_certification
            .replacement_commit_strategy_digest
            .clone(),
        merge_conflict_digest: certification_digest(&recovered_planning.digest_basis.conflict),
        merge_lowered_plan_digest: certification_digest(
            &recovered_planning.digest_basis.lowered_plan,
        ),
        aspect_overlap_merge_conflict_digest: certification_digest(
            &recovered_aspect_overlap_planning.digest_basis.conflict,
        ),
        aspect_overlap_merge_lowered_plan_digest: certification_digest(
            &recovered_aspect_overlap_planning.digest_basis.lowered_plan,
        ),
        aspect_disjoint_merge_conflict_digest: certification_digest(
            &recovered_aspect_disjoint_planning.digest_basis.conflict,
        ),
        aspect_disjoint_merge_lowered_plan_digest: certification_digest(
            &recovered_aspect_disjoint_planning.digest_basis.lowered_plan,
        ),
        controller_sequence_merge_conflict_digest: certification_digest(
            &recovered_controller_sequence_planning.digest_basis.conflict,
        ),
        controller_sequence_merge_lowered_plan_digest: certification_digest(
            &recovered_controller_sequence_planning
                .digest_basis
                .lowered_plan,
        ),
        main_replay_digest: full_replay_digest(&recovered_main_replay),
        feature_replay_digest: full_replay_digest(&recovered_feature_replay),
        controller_sequence_noop_digest: certification_digest(&(
            certification_digest(
                recovered
                    .replay_access()
                    .canonical_commit_envelope(
                        controller_feature_idempotent_commit.commit.commit_id,
                    )
                    .expect("recovered controller noop envelope")
                    .strategy_artifacts
                    .as_ref()
                    .expect("recovered controller noop strategy artifacts"),
            ),
            recovered
                .replay_access()
                .canonical_commit_envelope(controller_feature_idempotent_commit.commit.commit_id)
                .expect("recovered controller noop envelope")
                .patch
                .records
                .len(),
            recovered
                .replay_access()
                .canonical_commit_envelope(controller_feature_idempotent_commit.commit.commit_id)
                .expect("recovered controller noop envelope")
                .patch
                .records
                .len(),
        )),
        replacement_replay_digest: replacement_certification.replacement_replay_digest.clone(),
        replacement_lineage_digest: replacement_certification.replacement_lineage_digest.clone(),
        missing_executor_replay_digest: full_replay_digest(&missing_executor_replay),
        failing_executor_replay_digest: full_replay_digest(&failing_executor_replay),
        branch_heads_digest: certification_digest(&(
            recovered
                .history_access()
                .branch_head(&BranchId("main".to_string()))
                .cloned(),
            recovered
                .history_access()
                .branch_head(&feature_branch)
                .cloned(),
        )),
        visible_truth_digest: certification_digest(&(
            read_entity_name(
                recovered_current
                    .get_entity(entity)
                    .expect("recovered entity visible"),
            )
            .map(str::to_string),
            recovered
                .history_access()
                .branch_head(&BranchId("main".to_string()))
                .cloned(),
            recovered
                .history_access()
                .branch_head(&feature_branch)
                .cloned(),
        )),
    };
    assert_eq!(recovered_bundle, live_bundle);
    live_bundle
}

fn run_replacement_strategy_certification() -> ReplacementCertificationBundle {
    let root_path = unique_test_store_path("forge-relational-strategy-replacement-cert");
    let recovered_root = root_path.clone();
    let mut runtime = persisted_replacement_strategy_runtime(root_path);
    let replacement_entity = create_entity(&mut runtime, "replace-target");
    let replacement_start_lineage = runtime
        .lineage_access()
        .for_record(replacement_entity)
        .expect("replacement entity lineage before strategy")
        .lineage_id;
    let replacement_commit = execute_strategy_commit(
        &mut runtime,
        EntityReplacementReconciliationStrategy::DEFAULT_SEMANTIC_NAME,
        serde_json::to_value(EntityReplacementReconciliationInput {
            entity_id: replacement_entity,
            replacement_client_key: "replace-target-v2".to_string(),
            desired_payload: json!({"name":"replace-main","replicas":2}),
        })
        .expect("replacement input value"),
        None,
    );
    let current = runtime
        .visibility_reads()
        .read_version(runtime.current_version_id());
    let replacement_record = changed_entities(&replacement_commit)
        .into_iter()
        .find_map(|entity_id| current.get_entity(entity_id).map(|record| record.entity_id))
        .expect("replacement entity visible after strategy");
    let replacement_end_lineage = runtime
        .lineage_access()
        .for_record(replacement_record)
        .expect("replacement entity lineage after strategy")
        .lineage_id;
    assert_ne!(replacement_start_lineage, replacement_end_lineage);
    let replacement_envelope = runtime
        .replay_access()
        .canonical_commit_envelope(replacement_commit.commit.commit_id)
        .cloned()
        .expect("replacement envelope");
    assert!(replacement_envelope.lineage_decision_log().iter().any(
        |decision| decision.kind == crate::lineage::data::LineageDecisionKind::ReplaceAccepted
    ));
    let replacement_replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: replacement_commit.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::AuditRecoveryVerification,
        });
    assert!(
        replacement_replay.failure.is_none(),
        "replacement replay failed: {replacement_replay:?}"
    );
    assert!(replacement_replay
        .compared_surfaces
        .contains(&ReplayObservableSurface::Strategy));
    let live_bundle = ReplacementCertificationBundle {
        replacement_commit_strategy_digest: certification_digest(
            replacement_commit
                .publication
                .strategy_artifacts
                .as_ref()
                .expect("replacement strategy artifacts"),
        ),
        replacement_replay_digest: full_replay_digest(&replacement_replay),
        replacement_lineage_digest: certification_digest(&(
            replacement_start_lineage,
            replacement_end_lineage,
            replacement_envelope.lineage_digest_basis(),
            replacement_envelope.event_batch_digest_basis(),
            replacement_envelope.decision_log_digest_basis(),
            replacement_commit
                .publication
                .strategy_artifacts
                .as_ref()
                .expect("replacement strategy artifacts")
                .lowering_summary()
                .normalized_client_key_count(),
            replacement_commit
                .publication
                .strategy_artifacts
                .as_ref()
                .expect("replacement strategy artifacts")
                .lowering_summary()
                .lineage_transition_count(),
        )),
    };
    let (_recovery, mut recovered) = checkpoint_and_recover_with(&mut runtime, || {
        persisted_replacement_strategy_runtime(recovered_root)
    });
    let recovered_replacement_envelope = recovered
        .replay_access()
        .canonical_commit_envelope(replacement_commit.commit.commit_id)
        .cloned()
        .expect("recovered replacement envelope");
    let recovered_replacement_replay =
        recovered
            .replay_authority()
            .replay_commit(RelationalReplayRequest {
                commit_id: replacement_commit.commit.commit_id,
                branch_id: BranchId("main".to_string()),
                execution_mode: ReplayExecutionMode::SerialDeterministic,
                verification_mode: ReplayVerificationMode::AuditRecoveryVerification,
            });
    assert!(
        recovered_replacement_replay.failure.is_none(),
        "recovered replacement replay failed: {recovered_replacement_replay:?}"
    );
    let recovered_replacement_lineage = recovered
        .lineage_access()
        .for_record(replacement_record)
        .expect("recovered replacement entity lineage")
        .lineage_id;
    let recovered_bundle = ReplacementCertificationBundle {
        replacement_commit_strategy_digest: certification_digest(
            recovered_replacement_envelope
                .strategy_artifacts
                .as_ref()
                .expect("recovered replacement strategy artifacts"),
        ),
        replacement_replay_digest: full_replay_digest(&recovered_replacement_replay),
        replacement_lineage_digest: certification_digest(&(
            replacement_start_lineage,
            recovered_replacement_lineage,
            recovered_replacement_envelope.lineage_digest_basis(),
            recovered_replacement_envelope.event_batch_digest_basis(),
            recovered_replacement_envelope.decision_log_digest_basis(),
            recovered_replacement_envelope
                .strategy_artifacts
                .as_ref()
                .expect("recovered replacement strategy artifacts")
                .lowering_summary()
                .normalized_client_key_count(),
            recovered_replacement_envelope
                .strategy_artifacts
                .as_ref()
                .expect("recovered replacement strategy artifacts")
                .lowering_summary()
                .lineage_transition_count(),
        )),
    };
    assert_eq!(recovered_bundle, live_bundle);
    live_bundle
}

#[test]
fn milestone_8_5_strategy_certification_preserves_merge_replay_and_recovery_truth() {
    let certification = run_strategy_merge_certification();
    assert!(certification.main_commit_strategy_digest.len() > 8);
    assert!(certification.feature_commit_strategy_digest.len() > 8);
    assert!(certification.replacement_commit_strategy_digest.len() > 8);
    assert!(certification.merge_conflict_digest.len() > 8);
    assert!(certification.merge_lowered_plan_digest.len() > 8);
    assert!(certification.aspect_overlap_merge_conflict_digest.len() > 8);
    assert!(certification.aspect_overlap_merge_lowered_plan_digest.len() > 8);
    assert!(certification.aspect_disjoint_merge_conflict_digest.len() > 8);
    assert!(
        certification
            .aspect_disjoint_merge_lowered_plan_digest
            .len()
            > 8
    );
    assert!(
        certification
            .controller_sequence_merge_conflict_digest
            .len()
            > 8
    );
    assert!(
        certification
            .controller_sequence_merge_lowered_plan_digest
            .len()
            > 8
    );
    assert!(certification.main_replay_digest.len() > 8);
    assert!(certification.feature_replay_digest.len() > 8);
    assert!(certification.controller_sequence_noop_digest.len() > 8);
    assert!(certification.replacement_replay_digest.len() > 8);
    assert!(certification.replacement_lineage_digest.len() > 8);
    assert!(certification.missing_executor_replay_digest.len() > 8);
    assert!(certification.failing_executor_replay_digest.len() > 8);
    assert!(certification.branch_heads_digest.len() > 8);
    assert!(certification.visible_truth_digest.len() > 8);
}
