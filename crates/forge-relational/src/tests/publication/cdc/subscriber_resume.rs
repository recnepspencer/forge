use crate::tests::support::*;
use crate::{
    publication::cdc::data::{
        SubscriberContinuationClassSet, SubscriberContractDeclaration, SubscriberStrataSet,
    },
    publication::cdc::execution::collect_crossed_boundaries,
    schema::data::{
        DescriptorCanonicalizationVersion, DescriptorSemanticsVersion,
        HistoricalInterpretationSensitivity, ProposedSchemaTransition, SchemaBoundaryFingerprint,
        SchemaBridgeDescriptor, SchemaBridgeabilityClassification,
        SchemaContinuationClassification, SchemaContinuationDescriptor, SchemaDiffAtom,
        SchemaDiffDetail, SchemaElementKind, SchemaElementRef, SchemaId, SchemaPublicationImpact,
        SchemaReconciliationPolicy, SchemaStratum, SchemaSubscriberImpact, SchemaVersionId,
    },
};

fn schema_transition_for_subscriber_impact(
    target_schema_version_id: SchemaVersionId,
    subscriber_impact: SchemaSubscriberImpact,
) -> ProposedSchemaTransition {
    ProposedSchemaTransition {
        source_schema_id: SchemaId("test".to_string()),
        source_schema_version_id: SchemaVersionId(target_schema_version_id.0 - 1),
        target_schema_id: SchemaId("test".to_string()),
        target_schema_version_id,
        diff_atoms: vec![SchemaDiffAtom::new(
            SchemaElementRef::new(
                SchemaElementKind::Field,
                SchemaId("test".to_string()),
                target_schema_version_id,
                Some(KindId(1)),
                "tag",
            ),
            vec![
                SchemaStratum::StructuralShape,
                SchemaStratum::PublicationContract,
            ],
            SchemaPublicationImpact::ObservableSurfaceChanged,
            subscriber_impact,
            HistoricalInterpretationSensitivity::NotSensitive,
            SchemaDiffDetail::AddedField {
                field: field_key("tag"),
                required: false,
                default_expression: Some("null".into()),
            },
        )
        .with_boundary_visibility_proof(match subscriber_impact {
            SchemaSubscriberImpact::ConsumableSurfaceChanged => {
                crate::schema::data::SubscriberBoundaryVisibility::VisibleSemanticallyIgnorable
            }
            SchemaSubscriberImpact::ContractUpgradeRequired => {
                crate::schema::data::SubscriberBoundaryVisibility::VisibleRequiresContractUptake
            }
            _ => crate::schema::data::SubscriberBoundaryVisibility::NotVisible,
        })],
    }
}

#[test]
fn subscriber_stream_resume_uses_checkpoint_type_and_batches_without_duplication() {
    let mut runtime = runtime_with_test_schema();
    let _first = create_entity_outcome(&mut runtime, "a");
    let _second = create_entity_outcome(&mut runtime, "b");
    let _third = create_entity_outcome(&mut runtime, "c");

    let first_batch = runtime
        .publication()
        .read_subscriber_stream(SubscriberResumeRequest::from_head(2))
        .unwrap();
    let resumed = runtime
        .publication()
        .read_subscriber_stream(SubscriberResumeRequest::resume_after(
            first_batch.next_checkpoint.clone().unwrap(),
            2,
        ))
        .unwrap();

    assert_eq!(first_batch.patches.len(), 2);
    assert_eq!(first_batch.next_checkpoint.unwrap().position().0, 2);
    assert_eq!(
        first_batch
            .latest_available_checkpoint
            .unwrap()
            .position()
            .0,
        3
    );
    assert_eq!(resumed.patches.len(), 1);
    assert_eq!(resumed.resumed_from.unwrap().position().0, 2);
    assert_eq!(resumed.patches[0].position.0, 3);
}

