use crate::facade::durability::{
    DurabilityMode, DurableStore, DurableStoreLayout, RecoveryAuthorityParity,
    RecoveryCompatibilityCheck, RecoveryCompatibilityMismatch, RecoveryCursor,
    RecoveryFailureClass, RecoveryIntegrityReport, RecoveryPlan, RecoveryVerificationMode,
    RecoveryVerificationOutcome,
    RelationIntegrityContractFamily,
};
use crate::facade::diagnostics::{DiagnosticCode, DiagnosticsScope};
use crate::facade::history::BranchId;
use crate::facade::identity::KindId;
use crate::facade::indexes::{
    DerivedIndexBuildRequest, DerivedIndexDefinition, DerivedIndexId, DerivedIndexKind,
};
use crate::facade::lineage::{
    CorrespondencePromotionRejectionClass, LineageDecisionKind, LineageEventKind,
};
use crate::facade::replay::ReplayVerificationLayer;
use crate::facade::schema::{
    DescriptorCanonicalizationVersion, DescriptorSemanticsVersion,
};
use crate::facade::runtime::RelationalRuntimeApi;
use crate::facade::schema::{
    EntityKindRegistration, HistoricalInterpretationSensitivity, KindAspectDeclarations,
    ProposedSchemaTransition, RelationalSchemaRegistry, SchemaDiffAtom, SchemaDiffDetail,
    SchemaElementKind, SchemaElementRef, SchemaId, SchemaPublicationImpact,
    SchemaReconciliationPolicy, SchemaStratum, SchemaSubscriberImpact, SchemaVersionId,
};
use crate::facade::transactions::{TransactionCommitError, TransactionOptions};
use crate::tests::support::*;
use serde_json::json;

// CONTRACT: durability
// LANES: success, failure, recovery

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
fn durability_contract_recovery_rebuilds_branch_heads_and_latest_commit() {
    let mut runtime = persisted_runtime_with_test_schema();
    let main = create_entity_outcome(&mut runtime, "main-a");
    create_branch_from_main(&mut runtime, "feature");
    let feature =
        create_entity_outcome_on_branch(&mut runtime, "feature-a", BranchId("feature".to_string()));
    let (outcome, recovered) =
        checkpoint_and_recover_with(&mut runtime, persisted_runtime_with_test_schema);

    assert_eq!(outcome.recovered_commits, 2);
    assert_eq!(outcome.latest_commit, Some(feature.commit.clone()));
    assert_eq!(
        recovered
            .history_access()
            .branch_head(&BranchId("feature".to_string())),
        Some(&feature.commit)
    );
    assert_eq!(
        recovered
            .history_access()
            .branch_head(&BranchId("main".to_string())),
        Some(&main.commit)
    );
}

#[test]
fn durability_contract_recovery_preserves_aspect_bearing_patch_truth_and_history() {
    let mut runtime =
        persisted_runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let created = create_entity_outcome(&mut runtime, "before");
    let entity = changed_entities(&created)[0];
    let updated = update_entity(&mut runtime, entity, "after");
    let expected_history =
        runtime
            .history_access()
            .entity_aspect_history(&BranchId("main".to_string()), entity, None);
    let expected_digest = runtime
        .history_access()
        .entity_aspect_history_with_trace(&BranchId("main".to_string()), entity, None)
        .aspect_history_digest();
    let expected_envelope = runtime
        .replay_access()
        .canonical_commit_envelope(updated.commit.commit_id)
        .cloned()
        .unwrap();
    let (outcome, recovered) = checkpoint_and_recover_with(&mut runtime, || {
        persisted_runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations)
    });

    let recovered_history = recovered.history_access().entity_aspect_history(
        &BranchId("main".to_string()),
        entity,
        None,
    );
    let recovered_digest = recovered
        .history_access()
        .entity_aspect_history_with_trace(&BranchId("main".to_string()), entity, None)
        .aspect_history_digest();
    let recovered_replay = recovered.replay_access();
    let recovered_envelope = recovered_replay
        .canonical_commit_envelope(updated.commit.commit_id)
        .unwrap();

    assert_eq!(outcome.latest_commit, Some(updated.commit.clone()));
    assert_eq!(expected_history, recovered_history);
    assert_eq!(expected_digest, recovered_digest);
    assert_eq!(
        expected_envelope.patch.records,
        recovered_envelope.patch.records
    );
    assert_eq!(
        recovered_envelope.patch.records[0].aspects,
        CanonicalAspectSet::new([aspect_key("lifecycle"), aspect_key("name")])
    );
    assert!(!recovered_envelope.patch.records[0].contains_degraded_precision);
}

