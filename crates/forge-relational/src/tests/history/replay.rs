use crate::facade::diagnostics::{DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope};
use crate::facade::history::BranchId;
use crate::facade::indexes::{
    DerivedIndexBuildRequest, DerivedIndexDefinition, DerivedIndexId, DerivedIndexKind,
};
use crate::facade::replay::{
    RelationalReplayRequest, ReplayExecutionMode, ReplayFailureClass, ReplayMismatchClass,
    ReplayObservableSurface, ReplayVerificationMode,
};
use crate::facade::schema::{
    HistoricalInterpretationSensitivity, ProposedSchemaTransition, SchemaDiffAtom,
    SchemaDiffDetail, SchemaElementKind, SchemaElementRef, SchemaId,
    SchemaPublicationImpact, SchemaReconciliationPolicy, SchemaStratum,
    SchemaSubscriberImpact, SchemaVersionId,
};
use crate::tests::support::*;

// CONTRACT: replay
// LANES: success, failure, determinism

fn source_max_one_relation_integrity_runtime() -> RelationalRuntime {
    RelationIntegritySchemaFixture {
        relation_integrity: crate::schema::data::RelationIntegrityDeclarations::new(
            Vec::new(),
            vec![crate::schema::data::CardinalityContractDeclaration {
                contract_id: "source_max_one".to_string(),
                source_max: Some(1),
                target_max: None,
                pair_max: None,
            }],
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ),
        ..RelationIntegritySchemaFixture::default()
    }
    .build_runtime()
}

#[test]
fn replay_contract_success_reproduces_canonical_surfaces() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "replayable");
    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: outcome.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
        });

    assert!(runtime.replay_access().compare_outcome(&replay));
    assert_eq!(
        replay.reconstructed_parent_chain,
        vec![outcome.commit.commit_id]
    );
    assert!(runtime
        .publication_access()
        .diagnostics()
        .by_scope(DiagnosticsScope::Replay)
        .iter()
        .any(|artifact| artifact.kind == DiagnosticsArtifactKind::Comparison));
}

#[test]
fn replay_contract_failure_wrong_branch_is_explicit() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "replayable");
    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: outcome.commit.commit_id,
            branch_id: BranchId("wrong".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
        });

    assert_eq!(replay.failure, Some(ReplayFailureClass::BranchMismatch));
}

#[test]
fn replay_contract_failure_missing_parent_chain_is_explicit() {
    let mut runtime = runtime_with_test_schema();
    let parent = create_entity_outcome(&mut runtime, "parent");
    let child = create_entity_outcome(&mut runtime, "child");

    assert!(runtime
        .history_authority()
        .remove_commit_envelope_for_test(parent.commit.commit_id));

    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: child.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
        });

    assert_eq!(replay.failure, Some(ReplayFailureClass::MissingParentChain));
}

#[test]
fn replay_contract_success_preserves_merge_parent_order() {
    let mut runtime = runtime_with_test_schema();
    let main = create_entity_outcome(&mut runtime, "main");
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature =
        create_entity_outcome_on_branch(&mut runtime, "feature", BranchId("feature".to_string()));
    let merge = merge_commit_from_branches(
        &mut runtime,
        BranchId("main".to_string()),
        vec![BranchId("feature".to_string())],
    );
    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: merge.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
        });

    assert!(runtime.replay_access().compare_outcome(&replay));
    assert_eq!(
        runtime
            .replay_access()
            .canonical_commit_envelope(merge.commit.commit_id)
            .unwrap()
            .commit
            .parents,
        vec![main.commit.commit_id, feature.commit.commit_id]
    );
    assert_eq!(
        runtime
            .replay_access()
            .canonical_commit_envelope(merge.commit.commit_id)
            .unwrap()
            .merge_base_commits,
        vec![main.commit.commit_id]
    );
    assert!(replay
        .compared_surfaces
        .contains(&ReplayObservableSurface::History));
}