#[test]
fn subscriber_stream_propagates_declared_contract_identity_into_checkpoints() {
    let mut runtime = runtime_with_test_schema();
    let _first = create_entity_outcome(&mut runtime, "a");
    let _second = create_entity_outcome(&mut runtime, "b");
    let contract = SubscriberContractDeclaration {
        contract_id: "subscriber.contract.geometry.v1".to_string(),
        ..SubscriberContractDeclaration::default()
    };

    let batch = runtime
        .publication()
        .read_subscriber_stream(
            SubscriberResumeRequest::from_head(2).with_subscriber_contract(contract.clone()),
        )
        .unwrap();

    let next_checkpoint = batch.next_checkpoint.unwrap();
    let latest_available_checkpoint = batch.latest_available_checkpoint.unwrap();

    assert_eq!(
        next_checkpoint.subscriber_contract_id(),
        contract.contract_id.as_str()
    );
    assert_eq!(
        latest_available_checkpoint.subscriber_contract_id(),
        contract.contract_id.as_str()
    );
    assert_eq!(
        next_checkpoint.descriptor_semantics_version(),
        DescriptorSemanticsVersion::default()
    );
    assert_eq!(
        next_checkpoint
            .normalized_continuation_proof()
            .descriptor_semantics_version(),
        DescriptorSemanticsVersion::default()
    );
}

#[test]
fn subscriber_stream_without_schema_boundaries_reports_unchanged_continuity() {
    let mut runtime = runtime_with_test_schema();
    let _first = create_entity_outcome(&mut runtime, "a");
    let _second = create_entity_outcome(&mut runtime, "b");

    let batch = runtime
        .publication()
        .read_subscriber_stream(SubscriberResumeRequest::from_head(2))
        .unwrap();

    assert_eq!(
        batch.continuation.continuation_outcome(),
        SchemaContinuationClassification::ContinueUnchanged
    );
    assert!(batch.continuation.crossed_boundaries().is_empty());
    assert!(!batch.continuation.contract_upgrade_applied());

    let next_checkpoint = batch.next_checkpoint.unwrap();
    assert_eq!(
        next_checkpoint
            .normalized_continuation_proof()
            .normalized_boundary_count(),
        0
    );
    assert!(next_checkpoint
        .normalized_continuation_proof()
        .boundary_fingerprints()
        .is_empty());
}

#[test]
fn subscriber_stream_reports_crossed_schema_boundary_from_in_memory_history() {
    let mut runtime = runtime_with_test_schema();
    let _first = create_entity_outcome(&mut runtime, "a");

    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(2),
        ..AspectSchemaFixture::default()
    }
    .build_registry();

    let proposed_transition = schema_transition_for_subscriber_impact(
        SchemaVersionId(2),
        SchemaSubscriberImpact::ConsumableSurfaceChanged,
    );

    let mut txn = runtime.begin_transaction(TransactionOptions::default().with_schema_transition(
        proposed_transition,
        Some(SchemaReconciliationPolicy::PreserveInformation),
    ));
    txn.push_batch(batch_create("b"));
    txn.commit().unwrap();

    let batch = runtime
        .publication()
        .read_subscriber_stream(SubscriberResumeRequest::from_head(10))
        .unwrap();

    assert_eq!(
        batch.continuation.continuation_outcome(),
        SchemaContinuationClassification::ContinueWithVisibleBridge
    );
    assert_eq!(batch.continuation.crossed_boundaries().len(), 1);
    assert_eq!(
        batch
            .continuation
            .continuation_summary()
            .continuation_outcome,
        SchemaContinuationClassification::ContinueWithVisibleBridge
    );
    assert_eq!(
        batch
            .continuation
            .continuation_summary()
            .crossed_boundary_count,
        1
    );
    assert_eq!(
        batch
            .next_checkpoint
            .unwrap()
            .normalized_continuation_proof()
            .normalized_boundary_count(),
        1
    );
}

