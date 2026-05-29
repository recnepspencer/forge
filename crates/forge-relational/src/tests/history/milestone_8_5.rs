use crate::commit_strategies::data::{
    CommitStrategyExecutionRegistration, CommitStrategyExecutor, CommitStrategyRegistration,
    StrategyCommitArtifactBundle, StrategyExecutionResult, StrategyExecutorFailure,
    StrategyExecutorFailureClass, StrategyObservationContext,
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
use crate::facade::history::{BranchId, CommitReference};
use crate::facade::merge::{MergeIntent, MergePlanningRequest};
use crate::facade::replay::{
    RelationalReplayOutcome, RelationalReplayRequest, ReplayExecutionMode, ReplayMismatchClass,
    ReplayObservableSurface, ReplayVerificationMode,
};
use crate::facade::runtime::{RelationalRuntime, RelationalRuntimeApi};
use crate::facade::transactions::TransactionOptions;
use crate::tests::support::{
    changed_entities, checkpoint_and_recover_with, create_branch_from_main, create_entity,
    entity_field_aspect, entity_u64_field_aspect, lifecycle_aspect, read_entity_name,
    unique_test_store_path, AspectSchemaFixture,
};
use crate::transactions::data::{AspectFieldPatch, AspectFieldPatchTarget};
use forge_foundational::facade::{
    AspectFieldLocator, AspectKey, AspectValue, CanonicalFieldPath, FieldKey, InternedString,
    LocatorAuthority,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct StrategyCertificationBundle {
    main_commit_strategy_artifacts: StrategyCommitArtifactBundle,
    feature_commit_strategy_artifacts: StrategyCommitArtifactBundle,
    replacement: ReplacementCertificationBundle,
    merge_conflict: crate::merge::data::MergeConflictDigestBasis,
    merge_lowered_plan: crate::merge::data::MergeLoweredPlanDigestBasis,
    aspect_overlap_merge_conflict: crate::merge::data::MergeConflictDigestBasis,
    aspect_overlap_merge_lowered_plan: crate::merge::data::MergeLoweredPlanDigestBasis,
    aspect_disjoint_merge_conflict: crate::merge::data::MergeConflictDigestBasis,
    aspect_disjoint_merge_lowered_plan: crate::merge::data::MergeLoweredPlanDigestBasis,
    controller_sequence_merge_conflict: crate::merge::data::MergeConflictDigestBasis,
    controller_sequence_merge_lowered_plan: crate::merge::data::MergeLoweredPlanDigestBasis,
    main_replay: RelationalReplayOutcome,
    feature_replay: RelationalReplayOutcome,
    controller_sequence_noop: ControllerSequenceNoopEvidence,
    missing_executor_replay: StrategyReplayMismatchEvidence,
    failing_executor_replay: StrategyReplayMismatchEvidence,
    branch_heads: StrategyBranchHeadEvidence,
    visible_truth: StrategyVisibleTruthEvidence,
}

fn strategy_field_locator(aspect_key: AspectKey, field_key: FieldKey) -> AspectFieldLocator {
    AspectFieldLocator::new(
        LocatorAuthority::Planned,
        aspect_key,
        CanonicalFieldPath::single(field_key),
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReplacementCertificationBundle {
    replacement_commit_strategy_artifacts: StrategyCommitArtifactBundle,
    replacement_replay: RelationalReplayOutcome,
    replacement_lineage: ReplacementLineageEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ControllerSequenceNoopEvidence {
    strategy_artifacts: StrategyCommitArtifactBundle,
    changed_record_count: usize,
    patch_record_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StrategyReplayMismatchEvidence {
    strategy_surface_mismatch_present: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StrategyBranchHeadEvidence {
    main: Option<CommitReference>,
    feature: Option<CommitReference>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StrategyVisibleTruthEvidence {
    entity_name: Option<String>,
    branch_heads: StrategyBranchHeadEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReplacementLineageEvidence {
    start_lineage: crate::facade::identity::LineageId,
    end_lineage: crate::facade::identity::LineageId,
    lineage_basis: crate::lineage::data::LineageDigestBasis,
    event_batch_basis: crate::lineage::data::LineageEventBatchDigestBasis,
    decision_log_basis: crate::lineage::data::LineageDecisionLogDigestBasis,
    normalized_client_key_count: usize,
    lineage_transition_count: usize,
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

fn strategy_name_and_replicas_patch(name: &str, replicas: u64) -> AspectFieldPatch {
    AspectFieldPatch::from(std::collections::BTreeMap::from([
        (
            AspectFieldPatchTarget::single(
                AspectKey::new("name").expect("valid name aspect key"),
                FieldKey::new("name").expect("valid name field key"),
            ),
            AspectValue::String(InternedString::Raw(name.to_string())),
        ),
        (
            AspectFieldPatchTarget::single(
                AspectKey::new("replicas").expect("valid replicas aspect key"),
                FieldKey::new("replicas").expect("valid replicas field key"),
            ),
            AspectValue::UInt64(replicas),
        ),
    ]))
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
            entity_field_aspect(
                crate::tests::support::aspect_key("name"),
                crate::tests::support::field_key("name"),
            ),
            entity_u64_field_aspect(
                crate::tests::support::aspect_key("replicas"),
                crate::tests::support::field_key("replicas"),
            ),
            lifecycle_aspect(),
        ],
        ..AspectSchemaFixture::default()
    }
    .build_registry()
}

fn execute_strategy_commit(
    runtime: &mut RelationalRuntime,
    request: RawStrategyCommitRequest,
    target_branch: Option<BranchId>,
) -> crate::facade::transactions::CommitResult {
    let request = runtime
        .commit_strategies()
        .canonicalize_request(&request)
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
        IntentReconciliationInput {
            entity_id: entity,
            desired_fields: strategy_name_and_replicas_patch("service-main", 1),
        }
        .into_native_canonical_request(crate::facade::commit_strategies::StrategyCallerProvenance {
            request_origin: crate::facade::commit_strategies::StrategyRequestOrigin::Test,
            actor_identity: None,
            correlation_id: None,
        })
        .expect("native canonical strategy request"),
        None,
    );
    let feature_commit = execute_strategy_commit(
        &mut runtime,
        ReplicaConvergenceInput {
            entity_id: entity,
            desired_replicas: 7,
        }
        .into_native_canonical_request(crate::facade::commit_strategies::StrategyCallerProvenance {
            request_origin: crate::facade::commit_strategies::StrategyRequestOrigin::Test,
            actor_identity: None,
            correlation_id: None,
        })
        .expect("native canonical strategy request"),
        Some(feature_branch.clone()),
    );
    let planning = runtime
        .merge()
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
        AspectFieldReconciliationInput {
            entity_id: aspect_overlap_entity,
            field_locator: strategy_field_locator(
                crate::tests::support::aspect_key("name"),
                crate::tests::support::field_key("name"),
            ),
            desired_value: forge_foundational::facade::AspectValue::String("aspect-main".into()),
        }
        .into_native_canonical_request(crate::facade::commit_strategies::StrategyCallerProvenance {
            request_origin: crate::facade::commit_strategies::StrategyRequestOrigin::Test,
            actor_identity: None,
            correlation_id: None,
        })
        .expect("native canonical strategy request"),
        None,
    );
    let _aspect_overlap_feature_commit = execute_strategy_commit(
        &mut runtime,
        AspectFieldReconciliationInput {
            entity_id: aspect_overlap_entity,
            field_locator: strategy_field_locator(
                crate::tests::support::aspect_key("name"),
                crate::tests::support::field_key("name"),
            ),
            desired_value: forge_foundational::facade::AspectValue::String("aspect-feature".into()),
        }
        .into_native_canonical_request(crate::facade::commit_strategies::StrategyCallerProvenance {
            request_origin: crate::facade::commit_strategies::StrategyRequestOrigin::Test,
            actor_identity: None,
            correlation_id: None,
        })
        .expect("native canonical strategy request"),
        Some(aspect_overlap_branch.clone()),
    );
    let aspect_overlap_planning = runtime
        .merge()
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
        AspectFieldReconciliationInput {
            entity_id: aspect_disjoint_entity,
            field_locator: strategy_field_locator(
                crate::tests::support::aspect_key("name"),
                crate::tests::support::field_key("name"),
            ),
            desired_value: forge_foundational::facade::AspectValue::String("disjoint-main".into()),
        }
        .into_native_canonical_request(crate::facade::commit_strategies::StrategyCallerProvenance {
            request_origin: crate::facade::commit_strategies::StrategyRequestOrigin::Test,
            actor_identity: None,
            correlation_id: None,
        })
        .expect("native canonical strategy request"),
        None,
    );
    let _aspect_disjoint_feature_commit = execute_strategy_commit(
        &mut runtime,
        ReplicaConvergenceInput {
            entity_id: aspect_disjoint_entity,
            desired_replicas: 9,
        }
        .into_native_canonical_request(crate::facade::commit_strategies::StrategyCallerProvenance {
            request_origin: crate::facade::commit_strategies::StrategyRequestOrigin::Test,
            actor_identity: None,
            correlation_id: None,
        })
        .expect("native canonical strategy request"),
        Some(aspect_disjoint_branch.clone()),
    );
    let aspect_disjoint_planning = runtime
        .merge()
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
        IntentReconciliationInput {
            entity_id: controller_sequence_entity,
            desired_fields: strategy_name_and_replicas_patch("controller-main", 2),
        }
        .into_native_canonical_request(crate::facade::commit_strategies::StrategyCallerProvenance {
            request_origin: crate::facade::commit_strategies::StrategyRequestOrigin::Test,
            actor_identity: None,
            correlation_id: None,
        })
        .expect("native canonical strategy request"),
        None,
    );
    let _controller_feature_converge = execute_strategy_commit(
        &mut runtime,
        ReplicaConvergenceInput {
            entity_id: controller_sequence_entity,
            desired_replicas: 7,
        }
        .into_native_canonical_request(crate::facade::commit_strategies::StrategyCallerProvenance {
            request_origin: crate::facade::commit_strategies::StrategyRequestOrigin::Test,
            actor_identity: None,
            correlation_id: None,
        })
        .expect("native canonical strategy request"),
        Some(controller_sequence_branch.clone()),
    );
    let _controller_narrowed_intent = execute_strategy_commit(
        &mut runtime,
        IntentReconciliationInput {
            entity_id: controller_sequence_entity,
            desired_fields: crate::transactions::data::AspectFieldPatch::single(
                forge_foundational::facade::AspectKey::new("name").expect("valid test aspect key"),
                forge_foundational::facade::FieldKey::new("name").expect("valid test field key"),
                forge_foundational::facade::AspectValue::String(
                    forge_foundational::facade::InternedString::Raw(
                        "controller-renamed".to_string(),
                    ),
                ),
            ),
        }
        .into_native_canonical_request(crate::facade::commit_strategies::StrategyCallerProvenance {
            request_origin: crate::facade::commit_strategies::StrategyRequestOrigin::Test,
            actor_identity: None,
            correlation_id: None,
        })
        .expect("native canonical strategy request"),
        None,
    );
    let controller_feature_idempotent_commit = execute_strategy_commit(
        &mut runtime,
        ReplicaConvergenceInput {
            entity_id: controller_sequence_entity,
            desired_replicas: 7,
        }
        .into_native_canonical_request(crate::facade::commit_strategies::StrategyCallerProvenance {
            request_origin: crate::facade::commit_strategies::StrategyRequestOrigin::Test,
            actor_identity: None,
            correlation_id: None,
        })
        .expect("native canonical strategy request"),
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
        .merge()
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
        .merge()
        .inspect_planning_scope(MergePlanningRequest::new(
            BranchId("main".to_string()),
            feature_branch.clone(),
            MergeIntent::ReconcileIntoTarget,
        ))
        .expect("final merge planning");
    let aspect_overlap_planning = runtime
        .merge()
        .inspect_planning_scope(MergePlanningRequest::new(
            BranchId("main".to_string()),
            aspect_overlap_branch.clone(),
            MergeIntent::ReconcileIntoTarget,
        ))
        .expect("final aspect overlap merge planning");
    let aspect_disjoint_planning = runtime
        .merge()
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
        .read_truth()
        .read_version(runtime.current_version_id());
    let live_branch_heads = StrategyBranchHeadEvidence {
        main: runtime
            .history()
            .branch_head(&BranchId("main".to_string()))
            .cloned(),
        feature: runtime.history().branch_head(&feature_branch).cloned(),
    };
    let live_visible_truth = StrategyVisibleTruthEvidence {
        entity_name: read_entity_name(current.get_entity(entity).expect("entity visible")),
        branch_heads: live_branch_heads.clone(),
    };

    let mut live_bundle = StrategyCertificationBundle {
        main_commit_strategy_artifacts: main_commit
            .publication
            .strategy_artifacts
            .as_ref()
            .expect("main strategy artifacts")
            .clone(),
        feature_commit_strategy_artifacts: feature_commit
            .publication
            .strategy_artifacts
            .as_ref()
            .expect("feature strategy artifacts")
            .clone(),
        replacement: replacement_certification.clone(),
        merge_conflict: planning.digest_basis.conflict.clone(),
        merge_lowered_plan: planning.digest_basis.lowered_plan.clone(),
        aspect_overlap_merge_conflict: aspect_overlap_planning.digest_basis.conflict.clone(),
        aspect_overlap_merge_lowered_plan: aspect_overlap_planning
            .digest_basis
            .lowered_plan
            .clone(),
        aspect_disjoint_merge_conflict: aspect_disjoint_planning.digest_basis.conflict.clone(),
        aspect_disjoint_merge_lowered_plan: aspect_disjoint_planning
            .digest_basis
            .lowered_plan
            .clone(),
        controller_sequence_merge_conflict: controller_sequence_planning
            .digest_basis
            .conflict
            .clone(),
        controller_sequence_merge_lowered_plan: controller_sequence_planning
            .digest_basis
            .lowered_plan
            .clone(),
        main_replay,
        feature_replay,
        controller_sequence_noop: ControllerSequenceNoopEvidence {
            strategy_artifacts: controller_feature_idempotent_commit
                .publication
                .strategy_artifacts
                .as_ref()
                .expect("controller idempotent strategy artifacts")
                .clone(),
            changed_record_count: controller_feature_idempotent_commit
                .change_summary()
                .expect("controller idempotent change summary")
                .changed_record_count,
            patch_record_count: controller_feature_idempotent_commit
                .publication_summary()
                .expect("controller idempotent publication summary")
                .patch_record_count,
        },
        missing_executor_replay: StrategyReplayMismatchEvidence {
            strategy_surface_mismatch_present: false,
        },
        failing_executor_replay: StrategyReplayMismatchEvidence {
            strategy_surface_mismatch_present: false,
        },
        branch_heads: live_branch_heads,
        visible_truth: live_visible_truth,
    };

    let recovery_plan = runtime.durability().recovery_plan(
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
    let missing_executor_mismatch_present =
        missing_executor_replay.mismatches.iter().any(|mismatch| {
            mismatch.class == ReplayMismatchClass::StrategyExecutorUnavailable
                && mismatch.surface == ReplayObservableSurface::Strategy
        });

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
    let failing_executor_mismatch_present =
        failing_executor_replay.mismatches.iter().any(|mismatch| {
            mismatch.class == ReplayMismatchClass::StrategyExecutionFailure
                && mismatch.surface == ReplayObservableSurface::Strategy
        });
    live_bundle.missing_executor_replay = StrategyReplayMismatchEvidence {
        strategy_surface_mismatch_present: missing_executor_mismatch_present,
    };
    live_bundle.failing_executor_replay = StrategyReplayMismatchEvidence {
        strategy_surface_mismatch_present: failing_executor_mismatch_present,
    };

    let (_recovery, mut recovered) =
        checkpoint_and_recover_with(&mut runtime, || persisted_strategy_runtime(recovered_root));

    let recovered_planning = recovered
        .merge()
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
        .merge()
        .inspect_planning_scope(MergePlanningRequest::new(
            BranchId("main".to_string()),
            aspect_overlap_branch.clone(),
            MergeIntent::ReconcileIntoTarget,
        ))
        .expect("recovered aspect overlap planning");
    let recovered_aspect_disjoint_planning = recovered
        .merge()
        .inspect_planning_scope(MergePlanningRequest::new(
            BranchId("main".to_string()),
            aspect_disjoint_branch.clone(),
            MergeIntent::ReconcileIntoTarget,
        ))
        .expect("recovered aspect disjoint planning");
    let recovered_controller_sequence_planning = recovered
        .merge()
        .inspect_planning_scope(MergePlanningRequest::new(
            BranchId("main".to_string()),
            controller_sequence_branch.clone(),
            MergeIntent::ReconcileIntoTarget,
        ))
        .expect("recovered controller sequence planning");
    let recovered_main_envelope = recovered
        .replay()
        .canonical_commit_envelope(main_commit.commit.commit_id)
        .cloned()
        .expect("recovered main envelope");
    let recovered_feature_envelope = recovered
        .replay()
        .canonical_commit_envelope(feature_commit.commit.commit_id)
        .cloned()
        .expect("recovered feature envelope");
    let recovered_current = recovered
        .read_truth()
        .read_version(recovered.current_version_id());
    let recovered_controller_noop_envelope = recovered
        .replay()
        .canonical_commit_envelope(controller_feature_idempotent_commit.commit.commit_id)
        .cloned()
        .expect("recovered controller noop envelope");
    let recovered_branch_heads = StrategyBranchHeadEvidence {
        main: recovered
            .history()
            .branch_head(&BranchId("main".to_string()))
            .cloned(),
        feature: recovered.history().branch_head(&feature_branch).cloned(),
    };
    let recovered_visible_truth = StrategyVisibleTruthEvidence {
        entity_name: read_entity_name(
            recovered_current
                .get_entity(entity)
                .expect("recovered entity visible"),
        ),
        branch_heads: recovered_branch_heads.clone(),
    };
    let recovered_bundle = StrategyCertificationBundle {
        main_commit_strategy_artifacts: recovered_main_envelope
            .strategy_artifacts
            .as_ref()
            .expect("recovered main strategy artifacts")
            .clone(),
        feature_commit_strategy_artifacts: recovered_feature_envelope
            .strategy_artifacts
            .as_ref()
            .expect("recovered feature strategy artifacts")
            .clone(),
        replacement: replacement_certification,
        merge_conflict: recovered_planning.digest_basis.conflict.clone(),
        merge_lowered_plan: recovered_planning.digest_basis.lowered_plan.clone(),
        aspect_overlap_merge_conflict: recovered_aspect_overlap_planning
            .digest_basis
            .conflict
            .clone(),
        aspect_overlap_merge_lowered_plan: recovered_aspect_overlap_planning
            .digest_basis
            .lowered_plan
            .clone(),
        aspect_disjoint_merge_conflict: recovered_aspect_disjoint_planning
            .digest_basis
            .conflict
            .clone(),
        aspect_disjoint_merge_lowered_plan: recovered_aspect_disjoint_planning
            .digest_basis
            .lowered_plan
            .clone(),
        controller_sequence_merge_conflict: recovered_controller_sequence_planning
            .digest_basis
            .conflict
            .clone(),
        controller_sequence_merge_lowered_plan: recovered_controller_sequence_planning
            .digest_basis
            .lowered_plan
            .clone(),
        main_replay: recovered_main_replay,
        feature_replay: recovered_feature_replay,
        controller_sequence_noop: ControllerSequenceNoopEvidence {
            strategy_artifacts: recovered_controller_noop_envelope
                .strategy_artifacts
                .as_ref()
                .expect("recovered controller noop strategy artifacts")
                .clone(),
            changed_record_count: recovered_controller_noop_envelope.patch.records.len(),
            patch_record_count: recovered_controller_noop_envelope.patch.records.len(),
        },
        missing_executor_replay: StrategyReplayMismatchEvidence {
            strategy_surface_mismatch_present: missing_executor_mismatch_present,
        },
        failing_executor_replay: StrategyReplayMismatchEvidence {
            strategy_surface_mismatch_present: failing_executor_mismatch_present,
        },
        branch_heads: recovered_branch_heads,
        visible_truth: recovered_visible_truth,
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
        EntityReplacementReconciliationInput {
            entity_id: replacement_entity,
            replacement_client_key: "replace-target-v2".to_string(),
            desired_fields: strategy_name_and_replicas_patch("replace-main", 2),
        }
        .into_native_canonical_request(crate::facade::commit_strategies::StrategyCallerProvenance {
            request_origin: crate::facade::commit_strategies::StrategyRequestOrigin::Test,
            actor_identity: None,
            correlation_id: None,
        })
        .expect("native canonical strategy request"),
        None,
    );
    let current = runtime
        .read_truth()
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
        .replay()
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
    let replacement_strategy_artifacts = replacement_commit
        .publication
        .strategy_artifacts
        .as_ref()
        .expect("replacement strategy artifacts");
    let live_bundle = ReplacementCertificationBundle {
        replacement_commit_strategy_artifacts: replacement_strategy_artifacts.clone(),
        replacement_replay,
        replacement_lineage: ReplacementLineageEvidence {
            start_lineage: replacement_start_lineage,
            end_lineage: replacement_end_lineage,
            lineage_basis: replacement_envelope.lineage_digest_basis().clone(),
            event_batch_basis: replacement_envelope.event_batch_digest_basis().clone(),
            decision_log_basis: replacement_envelope.decision_log_digest_basis().clone(),
            normalized_client_key_count: replacement_strategy_artifacts
                .lowering_summary()
                .normalized_client_key_count(),
            lineage_transition_count: replacement_strategy_artifacts
                .lowering_summary()
                .lineage_transition_count(),
        },
    };
    let (_recovery, mut recovered) = checkpoint_and_recover_with(&mut runtime, || {
        persisted_replacement_strategy_runtime(recovered_root)
    });
    let recovered_replacement_envelope = recovered
        .replay()
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
    let recovered_replacement_strategy_artifacts = recovered_replacement_envelope
        .strategy_artifacts
        .as_ref()
        .expect("recovered replacement strategy artifacts");
    let recovered_bundle = ReplacementCertificationBundle {
        replacement_commit_strategy_artifacts: recovered_replacement_strategy_artifacts.clone(),
        replacement_replay: recovered_replacement_replay,
        replacement_lineage: ReplacementLineageEvidence {
            start_lineage: replacement_start_lineage,
            end_lineage: recovered_replacement_lineage,
            lineage_basis: recovered_replacement_envelope
                .lineage_digest_basis()
                .clone(),
            event_batch_basis: recovered_replacement_envelope
                .event_batch_digest_basis()
                .clone(),
            decision_log_basis: recovered_replacement_envelope
                .decision_log_digest_basis()
                .clone(),
            normalized_client_key_count: recovered_replacement_strategy_artifacts
                .lowering_summary()
                .normalized_client_key_count(),
            lineage_transition_count: recovered_replacement_strategy_artifacts
                .lowering_summary()
                .lineage_transition_count(),
        },
    };
    assert_eq!(recovered_bundle, live_bundle);
    live_bundle
}

#[test]
fn milestone_8_5_strategy_certification_preserves_merge_replay_and_recovery_truth() {
    let certification = run_strategy_merge_certification();
    assert!(
        certification
            .main_commit_strategy_artifacts
            .lowering_summary()
            .total_intent_count()
            > 0
    );
    assert!(
        certification
            .feature_commit_strategy_artifacts
            .lowering_summary()
            .total_intent_count()
            > 0
    );
    assert!(
        certification
            .replacement
            .replacement_commit_strategy_artifacts
            .lowering_summary()
            .lineage_transition_count()
            > 0
    );
    assert!(!certification.merge_conflict.records.is_empty());
    assert!(!certification.merge_lowered_plan.records.is_empty());
    assert!(!certification
        .aspect_overlap_merge_conflict
        .records
        .is_empty());
    assert!(!certification
        .aspect_overlap_merge_lowered_plan
        .records
        .is_empty());
    assert!(!certification
        .aspect_disjoint_merge_conflict
        .records
        .is_empty());
    assert!(!certification
        .aspect_disjoint_merge_lowered_plan
        .records
        .is_empty());
    assert!(!certification
        .controller_sequence_merge_conflict
        .records
        .is_empty());
    assert!(!certification
        .controller_sequence_merge_lowered_plan
        .records
        .is_empty());
    assert!(certification.main_replay.failure.is_none());
    assert!(certification.feature_replay.failure.is_none());
    assert!(certification
        .main_replay
        .compared_surfaces
        .contains(&ReplayObservableSurface::Strategy));
    assert!(certification
        .feature_replay
        .compared_surfaces
        .contains(&ReplayObservableSurface::Strategy));
    assert_eq!(
        certification.controller_sequence_noop.changed_record_count,
        certification.controller_sequence_noop.patch_record_count
    );
    assert!(certification
        .replacement
        .replacement_replay
        .failure
        .is_none());
    assert!(certification
        .replacement
        .replacement_replay
        .compared_surfaces
        .contains(&ReplayObservableSurface::Strategy));
    assert_ne!(
        certification.replacement.replacement_lineage.start_lineage,
        certification.replacement.replacement_lineage.end_lineage
    );
    assert!(
        certification
            .missing_executor_replay
            .strategy_surface_mismatch_present
    );
    assert!(
        certification
            .failing_executor_replay
            .strategy_surface_mismatch_present
    );
    assert!(certification.branch_heads.main.is_some());
    assert!(certification.branch_heads.feature.is_some());
    assert_eq!(
        certification.visible_truth.branch_heads,
        certification.branch_heads
    );
    assert_eq!(
        certification.visible_truth.entity_name.as_deref(),
        Some("service-main")
    );
}