#[test]
fn durability_contract_recovery_preserves_relation_aspect_history_for_retained_audit_relations() {
    let mut runtime =
        persisted_runtime_with_declared_aspect_schema(CascadeDeletePolicy::RetainDanglingForAudit);
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let relation_outcome = create_relation_outcome(&mut runtime, source, target, "r-audit");
    let relation = changed_relations(&relation_outcome)[0];
    let deleted = delete_entity(&mut runtime, source);
    let expected_history = runtime.history_access().relation_aspect_history(
        &BranchId("main".to_string()),
        relation,
        None,
    );
    let expected_digest = runtime
        .history_access()
        .relation_aspect_history_with_trace(&BranchId("main".to_string()), relation, None)
        .aspect_history_digest();
    let (outcome, recovered) = checkpoint_and_recover_with(&mut runtime, || {
        persisted_runtime_with_declared_aspect_schema(CascadeDeletePolicy::RetainDanglingForAudit)
    });

    let recovered_history = recovered.history_access().relation_aspect_history(
        &BranchId("main".to_string()),
        relation,
        None,
    );
    let recovered_digest = recovered
        .history_access()
        .relation_aspect_history_with_trace(&BranchId("main".to_string()), relation, None)
        .aspect_history_digest();

    assert_eq!(outcome.latest_commit, Some(deleted.commit.clone()));
    assert_eq!(expected_history, recovered_history);
    assert_eq!(expected_digest, recovered_digest);
    assert_eq!(recovered_history.len(), 2);
    assert_direct_history_origin_invariants(&recovered_history, RecordRef::Relation(relation));
    assert_eq!(
        recovered_history[0].origin.changed_aspects,
        CanonicalAspectSet::new([
            aspect_key("label"),
            aspect_key("lifecycle"),
            aspect_key("source"),
            aspect_key("target"),
        ])
    );
    assert_eq!(
        recovered_history[1].origin.changed_aspects,
        CanonicalAspectSet::new([aspect_key("lifecycle")])
    );
}

#[test]
fn durability_contract_recovery_preserves_branch_local_endpoint_deletion_retirement_histories() {
    let registry = RelationalSchemaRegistry::new()
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
                cascade_delete_policy: CascadeDeletePolicy::RetainDanglingForAudit,
                aspect_declarations: KindAspectDeclarations::default(),
                relation_integrity: crate::schema::data::RelationIntegrityDeclarations::new(
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    vec![crate::schema::data::EndpointDeletionIntegrityDeclaration {
                        contract_id: "require_retirement".to_string(),
                        mode: crate::schema::data::EndpointDeletionIntegrityMode::RequireRelationRetirement,
                    }],
                ),
            })
        })
        .unwrap();
    let store_layout = DurableStoreLayout {
        root_path: unique_test_store_path("forge-relational-endpoint-retirement-recovery"),
        segment_commit_capacity: 2,
    };
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(registry.clone())
        .durability_mode(DurabilityMode::PersistedSegmentedLocalFs)
        .durable_store_layout(store_layout.clone())
        .cascade_delete_policy(CascadeDeletePolicy::RetainDanglingForAudit)
        .build();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let relation_outcome = create_relation_outcome(&mut runtime, source, target, "retained");
    let relation = changed_relations(&relation_outcome)[0];
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let _main_delete = delete_entity(&mut runtime, source);
    let _feature_update = update_entity_on_branch(
        &mut runtime,
        target,
        "feature-target",
        BranchId("feature".to_string()),
    );

    let expected_main_digest = relation_aspect_history_digest_on_branch(
        &runtime,
        &BranchId("main".to_string()),
        relation,
        None,
    );
    let expected_feature_digest = relation_aspect_history_digest_on_branch(
        &runtime,
        &BranchId("feature".to_string()),
        relation,
        None,
    );
    let expected_main_inspection = runtime.inspection_access().inspect_historical_record(
        &BranchId("main".to_string()),
        runtime
            .history_access()
            .branch_head(&BranchId("main".to_string()))
            .unwrap()
            .version_id,
        RecordRef::Relation(relation),
        crate::facade::inspection::HistoricalInspectionMode::RetainedOnly,
    );
    let expected_feature_inspection = runtime.inspection_access().inspect_historical_record(
        &BranchId("feature".to_string()),
        runtime
            .history_access()
            .branch_head(&BranchId("feature".to_string()))
            .unwrap()
            .version_id,
        RecordRef::Relation(relation),
        crate::facade::inspection::HistoricalInspectionMode::RetainedOnly,
    );

    runtime.durability_authority().checkpoint().unwrap();
    let plan = runtime.durability_access().recovery_plan(crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification);
    let mut recovered = RelationalRuntimeApi::builder()
        .schema_registry(registry)
        .durability_mode(DurabilityMode::PersistedSegmentedLocalFs)
        .durable_store_layout(store_layout)
        .cascade_delete_policy(CascadeDeletePolicy::RetainDanglingForAudit)
        .build();
    let outcome = recovered.durability_authority().recover(plan).unwrap();

    let recovered_main_digest = relation_aspect_history_digest_on_branch(
        &recovered,
        &BranchId("main".to_string()),
        relation,
        None,
    );
    let recovered_feature_digest = relation_aspect_history_digest_on_branch(
        &recovered,
        &BranchId("feature".to_string()),
        relation,
        None,
    );
    let recovered_main_inspection = recovered.inspection_access().inspect_historical_record(
        &BranchId("main".to_string()),
        recovered
            .history_access()
            .branch_head(&BranchId("main".to_string()))
            .unwrap()
            .version_id,
        RecordRef::Relation(relation),
        crate::facade::inspection::HistoricalInspectionMode::RetainedOnly,
    );
    let recovered_feature_inspection = recovered.inspection_access().inspect_historical_record(
        &BranchId("feature".to_string()),
        recovered
            .history_access()
            .branch_head(&BranchId("feature".to_string()))
            .unwrap()
            .version_id,
        RecordRef::Relation(relation),
        crate::facade::inspection::HistoricalInspectionMode::RetainedOnly,
    );

    assert_eq!(
        outcome.latest_commit,
        runtime.history_access().latest_commit().cloned()
    );
    assert_eq!(expected_main_digest, recovered_main_digest);
    assert_eq!(expected_feature_digest, recovered_feature_digest);
    assert_eq!(expected_main_inspection, recovered_main_inspection);
    assert_eq!(expected_feature_inspection, recovered_feature_inspection);
}