#[test]
fn subscriber_stream_treats_unconsumed_boundary_as_unchanged() {
    let mut runtime = runtime_with_test_schema();
    let _first = create_entity_outcome(&mut runtime, "a");

    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(2),
        ..AspectSchemaFixture::default()
    }
    .build_registry();

    let proposed_transition = schema_transition_for_subscriber_impact(
        SchemaVersionId(2),
        SchemaSubscriberImpact::ConsumableSurfaceChanged,
    );

    let mut txn = runtime.begin_transaction(TransactionOptions::default().with_schema_transition(
        proposed_transition,
        Some(SchemaReconciliationPolicy::PreserveInformation),
    ));
    txn.push_batch(batch_create("b"));
    txn.commit().unwrap();

    let contract = SubscriberContractDeclaration {
        contract_id: "subscriber.contract.identity-only.v1".to_string(),
        consumable_strata: SubscriberStrataSet::new([SchemaStratum::EntityIdentitySemantics]),
        accepted_continuation_classes: SubscriberContinuationClassSet::new([
            SchemaContinuationClassification::ContinueUnchanged,
        ]),
        accepted_upgrade_classes: SubscriberContinuationClassSet::new([]),
    };

    let batch = runtime
        .publication()
        .read_subscriber_stream(
            SubscriberResumeRequest::from_head(10).with_subscriber_contract(contract),
        )
        .unwrap();

    assert_eq!(
        batch.continuation.continuation_outcome(),
        SchemaContinuationClassification::ContinueUnchanged
    );
    assert_eq!(batch.continuation.crossed_boundaries().len(), 1);
}

#[test]
fn subscriber_stream_rejects_unsupported_contract_upgrade_boundary() {
    let mut runtime = runtime_with_test_schema();
    let _first = create_entity_outcome(&mut runtime, "a");

    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(2),
        ..AspectSchemaFixture::default()
    }
    .build_registry();

    let proposed_transition = schema_transition_for_subscriber_impact(
        SchemaVersionId(2),
        SchemaSubscriberImpact::ContractUpgradeRequired,
    );

    let mut txn = runtime.begin_transaction(TransactionOptions::default().with_schema_transition(
        proposed_transition,
        Some(SchemaReconciliationPolicy::PreserveInformation),
    ));
    txn.push_batch(batch_create("b"));
    txn.commit().unwrap();

    let contract = SubscriberContractDeclaration {
        contract_id: "subscriber.contract.geometry.v1".to_string(),
        accepted_upgrade_classes: SubscriberContinuationClassSet::new([]),
        ..SubscriberContractDeclaration::default()
    };
    let error = runtime
        .publication()
        .read_subscriber_stream(
            SubscriberResumeRequest::from_head(10).with_subscriber_contract(contract),
        )
        .unwrap_err();

    assert_eq!(
        error.class,
        SubscriberStreamFailureClass::ContractUpgradeUnsupported
    );
    let rejection_entry = error
        .diagnostics
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .find(|entry| entry.code == DiagnosticCode::SubscriberContractEvaluated)
        .unwrap();
    assert_eq!(
        diagnostic_field(rejection_entry, "subscriber_contract_id"),
        &crate::diagnostics::data::RelationalDiagnosticValue::string(
            "subscriber.contract.geometry.v1"
        )
    );
    assert_eq!(
        diagnostic_field(rejection_entry, "failure_class"),
        &crate::diagnostics::data::RelationalDiagnosticValue::string("ContractUpgradeUnsupported")
    );
    assert_eq!(
        diagnostic_field(rejection_entry, "normalized_boundary_count_at_failure"),
        &crate::diagnostics::data::RelationalDiagnosticValue::Unsigned(1)
    );
}

#[test]
fn subscriber_stream_applies_contract_upgrade_when_declared_supported() {
    let mut runtime = runtime_with_test_schema();
    let _first = create_entity_outcome(&mut runtime, "a");

    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(2),
        ..AspectSchemaFixture::default()
    }
    .build_registry();

    let proposed_transition = schema_transition_for_subscriber_impact(
        SchemaVersionId(2),
        SchemaSubscriberImpact::ContractUpgradeRequired,
    );

    let mut txn = runtime.begin_transaction(TransactionOptions::default().with_schema_transition(
        proposed_transition,
        Some(SchemaReconciliationPolicy::PreserveInformation),
    ));
    txn.push_batch(batch_create("b"));
    txn.commit().unwrap();

    let contract = SubscriberContractDeclaration {
        contract_id: "subscriber.contract.geometry.v2".to_string(),
        accepted_upgrade_classes: SubscriberContinuationClassSet::new([
            SchemaContinuationClassification::ContinueWithContractUpgrade,
        ]),
        ..SubscriberContractDeclaration::default()
    };

    let batch = runtime
        .publication()
        .read_subscriber_stream(
            SubscriberResumeRequest::from_head(10).with_subscriber_contract(contract),
        )
        .unwrap();

    assert_eq!(
        batch.continuation.continuation_outcome(),
        SchemaContinuationClassification::ContinueWithContractUpgrade
    );
    assert!(batch.continuation.contract_upgrade_applied());
    assert_eq!(
        batch
            .continuation
            .continuation_summary()
            .continuation_outcome,
        SchemaContinuationClassification::ContinueWithContractUpgrade
    );
    assert_eq!(
        batch.recovery_decision.disposition,
        crate::publication::cdc::data::SubscriberRecoveryDisposition::ContinueWithContractUpgrade
    );
    assert!(batch.diagnostics.iter().any(|artifact| artifact
        .entries
        .iter()
        .any(|entry| { entry.code == DiagnosticCode::SubscriberContractEvaluated })));
    assert!(batch.diagnostics.iter().any(|artifact| artifact
        .entries
        .iter()
        .any(|entry| { entry.code == DiagnosticCode::SubscriberContractUpgradeDecision })));
}

