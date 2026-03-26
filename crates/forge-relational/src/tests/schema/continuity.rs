use std::sync::Arc;

use crate::authority::commit::phases::schema_continuity::{
    validate_schema_continuity_publication, SchemaContinuityPlan,
};
use crate::diagnostics::data::DiagnosticsArtifactKind;
use crate::facade::history::BranchId;
use crate::facade::identity::KindId;
use crate::facade::schema::{
    CompatibilityObservation, DescriptorCanonicalizationVersion, DescriptorSemanticsVersion,
    FreeFormSchemaDiffIntent, HistoricalInterpretationSensitivity, LoweredSchemaTransitionPlan,
    ProposedSchemaTransition, SchemaBoundaryFingerprint, SchemaBridgeDescriptor,
    SchemaBridgeabilityClassification, SchemaContinuationClassification,
    SchemaContinuationDescriptor, SchemaDiffAtom, SchemaDiffDetail, SchemaElementKind,
    SchemaElementRef, SchemaId, SchemaLineageArtifact, SchemaLineageOrderingSemantics,
    SchemaPublicationImpact, SchemaReconciliationClassification, SchemaReconciliationDescriptor,
    SchemaReconciliationOrderingMode, SchemaReconciliationPolicy, SchemaStratum,
    SchemaSubscriberImpact, SchemaTransitionArtifact, SchemaTransitionSummary, SchemaVersionId,
    ValidatedSchemaTransition,
};
use crate::schema::logic::{
    classify_schema_transition, lower_schema_transition, validate_schema_continuity_bundle,
    validate_schema_transition, SchemaContinuityBundleIssue,
};
use crate::tests::support::*;
use crate::transactions::data::ConflictClass;
use serde_json::Value;

#[test]
fn schema_boundary_fingerprint_is_explicit_256_bit_authority_surface() {
    let bytes = [7_u8; 32];
    let fingerprint = SchemaBoundaryFingerprint::new(bytes);

    assert_eq!(fingerprint, SchemaBoundaryFingerprint(bytes));
    assert_ne!(fingerprint, SchemaBoundaryFingerprint::ZERO);
}

#[test]
fn schema_diff_atom_requires_structured_detail_and_strata() {
    let element = SchemaElementRef::new(
        SchemaElementKind::Field,
        SchemaId("chip".to_string()),
        SchemaVersionId(4),
        None,
        Arc::<str>::from("timing_domain"),
    );
    let atom = SchemaDiffAtom::new(
        element.clone(),
        vec![
            SchemaStratum::StructuralShape,
            SchemaStratum::SubscriberContract,
        ],
        SchemaPublicationImpact::ProjectionContractChanged,
        SchemaSubscriberImpact::ContractUpgradeRequired,
        HistoricalInterpretationSensitivity::SensitiveToPublicationMeaning,
        SchemaDiffDetail::TypeChanged {
            field_name: Arc::<str>::from("timing_domain"),
            from_type: Arc::<str>::from("enum<legacy>"),
            to_type: Arc::<str>::from("enum<expanded>"),
        },
    );

    assert_eq!(atom.element, element);
    assert_eq!(
        atom.strata,
        vec![
            SchemaStratum::StructuralShape,
            SchemaStratum::SubscriberContract
        ]
    );
    assert!(matches!(atom.detail, SchemaDiffDetail::TypeChanged { .. }));
}

#[test]
fn compatibility_observation_remains_non_authoritative_summary_only() {
    let proposed = ProposedSchemaTransition {
        source_schema_id: SchemaId("cad".to_string()),
        source_schema_version_id: SchemaVersionId(1),
        target_schema_id: SchemaId("cad".to_string()),
        target_schema_version_id: SchemaVersionId(2),
        diff_atoms: Vec::new(),
    };
    let validated = ValidatedSchemaTransition {
        proposed,
        compatibility_observation: CompatibilityObservation::NonRejectedInAtLeastOneLayer,
        reconciliation: SchemaReconciliationClassification::Additive,
        continuation: SchemaContinuationClassification::ContinueWithVisibleBridge,
        bridgeability: SchemaBridgeabilityClassification::SubscriberVisible,
    };

    assert_eq!(
        validated.compatibility_observation,
        CompatibilityObservation::NonRejectedInAtLeastOneLayer
    );
    assert_eq!(
        validated.continuation,
        SchemaContinuationClassification::ContinueWithVisibleBridge
    );
}

#[test]
fn schema_descriptor_constructors_preserve_semantics_version_and_ordering_truth() {
    let fingerprint = SchemaBoundaryFingerprint::new([3_u8; 32]);
    let bridge = SchemaBridgeDescriptor::new(
        fingerprint,
        DescriptorSemanticsVersion(9),
        DescriptorCanonicalizationVersion(2),
        SchemaContinuationClassification::ContinueWithTransparentBridge,
        SchemaBridgeabilityClassification::Transparent,
        HistoricalInterpretationSensitivity::NotSensitive,
        vec![SchemaStratum::PublicationContract],
    );
    let continuation = SchemaContinuationDescriptor::new(fingerprint, bridge.clone(), 4);
    let lineage = SchemaLineageArtifact::new(
        SchemaId("forge".to_string()),
        SchemaVersionId(7),
        vec![
            SchemaId("forge".to_string()),
            SchemaId("forge-feature".to_string()),
        ],
        vec![SchemaVersionId(6), SchemaVersionId(6)],
        Some(BranchId("main".to_string())),
        SchemaReconciliationOrderingMode::CanonicalizedPair,
        SchemaLineageOrderingSemantics::SymmetricResult,
    );
    let reconciliation = SchemaReconciliationDescriptor::new(
        DescriptorSemanticsVersion(9),
        DescriptorCanonicalizationVersion(2),
        SchemaReconciliationClassification::Additive,
        SchemaReconciliationPolicy::PreserveInformation,
        lineage.clone(),
    );
    let plan = LoweredSchemaTransitionPlan::new(
        ValidatedSchemaTransition {
            proposed: ProposedSchemaTransition {
                source_schema_id: SchemaId("forge".to_string()),
                source_schema_version_id: SchemaVersionId(6),
                target_schema_id: SchemaId("forge".to_string()),
                target_schema_version_id: SchemaVersionId(7),
                diff_atoms: Vec::new(),
            },
            compatibility_observation: CompatibilityObservation::NonRejectedInAtLeastOneLayer,
            reconciliation: SchemaReconciliationClassification::Additive,
            continuation: SchemaContinuationClassification::ContinueWithTransparentBridge,
            bridgeability: SchemaBridgeabilityClassification::Transparent,
        },
        continuation.clone(),
        reconciliation.clone(),
    );

    assert_eq!(bridge.semantics_version, DescriptorSemanticsVersion(9));
    assert_eq!(continuation.normalized_boundary_count, 4);
    assert_eq!(reconciliation.resulting_lineage, lineage);
    assert_eq!(
        plan.continuation_descriptor.bridge.bridgeability,
        SchemaBridgeabilityClassification::Transparent
    );
}