#[test]
fn replay_contract_reports_branch_head_drift_at_digest_layer_when_merge_commit_reference_is_tampered(
) {
    let mut runtime = runtime_with_test_schema();
    let _main = create_entity_outcome(&mut runtime, "main");
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let _feature =
        create_entity_outcome_on_branch(&mut runtime, "feature", BranchId("feature".to_string()));
    let merge = merge_commit_from_branches(
        &mut runtime,
        BranchId("main".to_string()),
        vec![BranchId("feature".to_string())],
    );

    assert!(runtime.history_authority().tamper_commit_envelope_for_test(
        merge.commit.commit_id,
        |envelope| {
            envelope.commit.parents.reverse();
        }
    ));

    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: merge.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
        });

    assert_eq!(replay.failure, Some(ReplayFailureClass::ObservableMismatch));
    assert!(replay.mismatches.iter().any(|mismatch| {
        mismatch.class == ReplayMismatchClass::BranchHeadDrift
            && mismatch.surface == ReplayObservableSurface::BranchHead
            && mismatch.verification_layer
                == crate::facade::replay::ReplayVerificationLayer::DigestParity
    }), "{:?}", replay.mismatches);
}

#[test]
fn replay_contract_reports_structured_patch_drift_when_canonical_envelope_is_tampered() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "replayable");
    assert!(runtime.history_authority().tamper_commit_patch_for_test(
        outcome.commit.commit_id,
        |patch| {
            patch.records[0].detail =
                PatchDetail::StructuredJson(serde_json::json!({"tampered": true}));
        }
    ));

    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: outcome.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
        });

    assert_eq!(replay.failure, Some(ReplayFailureClass::ObservableMismatch));
    assert_eq!(replay.mismatches.len(), 1);
    assert_eq!(replay.mismatches[0].class, ReplayMismatchClass::PatchDrift);
    assert_eq!(replay.mismatches[0].surface, ReplayObservableSurface::Patch);
    assert_eq!(
        replay.mismatches[0].verification_layer,
        crate::facade::replay::ReplayVerificationLayer::DigestParity
    );
    assert!(replay.mismatches[0].expected.is_some());
    assert!(replay.mismatches[0].observed.is_some());
}

#[test]
fn replay_contract_reports_diagnostics_drift_at_digest_layer_when_envelope_is_tampered() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "replayable");
    assert!(runtime.history_authority().tamper_commit_envelope_for_test(
        outcome.commit.commit_id,
        |envelope| {
            envelope.diagnostics_summary.entries.clear();
        }
    ));

    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: outcome.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
        });

    assert_eq!(replay.failure, Some(ReplayFailureClass::ObservableMismatch));
    assert!(replay.mismatches.iter().any(|mismatch| {
        mismatch.class == ReplayMismatchClass::DiagnosticsDrift
            && mismatch.surface == ReplayObservableSurface::Diagnostics
            && mismatch.verification_layer
                == crate::facade::replay::ReplayVerificationLayer::DigestParity
    }), "{:?}", replay.mismatches);
}

#[test]
fn replay_contract_reports_schema_continuation_descriptor_drift_when_envelope_is_tampered() {
    let mut runtime = runtime_with_test_schema();
    let _first = create_entity_outcome(&mut runtime, "a");

    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(2),
        ..AspectSchemaFixture::default()
    }
    .build_registry();

    let proposed_transition = ProposedSchemaTransition {
        source_schema_id: SchemaId("test".to_string()),
        source_schema_version_id: SchemaVersionId(1),
        target_schema_id: SchemaId("test".to_string()),
        target_schema_version_id: SchemaVersionId(2),
        diff_atoms: vec![SchemaDiffAtom::new(
            SchemaElementRef::new(
                SchemaElementKind::Field,
                SchemaId("test".to_string()),
                SchemaVersionId(2),
                Some(KindId(1)),
                "tag",
            ),
            vec![SchemaStratum::StructuralShape, SchemaStratum::PublicationContract],
            SchemaPublicationImpact::ObservableSurfaceChanged,
            SchemaSubscriberImpact::ConsumableSurfaceChanged,
            HistoricalInterpretationSensitivity::NotSensitive,
            SchemaDiffDetail::AddedField {
                field_name: "tag".into(),
                required: false,
                default_expression: Some("null".into()),
            },
        )],
    };
    let mut txn = runtime.begin_transaction(
        TransactionOptions::default().with_schema_transition(
            proposed_transition,
            Some(SchemaReconciliationPolicy::PreserveInformation),
        ),
    );
    txn.push_batch(batch_create("b"));
    let outcome = txn.commit().unwrap();

    assert!(runtime.history_authority().tamper_commit_envelope_for_test(
        outcome.commit.commit_id,
        |envelope| {
            if let Some(descriptor) = envelope.schema_continuation_descriptor.as_mut() {
                descriptor.normalized_boundary_count += 1;
            }
        }
    ));

    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: outcome.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
        });

    assert_eq!(replay.failure, Some(ReplayFailureClass::ObservableMismatch));
    assert!(replay.mismatches.iter().any(|mismatch| {
        mismatch.class == ReplayMismatchClass::SchemaContinuationDescriptorDrift
            && mismatch.surface == ReplayObservableSurface::History
            && mismatch.verification_layer == crate::facade::replay::ReplayVerificationLayer::DigestParity
    }), "{:?}", replay.mismatches);
}