#[test]
fn subscriber_stream_composes_prior_and_new_boundaries_into_normalized_proof() {
    let mut runtime = runtime_with_test_schema();
    let _first = create_entity_outcome(&mut runtime, "a");

    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(2),
        ..AspectSchemaFixture::default()
    }
    .build_registry();

    let mut txn = runtime.begin_transaction(TransactionOptions::default().with_schema_transition(
        schema_transition_for_subscriber_impact(
            SchemaVersionId(2),
            SchemaSubscriberImpact::ConsumableSurfaceChanged,
        ),
        Some(SchemaReconciliationPolicy::PreserveInformation),
    ));
    txn.push_batch(batch_create("b"));
    txn.commit().unwrap();

    let first_batch = runtime
        .publication()
        .read_subscriber_stream(SubscriberResumeRequest::from_head(10))
        .unwrap();
    let checkpoint = first_batch.next_checkpoint.clone().unwrap();
    assert_eq!(
        checkpoint
            .normalized_continuation_proof()
            .normalized_boundary_count(),
        1
    );

    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(3),
        ..AspectSchemaFixture::default()
    }
    .build_registry();

    let mut second_txn =
        runtime.begin_transaction(TransactionOptions::default().with_schema_transition(
            schema_transition_for_subscriber_impact(
                SchemaVersionId(3),
                SchemaSubscriberImpact::ConsumableSurfaceChanged,
            ),
            Some(SchemaReconciliationPolicy::PreserveInformation),
        ));
    second_txn.push_batch(batch_create("c"));
    second_txn.commit().unwrap();

    let resumed = runtime
        .publication()
        .read_subscriber_stream(SubscriberResumeRequest::resume_after(checkpoint, 10))
        .unwrap();

    let next_checkpoint = resumed.next_checkpoint.unwrap();
    assert_eq!(
        next_checkpoint
            .normalized_continuation_proof()
            .normalized_boundary_count(),
        2
    );
    assert_eq!(
        next_checkpoint
            .continuation_summary()
            .normalized_boundary_count,
        2
    );
    assert_eq!(
        next_checkpoint
            .normalized_continuation_proof()
            .boundary_fingerprints()
            .len(),
        2
    );
}

#[test]
fn subscriber_stream_rejects_renegotiation_required_boundary() {
    let mut runtime = runtime_with_test_schema();
    let _first = create_entity_outcome(&mut runtime, "a");

    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(2),
        ..AspectSchemaFixture::default()
    }
    .build_registry();

    let proposed_transition = schema_transition_for_subscriber_impact(
        SchemaVersionId(2),
        SchemaSubscriberImpact::RenegotiationRequired,
    );

    let mut txn = runtime.begin_transaction(TransactionOptions::default().with_schema_transition(
        proposed_transition,
        Some(SchemaReconciliationPolicy::PreserveInformation),
    ));
    txn.push_batch(batch_create("b"));
    txn.commit().unwrap();

    let error = runtime
        .publication()
        .read_subscriber_stream(SubscriberResumeRequest::from_head(10))
        .unwrap_err();

    assert_eq!(
        error.class,
        SubscriberStreamFailureClass::RenegotiationRequired
    );
    assert!(error
        .diagnostics
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .any(|entry| entry.code == DiagnosticCode::SubscriberBoundaryEvaluated));
    assert!(error
        .diagnostics
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .any(|entry| entry.code == DiagnosticCode::SubscriberRenegotiationDecision));
    let rejection_entry = error
        .diagnostics
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .find(|entry| entry.code == DiagnosticCode::SubscriberContractEvaluated)
        .unwrap();
    assert_eq!(
        diagnostic_field(rejection_entry, "normalized_boundary_count_at_failure"),
        &crate::diagnostics::data::RelationalDiagnosticValue::Unsigned(1)
    );
}