#[test]
fn durability_contract_failure_schema_mismatch_is_explicit() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "main-a");
    let plan = runtime.durability_access().recovery_plan(crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification);
    let mismatched_registry = RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(3),
            kind_name: "other.entity".to_string(),
            schema_id: SchemaId("other".to_string()),
            schema_version_id: SchemaVersionId(2),
            aspect_declarations: KindAspectDeclarations::default(),
        })
        .unwrap();
    let mut recovered = RelationalRuntimeApi::builder()
        .schema_registry(mismatched_registry)
        .build();
    let error = recovered.durability_authority().recover(plan).unwrap_err();

    assert_eq!(error.class, RecoveryFailureClass::SchemaMismatch);
    assert!(matches!(
        error.compatibility_mismatch,
        Some(RecoveryCompatibilityMismatch::SchemaRegistryShape {
            expected_primary_schema_version: SchemaVersionId(1),
            found_primary_schema_version: SchemaVersionId(2),
            ..
        })
    ));
}

#[test]
fn durability_contract_failure_descriptor_semantics_version_mismatch_is_explicit() {
    let mut runtime = runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "main-a");
    let mut plan = runtime.durability_access().recovery_plan(crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification);
    plan.descriptor_semantics_version = DescriptorSemanticsVersion(99);

    let mut recovered = runtime_with_test_schema();
    let error = recovered.durability_authority().recover(plan).unwrap_err();

    assert_eq!(error.class, RecoveryFailureClass::SchemaMismatch);
    assert!(matches!(
        error.compatibility_mismatch,
        Some(RecoveryCompatibilityMismatch::DescriptorSemanticsVersion {
            expected: DescriptorSemanticsVersion(99),
            found: DescriptorSemanticsVersion(1),
        })
    ));
}

#[test]
fn durability_recovery_plan_preserves_explicit_verification_mode() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "main-a");

    let normal = runtime
        .durability_access()
        .recovery_plan(RecoveryVerificationMode::NormalRecoveryVerification);
    let audit = runtime
        .durability_access()
        .recovery_plan(RecoveryVerificationMode::AuditRecoveryVerification);

    assert_eq!(
        normal.verification_mode(),
        RecoveryVerificationMode::NormalRecoveryVerification
    );
    assert_eq!(
        audit.verification_mode(),
        RecoveryVerificationMode::AuditRecoveryVerification
    );
}

#[test]
fn durability_recovery_plan_reports_descriptor_version_mismatch_before_recovery() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "main-a");
    runtime.performance_access().reset_counters();
    let store = runtime.durability_access().recovery_plan(crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification).store.unwrap();
    let segment_path = store
        .segments
        .last()
        .expect("persisted segment after commit")
        .path
        .clone();
    let mut file: crate::durability::log::local_store::DurableSegmentFile =
        crate::durability::log::local_store::read_json(&segment_path).unwrap();
    file.entries[0].descriptor_semantics_version = DescriptorSemanticsVersion(99);
    crate::durability::log::local_store::write_json(&segment_path, &file).unwrap();

    let plan = runtime.durability_access().recovery_plan(crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification);

    assert_eq!(plan.descriptor_semantics_version, DescriptorSemanticsVersion(99));
    assert_eq!(
        plan.compatibility.descriptor_version_parity,
        RecoveryAuthorityParity::Drift
    );
    assert!(matches!(
        plan.compatibility.first_mismatch,
        Some(RecoveryCompatibilityMismatch::DescriptorSemanticsVersion {
            expected: DescriptorSemanticsVersion(1),
            found: DescriptorSemanticsVersion(99),
        })
    ));
    assert_eq!(
        plan.compatibility.verification_outcome,
        RecoveryVerificationOutcome::Rejected {
            layer: ReplayVerificationLayer::DigestParity,
            detail: "descriptor semantics version mismatch".to_string(),
        }
    );
    let counters = runtime.performance_access().counters();
    assert!(counters.descriptor_version_mismatches_encountered >= 1);
    assert!(counters.replay_digest_parity_checks >= 1);
}