#[test]
fn schema_transition_summary_derives_changed_strata_without_duplicate_noise() {
    let fingerprint = SchemaBoundaryFingerprint::new([11_u8; 32]);
    let artifact = SchemaTransitionArtifact::new(
        SchemaId("geom".to_string()),
        SchemaVersionId(2),
        SchemaId("geom".to_string()),
        SchemaVersionId(3),
        vec![
            SchemaDiffAtom::new(
                SchemaElementRef::new(
                    SchemaElementKind::Field,
                    SchemaId("geom".to_string()),
                    SchemaVersionId(3),
                    None,
                    Arc::<str>::from("length_units"),
                ),
                vec![
                    SchemaStratum::ValueDomain,
                    SchemaStratum::PublicationContract,
                ],
                SchemaPublicationImpact::ObservableSurfaceChanged,
                SchemaSubscriberImpact::RenegotiationRequired,
                HistoricalInterpretationSensitivity::SensitiveToValueMeaning,
                SchemaDiffDetail::FreeText {
                    detail: Arc::<str>::from("unit semantics widened"),
                    declared_intent: FreeFormSchemaDiffIntent::StructuralIncompatible,
                },
            ),
            SchemaDiffAtom::new(
                SchemaElementRef::new(
                    SchemaElementKind::ProjectionContract,
                    SchemaId("geom".to_string()),
                    SchemaVersionId(3),
                    None,
                    Arc::<str>::from("mass-properties"),
                ),
                vec![SchemaStratum::PublicationContract],
                SchemaPublicationImpact::ProjectionContractChanged,
                SchemaSubscriberImpact::ConsumableSurfaceChanged,
                HistoricalInterpretationSensitivity::SensitiveToPublicationMeaning,
                SchemaDiffDetail::ProjectionContractChanged {
                    projection_name: Arc::<str>::from("mass-properties"),
                },
            ),
        ],
        SchemaContinuationDescriptor::new(
            fingerprint,
            SchemaBridgeDescriptor::new(
                fingerprint,
                DescriptorSemanticsVersion::default(),
                DescriptorCanonicalizationVersion::default(),
                SchemaContinuationClassification::RequireRenegotiation,
                SchemaBridgeabilityClassification::RenegotiationOnly,
                HistoricalInterpretationSensitivity::SensitiveToPublicationMeaning,
                vec![
                    SchemaStratum::ValueDomain,
                    SchemaStratum::PublicationContract,
                ],
            ),
            1,
        ),
        SchemaReconciliationDescriptor::new(
            DescriptorSemanticsVersion::default(),
            DescriptorCanonicalizationVersion::default(),
            SchemaReconciliationClassification::Narrowing,
            SchemaReconciliationPolicy::RejectLossyNarrowing,
            SchemaLineageArtifact::new(
                SchemaId("geom".to_string()),
                SchemaVersionId(3),
                vec![SchemaId("geom".to_string())],
                vec![SchemaVersionId(2)],
                None,
                SchemaReconciliationOrderingMode::CanonicalizedPair,
                SchemaLineageOrderingSemantics::SymmetricResult,
            ),
        ),
    );

    let summary = SchemaTransitionSummary::from_artifact(&artifact);

    assert_eq!(summary.changed_atom_count, 2);
    assert_eq!(
        summary.changed_strata,
        vec![
            SchemaStratum::ValueDomain,
            SchemaStratum::PublicationContract
        ]
    );
    assert_eq!(
        summary.continuation,
        SchemaContinuationClassification::RequireRenegotiation
    );
}

#[test]
fn schema_transition_validation_rejects_unstratified_change_sets() {
    let proposed = ProposedSchemaTransition {
        source_schema_id: SchemaId("geom".to_string()),
        source_schema_version_id: SchemaVersionId(1),
        target_schema_id: SchemaId("geom".to_string()),
        target_schema_version_id: SchemaVersionId(2),
        diff_atoms: vec![SchemaDiffAtom::new(
            SchemaElementRef::new(
                SchemaElementKind::Field,
                SchemaId("geom".to_string()),
                SchemaVersionId(2),
                None,
                Arc::<str>::from("length"),
            ),
            Vec::new(),
            SchemaPublicationImpact::None,
            SchemaSubscriberImpact::None,
            HistoricalInterpretationSensitivity::NotSensitive,
            SchemaDiffDetail::AddedField {
                field_name: Arc::<str>::from("length"),
                required: false,
                default_expression: None,
            },
        )],
    };

    let error = validate_schema_transition(proposed, None).unwrap_err();
    assert!(error
        .detail()
        .contains("does not declare any schema strata"));
}

#[test]
fn schema_transition_validation_requires_explicit_policy_for_narrowing() {
    let proposed = ProposedSchemaTransition {
        source_schema_id: SchemaId("chip".to_string()),
        source_schema_version_id: SchemaVersionId(8),
        target_schema_id: SchemaId("chip".to_string()),
        target_schema_version_id: SchemaVersionId(9),
        diff_atoms: vec![SchemaDiffAtom::new(
            SchemaElementRef::new(
                SchemaElementKind::Field,
                SchemaId("chip".to_string()),
                SchemaVersionId(9),
                None,
                Arc::<str>::from("timing_margin"),
            ),
            vec![SchemaStratum::StructuralShape],
            SchemaPublicationImpact::ObservableSurfaceChanged,
            SchemaSubscriberImpact::RenegotiationRequired,
            HistoricalInterpretationSensitivity::SensitiveToPublicationMeaning,
            SchemaDiffDetail::RemovedField {
                field_name: Arc::<str>::from("timing_margin"),
            },
        )],
    };

    let error = validate_schema_transition(proposed, None).unwrap_err();
    assert!(error
        .detail()
        .contains("requires an explicit preservation policy"));
}