#[test]
fn replay_contract_audit_mode_confirms_schema_continuation_descriptor_drift_at_deep_layer() {
    let mut runtime = runtime_with_test_schema();
    let _first = create_entity_outcome(&mut runtime, "a");

    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(2),
        ..AspectSchemaFixture::default()
    }
    .build_registry();

    let proposed_transition = ProposedSchemaTransition {
        source_schema_id: SchemaId("test".to_string()),
        source_schema_version_id: SchemaVersionId(1),
        target_schema_id: SchemaId("test".to_string()),
        target_schema_version_id: SchemaVersionId(2),
        diff_atoms: vec![SchemaDiffAtom::new(
            SchemaElementRef::new(
                SchemaElementKind::Field,
                SchemaId("test".to_string()),
                SchemaVersionId(2),
                Some(KindId(1)),
                "tag",
            ),
            vec![SchemaStratum::StructuralShape, SchemaStratum::PublicationContract],
            SchemaPublicationImpact::ObservableSurfaceChanged,
            SchemaSubscriberImpact::ConsumableSurfaceChanged,
            HistoricalInterpretationSensitivity::NotSensitive,
            SchemaDiffDetail::AddedField {
                field_name: "tag".into(),
                required: false,
                default_expression: Some("null".into()),
            },
        )],
    };
    let mut txn = runtime.begin_transaction(
        TransactionOptions::default().with_schema_transition(
            proposed_transition,
            Some(SchemaReconciliationPolicy::PreserveInformation),
        ),
    );
    txn.push_batch(batch_create("b"));
    let outcome = txn.commit().unwrap();

    assert!(runtime.history_authority().tamper_commit_envelope_for_test(
        outcome.commit.commit_id,
        |envelope| {
            if let Some(descriptor) = envelope.schema_continuation_descriptor.as_mut() {
                descriptor.normalized_boundary_count += 1;
            }
        }
    ));

    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: outcome.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::AuditRecoveryVerification,
        });

    assert_eq!(replay.failure, Some(ReplayFailureClass::ObservableMismatch));
    assert!(replay.mismatches.iter().any(|mismatch| {
        mismatch.class == ReplayMismatchClass::SchemaContinuationDescriptorDrift
            && mismatch.surface == ReplayObservableSurface::History
            && mismatch.verification_layer
                == crate::facade::replay::ReplayVerificationLayer::DeepArtifactParity
    }), "{:?}", replay.mismatches);
}

#[test]
fn replay_certification_audit_drift_is_explained_and_counted() {
    let mut runtime = runtime_with_test_schema();
    let _first = create_entity_outcome(&mut runtime, "a");

    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(2),
        ..AspectSchemaFixture::default()
    }
    .build_registry();

    let proposed_transition = ProposedSchemaTransition {
        source_schema_id: SchemaId("test".to_string()),
        source_schema_version_id: SchemaVersionId(1),
        target_schema_id: SchemaId("test".to_string()),
        target_schema_version_id: SchemaVersionId(2),
        diff_atoms: vec![SchemaDiffAtom::new(
            SchemaElementRef::new(
                SchemaElementKind::Field,
                SchemaId("test".to_string()),
                SchemaVersionId(2),
                Some(KindId(1)),
                "tag",
            ),
            vec![SchemaStratum::StructuralShape, SchemaStratum::PublicationContract],
            SchemaPublicationImpact::ObservableSurfaceChanged,
            SchemaSubscriberImpact::ConsumableSurfaceChanged,
            HistoricalInterpretationSensitivity::NotSensitive,
            SchemaDiffDetail::AddedField {
                field_name: "tag".into(),
                required: false,
                default_expression: Some("null".into()),
            },
        )],
    };
    let mut txn = runtime.begin_transaction(
        TransactionOptions::default().with_schema_transition(
            proposed_transition,
            Some(SchemaReconciliationPolicy::PreserveInformation),
        ),
    );
    txn.push_batch(batch_create("b"));
    let outcome = txn.commit().unwrap();

    assert!(runtime.history_authority().tamper_commit_envelope_for_test(
        outcome.commit.commit_id,
        |envelope| {
            if let Some(descriptor) = envelope.schema_continuation_descriptor.as_mut() {
                descriptor.normalized_boundary_count += 1;
            }
        }
    ));

    runtime.performance_access().reset_counters();
    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: outcome.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::AuditRecoveryVerification,
        });

    assert_eq!(replay.failure, Some(ReplayFailureClass::ObservableMismatch));
    let diagnostics = runtime.publication_access().diagnostics();
    let compatibility_entry = diagnostics
        .by_scope(DiagnosticsScope::Replay)
        .into_iter()
        .flat_map(|artifact| artifact.entries.iter())
        .find(|entry| {
            entry.code == DiagnosticCode::InvariantViolation
                && entry.fields.get("verification_mode").is_some()
        })
        .expect("replay certification diagnostic");
    assert_eq!(
        compatibility_entry.fields["verification_mode"],
        serde_json::json!("AuditRecoveryVerification")
    );
    assert!(
        compatibility_entry.fields["mismatch_verification_layers"]
            .as_array()
            .unwrap()
            .iter()
            .any(|value| value == "DeepArtifactParity")
    );
    assert!(
        compatibility_entry.fields["mismatch_classes"]
            .as_array()
            .unwrap()
            .iter()
            .any(|value| value == "SchemaContinuationDescriptorDrift")
    );
    let counters = runtime.performance_access().counters();
    assert!(counters.replay_deep_artifact_parity_checks >= 1);
    assert_eq!(counters.replay_summary_parity_checks, 0);
}