#[test]
fn durability_contract_failure_descriptor_canonicalization_version_mismatch_is_explicit() {
    let mut runtime = persisted_runtime_with_test_schema();
    let _ = create_entity_outcome(&mut runtime, "main-a");

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
    txn.push_batch(batch_create("transitioned"));
    txn.commit().unwrap();

    let segment_path = runtime
        .durability_access()
        .recovery_plan(crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification)
        .store
        .unwrap()
        .segments
        .last()
        .expect("persisted segment after transition")
        .path
        .clone();
    let mut file: crate::durability::log::local_store::DurableSegmentFile =
        crate::durability::log::local_store::read_json(&segment_path).unwrap();
    if let Some(descriptor) = file.entries[1].schema_continuation_descriptor.as_mut() {
        descriptor.bridge.canonicalization_version = DescriptorCanonicalizationVersion(99);
    }
    crate::durability::log::local_store::write_json(&segment_path, &file).unwrap();

    let plan = runtime
        .durability_access()
        .recovery_plan(crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification);

    assert_eq!(
        plan.compatibility.descriptor_version_parity,
        RecoveryAuthorityParity::Drift
    );
    assert!(matches!(
        plan.compatibility.first_mismatch,
        Some(RecoveryCompatibilityMismatch::DescriptorCanonicalizationVersion {
            expected: DescriptorCanonicalizationVersion(1),
            found: DescriptorCanonicalizationVersion(99),
        })
    ));
    assert_eq!(
        plan.compatibility.verification_outcome,
        RecoveryVerificationOutcome::Rejected {
            layer: ReplayVerificationLayer::DigestParity,
            detail: "descriptor canonicalization version mismatch".to_string(),
        }
    );
}

#[test]
fn durability_recovery_emits_compatibility_diagnostic_before_execution() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "main-a");
    let plan = runtime.durability_access().recovery_plan(crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification);

    let mut recovered = persisted_runtime_with_test_schema();
    let _ = recovered.durability_authority().recover(plan).unwrap();

    let diagnostics = recovered.publication_access().diagnostics();
    let compatibility_entry = diagnostics
        .by_scope(DiagnosticsScope::History)
        .into_iter()
        .flat_map(|artifact| artifact.entries.iter())
        .find(|entry| entry.code == DiagnosticCode::DurableRecoveryCompatibilityEvaluated)
        .expect("recovery compatibility diagnostic");
    assert_eq!(compatibility_entry.fields["verification_rejected"], json!(false));
    assert_eq!(
        compatibility_entry.fields["verification_layer"],
        json!("DigestParity")
    );
}

#[test]
fn durability_certification_recovery_compatibility_is_explained_and_counted() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "main-a");
    runtime.performance_access().reset_counters();
    let plan = runtime.durability_access().recovery_plan(crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification);

    let mut recovered = persisted_runtime_with_test_schema();
    let _ = recovered.durability_authority().recover(plan).unwrap();

    let diagnostics = recovered.publication_access().diagnostics();
    let compatibility_entry = diagnostics
        .by_scope(DiagnosticsScope::History)
        .into_iter()
        .flat_map(|artifact| artifact.entries.iter())
        .find(|entry| entry.code == DiagnosticCode::DurableRecoveryCompatibilityEvaluated)
        .expect("recovery certification diagnostic");
    assert_eq!(
        compatibility_entry.fields["verification_mode"],
        json!("NormalRecoveryVerification")
    );
    assert_eq!(compatibility_entry.fields["verification_rejected"], json!(false));
    assert_eq!(
        compatibility_entry.fields["verification_layer"],
        json!("DigestParity")
    );
    assert_eq!(
        compatibility_entry.fields["descriptor_version_parity"],
        json!("VerifiedAtLayer(DigestParity)")
    );
    assert_eq!(
        compatibility_entry.fields["schema_transition_parity"],
        json!("VerifiedAtLayer(DigestParity)")
    );
    let counters = runtime.performance_access().counters();
    assert!(counters.replay_digest_parity_checks >= 1);
    assert_eq!(counters.descriptor_version_mismatches_encountered, 0);
}

#[test]
fn durable_recovery_and_schema_mismatch_test() {
    let mut runtime = persisted_runtime_with_test_schema();
    let _baseline = create_entity_outcome(&mut runtime, "main-a");

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
    txn.push_batch(batch_create("main-b"));
    let transitioned = txn.commit().unwrap();

    let plan = runtime.durability_access().recovery_plan(crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification);
    let recovery_schema_bundle_digest = certification_digest(&(
        transitioned.envelope().schema_transition.clone(),
        transitioned.envelope().schema_continuation_descriptor.clone(),
        transitioned.envelope().schema_reconciliation_descriptor.clone(),
        plan.compatibility.clone(),
    ));

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
            root_path: unique_test_store_path("forge-relational-durable-recovery-schema-match"),
            segment_commit_capacity: 2,
        })
        .build();
    let _outcome = recovered
        .durability_authority()
        .recover(plan.clone())
        .unwrap();
    let recovered_envelope = recovered
        .replay_access()
        .canonical_commit_envelope(transitioned.commit.commit_id)
        .cloned()
        .expect("recovered transitioned envelope");
    let recovered_diagnostics = recovered.publication_access().diagnostics();
    let recovery_compatibility_diagnostic_digest = certification_digest(
        &recovered_diagnostics
            .by_scope(DiagnosticsScope::History)
            .into_iter()
            .flat_map(|artifact| artifact.entries.iter())
            .find(|entry| entry.code == DiagnosticCode::DurableRecoveryCompatibilityEvaluated)
            .expect("recovery compatibility diagnostic"),
    );

    assert_eq!(
        recovery_schema_bundle_digest,
        certification_digest(&(
            recovered_envelope.schema_transition.clone(),
            recovered_envelope.schema_continuation_descriptor.clone(),
            recovered_envelope.schema_reconciliation_descriptor.clone(),
            plan.compatibility.clone(),
        ))
    );
    assert!(recovered_diagnostics
        .by_scope(DiagnosticsScope::History)
        .into_iter()
        .flat_map(|artifact| artifact.entries.iter())
        .any(|entry| {
            entry.code == DiagnosticCode::DurableRecoveryCompatibilityEvaluated
                && entry.fields["verification_layer"] == json!("DigestParity")
        }));
    let recovered_counters = recovered.performance_access().counters();
    assert!(recovered_counters.replay_digest_parity_checks >= 1);

    let mismatched_registry = RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(3),
            kind_name: "other.entity".to_string(),
            schema_id: SchemaId("other".to_string()),
            schema_version_id: SchemaVersionId(99),
            aspect_declarations: KindAspectDeclarations::default(),
        })
        .unwrap();
    let mut mismatched = RelationalRuntimeApi::builder()
        .schema_registry(mismatched_registry)
        .build();
    let error = mismatched.durability_authority().recover(plan).unwrap_err();
    let mismatch_failure_digest = certification_digest(&(
        &error.class,
        &error.compatibility_mismatch,
        &error.detail,
    ));

    assert_eq!(error.class, RecoveryFailureClass::SchemaMismatch);
    assert!(matches!(
        error.compatibility_mismatch,
        Some(RecoveryCompatibilityMismatch::SchemaRegistryShape { .. })
    ));
    assert!(matches!(
        error.compatibility_mismatch,
        Some(RecoveryCompatibilityMismatch::SchemaRegistryShape {
            expected_primary_schema_version: SchemaVersionId(2),
            ..
        })
    ));
    assert!(!recovery_compatibility_diagnostic_digest.is_empty());
    assert!(!mismatch_failure_digest.is_empty());
}

