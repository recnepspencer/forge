use serde::Serialize;
use serde_json::json;
use std::collections::BTreeSet;

use crate::capabilities::DurabilityRead;
use crate::commit_strategies::strategies::{
    IntentReconciliationInput, IntentReconciliationStrategy, ReplicaConvergenceInput,
    ReplicaConvergenceStrategy,
};
use crate::facade::commit_strategies::RawStrategyCommitRequest;
use crate::facade::config::RelationalRuntimeProfile;
use crate::facade::durability::{DurabilityMode, DurableStoreLayout};
use crate::facade::history::BranchId;
use crate::facade::merge::{MergeIntent, MergePlanningRequest};
use crate::facade::replay::{
    RelationalReplayRequest, ReplayExecutionMode, ReplayObservableSurface, ReplayVerificationMode,
};
use crate::facade::runtime::{RelationalRuntime, RelationalRuntimeApi};
use crate::facade::transactions::{CommitResult, TransactionOptions};
use crate::tests::support::{
    certification_digest, checkpoint_and_recover_with, create_branch_from_main, create_entity,
    entity_payload_aspect, lifecycle_aspect, read_entity_name, unique_test_store_path,
    AspectSchemaFixture,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct KubernetesIntentCertificationBundle {
    overlap_conflict_digest: String,
    narrowed_non_conflict_digest: String,
    rebroadened_conflict_digest: String,
    revalidated_shared_truth_digest: String,
    revalidation_noop_digest: String,
    broad_intent_replay_digest: String,
    first_converge_replay_digest: String,
    rebroadened_intent_replay_digest: String,
    revalidation_noop_replay_digest: String,
    branch_heads_digest: String,
    visible_truth_digest: String,
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
    strategy_name: &str,
    input: serde_json::Value,
    target_branch: Option<BranchId>,
) -> CommitResult {
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
    let snapshot = if let Some(branch_id) = target_branch.as_ref() {
        let branch_head = runtime
            .history_access()
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

fn planning_for(
    runtime: &RelationalRuntime,
    source_branch: BranchId,
    target_branch: BranchId,
) -> crate::merge::data::MergePlanningArtifactCore {
    runtime
        .merge_access()
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

fn planning_digest(planning: &crate::merge::data::MergePlanningArtifactCore) -> String {
    certification_digest(&(
        &planning.digest_basis.conflict,
        &planning.digest_basis.lowered_plan,
        &planning.decision_log_digest_basis,
    ))
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
        .history_access()
        .ancestor_closure_by_commit_id_order(source_head.commit_id)
        .into_iter()
        .chain(
            source
                .history_access()
                .ancestor_closure_by_commit_id_order(target_head.commit_id),
        )
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    chain.sort_unstable();
    let replay_access = source.replay_access();
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
        .history_access()
        .max_commit_id_common_ancestor(source_head.commit_id, target_head.commit_id)
    {
        let base_version_id = recovered
            .history_access()
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
        IntentReconciliationStrategy::DEFAULT_SEMANTIC_NAME,
        serde_json::to_value(IntentReconciliationInput {
            entity_id: entity,
            desired_payload: json!({"name":"svc-v1","replicas":3}),
        })
        .expect("broad intent input"),
        None,
    );
    let first_converge_commit = execute_strategy_commit(
        &mut runtime,
        ReplicaConvergenceStrategy::DEFAULT_SEMANTIC_NAME,
        serde_json::to_value(ReplicaConvergenceInput {
            entity_id: entity,
            desired_replicas: 7,
        })
        .expect("first converge input"),
        Some(controller_branch.clone()),
    );
    let overlap_planning = planning_for(&runtime, controller_branch.clone(), main_branch.clone());
    assert_strategy_conflict(&overlap_planning, entity, "overlap");
    let overlap_conflict_digest = planning_digest(&overlap_planning);
    let overlap_main_head = runtime
        .history_access()
        .branch_head(&main_branch)
        .cloned()
        .expect("overlap main head");
    let overlap_controller_head = runtime
        .history_access()
        .branch_head(&controller_branch)
        .cloned()
        .expect("overlap controller head");
    let mut runtime = recover_stage(&mut runtime, recovered_root.clone());
    let recovered_overlap_planning =
        planning_for(&runtime, controller_branch.clone(), main_branch.clone());
    assert_strategy_conflict(&recovered_overlap_planning, entity, "recovered overlap");
    assert_eq!(
        planning_digest(&recovered_overlap_planning),
        overlap_conflict_digest
    );

    let _narrowed_intent_commit = execute_strategy_commit(
        &mut runtime,
        IntentReconciliationStrategy::DEFAULT_SEMANTIC_NAME,
        serde_json::to_value(IntentReconciliationInput {
            entity_id: entity,
            desired_payload: json!({"name":"svc-v2"}),
        })
        .expect("narrowed intent input"),
        None,
    );
    let idempotent_converge_commit = execute_strategy_commit(
        &mut runtime,
        ReplicaConvergenceStrategy::DEFAULT_SEMANTIC_NAME,
        serde_json::to_value(ReplicaConvergenceInput {
            entity_id: entity,
            desired_replicas: 7,
        })
        .expect("idempotent converge input"),
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
    let narrowed_non_conflict_digest = planning_digest(&narrowed_planning);
    let narrowed_main_head = runtime
        .history_access()
        .branch_head(&main_branch)
        .cloned()
        .expect("narrowed main head");
    let narrowed_controller_head = runtime
        .history_access()
        .branch_head(&controller_branch)
        .cloned()
        .expect("narrowed controller head");
    let mut runtime = recover_stage(&mut runtime, recovered_root.clone());
    let recovered_narrowed_planning =
        planning_for(&runtime, controller_branch.clone(), main_branch.clone());
    assert_non_strategy_conflict(&recovered_narrowed_planning, entity, "recovered narrowed");
    assert_eq!(
        planning_digest(&recovered_narrowed_planning),
        narrowed_non_conflict_digest
    );

    let rebroadened_intent_commit = execute_strategy_commit(
        &mut runtime,
        IntentReconciliationStrategy::DEFAULT_SEMANTIC_NAME,
        serde_json::to_value(IntentReconciliationInput {
            entity_id: entity,
            desired_payload: json!({"name":"svc-v2","replicas":9}),
        })
        .expect("rebroadened intent input"),
        None,
    );
    let rebroadened_planning =
        planning_for(&runtime, controller_branch.clone(), main_branch.clone());
    assert_strategy_conflict(&rebroadened_planning, entity, "rebroadened");
    let rebroadened_conflict_digest = planning_digest(&rebroadened_planning);
    let rebroadened_main_head = runtime
        .history_access()
        .branch_head(&main_branch)
        .cloned()
        .expect("rebroadened main head");
    let rebroadened_controller_head = runtime
        .history_access()
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
        planning_digest(&recovered_rebroadened_planning),
        rebroadened_conflict_digest
    );
    let recovered_rebroadened_intent_replay = replay_commit(
        &mut runtime,
        rebroadened_intent_commit.commit.commit_id,
        main_branch.clone(),
    );
    assert_eq!(
        full_replay_digest(&recovered_rebroadened_intent_replay),
        full_replay_digest(&rebroadened_intent_replay)
    );

    let revalidation_commit = execute_strategy_commit(
        &mut runtime,
        ReplicaConvergenceStrategy::DEFAULT_SEMANTIC_NAME,
        serde_json::to_value(ReplicaConvergenceInput {
            entity_id: entity,
            desired_replicas: 9,
        })
        .expect("revalidation converge input"),
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
    let revalidated_shared_truth_digest = planning_digest(&revalidated_planning);
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
        .visibility_reads()
        .read_version(runtime.current_version_id());
    let live_bundle = KubernetesIntentCertificationBundle {
        overlap_conflict_digest,
        narrowed_non_conflict_digest,
        rebroadened_conflict_digest,
        revalidated_shared_truth_digest,
        revalidation_noop_digest: certification_digest(&(
            certification_digest(
                revalidation_commit
                    .publication
                    .strategy_artifacts
                    .as_ref()
                    .expect("revalidation strategy artifacts"),
            ),
            revalidation_commit
                .change_summary()
                .expect("revalidation change summary")
                .changed_record_count,
            revalidation_commit.publication.envelope.patch.records.len(),
        )),
        broad_intent_replay_digest: full_replay_digest(&broad_intent_replay),
        first_converge_replay_digest: full_replay_digest(&first_converge_replay),
        rebroadened_intent_replay_digest: full_replay_digest(&rebroadened_intent_replay),
        revalidation_noop_replay_digest: full_replay_digest(&revalidation_replay),
        branch_heads_digest: certification_digest(&(
            runtime.history_access().branch_head(&main_branch).cloned(),
            runtime
                .history_access()
                .branch_head(&controller_branch)
                .cloned(),
        )),
        visible_truth_digest: certification_digest(&(
            read_entity_name(current.get_entity(entity).expect("entity visible"))
                .map(str::to_string),
            current
                .get_entity(entity)
                .expect("entity visible")
                .payload
                .as_json()
                .and_then(|payload| payload.get("replicas"))
                .cloned(),
        )),
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
        planning_digest(&recovered_overlap_planning),
        live_bundle.overlap_conflict_digest
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
        planning_digest(&recovered_narrowed_planning),
        live_bundle.narrowed_non_conflict_digest
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
        planning_digest(&recovered_rebroadened_planning),
        live_bundle.rebroadened_conflict_digest
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
        .visibility_reads()
        .read_version(recovered.current_version_id());
    assert_eq!(
        planning_digest(&recovered_revalidated_planning),
        live_bundle.revalidated_shared_truth_digest
    );
    assert_eq!(
        certification_digest(&(
            certification_digest(
                recovered
                    .replay_access()
                    .canonical_commit_envelope(revalidation_commit.commit.commit_id)
                    .expect("recovered revalidation envelope")
                    .strategy_artifacts
                    .as_ref()
                    .expect("recovered revalidation strategy artifacts"),
            ),
            recovered
                .replay_access()
                .canonical_commit_envelope(revalidation_commit.commit.commit_id)
                .expect("recovered revalidation envelope")
                .patch
                .records
                .len(),
            recovered
                .replay_access()
                .canonical_commit_envelope(revalidation_commit.commit.commit_id)
                .expect("recovered revalidation envelope")
                .patch
                .records
                .len(),
        )),
        live_bundle.revalidation_noop_digest
    );
    assert_eq!(
        full_replay_digest(&recovered_broad_intent_replay),
        live_bundle.broad_intent_replay_digest
    );
    assert_eq!(
        full_replay_digest(&recovered_first_converge_replay),
        live_bundle.first_converge_replay_digest
    );
    assert_eq!(
        full_replay_digest(&recovered_rebroadened_intent_replay),
        live_bundle.rebroadened_intent_replay_digest
    );
    assert_eq!(
        full_replay_digest(&recovered_revalidation_replay),
        live_bundle.revalidation_noop_replay_digest
    );
    assert_strategy_replay_clean(
        &recovered_revalidation_replay,
        "recovered revalidation converge",
    );
    assert_eq!(
        certification_digest(&(
            recovered
                .history_access()
                .branch_head(&main_branch)
                .cloned(),
            recovered
                .history_access()
                .branch_head(&controller_branch)
                .cloned(),
        )),
        live_bundle.branch_heads_digest
    );
    assert_eq!(
        certification_digest(&(
            read_entity_name(
                recovered_current
                    .get_entity(entity)
                    .expect("recovered entity visible"),
            )
            .map(str::to_string),
            recovered_current
                .get_entity(entity)
                .expect("recovered entity visible")
                .payload
                .as_json()
                .and_then(|payload| payload.get("replicas"))
                .cloned(),
        )),
        live_bundle.visible_truth_digest
    );
    live_bundle
}

#[test]
fn milestone_8_5_kubernetes_style_intent_commit_certification_proves_staged_controller_outcomes() {
    let certification = run_kubernetes_style_certification();
    assert!(certification.overlap_conflict_digest.len() > 8);
    assert!(certification.narrowed_non_conflict_digest.len() > 8);
    assert!(certification.rebroadened_conflict_digest.len() > 8);
    assert!(certification.revalidated_shared_truth_digest.len() > 8);
    assert!(certification.revalidation_noop_digest.len() > 8);
    assert!(certification.broad_intent_replay_digest.len() > 8);
    assert!(certification.first_converge_replay_digest.len() > 8);
    assert!(certification.rebroadened_intent_replay_digest.len() > 8);
    assert!(certification.revalidation_noop_replay_digest.len() > 8);
    assert_ne!(
        certification.overlap_conflict_digest,
        certification.narrowed_non_conflict_digest
    );
    assert_ne!(
        certification.narrowed_non_conflict_digest,
        certification.rebroadened_conflict_digest
    );
    assert_ne!(
        certification.rebroadened_conflict_digest,
        certification.revalidated_shared_truth_digest
    );
}
