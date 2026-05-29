use crate::diagnostics::data::RelationalDiagnosticValue;
use crate::facade::diagnostics::{DiagnosticCode, DiagnosticsScope};
use crate::facade::durability::{
    DurabilityMode, DurableStore, DurableStoreLayout, RecoveryAuthorityParity,
    RecoveryCompatibilityCheck, RecoveryCompatibilityMismatch, RecoveryCursor,
    RecoveryFailureClass, RecoveryIntegrityReport, RecoveryPlan, RecoveryVerificationMode,
    RecoveryVerificationOutcome, RelationIntegrityContractFamily,
};
use crate::facade::history::BranchId;
use crate::facade::identity::KindId;
use crate::facade::indexes::{
    DerivedIndexBuildRequest, DerivedIndexDefinition, DerivedIndexId, DerivedIndexKind,
};
use crate::facade::lineage::{
    CorrespondencePromotionRejectionClass, LineageDecisionKind, LineageEventKind,
};
use crate::facade::merge::{MergeExecutionRequest, MergeIntent};
use crate::facade::replay::ReplayVerificationLayer;
use crate::facade::runtime::RelationalRuntimeApi;
use crate::facade::schema::{DescriptorCanonicalizationVersion, DescriptorSemanticsVersion};
use crate::facade::schema::{
    EntityKindRegistration, HistoricalInterpretationSensitivity, KindAspectDeclarations,
    ProposedSchemaTransition, RelationalSchemaRegistry, SchemaDiffAtom, SchemaDiffDetail,
    SchemaElementKind, SchemaElementRef, SchemaId, SchemaPublicationImpact,
    SchemaReconciliationPolicy, SchemaStratum, SchemaSubscriberImpact, SchemaVersionId,
};
use crate::facade::transactions::{TransactionCommitError, TransactionOptions};
use crate::tests::support::*;

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
            .history()
            .branch_head(&BranchId("feature".to_string())),
        Some(&feature.commit)
    );
    assert_eq!(
        recovered
            .history()
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
            .history()
            .entity_aspect_history(&BranchId("main".to_string()), entity, None);
    let expected_digest = runtime
        .history()
        .entity_aspect_history_with_trace(&BranchId("main".to_string()), entity, None)
        .aspect_history_digest();
    let expected_envelope = runtime
        .replay()
        .canonical_commit_envelope(updated.commit.commit_id)
        .cloned()
        .unwrap();
    let (outcome, recovered) = checkpoint_and_recover_with(&mut runtime, || {
        persisted_runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations)
    });

    let recovered_history =
        recovered
            .history()
            .entity_aspect_history(&BranchId("main".to_string()), entity, None);
    let recovered_digest = recovered
        .history()
        .entity_aspect_history_with_trace(&BranchId("main".to_string()), entity, None)
        .aspect_history_digest();
    let recovered_replay = recovered.replay();
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
        recovered_envelope.patch.records[0].authoritative_changed_aspects(),
        CanonicalAspectSet::new([aspect_key("name")])
    );
    assert!(!recovered_envelope.patch.records[0].contains_opaque_aspect);
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
    let expected_history =
        runtime
            .history()
            .relation_aspect_history(&BranchId("main".to_string()), relation, None);
    let expected_digest = runtime
        .history()
        .relation_aspect_history_with_trace(&BranchId("main".to_string()), relation, None)
        .aspect_history_digest();
    let (outcome, recovered) = checkpoint_and_recover_with(&mut runtime, || {
        persisted_runtime_with_declared_aspect_schema(CascadeDeletePolicy::RetainDanglingForAudit)
    });

    let recovered_history =
        recovered
            .history()
            .relation_aspect_history(&BranchId("main".to_string()), relation, None);
    let recovered_digest = recovered
        .history()
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
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::RetainDanglingForAudit,
                aspect_declarations: KindAspectDeclarations::default(),
                relation_integrity: crate::schema::data::RelationIntegrityDeclarations::new(
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    vec![crate::schema::data::EndpointDeletionIntegrityDeclaration {
                        contract_id: "require_retirement".into(),
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
    let expected_main_inspection = runtime.inspect_what_happened().inspect_historical_record(
        &BranchId("main".to_string()),
        runtime
            .history()
            .branch_head(&BranchId("main".to_string()))
            .unwrap()
            .version_id,
        RecordRef::Relation(relation),
        crate::facade::inspection::HistoricalInspectionMode::RetainedOnly,
    );
    let expected_feature_inspection = runtime.inspect_what_happened().inspect_historical_record(
        &BranchId("feature".to_string()),
        runtime
            .history()
            .branch_head(&BranchId("feature".to_string()))
            .unwrap()
            .version_id,
        RecordRef::Relation(relation),
        crate::facade::inspection::HistoricalInspectionMode::RetainedOnly,
    );

    runtime.durability_authority().checkpoint().unwrap();
    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
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
    let recovered_main_inspection = recovered.inspect_what_happened().inspect_historical_record(
        &BranchId("main".to_string()),
        recovered
            .history()
            .branch_head(&BranchId("main".to_string()))
            .unwrap()
            .version_id,
        RecordRef::Relation(relation),
        crate::facade::inspection::HistoricalInspectionMode::RetainedOnly,
    );
    let recovered_feature_inspection = recovered.inspect_what_happened().inspect_historical_record(
        &BranchId("feature".to_string()),
        recovered
            .history()
            .branch_head(&BranchId("feature".to_string()))
            .unwrap()
            .version_id,
        RecordRef::Relation(relation),
        crate::facade::inspection::HistoricalInspectionMode::RetainedOnly,
    );

    assert_eq!(
        outcome.latest_commit,
        runtime.history().latest_commit().cloned()
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
    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
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
    let mut plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
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
        .durability()
        .recovery_plan(RecoveryVerificationMode::NormalRecoveryVerification);
    let audit = runtime
        .durability()
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
    let store = runtime
        .durability()
        .recovery_plan(
            crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
        )
        .store
        .unwrap();
    let segment_path = store
        .segments
        .last()
        .expect("persisted segment after commit")
        .path
        .clone();
    let mut file =
        crate::durability::log::native_file_codec::read_segment_file(&segment_path).unwrap();
    file.entries[0].descriptor_semantics_version = DescriptorSemanticsVersion(99);
    crate::durability::log::native_file_codec::write_segment_file(&segment_path, &file).unwrap();

    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );

    assert_eq!(
        plan.descriptor_semantics_version,
        DescriptorSemanticsVersion(99)
    );
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
        ..AspectSchemaFixture::with_default_declared_aspects(
            CascadeDeletePolicy::CascadeDeleteRelations,
        )
    }
    .build_registry();
    let mut txn = runtime.begin_transaction(TransactionOptions::default().with_schema_transition(
        schema_transition_for_subscriber_impact(
            SchemaVersionId(2),
            SchemaSubscriberImpact::ConsumableSurfaceChanged,
        ),
        Some(SchemaReconciliationPolicy::PreserveInformation),
    ));
    txn.push_batch(batch_create("transitioned"));
    txn.commit().unwrap();

    let segment_path = runtime
        .durability()
        .recovery_plan(
            crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
        )
        .store
        .unwrap()
        .segments
        .last()
        .expect("persisted segment after transition")
        .path
        .clone();
    let mut file =
        crate::durability::log::native_file_codec::read_segment_file(&segment_path).unwrap();
    if let Some(descriptor) = file.entries[1].schema_continuation_descriptor.as_mut() {
        descriptor.bridge.canonicalization_version = DescriptorCanonicalizationVersion(99);
    }
    crate::durability::log::native_file_codec::write_segment_file(&segment_path, &file).unwrap();

    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );

    assert_eq!(
        plan.compatibility.descriptor_version_parity,
        RecoveryAuthorityParity::Drift
    );
    assert!(matches!(
        plan.compatibility.first_mismatch,
        Some(
            RecoveryCompatibilityMismatch::DescriptorCanonicalizationVersion {
                expected: DescriptorCanonicalizationVersion(1),
                found: DescriptorCanonicalizationVersion(99),
            }
        )
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
    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );

    let mut recovered = persisted_runtime_with_test_schema();
    let _ = recovered.durability_authority().recover(plan).unwrap();

    let diagnostics = recovered.publication().diagnostics();
    let compatibility_entry = diagnostics
        .by_scope(DiagnosticsScope::History)
        .into_iter()
        .flat_map(|artifact| artifact.entries.iter())
        .find(|entry| entry.code == DiagnosticCode::DurableRecoveryCompatibilityEvaluated)
        .expect("recovery compatibility diagnostic");
    assert_eq!(
        diagnostic_field(compatibility_entry, "verification_rejected"),
        &RelationalDiagnosticValue::Bool(false)
    );
    assert_eq!(
        diagnostic_field(compatibility_entry, "verification_layer"),
        &RelationalDiagnosticValue::string("DigestParity")
    );
}

#[test]
fn durability_certification_recovery_compatibility_is_explained_and_counted() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "main-a");
    runtime.performance_access().reset_counters();
    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );

    let mut recovered = persisted_runtime_with_test_schema();
    let _ = recovered.durability_authority().recover(plan).unwrap();

    let diagnostics = recovered.publication().diagnostics();
    let compatibility_entry = diagnostics
        .by_scope(DiagnosticsScope::History)
        .into_iter()
        .flat_map(|artifact| artifact.entries.iter())
        .find(|entry| entry.code == DiagnosticCode::DurableRecoveryCompatibilityEvaluated)
        .expect("recovery certification diagnostic");
    assert_eq!(
        diagnostic_field(compatibility_entry, "verification_mode"),
        &RelationalDiagnosticValue::string("NormalRecoveryVerification")
    );
    assert_eq!(
        diagnostic_field(compatibility_entry, "verification_rejected"),
        &RelationalDiagnosticValue::Bool(false)
    );
    assert_eq!(
        diagnostic_field(compatibility_entry, "verification_layer"),
        &RelationalDiagnosticValue::string("DigestParity")
    );
    assert_eq!(
        diagnostic_field(compatibility_entry, "descriptor_version_parity"),
        &verified_at_digest_parity_value()
    );
    assert_eq!(
        diagnostic_field(compatibility_entry, "schema_transition_parity"),
        &verified_at_digest_parity_value()
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
        ..AspectSchemaFixture::with_default_declared_aspects(
            CascadeDeletePolicy::CascadeDeleteRelations,
        )
    }
    .build_registry();
    let mut txn = runtime.begin_transaction(TransactionOptions::default().with_schema_transition(
        schema_transition_for_subscriber_impact(
            SchemaVersionId(2),
            SchemaSubscriberImpact::ConsumableSurfaceChanged,
        ),
        Some(SchemaReconciliationPolicy::PreserveInformation),
    ));
    txn.push_batch(batch_create("main-b"));
    let transitioned = txn.commit().unwrap();

    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let live_schema_transition = transitioned.envelope().schema_transition.clone();
    let live_schema_continuation_descriptor = transitioned
        .envelope()
        .schema_continuation_descriptor
        .clone();
    let live_schema_reconciliation_descriptor = transitioned
        .envelope()
        .schema_reconciliation_descriptor
        .clone();
    let live_recovery_compatibility = plan.compatibility.clone();

    let mut recovered = RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(
            AspectSchemaFixture {
                schema_version_id: SchemaVersionId(2),
                ..AspectSchemaFixture::with_default_declared_aspects(
                    CascadeDeletePolicy::CascadeDeleteRelations,
                )
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
        .replay()
        .canonical_commit_envelope(transitioned.commit.commit_id)
        .cloned()
        .expect("recovered transitioned envelope");
    let recovered_diagnostics = recovered.publication().diagnostics();
    let recovery_compatibility_diagnostic = recovered_diagnostics
        .by_scope(DiagnosticsScope::History)
        .into_iter()
        .flat_map(|artifact| artifact.entries.iter())
        .find(|entry| entry.code == DiagnosticCode::DurableRecoveryCompatibilityEvaluated)
        .expect("recovery compatibility diagnostic");

    assert_eq!(
        live_schema_transition,
        recovered_envelope.schema_transition.clone()
    );
    assert_eq!(
        live_schema_continuation_descriptor,
        recovered_envelope.schema_continuation_descriptor.clone()
    );
    assert_eq!(
        live_schema_reconciliation_descriptor,
        recovered_envelope.schema_reconciliation_descriptor.clone()
    );
    assert_eq!(live_recovery_compatibility, plan.compatibility.clone());
    assert!(recovered_diagnostics
        .by_scope(DiagnosticsScope::History)
        .into_iter()
        .flat_map(|artifact| artifact.entries.iter())
        .any(|entry| {
            entry.code == DiagnosticCode::DurableRecoveryCompatibilityEvaluated
                && diagnostic_field_optional(entry, "verification_layer")
                    == Some(&RelationalDiagnosticValue::string("DigestParity"))
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
    assert_eq!(
        diagnostic_field(recovery_compatibility_diagnostic, "verification_layer"),
        &RelationalDiagnosticValue::string("DigestParity")
    );
    assert!(!error.detail.is_empty());
}

#[test]
fn durability_contract_failure_aspect_plan_mismatch_is_explicit() {
    let mut runtime =
        persisted_runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    create_entity_outcome(&mut runtime, "main-a");
    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let expected_registry =
        declared_aspect_schema_registry(CascadeDeletePolicy::CascadeDeleteRelations);
    let mismatched_registry = AspectSchemaFixture {
        entity_aspects: vec![
            entity_field_aspect(
                crate::tests::support::aspect_key("display_name"),
                crate::tests::support::field_key("name"),
            ),
            lifecycle_aspect(),
        ],
        relation_aspects: vec![
            relation_field_aspect(
                crate::tests::support::aspect_key("label"),
                crate::tests::support::field_key("label"),
            ),
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
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                aspect_declarations: KindAspectDeclarations::default(),
                relation_integrity: crate::schema::data::RelationIntegrityDeclarations::new(
                    Vec::new(),
                    vec![crate::schema::data::CardinalityContractDeclaration {
                        contract_id: "source_max_one".into(),
                        source_max: Some(1),
                        source_min: None,
                        target_max: None,
                        target_min: None,
                        pair_max: None,
                        pair_min: None,
                        pair_min_semantics: crate::schema::data::PairMinimumSemantics::ObservedDirectedPairs,
                        minimum_enforcement:
                            crate::schema::data::MinimumCardinalityEnforcement::CertificationBoundary,
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
    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );

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
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                aspect_declarations: KindAspectDeclarations::default(),
                relation_integrity: crate::schema::data::RelationIntegrityDeclarations::new(
                    Vec::new(),
                    vec![crate::schema::data::CardinalityContractDeclaration {
                        contract_id: "source_max_two".into(),
                        source_max: Some(2),
                        source_min: None,
                        target_max: None,
                        target_min: None,
                        pair_max: None,
                        pair_min: None,
                        pair_min_semantics: crate::schema::data::PairMinimumSemantics::ObservedDirectedPairs,
                        minimum_enforcement:
                            crate::schema::data::MinimumCardinalityEnforcement::CertificationBoundary,
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
        }) if expected_contract_ids == &vec![crate::schema::data::ContractId::from("source_max_one")]
            && found_contract_ids == &vec![crate::schema::data::ContractId::from("source_max_two")]
    ));
}

#[test]
fn durability_contract_recovery_ignores_rejected_relation_integrity_attempts() {
    let fixture = RelationIntegritySchemaFixture {
        relation_integrity: crate::schema::data::RelationIntegrityDeclarations::new(
            Vec::new(),
            vec![crate::schema::data::CardinalityContractDeclaration {
                contract_id: "source_max_one".into(),
                source_max: Some(1),
                source_min: None,
                target_max: None,
                target_min: None,
                pair_max: None,
                pair_min: None,
                pair_min_semantics:
                    crate::schema::data::PairMinimumSemantics::ObservedDirectedPairs,
                minimum_enforcement:
                    crate::schema::data::MinimumCardinalityEnforcement::CertificationBoundary,
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
    let latest_commit_before = runtime.history().latest_commit().cloned();
    let latest_patch_before = runtime.publication().latest_patch().unwrap().position;
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
                client_key: crate::symbols::data::ClientKey::raw("illegal-overflow"),
                source: crate::transactions::data::EntityReference::Existing(source),
                target: crate::transactions::data::EntityReference::Existing(target_b),
                fields: crate::transactions::data::AspectFieldPatch::default(),
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

    assert_eq!(
        runtime.history().latest_commit().cloned(),
        latest_commit_before
    );
    assert_eq!(
        runtime.publication().latest_patch().unwrap().position,
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
    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
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
        .replay()
        .canonical_commit_envelope(latest_commit_before.unwrap().commit_id)
        .is_some());
}

#[test]
fn durability_contract_failure_missing_authoritative_parent_closure_is_explicit() {
    let mut runtime = runtime_with_test_schema();
    let parent = create_entity_outcome(&mut runtime, "main-a");
    let child = create_entity_outcome(&mut runtime, "main-b");
    let child_envelope = runtime
        .replay()
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
        vec![child.commit.commit_id],
    );
    let mut recovered = runtime_with_test_schema();
    let error = recovered
        .durability_authority()
        .recover(corrupt_plan)
        .unwrap_err();

    assert_eq!(parent.commit.commit_id.0, 1);
    assert_eq!(
        error.class,
        RecoveryFailureClass::MissingAuthoritativeParentClosure
    );
    assert_eq!(
        error.history_drift_class,
        Some(crate::facade::history::HistoryDriftClass::CanonicalHistoryDrift)
    );
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
    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let mut recovered = persisted_runtime_with_test_schema();
    recovered.durability_authority().recover(plan).unwrap();
    let replay = recovered.replay();
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
fn durability_contract_replays_merge_from_typed_authority_when_diagnostics_are_absent() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "main");
    create_branch_from_main(&mut runtime, "feature");
    create_entity_outcome_on_branch(&mut runtime, "feature", BranchId("feature".to_string()));
    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared merge execution");
    let merge = runtime
        .execute_prepared_merge(prepared)
        .expect("executed prepared merge");

    let segment_path = runtime
        .durability()
        .recovery_plan(RecoveryVerificationMode::NormalRecoveryVerification)
        .store
        .unwrap()
        .segments
        .last()
        .expect("persisted segment after merge")
        .path
        .clone();
    let mut file =
        crate::durability::log::native_file_codec::read_segment_file(&segment_path).unwrap();
    let merge_entry = file
        .entries
        .iter_mut()
        .find(|entry| entry.commit.commit_id == merge.commit.commit.commit_id)
        .expect("merge entry in durable segment");
    assert!(merge_entry.merge_execution_authority.is_some());
    merge_entry.diagnostics_summary.entries.clear();
    crate::durability::log::native_file_codec::write_segment_file(&segment_path, &file).unwrap();

    let plan = runtime
        .durability()
        .recovery_plan(RecoveryVerificationMode::NormalRecoveryVerification);
    let mut recovered = persisted_runtime_with_test_schema();
    recovered.durability_authority().recover(plan).unwrap();
    let replay = recovered.replay();
    let recovered_merge = replay
        .canonical_commit_envelope(merge.commit.commit.commit_id)
        .expect("recovered merge envelope");

    assert!(recovered_merge.diagnostics_summary.entries.is_empty());
    assert!(recovered_merge.merge_execution_authority.is_some());
    assert_eq!(
        recovered
            .history()
            .branch_head(&BranchId("main".to_string()))
            .expect("main head after recovery")
            .commit_id,
        merge.commit.commit.commit_id
    );
}

#[test]
fn durability_contract_reports_parent_order_parity_drift_when_durable_segment_is_tampered() {
    let mut runtime = persisted_runtime_with_test_schema();
    let main = create_entity_outcome(&mut runtime, "main");
    create_branch_from_main(&mut runtime, "feature");
    let feature =
        create_entity_outcome_on_branch(&mut runtime, "feature", BranchId("feature".to_string()));
    let merge = merge_commit_from_branches(
        &mut runtime,
        BranchId("main".to_string()),
        vec![BranchId("feature".to_string())],
    );

    let segment_path = runtime
        .durability()
        .recovery_plan(
            crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
        )
        .store
        .unwrap()
        .segments
        .last()
        .expect("persisted segment after merge")
        .path
        .clone();
    let mut file =
        crate::durability::log::native_file_codec::read_segment_file(&segment_path).unwrap();
    let merge_entry = file
        .entries
        .iter_mut()
        .find(|entry| entry.commit.commit_id == merge.commit.commit_id)
        .expect("merge entry in durable segment");
    assert_eq!(
        merge_entry.commit.parents,
        vec![main.commit.commit_id, feature.commit.commit_id]
    );
    merge_entry.commit.parents.reverse();
    crate::durability::log::native_file_codec::write_segment_file(&segment_path, &file).unwrap();

    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let mut recovered = persisted_runtime_with_test_schema();
    let error = recovered.durability_authority().recover(plan).unwrap_err();

    assert_eq!(error.class, RecoveryFailureClass::ReplayFailure);
    assert_eq!(
        error.history_drift_class,
        Some(crate::facade::history::HistoryDriftClass::DurabilityParityDrift)
    );
    assert!(error.detail.contains("parity drifted"));
}

#[test]
fn durability_contract_checkpoint_tail_recovery_preserves_post_checkpoint_commits() {
    let mut runtime = persisted_runtime_with_test_schema();
    let main = create_entity_outcome(&mut runtime, "main-a");
    let _checkpoint = runtime.durability_authority().checkpoint().unwrap();
    let later = create_entity_outcome(&mut runtime, "main-b");
    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let mut recovered = persisted_runtime_with_test_schema();
    let outcome = recovered.durability_authority().recover(plan).unwrap();

    assert_eq!(outcome.recovered_commits, 2);
    assert_eq!(outcome.latest_commit, Some(later.commit.clone()));
    assert_eq!(
        recovered
            .history()
            .branch_head(&BranchId("main".to_string())),
        Some(&later.commit)
    );
    assert_eq!(
        recovered
            .replay()
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
        kind: DerivedIndexKind::EntityField {
            field: field_key("name"),
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
    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
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
    let checkpoint = runtime.durability_authority().checkpoint().unwrap();
    assert_eq!(
        checkpoint
            .lineage
            .digest_basis()
            .published_lineage_commit_count,
        runtime
            .history()
            .commit_envelopes_snapshot()
            .iter()
            .filter(|envelope| envelope.has_lineage_authority())
            .count()
    );
    assert_eq!(
        checkpoint
            .lineage
            .digest_basis()
            .published_lineage_event_count,
        runtime
            .history()
            .commit_envelopes_snapshot()
            .iter()
            .map(|envelope| envelope.lineage_digest_basis().lineage_event_count())
            .sum::<usize>()
    );
    assert_eq!(
        checkpoint
            .lineage
            .digest_basis()
            .published_lineage_decision_count,
        runtime
            .history()
            .commit_envelopes_snapshot()
            .iter()
            .map(|envelope| envelope.lineage_digest_basis().lineage_decision_count())
            .sum::<usize>()
    );
    assert_eq!(
        checkpoint.lineage.counters().node_count,
        checkpoint.lineage.nodes().len()
    );
    assert_eq!(
        checkpoint.lineage.counters().correspondence_candidate_count,
        checkpoint.lineage.correspondence_candidates().len()
    );
    assert_eq!(
        checkpoint.lineage.counters().rejected_decision_count,
        checkpoint.lineage.rejected_decisions().len()
    );
    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let mut recovered = persisted_runtime_with_test_schema();
    recovered.durability_authority().recover(plan).unwrap();
    let graph = recovered
        .lineage_access()
        .graph(crate::facade::lineage::LineageGraphRequest {
            branch_id: BranchId("main".to_string()),
            traversal_basis:
                crate::facade::lineage::LineageGraphTraversalBasis::FullBranchGraphMaterialization,
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
    let store = runtime
        .durability()
        .recovery_plan(
            crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
        )
        .store
        .unwrap();
    let latest_checkpoint = store.checkpoints.last().unwrap();
    std::fs::write(&latest_checkpoint.path, b"{not-json").unwrap();

    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let mut recovered = persisted_runtime_with_test_schema();
    let outcome = recovered.durability_authority().recover(plan).unwrap();

    assert_eq!(outcome.latest_commit, Some(second.commit.clone()));
    assert!(!outcome
        .integrity_report
        .skipped_corrupt_checkpoints
        .is_empty());
    assert_eq!(
        recovered
            .history()
            .branch_head(&BranchId("main".to_string())),
        Some(&second.commit)
    );
    assert!(recovered
        .replay()
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
    let before = runtime
        .durability()
        .recovery_plan(
            crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
        )
        .store
        .unwrap();

    let compaction = runtime.durability_authority().compact_store().unwrap();
    let after = runtime
        .durability()
        .recovery_plan(
            crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
        )
        .store
        .unwrap();

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
    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let mut recovered = persisted_runtime_with_test_schema();
    recovered.durability_authority().recover(plan).unwrap();

    let retention = recovered.retention().inspect_plan();
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
        txn.push_batch(
            WorkerIntentBatch::new("feature-update").push(MutationIntent::Entity(
                EntityMutationIntent::UpdateFields(UpdateEntityFieldsIntent {
                    entity_id: entity,
                    fields: crate::tests::support::single_string_aspect_field_patch(
                        crate::tests::support::field_key("name"),
                        "feature",
                    ),
                }),
            )),
        );
        txn.commit().unwrap()
    };
    runtime.durability_authority().checkpoint().unwrap();
    let expected = capture_inspection_truth_bundle(
        &runtime,
        &BranchId("feature".to_string()),
        entity,
        created.version_id,
    );

    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
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
    let inspection = runtime.inspect_what_happened();
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
    let inspection = runtime.inspect_what_happened();
    assert_eq!(
        inspection
            .inspect_record_retention(RecordRef::Entity(entity))
            .expect("entity retention after branch create")
            .pins
            .branch_pins,
        2
    );

    update_entity(&mut runtime, entity, "main");
    let inspection = runtime.inspect_what_happened();
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
    let inspection = runtime.inspect_what_happened();
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
    assert!(runtime.history().latest_commit().is_none());
}

fn verified_at_digest_parity_value() -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::object([
        (
            "parity",
            RelationalDiagnosticValue::string("VerifiedAtLayer"),
        ),
        (
            "verification_layer",
            RelationalDiagnosticValue::string("DigestParity"),
        ),
    ])
}