#[test]
fn durability_contract_failure_aspect_plan_mismatch_is_explicit() {
    let mut runtime =
        persisted_runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    create_entity_outcome(&mut runtime, "main-a");
    let plan = runtime.durability_access().recovery_plan(crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification);
    let expected_registry =
        declared_aspect_schema_registry(CascadeDeletePolicy::CascadeDeleteRelations);
    let mismatched_registry = AspectSchemaFixture {
        entity_aspects: vec![
            entity_payload_aspect("display_name", "name"),
            lifecycle_aspect(),
        ],
        relation_aspects: vec![
            relation_payload_aspect("label", "label"),
            lifecycle_aspect(),
            relation_source_aspect(),
            relation_target_aspect(),
        ],
        ..AspectSchemaFixture::with_default_declared_aspects(
            CascadeDeletePolicy::CascadeDeleteRelations,
        )
    }
    .build_registry();
    let expected_revision = expected_registry
        .entity_aspect_declaration_trace(KindId(1))
        .unwrap()
        .plan_revision;
    let mismatched_revision = mismatched_registry
        .entity_aspect_declaration_trace(KindId(1))
        .unwrap()
        .plan_revision;
    let mut recovered = RelationalRuntimeApi::builder()
        .schema_registry(mismatched_registry)
        .build();
    let error = recovered.durability_authority().recover(plan).unwrap_err();

    assert_ne!(expected_revision, mismatched_revision);
    assert_eq!(error.class, RecoveryFailureClass::SchemaMismatch);
    assert!(matches!(
        error.compatibility_mismatch,
        Some(RecoveryCompatibilityMismatch::EntityAspectPlanRevision {
            kind_id: KindId(1),
            expected_revision: expected,
            found_revision: found,
            ..
        }) if expected == expected_revision.0 && found == mismatched_revision.0
    ));
}

#[test]
fn durability_contract_failure_relation_integrity_plan_mismatch_is_explicit() {
    let base_registry = RelationalSchemaRegistry::new()
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
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ),
            })
        })
        .unwrap();
    let store_layout = DurableStoreLayout {
        root_path: unique_test_store_path("forge-relational-relation-integrity-mismatch"),
        segment_commit_capacity: 2,
    };
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(base_registry)
        .durability_mode(DurabilityMode::PersistedSegmentedLocalFs)
        .durable_store_layout(store_layout.clone())
        .build();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    create_relation_outcome(&mut runtime, source, target, "guarded");
    let plan = runtime.durability_access().recovery_plan(crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification);

    let mismatched_registry = RelationalSchemaRegistry::new()
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
                        contract_id: "source_max_two".to_string(),
                        source_max: Some(2),
                        target_max: None,
                        pair_max: None,
                    }],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ),
            })
        })
        .unwrap();
    let mut recovered = RelationalRuntimeApi::builder()
        .schema_registry(mismatched_registry)
        .durability_mode(DurabilityMode::PersistedSegmentedLocalFs)
        .durable_store_layout(store_layout)
        .build();
    let error = recovered.durability_authority().recover(plan).unwrap_err();

    assert_eq!(error.class, RecoveryFailureClass::SchemaMismatch);
    assert!(matches!(
        error.compatibility_mismatch,
        Some(RecoveryCompatibilityMismatch::RelationIntegrityPlanRevision {
            kind_id: KindId(2),
            contract_family: RelationIntegrityContractFamily::Cardinality,
            ref expected_contract_ids,
            ref found_contract_ids,
            ..
        }) if expected_contract_ids == &vec!["source_max_one".to_string()]
            && found_contract_ids == &vec!["source_max_two".to_string()]
    ));
}