#[test]
fn subscriber_stream_mixed_boundaries_choose_strongest_supported_outcome_and_trace_each_boundary() {
    let mut runtime = runtime_with_test_schema();
    let _first = create_entity_outcome(&mut runtime, "a");

    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(2),
        ..AspectSchemaFixture::default()
    }
    .build_registry();
    let mut visible_txn =
        runtime.begin_transaction(TransactionOptions::default().with_schema_transition(
            schema_transition_for_subscriber_impact(
                SchemaVersionId(2),
                SchemaSubscriberImpact::ConsumableSurfaceChanged,
            ),
            Some(SchemaReconciliationPolicy::PreserveInformation),
        ));
    visible_txn.push_batch(batch_create("b"));
    visible_txn.commit().unwrap();

    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(3),
        ..AspectSchemaFixture::default()
    }
    .build_registry();
    let mut upgrade_txn =
        runtime.begin_transaction(TransactionOptions::default().with_schema_transition(
            schema_transition_for_subscriber_impact(
                SchemaVersionId(3),
                SchemaSubscriberImpact::ContractUpgradeRequired,
            ),
            Some(SchemaReconciliationPolicy::PreserveInformation),
        ));
    upgrade_txn.push_batch(batch_create("c"));
    upgrade_txn.commit().unwrap();

    let contract = SubscriberContractDeclaration {
        contract_id: "subscriber.contract.geometry.v3".to_string(),
        accepted_upgrade_classes: SubscriberContinuationClassSet::new([
            SchemaContinuationClassification::ContinueWithContractUpgrade,
        ]),
        ..SubscriberContractDeclaration::default()
    };
    let batch = runtime
        .publication()
        .read_subscriber_stream(
            SubscriberResumeRequest::from_head(10).with_subscriber_contract(contract),
        )
        .unwrap();

    assert_eq!(
        batch.continuation.continuation_outcome(),
        SchemaContinuationClassification::ContinueWithContractUpgrade
    );
    assert_eq!(batch.continuation.crossed_boundaries().len(), 2);
    let boundary_entries = batch
        .diagnostics
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .filter(|entry| entry.code == DiagnosticCode::SubscriberBoundaryEvaluated)
        .count();
    assert_eq!(boundary_entries, 2);
}