#[test]
fn schema_transition_classification_and_lowering_are_deterministic_for_visible_bridge_cases() {
    let proposed = ProposedSchemaTransition {
        source_schema_id: SchemaId("web".to_string()),
        source_schema_version_id: SchemaVersionId(3),
        target_schema_id: SchemaId("web".to_string()),
        target_schema_version_id: SchemaVersionId(4),
        diff_atoms: vec![
            SchemaDiffAtom::new(
                SchemaElementRef::new(
                    SchemaElementKind::Field,
                    SchemaId("web".to_string()),
                    SchemaVersionId(4),
                    None,
                    Arc::<str>::from("optional_tag"),
                ),
                vec![
                    SchemaStratum::StructuralShape,
                    SchemaStratum::PublicationContract,
                ],
                SchemaPublicationImpact::ObservableSurfaceChanged,
                SchemaSubscriberImpact::ConsumableSurfaceChanged,
                HistoricalInterpretationSensitivity::NotSensitive,
                SchemaDiffDetail::AddedField {
                    field_name: Arc::<str>::from("optional_tag"),
                    required: false,
                    default_expression: Some(Arc::<str>::from("null")),
                },
            )
            .with_boundary_visibility_proof(
                crate::schema::data::SubscriberBoundaryVisibility::VisibleSemanticallyIgnorable,
            ),
            SchemaDiffAtom::new(
                SchemaElementRef::new(
                    SchemaElementKind::ProjectionContract,
                    SchemaId("web".to_string()),
                    SchemaVersionId(4),
                    None,
                    Arc::<str>::from("orders.list"),
                ),
                vec![
                    SchemaStratum::PublicationContract,
                    SchemaStratum::SubscriberContract,
                ],
                SchemaPublicationImpact::ProjectionContractChanged,
                SchemaSubscriberImpact::ConsumableSurfaceChanged,
                HistoricalInterpretationSensitivity::NotSensitive,
                SchemaDiffDetail::ProjectionContractChanged {
                    projection_name: Arc::<str>::from("orders.list"),
                },
            )
            .with_boundary_visibility_proof(
                crate::schema::data::SubscriberBoundaryVisibility::VisibleSemanticallyIgnorable,
            ),
        ],
    };

    let classified = classify_schema_transition(
        proposed.clone(),
        Some(SchemaReconciliationPolicy::PreserveInformation),
    );
    let validated = validate_schema_transition(
        proposed.clone(),
        Some(SchemaReconciliationPolicy::PreserveInformation),
    )
    .unwrap();
    let lowered = lower_schema_transition(
        validated.clone(),
        Some(SchemaReconciliationPolicy::PreserveInformation),
        DescriptorSemanticsVersion::default(),
        DescriptorCanonicalizationVersion::default(),
    );
    let lowered_again = lower_schema_transition(
        validate_schema_transition(
            proposed,
            Some(SchemaReconciliationPolicy::PreserveInformation),
        )
        .unwrap(),
        Some(SchemaReconciliationPolicy::PreserveInformation),
        DescriptorSemanticsVersion::default(),
        DescriptorCanonicalizationVersion::default(),
    );

    assert_eq!(
        classified.continuation,
        SchemaContinuationClassification::ContinueWithVisibleBridge
    );
    assert_eq!(
        validated.continuation,
        SchemaContinuationClassification::ContinueWithVisibleBridge
    );
    assert_eq!(
        validated.bridgeability,
        SchemaBridgeabilityClassification::SubscriberVisible
    );
    assert_eq!(
        lowered.continuation_descriptor.boundary_fingerprint,
        lowered_again.continuation_descriptor.boundary_fingerprint
    );
    assert_eq!(
        lowered.reconciliation_descriptor.policy,
        SchemaReconciliationPolicy::PreserveInformation
    );
}

#[test]
fn consumable_surface_change_requires_explicit_visible_bridge_proof() {
    let proposed = ProposedSchemaTransition {
        source_schema_id: SchemaId("web".to_string()),
        source_schema_version_id: SchemaVersionId(3),
        target_schema_id: SchemaId("web".to_string()),
        target_schema_version_id: SchemaVersionId(4),
        diff_atoms: vec![SchemaDiffAtom::new(
            SchemaElementRef::new(
                SchemaElementKind::Field,
                SchemaId("web".to_string()),
                SchemaVersionId(4),
                None,
                Arc::<str>::from("optional_tag"),
            ),
            vec![
                SchemaStratum::StructuralShape,
                SchemaStratum::PublicationContract,
            ],
            SchemaPublicationImpact::ObservableSurfaceChanged,
            SchemaSubscriberImpact::ConsumableSurfaceChanged,
            HistoricalInterpretationSensitivity::NotSensitive,
            SchemaDiffDetail::AddedField {
                field_name: Arc::<str>::from("optional_tag"),
                required: false,
                default_expression: Some(Arc::<str>::from("null")),
            },
        )
        .with_boundary_visibility_proof(
            crate::schema::data::SubscriberBoundaryVisibility::VisibleRequiresContractUptake,
        )],
    };

    let classified = classify_schema_transition(
        proposed,
        Some(SchemaReconciliationPolicy::PreserveInformation),
    );

    assert_eq!(
        classified.continuation,
        SchemaContinuationClassification::RequireRenegotiation
    );
}

#[test]
fn descriptor_semantics_policy_supports_explicit_historical_versions() {
    let policy = crate::schema::data::DescriptorSemanticsCompatibilityPolicy::new(
        DescriptorSemanticsVersion(3),
        [DescriptorSemanticsVersion(1), DescriptorSemanticsVersion(2)],
    );

    assert_eq!(
        policy.current_write_version(),
        DescriptorSemanticsVersion(3)
    );
    assert!(policy.supports(DescriptorSemanticsVersion(1)));
    assert!(policy.supports(DescriptorSemanticsVersion(2)));
    assert!(policy.supports(DescriptorSemanticsVersion(3)));
    assert!(!policy.supports(DescriptorSemanticsVersion(4)));
}