#[test]
fn replay_contract_reports_schema_lineage_drift_at_summary_layer_when_digest_is_unavailable() {
    let mut runtime = runtime_with_test_schema();
    let _first = create_entity_outcome(&mut runtime, "a");

    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(2),
        ..AspectSchemaFixture::default()
    }
    .build_registry();

    let proposed_transition = ProposedSchemaTransition {
        source_schema_id: SchemaId("test".to_string()),
        source_schema_version_id: SchemaVersionId(1),
        target_schema_id: SchemaId("test".to_string()),
        target_schema_version_id: SchemaVersionId(2),
        diff_atoms: vec![SchemaDiffAtom::new(
            SchemaElementRef::new(
                SchemaElementKind::Field,
                SchemaId("test".to_string()),
                SchemaVersionId(2),
                Some(KindId(1)),
                "tag",
            ),
            vec![SchemaStratum::StructuralShape, SchemaStratum::PublicationContract],
            SchemaPublicationImpact::ObservableSurfaceChanged,
            SchemaSubscriberImpact::ConsumableSurfaceChanged,
            HistoricalInterpretationSensitivity::NotSensitive,
            SchemaDiffDetail::AddedField {
                field_name: "tag".into(),
                required: false,
                default_expression: Some("null".into()),
            },
        )],
    };
    let mut txn = runtime.begin_transaction(
        TransactionOptions::default().with_schema_transition(
            proposed_transition,
            Some(SchemaReconciliationPolicy::PreserveInformation),
        ),
    );
    txn.push_batch(batch_create("b"));
    let outcome = txn.commit().unwrap();

    assert!(runtime.history_authority().tamper_commit_envelope_for_test(
        outcome.commit.commit_id,
        |envelope| {
            if let Some(descriptor) = envelope.schema_reconciliation_descriptor.as_mut() {
                descriptor
                    .resulting_lineage
                    .parent_schema_version_ids
                    .push(SchemaVersionId(999));
            }
            if let Some(transition) = envelope.schema_transition.as_mut() {
                transition
                    .reconciliation_descriptor
                    .resulting_lineage
                    .parent_schema_version_ids
                    .push(SchemaVersionId(999));
            }
        }
    ));

    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: outcome.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
        });

    assert_eq!(replay.failure, Some(ReplayFailureClass::ObservableMismatch));
    assert!(replay.mismatches.iter().any(|mismatch| {
        mismatch.class == ReplayMismatchClass::SchemaLineageDrift
            && mismatch.surface == ReplayObservableSurface::History
            && mismatch.verification_layer
                == crate::facade::replay::ReplayVerificationLayer::SummaryParity
    }), "{:?}", replay.mismatches);
}

