use std::collections::BTreeSet;
use std::sync::Arc;

use crate::capabilities::DurabilityRead;
use crate::commit_strategies::data::StrategyCommitArtifactBundle;
use crate::commit_strategies::strategies::{
    IntentReconciliationInput, IntentReconciliationStrategy, ReplicaConvergenceInput,
    ReplicaConvergenceStrategy,
};
use crate::facade::commit_strategies::NativeStrategyCommitRequest;
use crate::facade::config::RelationalRuntimeProfile;
use crate::facade::durability::{DurabilityMode, DurableStoreLayout};
use crate::facade::history::{BranchId, CommitReference};
use crate::facade::merge::{MergeIntent, MergePlanningRequest};
use crate::facade::replay::{
    RelationalReplayOutcome, RelationalReplayRequest, ReplayExecutionMode, ReplayObservableSurface,
    ReplayVerificationMode,
};
use crate::facade::runtime::{RelationalRuntime, RelationalRuntimeApi};
use crate::facade::transactions::{CommitResult, TransactionOptions};
use crate::tests::support::{
    checkpoint_and_recover_with, create_branch_from_main, create_entity, entity_field_aspect,
    entity_u64_field_aspect, lifecycle_aspect, read_entity_name, unique_test_store_path,
    AspectSchemaFixture,
};
use crate::transactions::data::AspectFieldPatch;
use forge_foundational::facade::{
    AspectFieldLocator, AspectKey, AspectValue, CanonicalFieldPath, FieldKey, InternedString,
    LocatorAuthority,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct KubernetesIntentCertificationBundle {
    overlap_conflict: KubernetesPlanningEvidence,
    narrowed_non_conflict: KubernetesPlanningEvidence,
    rebroadened_conflict: KubernetesPlanningEvidence,
    revalidated_shared_truth: KubernetesPlanningEvidence,
    revalidation_noop: KubernetesNoopEvidence,
    broad_intent_replay: RelationalReplayOutcome,
    first_converge_replay: RelationalReplayOutcome,
    rebroadened_intent_replay: RelationalReplayOutcome,
    revalidation_noop_replay: RelationalReplayOutcome,
    branch_heads: KubernetesBranchHeadEvidence,
    visible_truth: KubernetesVisibleTruthEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct KubernetesPlanningEvidence {
    conflict: KubernetesConflictEvidence,
    lowered_plan: crate::merge::data::MergeLoweredPlanDigestBasis,
    decision_log: crate::merge::data::MergePlanningDecisionLogDigestBasis,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct KubernetesConflictEvidence {
    records: Arc<[crate::facade::transactions::RecordRef]>,
    classes: Arc<[crate::merge::data::MergeConflictClass]>,
    validated_schema_correspondence: Arc<[bool]>,
    strategy_conflict_classes: Arc<[Option<crate::merge::data::StrategyConflictClass>]>,
    source_strategy_descriptors:
        Arc<[Arc<[crate::commit_strategies::data::StrategyMergeDescriptor]>]>,
    target_strategy_descriptors:
        Arc<[Arc<[crate::commit_strategies::data::StrategyMergeDescriptor]>]>,
    relation_evidence: Arc<[Option<crate::merge::data::RelationConflictEvidence>]>,
    aspect_evidence_keys: Arc<[Arc<[forge_foundational::facade::AspectKey]>]>,
    aspect_evidence_comparisons: Arc<[Arc<[crate::merge::data::AspectComparisonState]>]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct KubernetesNoopEvidence {
    strategy_artifacts: StrategyCommitArtifactBundle,
    changed_record_count: usize,
    patch_record_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct KubernetesBranchHeadEvidence {
    main: Option<CommitReference>,
    controller: Option<CommitReference>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct KubernetesVisibleTruthEvidence {
    entity_name: Option<String>,
    replicas_canonical_bytes: Option<Vec<u8>>,
}

fn replicas_canonical_bytes(record: &crate::storage::data::EntityReadRecord) -> Option<Vec<u8>> {
    let locator = AspectFieldLocator::new(
        LocatorAuthority::Planned,
        AspectKey::new("replicas").expect("valid replicas aspect"),
        CanonicalFieldPath::single(FieldKey::new("replicas").expect("valid replicas field")),
    );
    crate::storage::data::entity_authoritative_aspect_field_comparison_key(record, &locator)
        .map(|key| key.canonical_value_bytes().to_vec())
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

fn strategy_name_and_replicas_patch(name: &str, replicas: u64) -> AspectFieldPatch {
    AspectFieldPatch::from(std::collections::BTreeMap::from([
        (
            crate::transactions::data::planned_single_field_locator(
                AspectKey::new("name").expect("valid name aspect key"),
                FieldKey::new("name").expect("valid name field key"),
            ),
            AspectValue::String(InternedString::Raw(name.to_string())),
        ),
        (
            crate::transactions::data::planned_single_field_locator(
                AspectKey::new("replicas").expect("valid replicas aspect key"),
                FieldKey::new("replicas").expect("valid replicas field key"),
            ),
            AspectValue::UInt64(replicas),
        ),
    ]))
}

fn persisted_strategy_runtime(root_path: std::path::PathBuf) -> RelationalRuntime {
    let intent_descriptor = IntentReconciliationStrategy::descriptor(
        crate::facade::commit_strategies::CommitStrategyId(881),
    );
    let replica_descriptor = ReplicaConvergenceStrategy::descriptor(
        crate::facade::commit_strategies::CommitStrategyId(882),
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
        .build()
}

fn execute_strategy_commit(
    runtime: &mut RelationalRuntime,
    request: NativeStrategyCommitRequest,
    target_branch: Option<BranchId>,
) -> CommitResult {
    let request = runtime
        .commit_strategies()
        .canonicalize_request(&request)
        .expect("canonical strategy request");
    let snapshot = if let Some(branch_id) = target_branch.as_ref() {
        let branch_head = runtime
            .history()
            .branch_head(branch_id)
            .cloned()
            .expect("target branch head for strategy snapshot");
        if branch_head.version_id == runtime.current_version_id() {
            runtime.visibility_authority().snapshot()
        } else {
            runtime
                .visibility_authority()
                .pin_snapshot(branch_head.version_id)
                .expect("pin target branch strategy snapshot")
                .handle()
                .clone()
        }
    } else {
        runtime.visibility_authority().snapshot()
    };
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

fn planning_for(
    runtime: &RelationalRuntime,
    source_branch: BranchId,
    target_branch: BranchId,
) -> crate::merge::data::MergePlanningArtifactCore {
    runtime
        .merge()
        .inspect_planning_scope(MergePlanningRequest::new(
            target_branch,
            source_branch,
            MergeIntent::ReconcileIntoTarget,
        ))
        .expect("merge planning")
}

fn entity_classification<'a>(
    planning: &'a crate::merge::data::MergePlanningArtifactCore,
    entity: crate::facade::identity::EntityId,
) -> Option<&'a crate::merge::data::MergeConflictClassification> {
    planning
        .conflict_classification
        .classifications
        .iter()
        .find(|classification| {
            classification.record == crate::facade::transactions::RecordRef::Entity(entity)
        })
}

fn assert_strategy_conflict(
    planning: &crate::merge::data::MergePlanningArtifactCore,
    entity: crate::facade::identity::EntityId,
    stage: &str,
) {
    let classification = entity_classification(planning, entity).unwrap_or_else(|| {
        panic!("missing entity classification for strategy-conflict stage {stage}")
    });
    assert!(
        classification.class == crate::merge::data::MergeConflictClass::StrategyIntentConflict,
        "expected strategy intent conflict during {stage}, got {:?}",
        classification.class
    );
    assert!(
        classification.strategy_evidence.is_some(),
        "expected strategy evidence for overlap conflict: {classification:?}"
    );
}

fn assert_non_strategy_conflict(
    planning: &crate::merge::data::MergePlanningArtifactCore,
    entity: crate::facade::identity::EntityId,
    stage: &str,
) {
    let classification = entity_classification(planning, entity)
        .unwrap_or_else(|| panic!("missing entity classification for benign stage {stage}"));
    assert_ne!(
        classification.class,
        crate::merge::data::MergeConflictClass::StrategyIntentConflict
    );
    assert!(
        classification.class == crate::merge::data::MergeConflictClass::ExactSharedTruth,
        "expected benign exact-shared-truth classification during {stage}, got {:?}",
        classification.class
    );
}

fn assert_exact_shared_truth(
    planning: &crate::merge::data::MergePlanningArtifactCore,
    entity: crate::facade::identity::EntityId,
    stage: &str,
) {
    let classification = entity_classification(planning, entity).unwrap_or_else(|| {
        panic!("missing entity classification for exact-shared-truth stage {stage}")
    });
    assert!(
        classification.class == crate::merge::data::MergeConflictClass::ExactSharedTruth,
        "expected exact shared truth during {stage}, got {:?}",
        classification.class
    );
    assert!(
        classification.strategy_evidence.is_some(),
        "expected preserved strategy evidence during exact shared truth stage {stage}: {classification:?}"
    );
}

fn replay_commit(
    runtime: &mut RelationalRuntime,
    commit_id: crate::history::data::CommitId,
    branch_id: BranchId,
) -> crate::facade::replay::RelationalReplayOutcome {
    runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id,
            branch_id,
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::AuditRecoveryVerification,
        })
}

fn assert_strategy_replay_clean(
    replay: &crate::facade::replay::RelationalReplayOutcome,
    stage: &str,
) {
    assert!(
        replay
            .compared_surfaces
            .contains(&ReplayObservableSurface::Strategy),
        "expected strategy replay surface during {stage}: {replay:?}"
    );
    assert!(
        replay
            .mismatches
            .iter()
            .all(|mismatch| mismatch.surface != ReplayObservableSurface::Strategy),
        "unexpected strategy replay mismatch during {stage}: {replay:?}"
    );
}

fn planning_evidence(
    planning: &crate::merge::data::MergePlanningArtifactCore,
) -> KubernetesPlanningEvidence {
    KubernetesPlanningEvidence {
        conflict: KubernetesConflictEvidence {
            records: planning.digest_basis.conflict.records.clone(),
            classes: planning.digest_basis.conflict.classes.clone(),
            validated_schema_correspondence: planning
                .digest_basis
                .conflict
                .validated_schema_correspondence
                .clone(),
            strategy_conflict_classes: planning
                .digest_basis
                .conflict
                .strategy_conflict_classes
                .clone(),
            source_strategy_descriptors: planning
                .digest_basis
                .conflict
                .source_strategy_descriptors
                .clone(),
            target_strategy_descriptors: planning
                .digest_basis
                .conflict
                .target_strategy_descriptors
                .clone(),
            relation_evidence: planning.digest_basis.conflict.relation_evidence.clone(),
            aspect_evidence_keys: planning.digest_basis.conflict.aspect_evidence_keys.clone(),
            aspect_evidence_comparisons: planning
                .digest_basis
                .conflict
                .aspect_evidence_comparisons
                .clone(),
        },
        lowered_plan: planning.digest_basis.lowered_plan.clone(),
        decision_log: planning.decision_log_digest_basis.clone(),
    }
}

fn recover_stage(
    runtime: &mut RelationalRuntime,
    root_path: std::path::PathBuf,
) -> RelationalRuntime {
    let (_recovery, recovered) =
        checkpoint_and_recover_with(runtime, || persisted_strategy_runtime(root_path));
    recovered
}

fn recover_stage_from_final_history(
    source: &RelationalRuntime,
    root_path: std::path::PathBuf,
    source_head: crate::history::data::CommitReference,
    target_head: crate::history::data::CommitReference,
) -> RelationalRuntime {
    let mut chain = source
        .history()
        .ancestor_closure_by_commit_id_order(source_head.commit_id)
        .into_iter()
        .chain(
            source
                .history()
                .ancestor_closure_by_commit_id_order(target_head.commit_id),
        )
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    chain.sort_unstable();
    let replay_access = source.replay();
    let checkpoint = source
        .durable_checkpoints()
        .iter()
        .rev()
        .find(|checkpoint| {
            checkpoint
                .coverage
                .up_to_commit
                .as_ref()
                .map(|commit| chain.contains(&commit.commit_id))
                .unwrap_or(false)
        })
        .cloned();
    let tail_start = checkpoint
        .as_ref()
        .and_then(|checkpoint| checkpoint.coverage.up_to_commit.as_ref())
        .map(|commit| commit.commit_id);
    let tail_log = chain
        .iter()
        .copied()
        .filter(|commit_id| tail_start.is_none_or(|start| *commit_id > start))
        .filter_map(|commit_id| replay_access.canonical_commit_envelope(commit_id))
        .cloned()
        .collect::<Vec<_>>();
    let restore_authoritative_envelope_commit_ids = tail_log
        .iter()
        .filter(|envelope| envelope.strategy_artifacts.is_some())
        .map(|envelope| envelope.commit.commit_id)
        .collect::<Vec<_>>();
    let descriptor_semantics_version = tail_log
        .last()
        .map(|envelope| envelope.descriptor_semantics_version)
        .unwrap_or_default();
    let plan = crate::durability::data::RecoveryPlan::new(
        source.config().clone(),
        source.durable_store().cloned(),
        None,
        checkpoint,
        tail_log,
        crate::durability::data::RecoveryCursor {
            checkpoint_id: None,
            segment_ids: Vec::new(),
        },
        crate::durability::data::RecoveryIntegrityReport {
            selected_checkpoint_id: None,
            skipped_corrupt_checkpoints: Vec::new(),
            verified_segment_ids: Vec::new(),
            corrupt_segment_id: None,
        },
        crate::durability::data::RecoveryCompatibilityCheck::verified_at(
            crate::replay::data::ReplayVerificationLayer::DigestParity,
        ),
        crate::durability::data::RecoveryVerificationMode::AuditRecoveryVerification,
        descriptor_semantics_version,
        restore_authoritative_envelope_commit_ids,
    )
    .with_commit_strategy_executors(source.commit_strategy_executor_registry().clone());
    let mut recovered = persisted_strategy_runtime(root_path);
    recovered
        .durability_authority()
        .recover(plan)
        .expect("recover staged runtime from final history");
    if let Some(base_commit_id) = recovered
        .history()
        .max_commit_id_common_ancestor(source_head.commit_id, target_head.commit_id)
    {
        let base_version_id = recovered
            .history()
            .commit_envelope(base_commit_id)
            .map(|envelope| envelope.commit.version_id);
        if let Some(base_version_id) = base_version_id {
            recovered
                .history_authority()
                .retain_version_for_replay(base_version_id);
        }
    }
    recovered
}

fn run_kubernetes_style_certification() -> KubernetesIntentCertificationBundle {
    let root_path = unique_test_store_path("forge-relational-m8-5-kubernetes-style");
    let recovered_root = root_path.clone();
    let mut runtime = persisted_strategy_runtime(root_path);
    let entity = create_entity(&mut runtime, "kube-service");
    let controller_branch = create_branch_from_main(&mut runtime, "kube-controller");
    let main_branch = BranchId("main".to_string());

    let broad_intent_commit = execute_strategy_commit(
        &mut runtime,
        IntentReconciliationInput {
            entity_id: entity,
            desired_aspect_fields: strategy_name_and_replicas_patch("svc-v1", 3),
        }
        .into_native_canonical_request(crate::facade::commit_strategies::StrategyCallerProvenance {
            request_origin: crate::facade::commit_strategies::StrategyRequestOrigin::Test,
            actor_identity: None,
            correlation_id: None,
        })
        .expect("native canonical strategy request"),
        None,
    );
    let first_converge_commit = execute_strategy_commit(
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
        Some(controller_branch.clone()),
    );
    let overlap_planning = planning_for(&runtime, controller_branch.clone(), main_branch.clone());
    assert_strategy_conflict(&overlap_planning, entity, "overlap");
    let overlap_conflict = planning_evidence(&overlap_planning);
    let overlap_main_head = runtime
        .history()
        .branch_head(&main_branch)
        .cloned()
        .expect("overlap main head");
    let overlap_controller_head = runtime
        .history()
        .branch_head(&controller_branch)
        .cloned()
        .expect("overlap controller head");
    let mut runtime = recover_stage(&mut runtime, recovered_root.clone());
    let recovered_overlap_planning =
        planning_for(&runtime, controller_branch.clone(), main_branch.clone());
    assert_strategy_conflict(&recovered_overlap_planning, entity, "recovered overlap");
    assert_eq!(
        planning_evidence(&recovered_overlap_planning),
        overlap_conflict
    );

    let _narrowed_intent_commit = execute_strategy_commit(
        &mut runtime,
        IntentReconciliationInput {
            entity_id: entity,
            desired_aspect_fields: crate::transactions::data::AspectFieldPatch::from_locator(
                crate::transactions::data::planned_single_field_locator(
                    forge_foundational::facade::AspectKey::new("name")
                        .expect("valid test aspect key"),
                    forge_foundational::facade::FieldKey::new("name")
                        .expect("valid test field key"),
                ),
                forge_foundational::facade::AspectValue::String(
                    forge_foundational::facade::InternedString::Raw("svc-v2".to_string()),
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
    let idempotent_converge_commit = execute_strategy_commit(
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
        Some(controller_branch.clone()),
    );
    assert_eq!(
        idempotent_converge_commit
            .change_summary()
            .expect("idempotent change summary")
            .changed_record_count,
        0
    );
    let narrowed_planning = planning_for(&runtime, controller_branch.clone(), main_branch.clone());
    assert_non_strategy_conflict(&narrowed_planning, entity, "narrowed");
    let narrowed_non_conflict = planning_evidence(&narrowed_planning);
    let narrowed_main_head = runtime
        .history()
        .branch_head(&main_branch)
        .cloned()
        .expect("narrowed main head");
    let narrowed_controller_head = runtime
        .history()
        .branch_head(&controller_branch)
        .cloned()
        .expect("narrowed controller head");
    let mut runtime = recover_stage(&mut runtime, recovered_root.clone());
    let recovered_narrowed_planning =
        planning_for(&runtime, controller_branch.clone(), main_branch.clone());
    assert_non_strategy_conflict(&recovered_narrowed_planning, entity, "recovered narrowed");
    assert_eq!(
        planning_evidence(&recovered_narrowed_planning),
        narrowed_non_conflict
    );

    let rebroadened_intent_commit = execute_strategy_commit(
        &mut runtime,
        IntentReconciliationInput {
            entity_id: entity,
            desired_aspect_fields: strategy_name_and_replicas_patch("svc-v2", 9),
        }
        .into_native_canonical_request(crate::facade::commit_strategies::StrategyCallerProvenance {
            request_origin: crate::facade::commit_strategies::StrategyRequestOrigin::Test,
            actor_identity: None,
            correlation_id: None,
        })
        .expect("native canonical strategy request"),
        None,
    );
    let rebroadened_planning =
        planning_for(&runtime, controller_branch.clone(), main_branch.clone());
    assert_strategy_conflict(&rebroadened_planning, entity, "rebroadened");
    let rebroadened_conflict = planning_evidence(&rebroadened_planning);
    let rebroadened_main_head = runtime
        .history()
        .branch_head(&main_branch)
        .cloned()
        .expect("rebroadened main head");
    let rebroadened_controller_head = runtime
        .history()
        .branch_head(&controller_branch)
        .cloned()
        .expect("rebroadened controller head");
    let rebroadened_intent_replay = replay_commit(
        &mut runtime,
        rebroadened_intent_commit.commit.commit_id,
        main_branch.clone(),
    );
    assert!(rebroadened_intent_replay.failure.is_none());
    assert!(rebroadened_intent_replay
        .compared_surfaces
        .contains(&ReplayObservableSurface::Strategy));
    let mut runtime = recover_stage(&mut runtime, recovered_root.clone());
    let recovered_rebroadened_planning =
        planning_for(&runtime, controller_branch.clone(), main_branch.clone());
    assert_strategy_conflict(
        &recovered_rebroadened_planning,
        entity,
        "recovered rebroadened",
    );
    assert_eq!(
        planning_evidence(&recovered_rebroadened_planning),
        rebroadened_conflict
    );
    let recovered_rebroadened_intent_replay = replay_commit(
        &mut runtime,
        rebroadened_intent_commit.commit.commit_id,
        main_branch.clone(),
    );
    assert_eq!(
        recovered_rebroadened_intent_replay,
        rebroadened_intent_replay
    );

    let revalidation_commit = execute_strategy_commit(
        &mut runtime,
        ReplicaConvergenceInput {
            entity_id: entity,
            desired_replicas: 9,
        }
        .into_native_canonical_request(crate::facade::commit_strategies::StrategyCallerProvenance {
            request_origin: crate::facade::commit_strategies::StrategyRequestOrigin::Test,
            actor_identity: None,
            correlation_id: None,
        })
        .expect("native canonical strategy request"),
        Some(controller_branch.clone()),
    );
    assert_eq!(
        revalidation_commit
            .change_summary()
            .expect("revalidation change summary")
            .changed_record_count,
        1
    );
    let revalidated_planning =
        planning_for(&runtime, controller_branch.clone(), main_branch.clone());
    assert_exact_shared_truth(&revalidated_planning, entity, "revalidated shared truth");
    let revalidated_shared_truth = planning_evidence(&revalidated_planning);
    let revalidation_replay = replay_commit(
        &mut runtime,
        revalidation_commit.commit.commit_id,
        controller_branch.clone(),
    );
    assert_strategy_replay_clean(&revalidation_replay, "revalidation converge");

    let broad_intent_replay = replay_commit(
        &mut runtime,
        broad_intent_commit.commit.commit_id,
        main_branch.clone(),
    );
    let first_converge_replay = replay_commit(
        &mut runtime,
        first_converge_commit.commit.commit_id,
        controller_branch.clone(),
    );
    let rebroadened_intent_replay = replay_commit(
        &mut runtime,
        rebroadened_intent_commit.commit.commit_id,
        main_branch.clone(),
    );
    assert_strategy_replay_clean(&broad_intent_replay, "broad intent");
    assert_strategy_replay_clean(&first_converge_replay, "first converge");
    assert_strategy_replay_clean(&rebroadened_intent_replay, "rebroadened intent");

    let current = runtime
        .read_truth()
        .read_version(runtime.current_version_id());
    let current_entity = current.get_entity(entity).expect("entity visible");
    let live_bundle = KubernetesIntentCertificationBundle {
        overlap_conflict,
        narrowed_non_conflict,
        rebroadened_conflict,
        revalidated_shared_truth,
        revalidation_noop: KubernetesNoopEvidence {
            strategy_artifacts: revalidation_commit
                .publication
                .strategy_artifacts
                .as_ref()
                .expect("revalidation strategy artifacts")
                .clone(),
            changed_record_count: revalidation_commit
                .change_summary()
                .expect("revalidation change summary")
                .changed_record_count,
            patch_record_count: revalidation_commit.publication.envelope.patch.records.len(),
        },
        broad_intent_replay,
        first_converge_replay,
        rebroadened_intent_replay,
        revalidation_noop_replay: revalidation_replay,
        branch_heads: KubernetesBranchHeadEvidence {
            main: runtime.history().branch_head(&main_branch).cloned(),
            controller: runtime.history().branch_head(&controller_branch).cloned(),
        },
        visible_truth: KubernetesVisibleTruthEvidence {
            entity_name: read_entity_name(current_entity),
            replicas_canonical_bytes: replicas_canonical_bytes(current_entity),
        },
    };

    let mut recovered = recover_stage(&mut runtime, recovered_root);
    let recovered_overlap_from_final = recover_stage_from_final_history(
        &recovered,
        unique_test_store_path("forge-relational-m8-5-kubernetes-style-final-overlap"),
        overlap_controller_head,
        overlap_main_head,
    );
    let recovered_overlap_planning = planning_for(
        &recovered_overlap_from_final,
        controller_branch.clone(),
        main_branch.clone(),
    );
    assert_strategy_conflict(
        &recovered_overlap_planning,
        entity,
        "final-history recovered overlap",
    );
    assert_eq!(
        planning_evidence(&recovered_overlap_planning),
        live_bundle.overlap_conflict
    );
    let recovered_narrowed_from_final = recover_stage_from_final_history(
        &recovered,
        unique_test_store_path("forge-relational-m8-5-kubernetes-style-final-narrowed"),
        narrowed_controller_head,
        narrowed_main_head,
    );
    let recovered_narrowed_planning = planning_for(
        &recovered_narrowed_from_final,
        controller_branch.clone(),
        main_branch.clone(),
    );
    assert_non_strategy_conflict(
        &recovered_narrowed_planning,
        entity,
        "final-history recovered narrowed",
    );
    assert_eq!(
        planning_evidence(&recovered_narrowed_planning),
        live_bundle.narrowed_non_conflict
    );
    let recovered_rebroadened_from_final = recover_stage_from_final_history(
        &recovered,
        unique_test_store_path("forge-relational-m8-5-kubernetes-style-final-rebroadened"),
        rebroadened_controller_head,
        rebroadened_main_head,
    );
    let recovered_rebroadened_planning = planning_for(
        &recovered_rebroadened_from_final,
        controller_branch.clone(),
        main_branch.clone(),
    );
    assert_strategy_conflict(
        &recovered_rebroadened_planning,
        entity,
        "final-history recovered rebroadened",
    );
    assert_eq!(
        planning_evidence(&recovered_rebroadened_planning),
        live_bundle.rebroadened_conflict
    );
    let recovered_revalidated_planning =
        planning_for(&recovered, controller_branch.clone(), main_branch.clone());
    assert_exact_shared_truth(
        &recovered_revalidated_planning,
        entity,
        "recovered revalidated shared truth",
    );

    let recovered_broad_intent_replay = replay_commit(
        &mut recovered,
        broad_intent_commit.commit.commit_id,
        main_branch.clone(),
    );
    let recovered_first_converge_replay = replay_commit(
        &mut recovered,
        first_converge_commit.commit.commit_id,
        controller_branch.clone(),
    );
    let recovered_rebroadened_intent_replay = replay_commit(
        &mut recovered,
        rebroadened_intent_commit.commit.commit_id,
        main_branch.clone(),
    );
    let recovered_revalidation_replay = replay_commit(
        &mut recovered,
        revalidation_commit.commit.commit_id,
        controller_branch.clone(),
    );
    assert_strategy_replay_clean(&recovered_broad_intent_replay, "recovered broad intent");
    assert_strategy_replay_clean(&recovered_first_converge_replay, "recovered first converge");
    assert_strategy_replay_clean(
        &recovered_rebroadened_intent_replay,
        "recovered rebroadened intent",
    );
    let recovered_current = recovered
        .read_truth()
        .read_version(recovered.current_version_id());
    assert_eq!(
        planning_evidence(&recovered_revalidated_planning),
        live_bundle.revalidated_shared_truth
    );
    let recovered_revalidation_envelope = recovered
        .replay()
        .canonical_commit_envelope(revalidation_commit.commit.commit_id)
        .cloned()
        .expect("recovered revalidation envelope");
    assert_eq!(
        KubernetesNoopEvidence {
            strategy_artifacts: recovered_revalidation_envelope
                .strategy_artifacts
                .as_ref()
                .expect("recovered revalidation strategy artifacts")
                .clone(),
            changed_record_count: recovered_revalidation_envelope.patch.records.len(),
            patch_record_count: recovered_revalidation_envelope.patch.records.len(),
        },
        live_bundle.revalidation_noop
    );
    assert_eq!(
        recovered_broad_intent_replay,
        live_bundle.broad_intent_replay
    );
    assert_eq!(
        recovered_first_converge_replay,
        live_bundle.first_converge_replay
    );
    assert_eq!(
        recovered_rebroadened_intent_replay,
        live_bundle.rebroadened_intent_replay
    );
    assert_eq!(
        recovered_revalidation_replay,
        live_bundle.revalidation_noop_replay
    );
    assert_strategy_replay_clean(
        &recovered_revalidation_replay,
        "recovered revalidation converge",
    );
    assert_eq!(
        KubernetesBranchHeadEvidence {
            main: recovered.history().branch_head(&main_branch).cloned(),
            controller: recovered.history().branch_head(&controller_branch).cloned(),
        },
        live_bundle.branch_heads
    );
    let recovered_entity = recovered_current
        .get_entity(entity)
        .expect("recovered entity visible");
    assert_eq!(
        KubernetesVisibleTruthEvidence {
            entity_name: read_entity_name(recovered_entity),
            replicas_canonical_bytes: replicas_canonical_bytes(recovered_entity),
        },
        live_bundle.visible_truth
    );
    live_bundle
}

#[test]
fn milestone_8_5_kubernetes_style_intent_commit_certification_proves_staged_controller_outcomes() {
    let certification = run_kubernetes_style_certification();
    assert!(!certification.overlap_conflict.conflict.records.is_empty());
    assert!(!certification
        .narrowed_non_conflict
        .conflict
        .records
        .is_empty());
    assert!(!certification
        .rebroadened_conflict
        .conflict
        .records
        .is_empty());
    assert!(!certification
        .revalidated_shared_truth
        .lowered_plan
        .records
        .is_empty());
    assert_eq!(
        certification.revalidation_noop.changed_record_count,
        certification.revalidation_noop.patch_record_count
    );
    assert_strategy_replay_clean(&certification.broad_intent_replay, "certified broad intent");
    assert_strategy_replay_clean(
        &certification.first_converge_replay,
        "certified first converge",
    );
    assert_strategy_replay_clean(
        &certification.rebroadened_intent_replay,
        "certified rebroadened intent",
    );
    assert_strategy_replay_clean(
        &certification.revalidation_noop_replay,
        "certified revalidation",
    );
    assert!(certification.branch_heads.main.is_some());
    assert!(certification.branch_heads.controller.is_some());
    assert_eq!(
        certification.visible_truth.entity_name.as_deref(),
        Some("svc-v2")
    );
    assert!(certification
        .visible_truth
        .replicas_canonical_bytes
        .is_some());
    assert_ne!(
        certification.overlap_conflict,
        certification.narrowed_non_conflict
    );
    assert_ne!(
        certification.narrowed_non_conflict,
        certification.rebroadened_conflict
    );
    assert_ne!(
        certification.rebroadened_conflict,
        certification.revalidated_shared_truth
    );
}