#[test]
fn schema_boundary_fingerprint_is_canonical_across_diff_atom_orderings() {
    let atom_a = SchemaDiffAtom::new(
        SchemaElementRef::new(
            SchemaElementKind::Field,
            SchemaId("web".to_string()),
            SchemaVersionId(4),
            None,
            Arc::<str>::from("optional_tag"),
        ),
        vec![
            SchemaStratum::PublicationContract,
            SchemaStratum::StructuralShape,
        ],
        SchemaPublicationImpact::ObservableSurfaceChanged,
        SchemaSubscriberImpact::ConsumableSurfaceChanged,
        HistoricalInterpretationSensitivity::NotSensitive,
        SchemaDiffDetail::AddedField {
            field_name: Arc::<str>::from("optional_tag"),
            required: false,
            default_expression: Some(Arc::<str>::from("null")),
        },
    )
    .with_boundary_visibility_proof(
        crate::schema::data::SubscriberBoundaryVisibility::VisibleSemanticallyIgnorable,
    );
    let atom_b = SchemaDiffAtom::new(
        SchemaElementRef::new(
            SchemaElementKind::ProjectionContract,
            SchemaId("web".to_string()),
            SchemaVersionId(4),
            None,
            Arc::<str>::from("orders.list"),
        ),
        vec![
            SchemaStratum::SubscriberContract,
            SchemaStratum::PublicationContract,
        ],
        SchemaPublicationImpact::ProjectionContractChanged,
        SchemaSubscriberImpact::ConsumableSurfaceChanged,
        HistoricalInterpretationSensitivity::NotSensitive,
        SchemaDiffDetail::ProjectionContractChanged {
            projection_name: Arc::<str>::from("orders.list"),
        },
    )
    .with_boundary_visibility_proof(
        crate::schema::data::SubscriberBoundaryVisibility::VisibleSemanticallyIgnorable,
    );

    let lowered_a = lower_schema_transition(
        validate_schema_transition(
            ProposedSchemaTransition {
                source_schema_id: SchemaId("web".to_string()),
                source_schema_version_id: SchemaVersionId(3),
                target_schema_id: SchemaId("web".to_string()),
                target_schema_version_id: SchemaVersionId(4),
                diff_atoms: vec![atom_a.clone(), atom_b.clone()],
            },
            Some(SchemaReconciliationPolicy::PreserveInformation),
        )
        .unwrap(),
        Some(SchemaReconciliationPolicy::PreserveInformation),
        DescriptorSemanticsVersion::default(),
        DescriptorCanonicalizationVersion::default(),
    );
    let lowered_b = lower_schema_transition(
        validate_schema_transition(
            ProposedSchemaTransition {
                source_schema_id: SchemaId("web".to_string()),
                source_schema_version_id: SchemaVersionId(3),
                target_schema_id: SchemaId("web".to_string()),
                target_schema_version_id: SchemaVersionId(4),
                diff_atoms: vec![atom_b, atom_a],
            },
            Some(SchemaReconciliationPolicy::PreserveInformation),
        )
        .unwrap(),
        Some(SchemaReconciliationPolicy::PreserveInformation),
        DescriptorSemanticsVersion::default(),
        DescriptorCanonicalizationVersion::default(),
    );

    assert_eq!(
        lowered_a.continuation_descriptor.boundary_fingerprint,
        lowered_b.continuation_descriptor.boundary_fingerprint
    );
}

#[test]
fn type_incompatible_schema_transition_is_rejected_not_continued() {
    let proposed = ProposedSchemaTransition {
        source_schema_id: SchemaId("chip".to_string()),
        source_schema_version_id: SchemaVersionId(1),
        target_schema_id: SchemaId("chip".to_string()),
        target_schema_version_id: SchemaVersionId(2),
        diff_atoms: vec![SchemaDiffAtom::new(
            SchemaElementRef::new(
                SchemaElementKind::Field,
                SchemaId("chip".to_string()),
                SchemaVersionId(2),
                None,
                Arc::<str>::from("timing_domain"),
            ),
            vec![
                SchemaStratum::ValueDomain,
                SchemaStratum::PublicationContract,
            ],
            SchemaPublicationImpact::ObservableSurfaceChanged,
            SchemaSubscriberImpact::ConsumableSurfaceChanged,
            HistoricalInterpretationSensitivity::SensitiveToValueMeaning,
            SchemaDiffDetail::TypeChanged {
                field_name: Arc::<str>::from("timing_domain"),
                from_type: Arc::<str>::from("enum<legacy>"),
                to_type: Arc::<str>::from("enum<expanded>"),
            },
        )],
    };

    let validated = validate_schema_transition(
        proposed,
        Some(SchemaReconciliationPolicy::PreserveInformation),
    )
    .unwrap();

    assert_eq!(
        validated.reconciliation,
        SchemaReconciliationClassification::TypeIncompatible
    );
    assert_eq!(
        validated.continuation,
        SchemaContinuationClassification::Rejected
    );
    assert_eq!(
        validated.bridgeability,
        SchemaBridgeabilityClassification::Rejected
    );
    assert_eq!(
        validated.compatibility_observation,
        CompatibilityObservation::RejectedInAllLayers
    );
}

#[test]
fn schema_registry_authoritative_basis_rejects_mixed_schema_identity() {
    let registry = RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "entity".to_string(),
            schema_id: SchemaId("test-a".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: KindAspectDeclarations::default(),
        })
        .and_then(|registry| {
            registry.register_relation_kind(RelationKindRegistration {
                kind_id: KindId(2),
                kind_name: "relation".to_string(),
                schema_id: SchemaId("test-b".to_string()),
                schema_version_id: SchemaVersionId(1),
                payload_class: RelationPayloadClass::PayloadBearingRelation,
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                aspect_declarations: KindAspectDeclarations::default(),
                relation_integrity: RelationIntegrityDeclarations::default(),
            })
        })
        .unwrap();

    let error = registry.authoritative_schema_basis().unwrap_err();
    assert!(error.detail.contains("mixed schema basis"));
}