#[test]
fn replay_contract_preserves_aspect_bearing_patch_and_history_surfaces() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let created = create_entity_outcome(&mut runtime, "before");
    let entity = changed_entities(&created)[0];
    let updated = update_entity(&mut runtime, entity, "after");
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let relation_outcome = create_relation_outcome(&mut runtime, source, target, "r1");
    let relation = changed_relations(&relation_outcome)[0];
    let expected_entity_history =
        runtime
            .history_access()
            .entity_aspect_history(&BranchId("main".to_string()), entity, None);
    let expected_relation_history = runtime.history_access().relation_aspect_history(
        &BranchId("main".to_string()),
        relation,
        None,
    );
    let expected_entity_digest = runtime
        .history_access()
        .entity_aspect_history_with_trace(&BranchId("main".to_string()), entity, None)
        .aspect_history_digest();
    let expected_relation_digest = runtime
        .history_access()
        .relation_aspect_history_with_trace(&BranchId("main".to_string()), relation, None)
        .aspect_history_digest();

    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: relation_outcome.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
        });

    assert!(runtime.replay_access().compare_outcome(&replay));
    assert!(replay
        .compared_surfaces
        .contains(&ReplayObservableSurface::Patch));
    assert!(replay
        .compared_surfaces
        .contains(&ReplayObservableSurface::Diagnostics));
    assert!(replay
        .compared_surfaces
        .contains(&ReplayObservableSurface::History));
    assert_eq!(expected_entity_history.len(), 2);
    assert_eq!(expected_relation_history.len(), 1);
    assert_eq!(expected_entity_digest.entry_count, 2);
    assert_eq!(expected_relation_digest.entry_count, 1);
    assert_patch_truth_invariants(&updated);
    assert_patch_truth_invariants(&relation_outcome);
}

#[test]
fn replay_contract_reports_lineage_event_drift_at_digest_layer_when_artifacts_are_tampered() {
    let mut runtime = runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "first");
    let second = create_entity_outcome(&mut runtime, "second");
    let first_lineage = runtime
        .lineage_access()
        .for_record(changed_entities(&first)[0])
        .unwrap()
        .lineage_id;
    let second_lineage = runtime
        .lineage_access()
        .for_record(changed_entities(&second)[0])
        .unwrap()
        .lineage_id;
    let candidate = runtime.lineage_authority().record_correspondence_candidate(
        BranchId("main".to_string()),
        vec![first_lineage],
        vec![second_lineage],
        "lineage-drift",
    );
    runtime
        .lineage_authority()
        .promote_correspondence(candidate.candidate_id, second.commit.clone())
        .unwrap();

    assert!(runtime.history_authority().tamper_commit_envelope_for_test(
        second.commit.commit_id,
        |envelope| {
            if let Some(event) = envelope.lineage_events.first_mut() {
                event.kind = crate::facade::lineage::LineageEventKind::Retire;
            }
        }
    ));

    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: second.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
        });

    assert_eq!(replay.failure, Some(ReplayFailureClass::ObservableMismatch));
    assert!(replay.mismatches.iter().any(|mismatch| {
        mismatch.class == ReplayMismatchClass::LineageDrift
            && mismatch.surface == ReplayObservableSurface::Lineage
            && mismatch.verification_layer
                == crate::facade::replay::ReplayVerificationLayer::DigestParity
    }), "{:?}", replay.mismatches);
}