#[test]
fn durability_contract_recovery_ignores_rejected_relation_integrity_attempts() {
    let fixture = RelationIntegritySchemaFixture {
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
    };
    let store_layout = DurableStoreLayout {
        root_path: unique_test_store_path("forge-relational-rejected-relation-integrity-recovery"),
        segment_commit_capacity: 2,
    };
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(fixture.build_registry())
        .durability_mode(DurabilityMode::PersistedSegmentedLocalFs)
        .durable_store_layout(store_layout.clone())
        .build();
    let source = create_entity(&mut runtime, "source");
    let target_a = create_entity(&mut runtime, "target-a");
    let target_b = create_entity(&mut runtime, "target-b");
    let accepted = create_relation_outcome(&mut runtime, source, target_a, "accepted");
    let relation = changed_relations(&accepted)[0];
    let latest_commit_before = runtime.history_access().latest_commit().cloned();
    let latest_patch_before = runtime.publication_access().latest_patch().unwrap().position;
    let main_digest_before = relation_aspect_history_digest_on_branch(
        &runtime,
        &BranchId("main".to_string()),
        relation,
        None,
    );

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("illegal-overflow").push(MutationIntent::Create(
            CreateIntent::Relation(crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw("illegal-overflow".to_string()),
                source,
                target: target_b,
                payload: Some(RecordPayload::StructuredJson(json!({"label":"illegal-overflow"}))),
            }),
        )),
    );
    let error = txn.commit().unwrap_err();
    match error {
        TransactionCommitError::Conflict { error, .. } => {
            assert_eq!(error.code(), DiagnosticCode::RelationCardinalityViolation);
        }
        TransactionCommitError::Publication { .. } => {
            panic!("expected relation-integrity conflict, got publication error")
        }
    }

    assert_eq!(runtime.history_access().latest_commit().cloned(), latest_commit_before);
    assert_eq!(
        runtime.publication_access().latest_patch().unwrap().position,
        latest_patch_before
    );
    assert_eq!(
        relation_aspect_history_digest_on_branch(
            &runtime,
            &BranchId("main".to_string()),
            relation,
            None,
        ),
        main_digest_before
    );

    runtime.durability_authority().checkpoint().unwrap();
    let plan = runtime.durability_access().recovery_plan(crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification);
    let mut recovered = RelationalRuntimeApi::builder()
        .schema_registry(fixture.build_registry())
        .durability_mode(DurabilityMode::PersistedSegmentedLocalFs)
        .durable_store_layout(store_layout)
        .build();
    let outcome = recovered.durability_authority().recover(plan).unwrap();

    assert_eq!(outcome.latest_commit, latest_commit_before.clone());
    assert_eq!(
        relation_aspect_history_digest_on_branch(
            &recovered,
            &BranchId("main".to_string()),
            relation,
            None,
        ),
        main_digest_before
    );
    assert!(recovered
        .replay_access()
        .canonical_commit_envelope(latest_commit_before.unwrap().commit_id)
        .is_some());
}

#[test]
fn durability_contract_failure_missing_parent_chain_is_explicit() {
    let mut runtime = runtime_with_test_schema();
    let parent = create_entity_outcome(&mut runtime, "main-a");
    let child = create_entity_outcome(&mut runtime, "main-b");
    let child_envelope = runtime
        .replay_access()
        .canonical_commit_envelope(child.commit.commit_id)
        .cloned()
        .unwrap();
    let corrupt_plan = RecoveryPlan::new(
        runtime.config().clone(),
        runtime
            .config()
            .durability
            .policy
            .store_layout
            .clone()
            .map(|layout| DurableStore {
                layout,
                segments: Vec::new(),
                checkpoints: Vec::new(),
            }),
        None,
        None,
        vec![child_envelope],
        RecoveryCursor {
            checkpoint_id: None,
            segment_ids: Vec::new(),
        },
        RecoveryIntegrityReport {
            selected_checkpoint_id: None,
            skipped_corrupt_checkpoints: Vec::new(),
            verified_segment_ids: Vec::new(),
            corrupt_segment_id: None,
        },
        RecoveryCompatibilityCheck::verified_at(ReplayVerificationLayer::DigestParity),
        RecoveryVerificationMode::NormalRecoveryVerification,
        DescriptorSemanticsVersion::default(),
    );
    let mut recovered = runtime_with_test_schema();
    let error = recovered
        .durability_authority()
        .recover(corrupt_plan)
        .unwrap_err();

    assert_eq!(parent.commit.commit_id.0, 1);
    assert_eq!(error.class, RecoveryFailureClass::MissingParentChain);
}

#[test]
fn durability_contract_recovery_preserves_merge_parent_order() {
    let mut runtime = persisted_runtime_with_test_schema();
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
    let plan = runtime.durability_access().recovery_plan(crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification);
    let mut recovered = persisted_runtime_with_test_schema();
    recovered.durability_authority().recover(plan).unwrap();
    let replay = recovered.replay_access();
    let recovered_merge = replay
        .canonical_commit_envelope(merge.commit.commit_id)
        .unwrap();

    assert_eq!(
        recovered_merge.commit.parents,
        vec![main.commit.commit_id, feature.commit.commit_id]
    );
    assert_eq!(
        recovered_merge.merge_parent_branches,
        vec![BranchId("feature".to_string())]
    );
    assert_eq!(
        recovered_merge.merge_base_commits,
        vec![main.commit.commit_id]
    );
}

