use std::path::PathBuf;
use std::sync::Arc;

use crate::facade::history::BranchId;
use crate::facade::identity::{KindId, PartitionId};
use crate::facade::merge::{
    AspectMergePolicyDeclaration, AspectMergePolicyKind, IdentityBasisDeclaration,
    IdentityBasisKind, IdentityBasisScope, MergeExecutionError, MergeExecutionRequest, MergeIntent,
};
use crate::facade::runtime::{RelationalRuntime, RelationalRuntimeApi};
use crate::facade::schema::{
    AspectBinding, AspectComparator, AspectKey, AspectPrecision, DeclaredAspect,
    EntityKindRegistration, KindAspectDeclarations, RelationKindRegistration,
    RelationalSchemaRegistry, SchemaId, SchemaVersionId,
};
use crate::facade::transactions::{
    CreateIntent, MutationIntent, TransactionOptions, WorkerIntentBatch,
};
use crate::schema::data::{RelationIntegrityDeclarations, RelationPayloadClass};
use crate::symbols::data::InternedString;
use crate::tests::support::{
    capture_aspect_truth_bundle, certification_digest, checkpoint_and_recover_with,
    create_branch_from_main, create_entity, create_entity_outcome_on_branch, entity_payload_aspect,
    persisted_runtime_with_test_schema, read_entity_name, unique_test_store_path, update_entity,
    update_entity_on_branch, CascadeDeletePolicy, CrossContextPolicy, DurabilityMode,
    DurableStoreLayout, RelationalRuntimeProfile,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct MergeExecutionCertificationArtifacts {
    merge_execution_digest: String,
    merge_execution_diagnostics_digest: String,
    merge_execution_truth_digest: String,
    merge_execution_replay_digest: String,
    merge_execution_recovery_digest: String,
    merge_execution_branch_heads_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AuthoritativeMergeExecutionCertificationSuite {
    exact_shared: MergeExecutionCertificationArtifacts,
    source_only_addition: MergeExecutionCertificationArtifacts,
    prefer_richer_reconcile: MergeExecutionCertificationArtifacts,
}

#[test]
fn authoritative_merge_execution_certification_emits_machine_checkable_artifacts() {
    let suite = AuthoritativeMergeExecutionCertificationSuite {
        exact_shared: certify_exact_shared_merge_execution(),
        source_only_addition: certify_source_only_addition_merge_execution(),
        prefer_richer_reconcile: certify_prefer_richer_merge_execution(),
    };

    for certification in [
        &suite.exact_shared,
        &suite.source_only_addition,
        &suite.prefer_richer_reconcile,
    ] {
        assert!(certification.merge_execution_digest.len() > 8);
        assert!(certification.merge_execution_diagnostics_digest.len() > 8);
        assert!(certification.merge_execution_truth_digest.len() > 8);
        assert!(certification.merge_execution_replay_digest.len() > 8);
        assert!(certification.merge_execution_recovery_digest.len() > 8);
        assert!(certification.merge_execution_branch_heads_digest.len() > 8);
    }
}

#[test]
fn authoritative_merge_execution_certification_rejects_stale_prepared_merge_after_target_advances()
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
        .expect("prepared merge execution");
    create_entity(&mut runtime, "main-advance");

    match runtime.execute_prepared_merge(prepared) {
        Err(MergeExecutionError::StaleBranchHead { branch, .. }) => {
            assert_eq!(branch, BranchId("main".to_string()));
        }
        other => panic!("expected target stale-head rejection, got {other:?}"),
    }
}

#[test]
fn authoritative_merge_execution_certification_rejects_stale_prepared_merge_after_source_advances()
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
        .expect("prepared merge execution");
    create_entity_outcome_on_branch(
        &mut runtime,
        "feature-advance",
        BranchId("feature".to_string()),
    );

    match runtime.execute_prepared_merge(prepared) {
        Err(MergeExecutionError::StaleBranchHead { branch, .. }) => {
            assert_eq!(branch, BranchId("feature".to_string()));
        }
        other => panic!("expected source stale-head rejection, got {other:?}"),
    }
}

#[test]
fn authoritative_merge_execution_certification_rejects_schema_semantic_drift_after_planning() {
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
        .expect("prepared merge execution");
    runtime.config.schema.registry = drifted_schema_registry();

    match runtime.execute_prepared_merge(prepared) {
        Err(MergeExecutionError::SchemaSemanticDrift { .. }) => {}
        other => panic!("expected schema semantic drift rejection, got {other:?}"),
    }
}

