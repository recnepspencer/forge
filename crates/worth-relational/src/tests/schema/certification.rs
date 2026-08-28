use crate::facade::schema::{
    DescriptorCanonicalBasisVersion, DescriptorSemanticsVersion, FreeFormSchemaDiffIntent,
    HistoricalInterpretationSensitivity, ProposedSchemaTransition, SchemaDiffAtom,
    SchemaDiffDetail, SchemaElementKind, SchemaElementRef, SchemaId, SchemaPublicationImpact,
    SchemaReconciliationClassification, SchemaReconciliationPolicy, SchemaStratum,
    SchemaSubscriberImpact, SchemaVersionId,
};
use crate::replay::data::{
    digest_schema_transition_decision, digest_schema_transition_descriptor,
    digest_subscriber_boundary_cdc_surface, digest_subscriber_continuation_summary,
};
use crate::schema::{
    classify_schema_transition, lower_schema_transition, validate_schema_transition,
};
use crate::tests::support::*;
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
fn schema_evolution_cdc_contract_test() {
    let mut runtime = persisted_runtime_with_test_schema();
    let baseline = create_entity_outcome(&mut runtime, "anchor");
    let baseline_checkpoint =
        checkpoint_for_schema_version(baseline.patch_position(), SchemaVersionId(1));

    let schema_v2 = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(2),
        ..AspectSchemaFixture::default()
    }
    .build_registry();
    runtime.set_schema_registry_for_test(schema_v2);

    let context = crate::tests::support::test_owner_transaction_validation_input_for_main(&runtime)
        .with_schema_transition(
            schema_transition_for_subscriber_impact(
                SchemaVersionId(2),
                SchemaSubscriberImpact::ConsumableSurfaceChanged,
            ),
            Some(SchemaReconciliationPolicy::PreserveInformation),
        );
    let mut txn = runtime
        .begin_branch_transaction(context.basis(), context.intent().clone())
        .expect("owner-admitted transaction context");
    txn.push_batch(batch_create("boundary"))
        .expect("test staging stays within configured resource budgets");
    let committed = txn.commit(&mut runtime).unwrap();

    let live_batch = runtime
        .publication()
        .read_subscriber_stream(SubscriberResumeRequest::resume_after(
            baseline_checkpoint.clone(),
            32,
        ))
        .unwrap();

    let schema_transition_digest = digest_schema_transition_descriptor(
        committed.envelope().schema_transition.as_ref().unwrap(),
        committed.envelope().descriptor_semantics_version,
    );
    let schema_boundary_cdc_digest = digest_subscriber_boundary_cdc_surface(
        &live_batch.patches,
        live_batch.continuation.crossed_boundaries(),
        live_batch.continuation.continuation_summary(),
        &live_batch.recovery_decision,
    );
    let subscriber_contract_matrix =
        digest_subscriber_continuation_summary(live_batch.continuation.continuation_summary());
    let transition_decision_digest = digest_schema_transition_decision(
        committed
            .envelope()
            .schema_continuation_descriptor
            .as_ref()
            .unwrap(),
        committed
            .envelope()
            .schema_reconciliation_descriptor
            .as_ref()
            .unwrap(),
        committed.envelope().descriptor_semantics_version,
    );
    let descriptor_semantics_version = committed.envelope().descriptor_semantics_version;

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
                root_path: unique_test_store_path("worth-relational-m5-schema-evolution-cdc"),
                segment_commit_capacity: 2,
            })
            .build()
    });

    let recovered_envelope = recovered
        .replay()
        .canonical_commit_envelope(committed.commit.commit_id)
        .expect("recovered canonical envelope");
    let recovered_batch = recovered
        .publication()
        .read_subscriber_stream(SubscriberResumeRequest::resume_after(
            baseline_checkpoint,
            32,
        ))
        .unwrap();

    assert_eq!(
        schema_transition_digest,
        digest_schema_transition_descriptor(
            recovered_envelope.schema_transition.as_ref().unwrap(),
            recovered_envelope.descriptor_semantics_version,
        )
    );
    assert_eq!(
        schema_boundary_cdc_digest,
        digest_subscriber_boundary_cdc_surface(
            &recovered_batch.patches,
            recovered_batch.continuation.crossed_boundaries(),
            recovered_batch.continuation.continuation_summary(),
            &recovered_batch.recovery_decision,
        )
    );
    assert_eq!(
        subscriber_contract_matrix,
        digest_subscriber_continuation_summary(recovered_batch.continuation.continuation_summary())
    );
    assert_eq!(
        transition_decision_digest,
        digest_schema_transition_decision(
            recovered_envelope
                .schema_continuation_descriptor
                .as_ref()
                .unwrap(),
            recovered_envelope
                .schema_reconciliation_descriptor
                .as_ref()
                .unwrap(),
            recovered_envelope.descriptor_semantics_version,
        )
    );
    assert_eq!(
        descriptor_semantics_version,
        recovered_envelope.descriptor_semantics_version
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
        DescriptorCanonicalBasisVersion::default(),
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
            vec![
                SchemaStratum::StructuralShape,
                SchemaStratum::PublicationContract,
            ],
            SchemaPublicationImpact::ObservableSurfaceChanged,
            SchemaSubscriberImpact::RenegotiationRequired,
            HistoricalInterpretationSensitivity::SensitiveToPublicationMeaning,
            SchemaDiffDetail::RemovedField {
                field: field_key("obsolete_field"),
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
        DescriptorCanonicalBasisVersion::default(),
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
                vec![
                    SchemaStratum::ValueDomain,
                    SchemaStratum::PublicationContract,
                ],
                SchemaPublicationImpact::ObservableSurfaceChanged,
                SchemaSubscriberImpact::RenegotiationRequired,
                HistoricalInterpretationSensitivity::SensitiveToValueMeaning,
                SchemaDiffDetail::TypeChanged {
                    field: field_key("timing_domain"),
                    from_type: "enum<previous>".into(),
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
                vec![
                    SchemaStratum::BehavioralSemantics,
                    SchemaStratum::PublicationContract,
                ],
                SchemaPublicationImpact::ProjectionContractChanged,
                SchemaSubscriberImpact::RenegotiationRequired,
                HistoricalInterpretationSensitivity::SensitiveToPublicationMeaning,
                SchemaDiffDetail::FreeText {
                    detail: "projection semantics became ambiguous".into(),
                    declared_intent: FreeFormSchemaDiffIntent::StructuralContinuityDenied,
                },
            )],
        },
        Some(SchemaReconciliationPolicy::PreserveInformation),
    );

    let schema_conflict_localization_report = [
        additive_plan.validated.proposed.diff_atoms[0]
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
    ];
    let replayed_additive_reconciliation = lower_schema_transition(
        additive,
        Some(SchemaReconciliationPolicy::PreserveInformation),
        DescriptorSemanticsVersion::default(),
        DescriptorCanonicalBasisVersion::default(),
    )
    .reconciliation_descriptor;
    let descriptor_semantics_versions = [
        lower_schema_transition(
            narrowing_validated.clone(),
            Some(SchemaReconciliationPolicy::PreserveInformation),
            DescriptorSemanticsVersion::default(),
            DescriptorCanonicalBasisVersion::default(),
        )
        .continuation_descriptor
        .bridge
        .semantics_version,
        additive_plan
            .continuation_descriptor
            .bridge
            .semantics_version,
    ];

    assert!(narrowing_error
        .detail()
        .contains("requires an explicit preservation policy"));
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
        SchemaReconciliationClassification::TypeContinuityDenied
    );
    assert_eq!(
        structural_conflict.reconciliation,
        SchemaReconciliationClassification::StructuralContinuityDenied
    );
    assert_eq!(
        additive_plan
            .reconciliation_descriptor
            .resulting_lineage
            .resulting_schema_version_id,
        SchemaVersionId(2)
    );
    assert_eq!(
        narrowing_plan
            .reconciliation_descriptor
            .resulting_lineage
            .resulting_schema_version_id,
        SchemaVersionId(3)
    );
    assert_ne!(
        SchemaReconciliationPolicy::PreserveInformation,
        SchemaReconciliationPolicy::RejectLossyNarrowing
    );
    assert_eq!(
        schema_conflict_localization_report,
        ["tag", "obsolete_field", "timing_domain", "mass-properties"]
    );
    assert_eq!(
        additive_plan.reconciliation_descriptor,
        replayed_additive_reconciliation
    );
    assert_eq!(
        descriptor_semantics_versions,
        [
            DescriptorSemanticsVersion::default(),
            DescriptorSemanticsVersion::default()
        ]
    );
}