#[test]
fn durability_contract_checkpoint_tail_recovery_preserves_post_checkpoint_commits() {
    let mut runtime = persisted_runtime_with_test_schema();
    let main = create_entity_outcome(&mut runtime, "main-a");
    let _checkpoint = runtime.durability_authority().checkpoint().unwrap();
    let later = create_entity_outcome(&mut runtime, "main-b");
    let plan = runtime.durability_access().recovery_plan(crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification);
    let mut recovered = persisted_runtime_with_test_schema();
    let outcome = recovered.durability_authority().recover(plan).unwrap();

    assert_eq!(outcome.recovered_commits, 2);
    assert_eq!(outcome.latest_commit, Some(later.commit.clone()));
    assert_eq!(
        recovered
            .history_access()
            .branch_head(&BranchId("main".to_string())),
        Some(&later.commit)
    );
    assert_eq!(
        recovered
            .replay_access()
            .canonical_commit_envelope(main.commit.commit_id)
            .unwrap()
            .commit
            .commit_id,
        main.commit.commit_id
    );
}

#[test]
fn durability_contract_checkpoint_recovers_index_metadata() {
    let mut runtime = persisted_runtime_with_test_schema();
    let commit = create_entity_outcome(&mut runtime, "indexed");
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(0),
        name: "entity-name".to_string(),
        kind: DerivedIndexKind::EntityPayloadField {
            field: "name".to_string(),
        },
        branch_scoped: false,
    });
    let build = runtime
        .index_authority()
        .build_for_commit(DerivedIndexBuildRequest {
            source_commit_id: commit.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            index_ids: vec![index.index_id],
        });
    runtime.durability_authority().checkpoint().unwrap();
    let plan = runtime.durability_access().recovery_plan(crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification);
    let mut recovered = persisted_runtime_with_test_schema();
    recovered.durability_authority().recover(plan).unwrap();

    let index_access = recovered.index_access();
    let generation = index_access
        .latest_generation(index.index_id, &BranchId("main".to_string()))
        .unwrap();
    assert_eq!(generation.generation_id, build.generations[0].generation_id);
    assert_eq!(generation.source_commit_id, commit.commit.commit_id);
}

#[test]
fn durability_contract_checkpoint_recovers_lineage_metadata() {
    let mut runtime = persisted_runtime_with_test_schema();
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
        "recover-me",
    );
    runtime
        .lineage_authority()
        .promote_correspondence(candidate.candidate_id, second.commit.clone())
        .unwrap();
    let rejected_candidate = runtime.lineage_authority().record_correspondence_candidate(
        BranchId("main".to_string()),
        vec![LineageId(999)],
        vec![LineageId(1000)],
        "reject-me",
    );
    let rejected_resolution = runtime
        .lineage_authority()
        .promote_correspondence(rejected_candidate.candidate_id, second.commit.clone());
    assert_eq!(
        rejected_resolution,
        Err(CorrespondencePromotionRejectionClass::MissingLineageReference)
    );
    runtime.durability_authority().checkpoint().unwrap();
    let plan = runtime.durability_access().recovery_plan(crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification);
    let mut recovered = persisted_runtime_with_test_schema();
    recovered.durability_authority().recover(plan).unwrap();
    let graph = recovered
        .lineage_access()
        .graph(crate::facade::lineage::LineageGraphRequest {
            branch_id: BranchId("main".to_string()),
        });

    assert_eq!(graph.nodes.len(), 2);
    assert!(graph
        .events
        .iter()
        .any(|event| event.kind == LineageEventKind::Correspond));
      assert!(graph
          .correspondence_candidates
          .iter()
          .any(|entry| entry.candidate_id == candidate.candidate_id));
      assert!(recovered
          .lineage_access()
          .rejected_decisions_snapshot()
          .iter()
          .any(|decision| {
              decision.kind == LineageDecisionKind::CorrespondencePromotionRejected
                  && decision.candidate_id == Some(rejected_candidate.candidate_id)
                  && decision.rejection_class
                      == Some(CorrespondencePromotionRejectionClass::MissingLineageReference)
          }));
  }

#[test]
fn durability_contract_corrupt_latest_checkpoint_falls_back_to_prior_valid_checkpoint() {
    let mut runtime = persisted_runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "first");
    runtime.durability_authority().checkpoint().unwrap();
    let second = create_entity_outcome(&mut runtime, "second");
    runtime.durability_authority().checkpoint().unwrap();
    let store = runtime.durability_access().recovery_plan(crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification).store.unwrap();
    let latest_checkpoint = store.checkpoints.last().unwrap();
    std::fs::write(&latest_checkpoint.path, b"{not-json").unwrap();

    let plan = runtime.durability_access().recovery_plan(crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification);
    let mut recovered = persisted_runtime_with_test_schema();
    let outcome = recovered.durability_authority().recover(plan).unwrap();

    assert_eq!(outcome.latest_commit, Some(second.commit.clone()));
    assert!(!outcome
        .integrity_report
        .skipped_corrupt_checkpoints
        .is_empty());
    assert_eq!(
        recovered
            .history_access()
            .branch_head(&BranchId("main".to_string())),
        Some(&second.commit)
    );
    assert!(recovered
        .replay_access()
        .canonical_commit_envelope(first.commit.commit_id)
        .is_some());
}

