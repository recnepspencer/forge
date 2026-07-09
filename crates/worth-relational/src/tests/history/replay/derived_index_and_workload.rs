use super::*;

#[test]
fn replay_contract_reports_derived_index_drift_at_digest_layer_when_artifacts_are_tampered() {
    let mut runtime = runtime_with_test_schema();
    let commit = create_entity_outcome(&mut runtime, "indexed");
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(0),
        name: "entity-name".to_string(),
        kind: DerivedIndexKind::EntityField {
            field_locator: aspect_field_locator(aspect_key("name"), field_key("name")),
        },
        branch_scoped: false,
    });
    runtime
        .index_authority()
        .build_for_commit(DerivedIndexBuildRequest {
            source_commit_id: commit.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            index_ids: vec![index.index_id],
        });

    assert!(runtime.history_authority().tamper_commit_envelope_for_test(
        commit.commit.commit_id,
        |envelope| {
            if let Some(generation) = envelope
                .derived_index_artifacts
                .generations_mut_for_test()
                .first_mut()
            {
                generation.status =
                    crate::facade::indexes::DerivedIndexPublicationStatus::BuildFailed;
            }
        }
    ));

    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: commit.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
        });

    assert_eq!(replay.failure, Some(ReplayFailureClass::ObservableMismatch));
    assert!(
        replay.mismatches.iter().any(|mismatch| {
            mismatch.class == ReplayMismatchClass::DerivedIndexDrift
                && mismatch.surface == ReplayObservableSurface::DerivedIndexes
                && mismatch.verification_layer
                    == crate::facade::replay::ReplayVerificationLayer::DigestParity
        }),
        "{:?}",
        replay.mismatches
    );
}

#[test]
fn replay_and_recovery_preserve_aspect_bearing_truth_across_a_hostile_mixed_workload() {
    let mut runtime =
        persisted_runtime_with_declared_aspect_schema(CascadeDeletePolicy::RetainDanglingForAudit);
    let created = create_entity_outcome(&mut runtime, "anchor");
    let anchor = changed_entities(&created)[0];
    let _updated = update_entity(&mut runtime, anchor, "anchor-updated");
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let relation_outcome = create_relation_outcome(&mut runtime, source, target, "net-edge");
    let relation = changed_relations(&relation_outcome)[0];
    let _retained = delete_entity(&mut runtime, source);
    let replace_outcome = {
        let mut txn = runtime.begin_transaction(TransactionOptions::default());
        txn.push_batch(
            WorkerIntentBatch::new("replace-anchor").push(MutationIntent::Entity(
                EntityMutationIntent::Replace(ReplaceEntityIntent {
                    entity_id: anchor,
                    replacement: crate::transactions::data::EntitySpec {
                        partition_id: PartitionId::main(),
                        kind_id: KindId(1),
                        client_key: crate::symbols::data::ClientKey::raw("anchor-replaced"),
                        fields: crate::tests::support::single_string_aspect_field_patch(
                            crate::tests::support::aspect_key("name"),
                            crate::tests::support::field_key("name"),
                            "anchor-replaced",
                        ),
                    },
                }),
            )),
        );
        txn.commit().unwrap()
    };
    runtime.durability_authority().checkpoint().unwrap();

    let start_lineage = runtime
        .lineage_access()
        .for_record(anchor)
        .unwrap()
        .lineage_id;

    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: replace_outcome.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
        });
    assert!(runtime.replay().compare_outcome(&replay));
    let replay_diagnostics = runtime.publication().diagnostics();
    assert!(replay_diagnostics
        .by_scope(DiagnosticsScope::Replay)
        .into_iter()
        .flat_map(|artifact| artifact.entries.iter())
        .any(|entry| {
            entry.code == DiagnosticCode::CommitPublished
                && diagnostic_field_optional(entry, "verification_mode")
                    == Some(&RelationalDiagnosticValue::string(
                        "NormalRecoveryVerification",
                    ))
        }));
    let replay_counters = runtime.performance_access().counters();
    assert!(replay_counters.replay_digest_parity_checks > 0);

    let recovery_plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let mut recovered =
        persisted_runtime_with_declared_aspect_schema(CascadeDeletePolicy::RetainDanglingForAudit);
    recovered
        .durability_authority()
        .recover(recovery_plan)
        .unwrap();
    let recovered_replay_check =
        recovered
            .replay_authority()
            .replay_commit(RelationalReplayRequest {
                commit_id: replace_outcome.commit.commit_id,
                branch_id: BranchId("main".to_string()),
                execution_mode: ReplayExecutionMode::SerialDeterministic,
                verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
            });

    assert_recovered_commit_truth_matches(
        &mut runtime,
        &mut recovered,
        replace_outcome.commit.commit_id,
        &[anchor],
        &[relation],
        &[start_lineage],
    );
    assert!(recovered.replay().compare_outcome(&recovered_replay_check));
    let recovered_bundle =
        capture_aspect_truth_bundle(&mut recovered, &[anchor], &[relation], &[start_lineage]);
    assert_eq!(
        recovered_replay_check.requested.commit_id,
        replace_outcome.commit.commit_id
    );
    assert!(recovered_bundle.latest_patch.is_none());
    assert!(recovered_bundle.latest_replay.is_none());
}