#[test]
fn resumed_subscriber_stream_mixed_boundaries_choose_strongest_supported_outcome() {
    let mut runtime = runtime_with_test_schema();
    let _first = create_entity_outcome(&mut runtime, "a");
    let contract = SubscriberContractDeclaration {
        contract_id: "subscriber.contract.geometry.v3".to_string(),
        accepted_upgrade_classes: SubscriberContinuationClassSet::new([
            SchemaContinuationClassification::ContinueWithContractUpgrade,
        ]),
        ..SubscriberContractDeclaration::default()
    };

    let first_batch = runtime
        .publication()
        .read_subscriber_stream(
            SubscriberResumeRequest::from_head(1).with_subscriber_contract(contract.clone()),
        )
        .unwrap();
    let checkpoint = first_batch.next_checkpoint.unwrap();

    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(2),
        ..AspectSchemaFixture::default()
    }
    .build_registry();
    let mut visible_txn =
        runtime.begin_transaction(TransactionOptions::default().with_schema_transition(
            schema_transition_for_subscriber_impact(
                SchemaVersionId(2),
                SchemaSubscriberImpact::ConsumableSurfaceChanged,
            ),
            Some(SchemaReconciliationPolicy::PreserveInformation),
        ));
    visible_txn.push_batch(batch_create("b"));
    visible_txn.commit().unwrap();

    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(3),
        ..AspectSchemaFixture::default()
    }
    .build_registry();
    let mut upgrade_txn =
        runtime.begin_transaction(TransactionOptions::default().with_schema_transition(
            schema_transition_for_subscriber_impact(
                SchemaVersionId(3),
                SchemaSubscriberImpact::ContractUpgradeRequired,
            ),
            Some(SchemaReconciliationPolicy::PreserveInformation),
        ));
    upgrade_txn.push_batch(batch_create("c"));
    upgrade_txn.commit().unwrap();

    let resumed = runtime
        .publication()
        .read_subscriber_stream(
            SubscriberResumeRequest::resume_after(checkpoint, 10)
                .with_subscriber_contract(contract),
        )
        .unwrap();

    assert_eq!(
        resumed.continuation.continuation_outcome(),
        SchemaContinuationClassification::ContinueWithContractUpgrade
    );
    assert_eq!(
        resumed.recovery_decision.disposition,
        crate::publication::cdc::data::SubscriberRecoveryDisposition::ContinueWithContractUpgrade
    );
    assert_eq!(resumed.continuation.crossed_boundaries().len(), 2);
    let next_checkpoint = resumed.next_checkpoint.unwrap();
    assert_eq!(
        next_checkpoint
            .normalized_continuation_proof()
            .normalized_boundary_count(),
        2
    );
    assert_eq!(
        next_checkpoint.continuation_summary().continuation_outcome,
        SchemaContinuationClassification::ContinueWithContractUpgrade
    );
    let boundary_entries = resumed
        .diagnostics
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .filter(|entry| entry.code == DiagnosticCode::SubscriberBoundaryEvaluated)
        .count();
    assert_eq!(boundary_entries, 2);
}

#[test]
fn resumed_subscriber_stream_preserves_prior_boundary_and_adds_new_boundary_trace() {
    let mut runtime = runtime_with_test_schema();
    let _first = create_entity_outcome(&mut runtime, "a");

    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(2),
        ..AspectSchemaFixture::default()
    }
    .build_registry();
    let mut first_transition =
        runtime.begin_transaction(TransactionOptions::default().with_schema_transition(
            schema_transition_for_subscriber_impact(
                SchemaVersionId(2),
                SchemaSubscriberImpact::ConsumableSurfaceChanged,
            ),
            Some(SchemaReconciliationPolicy::PreserveInformation),
        ));
    first_transition.push_batch(batch_create("b"));
    first_transition.commit().unwrap();

    let first_batch = runtime
        .publication()
        .read_subscriber_stream(SubscriberResumeRequest::from_head(10))
        .unwrap();
    let checkpoint = first_batch.next_checkpoint.unwrap();
    assert_eq!(
        checkpoint
            .normalized_continuation_proof()
            .normalized_boundary_count(),
        1
    );

    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(3),
        ..AspectSchemaFixture::default()
    }
    .build_registry();
    let mut second_transition =
        runtime.begin_transaction(TransactionOptions::default().with_schema_transition(
            schema_transition_for_subscriber_impact(
                SchemaVersionId(3),
                SchemaSubscriberImpact::ConsumableSurfaceChanged,
            ),
            Some(SchemaReconciliationPolicy::PreserveInformation),
        ));
    second_transition.push_batch(batch_create("c"));
    second_transition.commit().unwrap();

    let resumed = runtime
        .publication()
        .read_subscriber_stream(SubscriberResumeRequest::resume_after(checkpoint, 10))
        .unwrap();

    let next_checkpoint = resumed.next_checkpoint.unwrap();
    assert_eq!(
        next_checkpoint
            .normalized_continuation_proof()
            .normalized_boundary_count(),
        2
    );
    assert_eq!(
        next_checkpoint
            .normalized_continuation_proof()
            .boundary_fingerprints()
            .len(),
        2
    );
    assert_eq!(resumed.continuation.crossed_boundaries().len(), 1);
    let boundary_entries = resumed
        .diagnostics
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .filter(|entry| entry.code == DiagnosticCode::SubscriberBoundaryEvaluated)
        .count();
    assert_eq!(boundary_entries, 1);
}