#[test]
fn durability_contract_compaction_only_removes_segments_covered_by_checkpoint() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "a");
    create_entity_outcome(&mut runtime, "b");
    runtime.durability_authority().checkpoint().unwrap();
    create_entity_outcome(&mut runtime, "c");
    let before = runtime.durability_access().recovery_plan(crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification).store.unwrap();

    let compaction = runtime.durability_authority().compact_store().unwrap();
    let after = runtime.durability_access().recovery_plan(crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification).store.unwrap();

    assert!(!before.segments.is_empty());
    assert!(after.segments.len() <= before.segments.len());
    assert_eq!(after.segments.len(), compaction.retained_segments.len());
}

#[test]
fn durability_contract_recovery_rebuilds_branch_pinned_retention_from_branch_heads() {
    let mut runtime = persisted_runtime_with_test_schema();
    let source = create_entity_outcome(&mut runtime, "source");
    let source_entity = changed_entities(&source)[0];
    let target = create_entity_outcome(&mut runtime, "target");
    let target_entity = changed_entities(&target)[0];
    let _relation = create_relation_outcome(&mut runtime, source_entity, target_entity, "r1");
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let _deleted = delete_entity(&mut runtime, source_entity);
    runtime.durability_authority().checkpoint().unwrap();
    let plan = runtime.durability_access().recovery_plan(crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification);
    let mut recovered = persisted_runtime_with_test_schema();
    recovered.durability_authority().recover(plan).unwrap();

    let retention = recovered.retention_authority().inspect_plan();
    assert_eq!(retention.active_snapshot_count, 0);
    assert!(retention.branch_pinned_entities >= 1);
    assert!(retention.branch_pinned_relations >= 1);
    assert_eq!(retention.reclaimable_entities, 0);
    assert_eq!(retention.reclaimable_relations, 0);
}

#[test]
fn durability_contract_recovery_preserves_inspection_truth_bundle() {
    let mut runtime = persisted_runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "source");
    let entity = changed_entities(&created)[0];
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let _main_update = update_entity(&mut runtime, entity, "main");
    let _feature_update = {
        let mut txn = runtime.begin_transaction(TransactionOptions {
            target_branch: Some(BranchId("feature".to_string())),
            ..TransactionOptions::default()
        });
        txn.push_batch(WorkerIntentBatch::new("feature-update").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: entity,
                payload: RecordPayload::StructuredJson(json!({"name":"feature"})),
            }),
        )));
        txn.commit().unwrap()
    };
    runtime.durability_authority().checkpoint().unwrap();
    let expected = capture_inspection_truth_bundle(
        &runtime,
        &BranchId("feature".to_string()),
        entity,
        created.version_id,
    );

    let plan = runtime.durability_access().recovery_plan(crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification);
    let mut recovered = persisted_runtime_with_test_schema();
    recovered.durability_authority().recover(plan).unwrap();
    let actual = capture_inspection_truth_bundle(
        &recovered,
        &BranchId("feature".to_string()),
        entity,
        created.version_id,
    );

    assert_eq!(expected, actual);
}

#[test]
fn durability_contract_live_branch_pin_counts_match_branch_head_membership() {
    let mut runtime = persisted_runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "source");
    let entity = changed_entities(&created)[0];
    let inspection = runtime.inspection_access();
    assert_eq!(
        inspection
            .inspect_record_retention(RecordRef::Entity(entity))
            .expect("entity retention after create")
            .pins
            .branch_pins,
        1
    );

    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let inspection = runtime.inspection_access();
    assert_eq!(
        inspection
            .inspect_record_retention(RecordRef::Entity(entity))
            .expect("entity retention after branch create")
            .pins
            .branch_pins,
        2
    );

    update_entity(&mut runtime, entity, "main");
    let inspection = runtime.inspection_access();
    assert_eq!(
        inspection
            .inspect_record_retention(RecordRef::Entity(entity))
            .expect("entity retention after main update")
            .pins
            .branch_pins,
        2
    );

    update_entity_on_branch(
        &mut runtime,
        entity,
        "feature",
        BranchId("feature".to_string()),
    );
    let inspection = runtime.inspection_access();
    assert_eq!(
        inspection
            .inspect_record_retention(RecordRef::Entity(entity))
            .expect("entity retention after feature update")
            .pins
            .branch_pins,
        2
    );
}

#[test]
fn durability_contract_persisted_commit_fails_closed_when_store_path_is_not_directory() {
    let root_path = unique_test_store_path("forge-relational-bad-store");
    std::fs::write(&root_path, b"not-a-directory").unwrap();
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .durability_mode(DurabilityMode::PersistedSegmentedLocalFs)
        .durable_store_layout(DurableStoreLayout {
            root_path: root_path.clone(),
            segment_commit_capacity: 2,
        })
        .build();

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(batch_create("fail-closed"));
    let error = txn.commit().unwrap_err();

    assert!(matches!(error, TransactionCommitError::Publication { .. }));
    assert!(runtime.history_access().latest_commit().is_none());
}