#[test]
fn hostile_commit_replay_equivalence_test() {
    let mut runtime =
        persisted_runtime_with_declared_aspect_schema(CascadeDeletePolicy::RetainDanglingForAudit);
    let created = create_entity_outcome(&mut runtime, "anchor");
    let anchor = changed_entities(&created)[0];
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let relation_outcome = create_relation_outcome(&mut runtime, source, target, "net-edge");
    let relation = changed_relations(&relation_outcome)[0];
    create_branch_from_main(&mut runtime, "feature");
    let _feature_update = update_entity_on_branch(
        &mut runtime,
        anchor,
        "feature-anchor",
        BranchId("feature".to_string()),
    );

    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(2),
        ..AspectSchemaFixture::default()
    }
    .build_registry();
    let mut transition_txn =
        runtime.begin_transaction(TransactionOptions::default().with_schema_transition(
            schema_transition_for_subscriber_impact(
                SchemaVersionId(2),
                SchemaSubscriberImpact::ConsumableSurfaceChanged,
            ),
            Some(SchemaReconciliationPolicy::PreserveInformation),
        ));
    transition_txn.push_batch(batch_create("after-boundary"));
    let _transition_outcome = transition_txn.commit().unwrap();

    let merge = merge_commit_from_branches(
        &mut runtime,
        BranchId("main".to_string()),
        vec![BranchId("feature".to_string())],
    );
    runtime.durability_authority().checkpoint().unwrap();

    let anchor_lineage = runtime
        .lineage_access()
        .for_record(anchor)
        .unwrap()
        .lineage_id;
    let original_bundle =
        capture_aspect_truth_bundle(&mut runtime, &[anchor], &[relation], &[anchor_lineage]);
    let original_inspection = capture_inspection_truth_bundle(
        &runtime,
        &BranchId("main".to_string()),
        anchor,
        merge.commit.version_id,
    );
    let original_envelope = runtime
        .replay()
        .canonical_commit_envelope(merge.commit.commit_id)
        .cloned()
        .unwrap();
    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: merge.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
        });
    assert!(runtime.replay().compare_outcome(&replay));

    let original_main_branch_head = runtime
        .history()
        .branch_head(&BranchId("main".to_string()))
        .cloned();
    let original_feature_branch_head = runtime
        .history()
        .branch_head(&BranchId("feature".to_string()))
        .cloned();

    let recovery_plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let mut recovered = RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(
            AspectSchemaFixture {
                schema_version_id: SchemaVersionId(2),
                ..AspectSchemaFixture::default()
            }
            .build_registry(),
        )
        .durability_mode(DurabilityMode::PersistedSegmentedLocalFs)
        .durable_store_layout(DurableStoreLayout {
            root_path: unique_test_store_path("worth-relational-hostile-replay-equivalence"),
            segment_commit_capacity: 2,
        })
        .build();
    recovered
        .durability_authority()
        .recover(recovery_plan)
        .unwrap();
    let recovered_bundle =
        capture_aspect_truth_bundle(&mut recovered, &[anchor], &[relation], &[anchor_lineage]);
    let recovered_inspection = capture_inspection_truth_bundle(
        &recovered,
        &BranchId("main".to_string()),
        anchor,
        merge.commit.version_id,
    );
    let recovered_envelope = recovered
        .replay()
        .canonical_commit_envelope(merge.commit.commit_id)
        .cloned()
        .unwrap();
    let recovered_replay = recovered
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: merge.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
        });
    let recovered_replay_diagnostics = recovered.publication().diagnostics();
    assert!(recovered_replay_diagnostics
        .by_scope(DiagnosticsScope::Replay)
        .into_iter()
        .flat_map(|artifact| artifact.entries.iter())
        .any(|entry| {
            entry.code == DiagnosticCode::CommitPublished
                && diagnostic_field_optional(entry, "verification_mode")
                    == Some(&RelationalDiagnosticValue::string(
                        "NormalRecoveryVerification",
                    ))
        }));

    assert_stable_aspect_truth_bundle_eq(&original_bundle, &recovered_bundle);
    assert_eq!(original_envelope, recovered_envelope);
    assert_eq!(replay, recovered_replay);
    assert_eq!(
        original_main_branch_head,
        recovered
            .history()
            .branch_head(&BranchId("main".to_string()))
            .cloned()
    );
    assert_eq!(
        original_feature_branch_head,
        recovered
            .history()
            .branch_head(&BranchId("feature".to_string()))
            .cloned()
    );
    assert_eq!(original_inspection, recovered_inspection);
    let recovered_counters = recovered.performance_access().counters();
    assert!(recovered_counters.replay_digest_parity_checks > 0);
}