#[test]
fn commit_rejects_undeclared_schema_drift_against_branch_head() {
    let mut runtime = runtime_with_test_schema();
    let _first = create_entity_outcome(&mut runtime, "a");

    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(2),
        ..AspectSchemaFixture::default()
    }
    .build_registry();

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("schema-drift").push(MutationIntent::Create(CreateIntent::Entity(
            crate::transactions::data::EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: InternedString::Raw("b".to_string()),
                payload: RecordPayload::StructuredJson(serde_json::json!({ "name": "b" })),
            },
        ))),
    );
    let error = txn.commit().unwrap_err();

    match error {
        crate::transactions::data::TransactionCommitError::Conflict { error, .. } => {
            assert!(matches!(
                error.class,
                ConflictClass::UndeclaredSchemaTransition {
                    previous_schema_version: SchemaVersionId(1),
                    current_schema_version: SchemaVersionId(2),
                    ..
                }
            ));
            assert_eq!(
                error.code(),
                crate::diagnostics::data::DiagnosticCode::SchemaContinuityViolation
            );
        }
        other => panic!("expected schema continuity conflict, got {other:?}"),
    }
    let diagnostics = runtime.publication_access().diagnostics();
    let failure_artifact = diagnostics
        .by_scope(DiagnosticsScope::Schema)
        .into_iter()
        .find(|artifact| artifact.kind == DiagnosticsArtifactKind::Failure)
        .expect("schema continuity failure artifact");
    assert!(failure_artifact.entries.iter().any(|entry| {
        entry.code == DiagnosticCode::SchemaContinuityViolation
            && entry.fields["conflict_class"]
                .as_str()
                .is_some_and(|class| class.contains("UndeclaredSchemaTransition"))
    }));
}

#[test]
fn successful_commit_surfaces_descriptor_semantics_version_in_result_and_envelope() {
    let mut runtime = runtime_with_test_schema();

    let outcome = create_entity_outcome(&mut runtime, "a");

    assert_eq!(
        outcome.descriptor_semantics_version(),
        DescriptorSemanticsVersion::default()
    );
    assert_eq!(
        outcome.schema_summary().descriptor_semantics_version,
        DescriptorSemanticsVersion::default()
    );
    assert_eq!(
        outcome.envelope().descriptor_semantics_version,
        DescriptorSemanticsVersion::default()
    );
}

#[test]
fn explicit_schema_transition_is_lowered_into_canonical_commit_artifacts() {
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
                Arc::<str>::from("tag"),
            ),
            vec![
                SchemaStratum::StructuralShape,
                SchemaStratum::PublicationContract,
            ],
            SchemaPublicationImpact::ObservableSurfaceChanged,
            SchemaSubscriberImpact::ConsumableSurfaceChanged,
            HistoricalInterpretationSensitivity::NotSensitive,
            SchemaDiffDetail::AddedField {
                field_name: Arc::<str>::from("tag"),
                required: false,
                default_expression: Some(Arc::<str>::from("null")),
            },
        )
        .with_boundary_visibility_proof(
            crate::schema::data::SubscriberBoundaryVisibility::VisibleSemanticallyIgnorable,
        )],
    };

    let mut txn = runtime.begin_transaction(TransactionOptions::default().with_schema_transition(
        proposed_transition,
        Some(SchemaReconciliationPolicy::PreserveInformation),
    ));
    txn.push_batch(
        WorkerIntentBatch::new("schema-transition").push(MutationIntent::Create(
            CreateIntent::Entity(crate::transactions::data::EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: InternedString::Raw("b".to_string()),
                payload: RecordPayload::StructuredJson(serde_json::json!({ "name": "b" })),
            }),
        )),
    );
    let outcome = txn.commit().unwrap();

    let transition = outcome.schema_transition_summary().unwrap();
    assert_eq!(transition.changed_atom_count, 1);
    assert_eq!(
        transition.continuation,
        SchemaContinuationClassification::ContinueWithVisibleBridge
    );
    assert!(outcome.envelope().schema_transition.is_some());
    assert!(outcome.envelope().schema_continuation_descriptor.is_some());
    assert!(outcome
        .envelope()
        .schema_reconciliation_descriptor
        .is_some());
    assert!(outcome.diagnostics().iter().any(|artifact| artifact.scope
        == DiagnosticsScope::Schema
        && artifact
            .entries
            .iter()
            .any(|entry| { entry.code == DiagnosticCode::SchemaTransitionTraced })));
    let detailed_trace = outcome
        .diagnostics()
        .iter()
        .find(|artifact| {
            artifact.scope == DiagnosticsScope::Schema
                && artifact.kind == DiagnosticsArtifactKind::DetailedTrace
        })
        .expect("schema detailed trace artifact");
    assert!(detailed_trace
        .entries
        .iter()
        .any(|entry| { entry.code == DiagnosticCode::SchemaLineageTraced }));
    assert!(detailed_trace
        .entries
        .iter()
        .any(|entry| { entry.code == DiagnosticCode::SchemaBridgeDescriptorConstructed }));
    assert!(detailed_trace
        .entries
        .iter()
        .any(|entry| { entry.code == DiagnosticCode::SchemaReconciliationResolved }));
    assert!(detailed_trace
        .entries
        .iter()
        .any(|entry| { entry.code == DiagnosticCode::SchemaInterpretationSensitivityClassified }));
    assert!(detailed_trace
        .entries
        .iter()
        .any(|entry| { entry.code == DiagnosticCode::SchemaDescriptorVersionSelected }));
    let diff_entry = detailed_trace
        .entries
        .iter()
        .find(|entry| entry.message.contains("schema diff atom 0"))
        .expect("per-diff schema trace entry");
    assert_eq!(
        diff_entry.fields["strata"],
        Value::Array(vec![
            Value::String("StructuralShape".to_string()),
            Value::String("PublicationContract".to_string()),
        ])
    );
    assert_eq!(
        diff_entry.fields["detail"]["kind"],
        Value::String("AddedField".to_string())
    );
}

