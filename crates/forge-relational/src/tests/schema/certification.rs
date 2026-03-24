use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::facade::schema::{
    DescriptorCanonicalizationVersion, DescriptorSemanticsVersion, FreeFormSchemaDiffIntent,
    HistoricalInterpretationSensitivity, ProposedSchemaTransition, SchemaDiffAtom,
    SchemaDiffDetail, SchemaElementKind, SchemaElementRef, SchemaId,
    SchemaPublicationImpact, SchemaReconciliationClassification, SchemaReconciliationPolicy,
    SchemaStratum, SchemaSubscriberImpact, SchemaVersionId,
};
use crate::schema::logic::{
    classify_schema_transition, lower_schema_transition, validate_schema_transition,
};
use crate::tests::support::*;

fn certification_digest<T: Serialize>(value: &T) -> String {
    let bytes = serde_json::to_vec(value).expect("certification serialization");
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

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
            vec![SchemaStratum::StructuralShape, SchemaStratum::PublicationContract],
            SchemaPublicationImpact::ObservableSurfaceChanged,
            subscriber_impact,
            HistoricalInterpretationSensitivity::NotSensitive,
            SchemaDiffDetail::AddedField {
                field_name: "tag".into(),
                required: false,
                default_expression: Some("null".into()),
            },
        )
        .with_boundary_visibility_proof(
            match subscriber_impact {
                SchemaSubscriberImpact::ConsumableSurfaceChanged => {
                    crate::schema::data::SubscriberBoundaryVisibility::VisibleSemanticallyIgnorable
                }
                SchemaSubscriberImpact::ContractUpgradeRequired => {
                    crate::schema::data::SubscriberBoundaryVisibility::VisibleRequiresContractUptake
                }
                _ => crate::schema::data::SubscriberBoundaryVisibility::NotVisible,
            },
        )],
    }
}

#[test]
fn schema_evolution_cdc_contract_test() {
    let mut runtime = persisted_runtime_with_test_schema();
    let baseline = create_entity_outcome(&mut runtime, "anchor");
    let baseline_checkpoint =
        checkpoint_for_schema_version(baseline.patch_position(), SchemaVersionId(1));

    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(2),
        ..AspectSchemaFixture::default()
    }
    .build_registry();

    let mut txn = runtime.begin_transaction(
        TransactionOptions::default().with_schema_transition(
            schema_transition_for_subscriber_impact(
                SchemaVersionId(2),
                SchemaSubscriberImpact::ConsumableSurfaceChanged,
            ),
            Some(SchemaReconciliationPolicy::PreserveInformation),
        ),
    );
    txn.push_batch(batch_create("boundary"));
    let committed = txn.commit().unwrap();

    let live_batch = runtime
        .publication_access()
        .read_subscriber_stream(SubscriberResumeRequest::resume_after(
            baseline_checkpoint.clone(),
            32,
        ))
        .unwrap();

    let schema_transition_digest =
        certification_digest(committed.envelope().schema_transition.as_ref().unwrap());
    let schema_boundary_cdc_digest = certification_digest(&(
        &live_batch.patches,
        &live_batch.crossed_boundaries,
        &live_batch.continuation_summary,
        &live_batch.recovery_decision,
    ));
    let subscriber_contract_matrix = certification_digest(&live_batch.continuation_summary);
    let transition_decision_digest = certification_digest(&(
        committed.envelope().schema_continuation_descriptor.as_ref().unwrap(),
        committed
            .envelope()
            .schema_reconciliation_descriptor
            .as_ref()
            .unwrap(),
    ));
    let descriptor_version_digest =
        certification_digest(&committed.envelope().descriptor_semantics_version);

    let (_recovery, recovered) = checkpoint_and_recover_with(&mut runtime, || {
        let registry = AspectSchemaFixture {
            schema_version_id: SchemaVersionId(2),
            ..AspectSchemaFixture::default()
        }
        .build_registry();
        RelationalRuntimeApi::builder()
            .profile(RelationalRuntimeProfile::CertificationCore)
            .schema_registry(registry)
            .durability_mode(DurabilityMode::PersistedSegmentedLocalFs)
            .durable_store_layout(DurableStoreLayout {
                root_path: unique_test_store_path("forge-relational-m5-schema-evolution-cdc"),
                segment_commit_capacity: 2,
            })
            .build()
    });

    let recovered_envelope = recovered
        .replay_access()
        .canonical_commit_envelope(committed.commit.commit_id)
        .cloned()
        .expect("recovered canonical envelope");
    let recovered_batch = recovered
        .publication_access()
        .read_subscriber_stream(SubscriberResumeRequest::resume_after(
            baseline_checkpoint,
            32,
        ))
        .unwrap();

    assert_eq!(
        schema_transition_digest,
        certification_digest(recovered_envelope.schema_transition.as_ref().unwrap())
    );
    assert_eq!(
        schema_boundary_cdc_digest,
        certification_digest(&(
            &recovered_batch.patches,
            &recovered_batch.crossed_boundaries,
            &recovered_batch.continuation_summary,
            &recovered_batch.recovery_decision,
        ))
    );
    assert_eq!(
        subscriber_contract_matrix,
        certification_digest(&recovered_batch.continuation_summary)
    );
    assert_eq!(
        transition_decision_digest,
        certification_digest(&(
            recovered_envelope
                .schema_continuation_descriptor
                .as_ref()
                .unwrap(),
            recovered_envelope
                .schema_reconciliation_descriptor
                .as_ref()
                .unwrap(),
        ))
    );
    assert_eq!(
        descriptor_version_digest,
        certification_digest(&recovered_envelope.descriptor_semantics_version)
    );
}