#[test]
fn replay_contract_reports_derived_index_drift_at_digest_layer_when_artifacts_are_tampered() {
    let mut runtime = runtime_with_test_schema();
    let commit = create_entity_outcome(&mut runtime, "indexed");
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(0),
        name: "entity-name".to_string(),
        kind: DerivedIndexKind::EntityPayloadField {
            field: "name".to_string(),
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
            if let Some(generation) = envelope.index_generations.first_mut() {
                generation.status = crate::facade::indexes::DerivedIndexPublicationStatus::BuildFailed;
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
    assert!(replay.mismatches.iter().any(|mismatch| {
        mismatch.class == ReplayMismatchClass::DerivedIndexDrift
            && mismatch.surface == ReplayObservableSurface::DerivedIndexes
            && mismatch.verification_layer
                == crate::facade::replay::ReplayVerificationLayer::DigestParity
    }), "{:?}", replay.mismatches);
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
                        client_key: InternedString::Raw("anchor-replaced".to_string()),
                        payload: RecordPayload::StructuredJson(json!({"name":"anchor-replaced"})),
                    },
                }),
            )),
        );
        txn.commit().unwrap()
    };
    runtime.durability_authority().checkpoint().unwrap();

    let start_lineage = runtime.lineage_access().for_record(anchor).unwrap().lineage_id;

    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: replace_outcome.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
        });
    assert!(runtime.replay_access().compare_outcome(&replay));

    let recovery_plan = runtime.durability_access().recovery_plan();
    let mut recovered =
        persisted_runtime_with_declared_aspect_schema(CascadeDeletePolicy::RetainDanglingForAudit);
    recovered
        .durability_authority()
        .recover(recovery_plan)
        .unwrap();
    let recovered_replay_check = recovered
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
    assert!(recovered.replay_access().compare_outcome(&recovered_replay_check));
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
fn replay_contract_preserves_relation_integrity_declared_schema() {
    let schema = RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: KindAspectDeclarations::default(),
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
                relation_integrity: crate::schema::data::RelationIntegrityDeclarations::new(
                    Vec::new(),
                    vec![crate::schema::data::CardinalityContractDeclaration {
                        contract_id: "source_max_one".to_string(),
                        source_max: Some(1),
                        target_max: None,
                        pair_max: None,
                    }],
                    vec![crate::schema::data::UniquenessContractDeclaration {
                        contract_id: "uniq".to_string(),
                        scope: crate::schema::data::UniquenessScope::DirectedSemanticEdge,
                    }],
                    Vec::new(),
                    Vec::new(),
                ),
            })
        })
        .unwrap();
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(schema)
        .build();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let outcome = create_relation_outcome(&mut runtime, source, target, "guarded");

    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: outcome.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
        });
    let replay_access = runtime.replay_access();
    let envelope = replay_access
        .canonical_commit_envelope(outcome.commit.commit_id)
        .unwrap();

    assert!(runtime.replay_access().compare_outcome(&replay));
    assert_eq!(
        envelope
            .schema_registry
            .relation_registration(KindId(2))
            .unwrap()
            .relation_integrity
            .cardinality_contracts[0]
            .contract_id,
        "source_max_one"
    );
    assert_eq!(
        envelope
            .schema_registry
            .relation_registration(KindId(2))
            .unwrap()
            .relation_integrity
            .uniqueness_contracts[0]
            .contract_id,
        "uniq"
    );
}

#[test]
fn replay_contract_preserves_branch_local_relation_integrity_truth_after_rejected_feature_attempt() {
    let mut runtime = source_max_one_relation_integrity_runtime();
    let source = create_entity(&mut runtime, "source");
    let target_a = create_entity(&mut runtime, "target-a");
    let target_b = create_entity(&mut runtime, "target-b");
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();

    let accepted_feature = {
        let mut txn = runtime.begin_transaction(TransactionOptions {
            target_branch: Some(BranchId("feature".to_string())),
            ..TransactionOptions::default()
        });
        txn.push_batch(
            WorkerIntentBatch::new("accepted-feature-relation").push(MutationIntent::Create(
                CreateIntent::Relation(crate::transactions::data::RelationSpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(2),
                    client_key: InternedString::Raw("feature-accepted".to_string()),
                    source,
                    target: target_a,
                    payload: Some(RecordPayload::StructuredJson(
                        json!({"label":"feature-accepted"}),
                    )),
                }),
            )),
        );
        txn.commit().unwrap()
    };
    let feature_head_before_reject = runtime
        .history_access()
        .branch_head(&BranchId("feature".to_string()))
        .cloned();

    let mut rejected_txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(BranchId("feature".to_string())),
        ..TransactionOptions::default()
    });
    rejected_txn.push_batch(
        WorkerIntentBatch::new("rejected-feature-relation").push(MutationIntent::Create(
            CreateIntent::Relation(crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw("feature-rejected".to_string()),
                source,
                target: target_b,
                payload: Some(RecordPayload::StructuredJson(
                    json!({"label":"feature-rejected"}),
                )),
            }),
        )),
    );
    let rejected = rejected_txn.commit().unwrap_err();

    match rejected {
        TransactionCommitError::Conflict { error, .. } => {
            assert_eq!(error.code(), DiagnosticCode::RelationCardinalityViolation);
        }
        other => panic!("expected conflict, got {:?}", other),
    }
    assert_eq!(
        runtime.history_access().branch_head(&BranchId("feature".to_string())),
        feature_head_before_reject.as_ref()
    );

    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: accepted_feature.commit.commit_id,
            branch_id: BranchId("feature".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
        });

    assert!(runtime.replay_access().compare_outcome(&replay));
    assert!(replay
        .compared_surfaces
        .contains(&ReplayObservableSurface::History));
    assert_eq!(
        runtime
            .history_access()
            .branch_head(&BranchId("feature".to_string()))
            .unwrap()
            .commit_id,
        accepted_feature.commit.commit_id
    );
}