#[test]
fn schema_certification_transition_is_explained_and_counted() {
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
                Arc::<str>::from("tag"),
            ),
            vec![
                SchemaStratum::StructuralShape,
                SchemaStratum::PublicationContract,
            ],
            SchemaPublicationImpact::ObservableSurfaceChanged,
            SchemaSubscriberImpact::ConsumableSurfaceChanged,
            HistoricalInterpretationSensitivity::NotSensitive,
            SchemaDiffDetail::AddedField {
                field_name: Arc::<str>::from("tag"),
                required: false,
                default_expression: Some(Arc::<str>::from("null")),
            },
        )
        .with_boundary_visibility_proof(
            crate::schema::data::SubscriberBoundaryVisibility::VisibleSemanticallyIgnorable,
        )],
    };

    runtime.performance_access().reset_counters();
    let mut txn = runtime.begin_transaction(TransactionOptions::default().with_schema_transition(
        proposed_transition,
        Some(SchemaReconciliationPolicy::PreserveInformation),
    ));
    txn.push_batch(WorkerIntentBatch::new("schema-transition-certified").push(
        MutationIntent::Create(CreateIntent::Entity(
            crate::transactions::data::EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: InternedString::Raw("b".to_string()),
                payload: RecordPayload::StructuredJson(serde_json::json!({ "name": "b" })),
            },
        )),
    ));
    let outcome = txn.commit().unwrap();

    let diagnostics = outcome.diagnostics();
    let detailed_trace = diagnostics
        .iter()
        .find(|artifact| {
            artifact.scope == DiagnosticsScope::Schema
                && artifact.kind == DiagnosticsArtifactKind::DetailedTrace
        })
        .expect("schema detailed trace artifact");
    assert!(detailed_trace
        .entries
        .iter()
        .any(|entry| entry.code == DiagnosticCode::SchemaTransitionClassified));
    assert!(detailed_trace
        .entries
        .iter()
        .any(|entry| entry.code == DiagnosticCode::SchemaBridgeDescriptorConstructed));
    assert!(detailed_trace
        .entries
        .iter()
        .any(|entry| entry.code == DiagnosticCode::SchemaReconciliationResolved));
    assert!(detailed_trace
        .entries
        .iter()
        .any(|entry| { entry.code == DiagnosticCode::SchemaInterpretationSensitivityClassified }));
    assert!(detailed_trace
        .entries
        .iter()
        .any(|entry| entry.code == DiagnosticCode::SchemaDescriptorVersionSelected));

    let counters = runtime.performance_access().counters();
    assert_eq!(counters.schema_transition_atoms_inspected, 1);
    assert_eq!(counters.schema_changed_subtrees_inspected, 1);
    assert_eq!(counters.schema_bridge_descriptors_built, 1);
    assert_eq!(counters.schema_transition_continue_visible_bridge_count, 1);
    assert_eq!(counters.schema_reconciliation_preserve_information_count, 1);
    assert_eq!(
        counters.schema_historical_interpretation_sensitive_boundaries,
        0
    );
}

#[test]
fn declared_schema_transition_rejects_wrong_source_basis() {
    let mut runtime = runtime_with_test_schema();
    let _first = create_entity_outcome(&mut runtime, "a");

    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id: SchemaVersionId(2),
        ..AspectSchemaFixture::default()
    }
    .build_registry();

    let proposed_transition = ProposedSchemaTransition {
        source_schema_id: SchemaId("wrong".to_string()),
        source_schema_version_id: SchemaVersionId(1),
        target_schema_id: SchemaId("test".to_string()),
        target_schema_version_id: SchemaVersionId(2),
        diff_atoms: vec![SchemaDiffAtom::new(
            SchemaElementRef::new(
                SchemaElementKind::Field,
                SchemaId("test".to_string()),
                SchemaVersionId(2),
                Some(KindId(1)),
                Arc::<str>::from("tag"),
            ),
            vec![SchemaStratum::StructuralShape],
            SchemaPublicationImpact::None,
            SchemaSubscriberImpact::None,
            HistoricalInterpretationSensitivity::NotSensitive,
            SchemaDiffDetail::AddedField {
                field_name: Arc::<str>::from("tag"),
                required: false,
                default_expression: None,
            },
        )],
    };

    let mut txn = runtime.begin_transaction(TransactionOptions::default().with_schema_transition(
        proposed_transition,
        Some(SchemaReconciliationPolicy::PreserveInformation),
    ));
    txn.push_batch(batch_create("b"));
    let error = txn.commit().unwrap_err();

    match error {
        crate::transactions::data::TransactionCommitError::Conflict { error, .. } => {
            assert!(matches!(
                error.class,
                ConflictClass::InvalidSchemaTransitionSourceBasis {
                    declared_schema_version: SchemaVersionId(1),
                    expected_schema_version: SchemaVersionId(1),
                    ..
                }
            ));
        }
        other => panic!("expected source-basis conflict, got {other:?}"),
    }
}

#[test]
fn declared_schema_transition_rejects_wrong_target_basis() {
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
        target_schema_version_id: SchemaVersionId(99),
        diff_atoms: vec![SchemaDiffAtom::new(
            SchemaElementRef::new(
                SchemaElementKind::Field,
                SchemaId("test".to_string()),
                SchemaVersionId(2),
                Some(KindId(1)),
                Arc::<str>::from("tag"),
            ),
            vec![SchemaStratum::StructuralShape],
            SchemaPublicationImpact::None,
            SchemaSubscriberImpact::None,
            HistoricalInterpretationSensitivity::NotSensitive,
            SchemaDiffDetail::AddedField {
                field_name: Arc::<str>::from("tag"),
                required: false,
                default_expression: None,
            },
        )],
    };

    let mut txn = runtime.begin_transaction(TransactionOptions::default().with_schema_transition(
        proposed_transition,
        Some(SchemaReconciliationPolicy::PreserveInformation),
    ));
    txn.push_batch(batch_create("b"));
    let error = txn.commit().unwrap_err();

    match error {
        crate::transactions::data::TransactionCommitError::Conflict { error, .. } => {
            assert!(matches!(
                error.class,
                ConflictClass::InvalidSchemaTransitionTargetBasis {
                    declared_schema_version: SchemaVersionId(99),
                    expected_schema_version: SchemaVersionId(2),
                    ..
                }
            ));
        }
        other => panic!("expected target-basis conflict, got {other:?}"),
    }
    let diagnostics = runtime.publication_access().diagnostics();
    let failure_artifact = diagnostics
        .by_scope(DiagnosticsScope::Schema)
        .into_iter()
        .find(|artifact| artifact.kind == DiagnosticsArtifactKind::Failure)
        .expect("schema continuity failure artifact");
    assert!(failure_artifact.entries.iter().any(|entry| {
        entry.message.contains("rejected schema diff atom 0")
            && entry.fields["detail"]["kind"] == Value::String("AddedField".to_string())
            && entry.fields["strata"]
                == Value::Array(vec![Value::String("StructuralShape".to_string())])
    }));
}