#[test]
fn schema_reconciliation_classification_test() {
    let additive = validate_schema_transition(
        schema_transition_for_subscriber_impact(
            SchemaVersionId(2),
            SchemaSubscriberImpact::ConsumableSurfaceChanged,
        ),
        Some(SchemaReconciliationPolicy::PreserveInformation),
    )
    .unwrap();
    let additive_plan = lower_schema_transition(
        additive.clone(),
        Some(SchemaReconciliationPolicy::PreserveInformation),
        DescriptorSemanticsVersion::default(),
        DescriptorCanonicalizationVersion::default(),
    );

    let narrowing = ProposedSchemaTransition {
        source_schema_id: SchemaId("test".to_string()),
        source_schema_version_id: SchemaVersionId(2),
        target_schema_id: SchemaId("test".to_string()),
        target_schema_version_id: SchemaVersionId(3),
        diff_atoms: vec![SchemaDiffAtom::new(
            SchemaElementRef::new(
                SchemaElementKind::Field,
                SchemaId("test".to_string()),
                SchemaVersionId(3),
                Some(KindId(1)),
                "obsolete_field",
            ),
            vec![SchemaStratum::StructuralShape, SchemaStratum::PublicationContract],
            SchemaPublicationImpact::ObservableSurfaceChanged,
            SchemaSubscriberImpact::RenegotiationRequired,
            HistoricalInterpretationSensitivity::SensitiveToPublicationMeaning,
            SchemaDiffDetail::RemovedField {
                field_name: "obsolete_field".into(),
            },
        )],
    };
    let narrowing_error = validate_schema_transition(narrowing.clone(), None).unwrap_err();
    let narrowing_validated = validate_schema_transition(
        narrowing,
        Some(SchemaReconciliationPolicy::PreserveInformation),
    )
    .unwrap();
    let narrowing_plan = lower_schema_transition(
        narrowing_validated.clone(),
        Some(SchemaReconciliationPolicy::PreserveInformation),
        DescriptorSemanticsVersion::default(),
        DescriptorCanonicalizationVersion::default(),
    );

    let type_conflict = classify_schema_transition(
        ProposedSchemaTransition {
            source_schema_id: SchemaId("test".to_string()),
            source_schema_version_id: SchemaVersionId(3),
            target_schema_id: SchemaId("test".to_string()),
            target_schema_version_id: SchemaVersionId(4),
            diff_atoms: vec![SchemaDiffAtom::new(
                SchemaElementRef::new(
                    SchemaElementKind::Field,
                    SchemaId("test".to_string()),
                    SchemaVersionId(4),
                    Some(KindId(1)),
                    "timing_domain",
                ),
                vec![SchemaStratum::ValueDomain, SchemaStratum::PublicationContract],
                SchemaPublicationImpact::ObservableSurfaceChanged,
                SchemaSubscriberImpact::RenegotiationRequired,
                HistoricalInterpretationSensitivity::SensitiveToValueMeaning,
                SchemaDiffDetail::TypeChanged {
                    field_name: "timing_domain".into(),
                    from_type: "enum<legacy>".into(),
                    to_type: "enum<expanded>".into(),
                },
            )],
        },
        Some(SchemaReconciliationPolicy::PreserveInformation),
    );

    let structural_conflict = classify_schema_transition(
        ProposedSchemaTransition {
            source_schema_id: SchemaId("test".to_string()),
            source_schema_version_id: SchemaVersionId(4),
            target_schema_id: SchemaId("test".to_string()),
            target_schema_version_id: SchemaVersionId(5),
            diff_atoms: vec![SchemaDiffAtom::new(
                SchemaElementRef::new(
                    SchemaElementKind::ProjectionContract,
                    SchemaId("test".to_string()),
                    SchemaVersionId(5),
                    None,
                    "mass-properties",
                ),
                vec![SchemaStratum::BehavioralSemantics, SchemaStratum::PublicationContract],
                SchemaPublicationImpact::ProjectionContractChanged,
                SchemaSubscriberImpact::RenegotiationRequired,
                HistoricalInterpretationSensitivity::SensitiveToPublicationMeaning,
                SchemaDiffDetail::FreeText {
                    detail: "projection semantics became ambiguous".into(),
                    declared_intent: FreeFormSchemaDiffIntent::StructuralIncompatible,
                },
            )],
        },
        Some(SchemaReconciliationPolicy::PreserveInformation),
    );

    let schema_reconciliation_digest = certification_digest(&(
        &additive_plan.reconciliation_descriptor,
        &narrowing_plan.reconciliation_descriptor,
        &type_conflict.reconciliation,
        &structural_conflict.reconciliation,
    ));
    let schema_lineage_digest = certification_digest(&(
        &additive_plan.reconciliation_descriptor.resulting_lineage,
        &narrowing_plan.reconciliation_descriptor.resulting_lineage,
    ));
    let reconciliation_policy_matrix = certification_digest(&[
        SchemaReconciliationPolicy::PreserveInformation,
        SchemaReconciliationPolicy::RejectLossyNarrowing,
    ]);
    let schema_conflict_localization_report = certification_digest(&[
        additive_plan
            .validated
            .proposed
            .diff_atoms[0]
            .element
            .element_name
            .to_string(),
        narrowing_validated.proposed.diff_atoms[0]
            .element
            .element_name
            .to_string(),
        type_conflict.proposed.diff_atoms[0]
            .element
            .element_name
            .to_string(),
        structural_conflict.proposed.diff_atoms[0]
            .element
            .element_name
            .to_string(),
    ]);
    let reconciliation_replay_digest = certification_digest(&(
        additive_plan.reconciliation_descriptor.clone(),
        lower_schema_transition(
            additive,
            Some(SchemaReconciliationPolicy::PreserveInformation),
            DescriptorSemanticsVersion::default(),
            DescriptorCanonicalizationVersion::default(),
        )
        .reconciliation_descriptor,
    ));
    let descriptor_version_digest = certification_digest(&(
        additive_plan.continuation_descriptor.bridge.semantics_version,
        narrowing_plan.continuation_descriptor.bridge.semantics_version,
    ));

    assert!(
        narrowing_error
            .detail()
            .contains("requires an explicit preservation policy")
    );
    assert_eq!(
        additive_plan.reconciliation_descriptor.classification,
        SchemaReconciliationClassification::Additive
    );
    assert_eq!(
        narrowing_plan.reconciliation_descriptor.classification,
        SchemaReconciliationClassification::Narrowing
    );
    assert_eq!(
        type_conflict.reconciliation,
        SchemaReconciliationClassification::TypeIncompatible
    );
    assert_eq!(
        structural_conflict.reconciliation,
        SchemaReconciliationClassification::StructuralIncompatible
    );
    assert!(!schema_reconciliation_digest.is_empty());
    assert!(!schema_lineage_digest.is_empty());
    assert!(!reconciliation_policy_matrix.is_empty());
    assert!(!schema_conflict_localization_report.is_empty());
    assert!(!reconciliation_replay_digest.is_empty());
    assert!(!descriptor_version_digest.is_empty());
}