fn certify_exact_shared_merge_execution() -> MergeExecutionCertificationArtifacts {
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

fn certify_source_only_addition_merge_execution() -> MergeExecutionCertificationArtifacts {
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

fn certify_prefer_richer_merge_execution() -> MergeExecutionCertificationArtifacts {
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
                client_key: InternedString::Raw("feature-shared".to_string()),
                payload: crate::payloads::data::RecordPayload::StructuredJson(serde_json::json!({
                    "name": "shared-name",
                    "status": "active"
                })),
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
        .visibility_reads()
        .read_snapshot(&merge.commit.snapshot)
        .expect("current merge snapshot");
    let current_record = current
        .get_entity(main_entity)
        .expect("merged target entity remains visible");
    assert_eq!(read_entity_name(current_record), Some("shared-name"));
    assert_eq!(
        current_record
            .payload
            .as_json()
            .and_then(|json| json.get("status"))
            .and_then(|value| value.as_str()),
        Some("active")
    );

    certify_merge_execution_with_recovery(&mut runtime, &merge, move || {
        persisted_runtime_with_registry(registry.clone(), store_path.clone())
    })
}

fn certify_merge_execution_with_recovery<F>(
    runtime: &mut RelationalRuntime,
    merge: &crate::facade::merge::MergeExecutionOutcome,
    recovered_factory: F,
) -> MergeExecutionCertificationArtifacts
where
    F: FnOnce() -> RelationalRuntime,
{
    let envelope = runtime
        .replay_access()
        .canonical_commit_envelope(merge.commit.commit.commit_id)
        .cloned()
        .expect("canonical merge envelope");
    let truth_bundle = capture_aspect_truth_bundle(runtime, &[], &[], &[]);
    let direct_rebuild_plan = runtime.durability_access().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::AuditRecoveryVerification,
    );
    crate::logic::runtime::RelationalRuntime::rebuild_runtime_from_plan(direct_rebuild_plan)
        .unwrap_or_else(|error| panic!("direct replay rebuild failed: {error:?}"));
    let replay =
        runtime
            .replay_authority()
            .replay_commit(crate::facade::replay::RelationalReplayRequest {
                commit_id: merge.commit.commit.commit_id,
                branch_id: BranchId("main".to_string()),
                execution_mode: crate::facade::replay::ReplayExecutionMode::SerialDeterministic,
                verification_mode:
                    crate::facade::replay::ReplayVerificationMode::AuditRecoveryVerification,
            });
    assert!(
        replay.failure.is_none(),
        "replay certification failure: {replay:?}"
    );

    let (_recovery, mut recovered) = checkpoint_and_recover_with(runtime, recovered_factory);
    let recovered_envelope = recovered
        .replay_access()
        .canonical_commit_envelope(merge.commit.commit.commit_id)
        .cloned()
        .expect("recovered merge envelope");
    let recovered_truth_bundle = capture_aspect_truth_bundle(&mut recovered, &[], &[], &[]);

    assert_eq!(envelope, recovered_envelope);
    assert_eq!(
        truth_bundle.visible_truth,
        recovered_truth_bundle.visible_truth
    );
    assert_eq!(
        runtime
            .history_access()
            .latest_common_ancestor_between_branches(
                &BranchId("main".to_string()),
                &BranchId("feature".to_string())
            ),
        recovered
            .history_access()
            .latest_common_ancestor_between_branches(
                &BranchId("main".to_string()),
                &BranchId("feature".to_string())
            )
    );

    MergeExecutionCertificationArtifacts {
        merge_execution_digest: merge.execution_summary.execution_digest.clone(),
        merge_execution_diagnostics_digest: merge.execution_summary.diagnostics_digest.clone(),
        merge_execution_truth_digest: certification_digest(&format!(
            "{:?}",
            truth_bundle.visible_truth
        )),
        merge_execution_replay_digest: certification_digest(&replay),
        merge_execution_recovery_digest: certification_digest(&(
            recovered_envelope.clone(),
            format!("{:?}", recovered_truth_bundle.visible_truth),
        )),
        merge_execution_branch_heads_digest: certification_digest(&(
            runtime
                .history_access()
                .branch_head(&BranchId("main".to_string()))
                .map(|head| head.commit_id),
            runtime
                .history_access()
                .branch_head(&BranchId("feature".to_string()))
                .map(|head| head.commit_id),
        )),
    }
}

fn prefer_richer_registry() -> RelationalSchemaRegistry {
    let name_key = AspectKey(InternedString::Raw("name".to_string()));
    let status_key = AspectKey(InternedString::Raw("status".to_string()));
    RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: KindAspectDeclarations::new(vec![
                entity_payload_aspect("name", "name"),
                DeclaredAspect {
                    key: status_key.clone(),
                    binding: AspectBinding::EntityPayloadField {
                        field: InternedString::Raw("status".to_string()),
                    },
                    comparator: AspectComparator::JsonScalarEquality,
                    precision: AspectPrecision::Structured,
                },
            ])
            .with_identity_declarations(vec![IdentityBasisDeclaration {
                scope: IdentityBasisScope::AspectKey(name_key.clone()),
                basis: IdentityBasisKind::DeclaredKeySet(Arc::from([name_key.clone()])),
            }])
            .with_merge_policy_declarations(vec![AspectMergePolicyDeclaration {
                aspect_key: status_key,
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
        .expect("prefer-richer registry")
}

fn drifted_schema_registry() -> RelationalSchemaRegistry {
    RelationalSchemaRegistry::new()
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
        .expect("drifted schema registry")
}

fn persisted_runtime_with_registry(
    registry: RelationalSchemaRegistry,
    root_path: PathBuf,
) -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(registry)
        .durability_mode(DurabilityMode::PersistedSegmentedLocalFs)
        .durable_store_layout(DurableStoreLayout {
            root_path,
            segment_commit_capacity: 2,
        })
        .build()
}