#[test]
fn declared_schema_transition_requires_non_empty_runtime_basis() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(RelationalSchemaRegistry::new())
        .build();

    let proposed_transition = ProposedSchemaTransition {
        source_schema_id: SchemaId("empty".to_string()),
        source_schema_version_id: SchemaVersionId(0),
        target_schema_id: SchemaId("empty".to_string()),
        target_schema_version_id: SchemaVersionId(0),
        diff_atoms: vec![SchemaDiffAtom::new(
            SchemaElementRef::new(
                SchemaElementKind::Schema,
                SchemaId("empty".to_string()),
                SchemaVersionId(0),
                None,
                Arc::<str>::from("root"),
            ),
            vec![SchemaStratum::StructuralShape],
            SchemaPublicationImpact::None,
            SchemaSubscriberImpact::None,
            HistoricalInterpretationSensitivity::NotSensitive,
            SchemaDiffDetail::FreeText {
                detail: Arc::<str>::from("bootstrap"),
                declared_intent: FreeFormSchemaDiffIntent::Additive,
            },
        )],
    };

    let txn = runtime.begin_transaction(TransactionOptions::default().with_schema_transition(
        proposed_transition,
        Some(SchemaReconciliationPolicy::PreserveInformation),
    ));
    let error = txn.commit().unwrap_err();

    match error {
        crate::transactions::data::TransactionCommitError::Conflict { error, .. } => {
            assert!(matches!(
                error.class,
                ConflictClass::MissingSchemaBasisForTransition { ref role }
                    if role == "runtime"
            ));
        }
        other => panic!("expected missing-runtime-basis conflict, got {other:?}"),
    }
}

#[test]
fn declared_type_incompatible_schema_transition_reports_specific_conflict_class() {
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
                Arc::<str>::from("tag"),
            ),
            vec![
                SchemaStratum::ValueDomain,
                SchemaStratum::PublicationContract,
            ],
            SchemaPublicationImpact::ObservableSurfaceChanged,
            SchemaSubscriberImpact::ConsumableSurfaceChanged,
            HistoricalInterpretationSensitivity::SensitiveToValueMeaning,
            SchemaDiffDetail::TypeChanged {
                field_name: Arc::<str>::from("tag"),
                from_type: Arc::<str>::from("string"),
                to_type: Arc::<str>::from("enum<tag>"),
            },
        )],
    };

    let mut txn = runtime.begin_transaction(TransactionOptions::default().with_schema_transition(
        proposed_transition,
        Some(SchemaReconciliationPolicy::PreserveInformation),
    ));
    txn.push_batch(batch_create("b"));
    let error = txn.commit().unwrap_err();

    match error {
        crate::transactions::data::TransactionCommitError::Conflict { error, .. } => {
            assert!(matches!(
                error.class,
                ConflictClass::TypeIncompatibleSchemaTransition { .. }
            ));
        }
        other => panic!("expected type-incompatible conflict, got {other:?}"),
    }
}

#[test]
fn schema_continuity_publication_rejects_incomplete_canonical_bundle() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "a");
    let mut envelope = outcome.envelope().clone();
    let fingerprint = SchemaBoundaryFingerprint::new([13_u8; 32]);
    let transition = SchemaTransitionArtifact::new(
        SchemaId("test".to_string()),
        SchemaVersionId(1),
        SchemaId("test".to_string()),
        SchemaVersionId(1),
        vec![SchemaDiffAtom::new(
            SchemaElementRef::new(
                SchemaElementKind::Field,
                SchemaId("test".to_string()),
                SchemaVersionId(1),
                Some(KindId(1)),
                Arc::<str>::from("tag"),
            ),
            vec![SchemaStratum::StructuralShape],
            SchemaPublicationImpact::None,
            SchemaSubscriberImpact::None,
            HistoricalInterpretationSensitivity::NotSensitive,
            SchemaDiffDetail::AddedField {
                field_name: Arc::<str>::from("tag"),
                required: false,
                default_expression: None,
            },
        )],
        SchemaContinuationDescriptor::new(
            fingerprint,
            SchemaBridgeDescriptor::new(
                fingerprint,
                DescriptorSemanticsVersion::default(),
                DescriptorCanonicalizationVersion::default(),
                SchemaContinuationClassification::ContinueWithTransparentBridge,
                SchemaBridgeabilityClassification::Transparent,
                HistoricalInterpretationSensitivity::NotSensitive,
                vec![SchemaStratum::StructuralShape],
            ),
            1,
        ),
        SchemaReconciliationDescriptor::new(
            DescriptorSemanticsVersion::default(),
            DescriptorCanonicalizationVersion::default(),
            SchemaReconciliationClassification::Additive,
            SchemaReconciliationPolicy::PreserveInformation,
            SchemaLineageArtifact::new(
                SchemaId("test".to_string()),
                SchemaVersionId(1),
                vec![SchemaId("test".to_string())],
                vec![SchemaVersionId(1)],
                Some(BranchId("main".to_string())),
                SchemaReconciliationOrderingMode::CanonicalizedPair,
                SchemaLineageOrderingSemantics::SymmetricResult,
            ),
        ),
    );
    envelope.schema_transition = Some(transition.clone());
    envelope.schema_continuation_descriptor = Some(transition.continuation_descriptor.clone());
    envelope.schema_reconciliation_descriptor = None;
    let plan = SchemaContinuityPlan {
        descriptor_semantics_version: DescriptorSemanticsVersion::default(),
        schema_transition: Some(transition),
        schema_continuation_descriptor: envelope.schema_continuation_descriptor.clone(),
        schema_reconciliation_descriptor: None,
    };

    let error = validate_schema_continuity_publication(
        &mut runtime,
        &BranchId("main".to_string()),
        &plan,
        &envelope,
    )
    .unwrap_err();

    match error {
        crate::transactions::data::TransactionCommitError::Conflict { error, .. } => {
            assert!(matches!(
                error.class,
                ConflictClass::UnsupportedBridgeDescriptor { .. }
            ));
            assert!(error.detail().contains("must appear together"));
        }
        other => panic!("expected continuity publication conflict, got {other:?}"),
    }
}