#[test]
fn latest_available_checkpoint_reflects_head_continuation_state_for_subscriber_contract() {
    let mut runtime = runtime_with_test_schema();
    let _first = create_entity_outcome(&mut runtime, "a");
    let contract = SubscriberContractDeclaration {
        contract_id: "subscriber.contract.geometry.v3".to_string(),
        accepted_upgrade_classes: SubscriberContinuationClassSet::new([
            SchemaContinuationClassification::ContinueWithContractUpgrade,
        ]),
        ..SubscriberContractDeclaration::default()
    };

    let first_batch = runtime
        .publication()
        .read_subscriber_stream(
            SubscriberResumeRequest::from_head(1).with_subscriber_contract(contract.clone()),
        )
        .unwrap();
    let checkpoint = first_batch.next_checkpoint.unwrap();

    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(2),
        ..AspectSchemaFixture::default()
    }
    .build_registry();
    let mut visible_txn =
        runtime.begin_transaction(TransactionOptions::default().with_schema_transition(
            schema_transition_for_subscriber_impact(
                SchemaVersionId(2),
                SchemaSubscriberImpact::ConsumableSurfaceChanged,
            ),
            Some(SchemaReconciliationPolicy::PreserveInformation),
        ));
    visible_txn.push_batch(batch_create("b"));
    visible_txn.commit().unwrap();

    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(3),
        ..AspectSchemaFixture::default()
    }
    .build_registry();
    let mut upgrade_txn =
        runtime.begin_transaction(TransactionOptions::default().with_schema_transition(
            schema_transition_for_subscriber_impact(
                SchemaVersionId(3),
                SchemaSubscriberImpact::ContractUpgradeRequired,
            ),
            Some(SchemaReconciliationPolicy::PreserveInformation),
        ));
    upgrade_txn.push_batch(batch_create("c"));
    upgrade_txn.commit().unwrap();

    let resumed = runtime
        .publication()
        .read_subscriber_stream(
            SubscriberResumeRequest::resume_after(checkpoint, 1).with_subscriber_contract(contract),
        )
        .unwrap();

    let latest_available_checkpoint = resumed.latest_available_checkpoint.unwrap();
    assert_eq!(
        latest_available_checkpoint
            .continuation_summary()
            .continuation_outcome,
        SchemaContinuationClassification::ContinueWithContractUpgrade
    );
    assert_eq!(
        latest_available_checkpoint
            .normalized_continuation_proof()
            .normalized_boundary_count(),
        2
    );
}

#[test]
fn crossed_boundary_collection_deduplicates_without_losing_first_seen_order() {
    let mut runtime = runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "a");
    let second = create_entity_outcome(&mut runtime, "b");
    let third = create_entity_outcome(&mut runtime, "c");

    let fingerprint_a = SchemaBoundaryFingerprint::new([1_u8; 32]);
    let fingerprint_b = SchemaBoundaryFingerprint::new([2_u8; 32]);

    let mut first_envelope = first.envelope().clone();
    first_envelope.schema_continuation_descriptor = Some(SchemaContinuationDescriptor::new(
        fingerprint_a,
        SchemaBridgeDescriptor::new(
            fingerprint_a,
            DescriptorSemanticsVersion::default(),
            DescriptorCanonicalizationVersion::default(),
            SchemaContinuationClassification::ContinueWithVisibleBridge,
            SchemaBridgeabilityClassification::SubscriberVisible,
            HistoricalInterpretationSensitivity::NotSensitive,
            Vec::new(),
        ),
        1,
    ));

    let mut second_envelope = second.envelope().clone();
    second_envelope.schema_continuation_descriptor =
        first_envelope.schema_continuation_descriptor.clone();

    let mut third_envelope = third.envelope().clone();
    third_envelope.schema_continuation_descriptor = Some(SchemaContinuationDescriptor::new(
        fingerprint_b,
        SchemaBridgeDescriptor::new(
            fingerprint_b,
            DescriptorSemanticsVersion::default(),
            DescriptorCanonicalizationVersion::default(),
            SchemaContinuationClassification::ContinueWithVisibleBridge,
            SchemaBridgeabilityClassification::SubscriberVisible,
            HistoricalInterpretationSensitivity::NotSensitive,
            Vec::new(),
        ),
        1,
    ));

    let crossed = collect_crossed_boundaries(&[first_envelope, second_envelope, third_envelope]);

    assert_eq!(crossed, vec![fingerprint_a, fingerprint_b]);
}