#[test]
fn schema_continuity_publication_rejects_descriptor_semantics_mismatch() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "a");
    let fingerprint = SchemaBoundaryFingerprint::new([17_u8; 32]);
    let transition = SchemaTransitionArtifact::new(
        SchemaId("test".to_string()),
        SchemaVersionId(1),
        SchemaId("test".to_string()),
        SchemaVersionId(1),
        vec![SchemaDiffAtom::new(
            SchemaElementRef::new(
                SchemaElementKind::Field,
                SchemaId("test".to_string()),
                SchemaVersionId(1),
                Some(KindId(1)),
                Arc::<str>::from("tag"),
            ),
            vec![SchemaStratum::PublicationContract],
            SchemaPublicationImpact::ObservableSurfaceChanged,
            SchemaSubscriberImpact::ConsumableSurfaceChanged,
            HistoricalInterpretationSensitivity::NotSensitive,
            SchemaDiffDetail::AddedField {
                field_name: Arc::<str>::from("tag"),
                required: false,
                default_expression: Some(Arc::<str>::from("null")),
            },
        )],
        SchemaContinuationDescriptor::new(
            fingerprint,
            SchemaBridgeDescriptor::new(
                fingerprint,
                DescriptorSemanticsVersion(99),
                DescriptorCanonicalizationVersion::default(),
                SchemaContinuationClassification::ContinueWithVisibleBridge,
                SchemaBridgeabilityClassification::SubscriberVisible,
                HistoricalInterpretationSensitivity::NotSensitive,
                vec![SchemaStratum::PublicationContract],
            ),
            1,
        ),
        SchemaReconciliationDescriptor::new(
            DescriptorSemanticsVersion(99),
            DescriptorCanonicalizationVersion::default(),
            SchemaReconciliationClassification::Additive,
            SchemaReconciliationPolicy::PreserveInformation,
            SchemaLineageArtifact::new(
                SchemaId("test".to_string()),
                SchemaVersionId(1),
                vec![SchemaId("test".to_string())],
                vec![SchemaVersionId(1)],
                Some(BranchId("main".to_string())),
                SchemaReconciliationOrderingMode::CanonicalizedPair,
                SchemaLineageOrderingSemantics::SymmetricResult,
            ),
        ),
    );
    let mut envelope = outcome.envelope().clone();
    envelope.schema_transition = Some(transition.clone());
    envelope.schema_continuation_descriptor = Some(transition.continuation_descriptor.clone());
    envelope.schema_reconciliation_descriptor = Some(transition.reconciliation_descriptor.clone());
    envelope.descriptor_semantics_version = DescriptorSemanticsVersion::default();
    let plan = SchemaContinuityPlan {
        descriptor_semantics_version: DescriptorSemanticsVersion::default(),
        schema_transition: Some(transition.clone()),
        schema_continuation_descriptor: Some(transition.continuation_descriptor.clone()),
        schema_reconciliation_descriptor: Some(transition.reconciliation_descriptor.clone()),
    };

    let error = validate_schema_continuity_publication(
        &mut runtime,
        &BranchId("main".to_string()),
        &plan,
        &envelope,
    )
    .unwrap_err();

    match error {
        crate::transactions::data::TransactionCommitError::Conflict { error, .. } => {
            assert!(matches!(
                error.class,
                ConflictClass::UnsupportedBridgeDescriptor { .. }
            ));
            assert!(error.detail().contains("descriptor semantics version"));
        }
        other => panic!("expected continuity publication conflict, got {other:?}"),
    }
}

#[test]
fn shared_continuity_bundle_validator_reports_boundary_fingerprint_mismatch() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "a");
    let fingerprint = SchemaBoundaryFingerprint::new([23_u8; 32]);
    let bridge_fingerprint = SchemaBoundaryFingerprint::new([24_u8; 32]);
    let transition = SchemaTransitionArtifact::new(
        SchemaId("test".to_string()),
        SchemaVersionId(1),
        SchemaId("test".to_string()),
        SchemaVersionId(1),
        vec![SchemaDiffAtom::new(
            SchemaElementRef::new(
                SchemaElementKind::Field,
                SchemaId("test".to_string()),
                SchemaVersionId(1),
                Some(KindId(1)),
                Arc::<str>::from("tag"),
            ),
            vec![SchemaStratum::PublicationContract],
            SchemaPublicationImpact::ObservableSurfaceChanged,
            SchemaSubscriberImpact::ConsumableSurfaceChanged,
            HistoricalInterpretationSensitivity::NotSensitive,
            SchemaDiffDetail::AddedField {
                field_name: Arc::<str>::from("tag"),
                required: false,
                default_expression: Some(Arc::<str>::from("null")),
            },
        )],
        SchemaContinuationDescriptor::new(
            fingerprint,
            SchemaBridgeDescriptor::new(
                bridge_fingerprint,
                DescriptorSemanticsVersion::default(),
                DescriptorCanonicalizationVersion::default(),
                SchemaContinuationClassification::ContinueWithVisibleBridge,
                SchemaBridgeabilityClassification::SubscriberVisible,
                HistoricalInterpretationSensitivity::NotSensitive,
                vec![SchemaStratum::PublicationContract],
            ),
            1,
        ),
        SchemaReconciliationDescriptor::new(
            DescriptorSemanticsVersion::default(),
            DescriptorCanonicalizationVersion::default(),
            SchemaReconciliationClassification::Additive,
            SchemaReconciliationPolicy::PreserveInformation,
            SchemaLineageArtifact::new(
                SchemaId("test".to_string()),
                SchemaVersionId(1),
                vec![SchemaId("test".to_string())],
                vec![SchemaVersionId(1)],
                Some(BranchId("main".to_string())),
                SchemaReconciliationOrderingMode::CanonicalizedPair,
                SchemaLineageOrderingSemantics::SymmetricResult,
            ),
        ),
    );
    let mut envelope = outcome.envelope().clone();
    envelope.schema_transition = Some(transition.clone());
    envelope.schema_continuation_descriptor = Some(transition.continuation_descriptor.clone());
    envelope.schema_reconciliation_descriptor = Some(transition.reconciliation_descriptor.clone());

    let issue = validate_schema_continuity_bundle(&envelope).unwrap_err();

    assert!(matches!(
        issue,
        SchemaContinuityBundleIssue::ContinuationBoundaryFingerprintMismatch {
            boundary_fingerprint
        } if boundary_fingerprint == fingerprint
    ));
}
