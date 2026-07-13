use crate::tests::harness::fixtures::stores::{unique_test_sqlite_path, unique_test_store_path};
use crate::{
    ArtifactCompatibilityWindow, ArtifactFamilyId, ArtifactFormatVersion, ArtifactSemanticVersion,
    CompatibilityAdmissionCounters, CompatibilityManifestDigest, CompatibilityReadAdmissionOutcome,
    CompatibilityRejection, CompatibilityRejectionKind, WORTHStoreBuilder,
    QuarantinedDecodedArtifact, RawSubscriptionSupportDeclaration, RawSupportProgramAction,
    StoreErrorKind, SubscriptionResumeClassification, SubscriptionSupportAccessStructure,
    SubscriptionSupportActionOrigin, SubscriptionSupportAllocationScope,
    SubscriptionSupportArtifactId, SubscriptionSupportAuthority,
    SubscriptionSupportCompatibilityDecision, SubscriptionSupportCompatibilityDecisionKind,
    SubscriptionSupportCompatibilityOutcome, SubscriptionSupportDensityClass,
    SubscriptionSupportDriftCause, SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
    SubscriptionSupportFetchRequest, SubscriptionSupportMaintenanceDecision,
    SubscriptionSupportMissingSupportMaintenanceAdmission,
    SubscriptionSupportMissingSupportRecoveryRequest, SubscriptionSupportOperationalBasis,
    SubscriptionSupportOperationalVerdict, SubscriptionSupportOperationalVerdictTranslationRequest,
    SubscriptionSupportPayloadBudget, SubscriptionSupportPayloadDigest,
    SubscriptionSupportPlanFamily, SubscriptionSupportPortabilityDecision,
    SubscriptionSupportPortabilityDecisionKind, SubscriptionSupportPortabilityOutcome,
    SubscriptionSupportRestartReconstructionRequest, SubscriptionSupportRestartShard,
    SubscriptionSupportResumeEvidence, SubscriptionSupportResumeRequest,
    SubscriptionSupportRetentionDecision, SubscriptionSupportRetentionDecisionKind,
    SubscriptionSupportRetentionMaterialization, SubscriptionSupportRole, SubscriptionSupportScope,
    SupportActionBreadthBudget, SupportActionId, SupportActionPublicationState,
    SupportActionRecoveryDisposition, SupportAllocationScope, SupportCompatibilityReceiptWitness,
    SupportFamilyVersionWindow, SupportPathClass, SupportPortabilityManifestBudget,
    SupportProgramDensityClass,
};

fn raw_exact() -> RawSubscriptionSupportDeclaration {
    RawSubscriptionSupportDeclaration::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportAuthority::WorthQuery,
        "worth-query-live-v1",
        SubscriptionSupportScope::from_canonical(vec!["account:1".into(), "feed:2".into()])
            .unwrap(),
        SubscriptionSupportPayloadDigest::new("payload:abc").unwrap(),
    )
}

fn raw_degraded() -> RawSubscriptionSupportDeclaration {
    RawSubscriptionSupportDeclaration::new(
        SubscriptionSupportFamilyId::new("degraded-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::DegradedContinuationSupport,
        SubscriptionSupportRole::DegradedContinuation,
        SubscriptionSupportAuthority::WorthQuery,
        "worth-query-live-v1",
        SubscriptionSupportScope::from_canonical(vec!["feed:2".into()]).unwrap(),
        SubscriptionSupportPayloadDigest::new("payload:def").unwrap(),
    )
}

fn raw_materialized() -> RawSubscriptionSupportDeclaration {
    RawSubscriptionSupportDeclaration::new(
        SubscriptionSupportFamilyId::new("materialized-narrowing-support").unwrap(),
        SubscriptionSupportFamilyKind::MaterializedNarrowingSupport,
        SubscriptionSupportRole::NarrowingMaterialization,
        SubscriptionSupportAuthority::WORTHRuntimeBridge,
        "worth-runtime-bridge-v1",
        SubscriptionSupportScope::from_canonical(vec!["account:1".into(), "narrow:active".into()])
            .unwrap(),
        SubscriptionSupportPayloadDigest::new("payload:materialized").unwrap(),
    )
}

fn retention_basis(artifact_suffix: &str) -> SubscriptionSupportOperationalBasis {
    SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId(format!("artifact:store-retention:{artifact_suffix}")),
        "basis:store-retention",
        "cursor:store-retention",
        "checkpoint:store-retention",
        "compatibility:store-retention",
        "portability:store-retention",
        SubscriptionSupportActionOrigin::Retention,
    )
    .unwrap()
}

fn compatibility_basis(artifact_suffix: &str) -> SubscriptionSupportOperationalBasis {
    SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId(format!("artifact:store-compatibility:{artifact_suffix}")),
        "basis:store-compatibility",
        "cursor:store-compatibility",
        "checkpoint:store-compatibility",
        "compatibility:store-compatibility",
        "portability:store-compatibility",
        SubscriptionSupportActionOrigin::Compatibility,
    )
    .unwrap()
}

fn portability_basis(
    action_origin: SubscriptionSupportActionOrigin,
    artifact_suffix: &str,
) -> SubscriptionSupportOperationalBasis {
    SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId(format!("artifact:store-portability:{artifact_suffix}")),
        "basis:store-portability",
        "cursor:store-portability",
        "checkpoint:store-portability",
        "compatibility:store-portability",
        format!("portability:store-portability:{artifact_suffix}"),
        action_origin,
    )
    .unwrap()
}

fn maintenance_basis(artifact_suffix: &str) -> SubscriptionSupportOperationalBasis {
    SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId(format!("artifact:store-maintenance:{artifact_suffix}")),
        format!("basis:store-maintenance:{artifact_suffix}"),
        "cursor:store-maintenance",
        "checkpoint:store-maintenance",
        "compatibility:store-maintenance",
        "portability:store-maintenance",
        SubscriptionSupportActionOrigin::Maintenance,
    )
    .unwrap()
}

fn support_version_window() -> SupportFamilyVersionWindow {
    SupportFamilyVersionWindow::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        1,
        2,
    )
    .unwrap()
}

fn compatibility_manifest_digest(family_id: &ArtifactFamilyId) -> CompatibilityManifestDigest {
    CompatibilityManifestDigest::compute(
        family_id,
        &ArtifactCompatibilityWindow::native(1),
        "authoritative",
    )
}

fn rejected_read_outcome_witness(
    rejection_kind: CompatibilityRejectionKind,
) -> SupportCompatibilityReceiptWitness {
    let family_id = ArtifactFamilyId::new("basis-bound-continuation-support");
    let manifest_digest = compatibility_manifest_digest(&family_id);
    let artifact = QuarantinedDecodedArtifact::new(
        family_id.clone(),
        ArtifactFormatVersion::new(2),
        ArtifactSemanticVersion::new(2),
        manifest_digest,
        "structural:store-support-compatibility",
        "store support compatibility rejection fixture",
    );
    let rejection = CompatibilityRejection::new(
        rejection_kind,
        family_id,
        "support compatibility rejected by Milestone 12",
    );
    let counters = CompatibilityAdmissionCounters::default();
    let outcome = CompatibilityReadAdmissionOutcome::rejected(&artifact, &rejection, &counters);
    SupportCompatibilityReceiptWitness::from_read_admission_outcome(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        support_version_window(),
        &outcome,
    )
    .unwrap()
}

#[test]
fn subscription_support_operational_facade_helpers_record_backend_counters() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let basis = retention_basis("facade");
    let retention_plan = store
        .admit_subscription_support_retention_batch(
            SupportActionId::new("support-retention:facade-translation").unwrap(),
            vec![basis.clone()],
            SubscriptionSupportRetentionDecision::retain_exact(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();
    let retention_report = store
        .publish_subscription_support_retention_consequence(retention_plan)
        .unwrap();

    store
        .translate_subscription_support_operational_verdict(
            SubscriptionSupportOperationalVerdictTranslationRequest::exact_from_retention_report(
                &retention_report,
                basis,
            )
            .unwrap(),
        )
        .unwrap();

    let plan = store
        .admit_subscription_support_program_path(
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            2,
            128,
        )
        .unwrap();

    assert_eq!(
        store
            .subscription_support_counters()
            .operational_verdict_translation_count(),
        1
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_batch_receipt_reuse_count(),
        0
    );

    store
        .reuse_subscription_support_batch_receipt(&plan)
        .unwrap();

    assert_eq!(
        store
            .subscription_support_counters()
            .support_batch_receipt_reuse_count(),
        1
    );
}

#[test]
fn subscription_support_retention_facade_publishes_consequence_and_counters() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let plan = store
        .admit_subscription_support_retention_batch(
            SupportActionId::new("support-retention:facade").unwrap(),
            vec![retention_basis("facade-1"), retention_basis("facade-2")],
            SubscriptionSupportRetentionDecision::expire_by_policy(
                "support retention window expired",
            )
            .unwrap(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            256,
        )
        .unwrap();

    assert_eq!(
        store
            .subscription_support_counters()
            .support_retention_plan_count(),
        1
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_policy_expiration_count(),
        0
    );

    let report = store
        .publish_subscription_support_retention_consequence(plan)
        .unwrap();

    assert_eq!(
        report.survival_witness().verdict(),
        SubscriptionSupportOperationalVerdict::RejectedByPolicy
    );
    assert_eq!(
        report.retention_record().decision_kind(),
        SubscriptionSupportRetentionDecisionKind::ExpireByPolicy
    );
    assert_eq!(
        report.retention_record().affected_set_digest(),
        report.survival_witness().affected_set_digest()
    );
    assert!(matches!(
        report.materialization(),
        SubscriptionSupportRetentionMaterialization::Expired(_)
    ));
    assert_eq!(
        store
            .subscription_support_counters()
            .support_action_envelope_publications(),
        1
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_expired_family_count(),
        1
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_policy_expiration_count(),
        1
    );
}

#[test]
fn subscription_support_compatibility_facade_publishes_version_skew_consequence_and_counters() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let plan = store
        .admit_subscription_support_compatibility_batch(
            SupportActionId::new("support-compatibility:facade").unwrap(),
            vec![compatibility_basis("facade")],
            rejected_read_outcome_witness(CompatibilityRejectionKind::MissingCompatibilityEdge),
            "semantic:store-compatibility",
            SubscriptionSupportCompatibilityDecision::version_skew_rejected(
                "payload version has no admitted support reader",
            )
            .unwrap(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();

    let report = store
        .publish_subscription_support_compatibility_consequence(plan)
        .unwrap();
    let SubscriptionSupportCompatibilityOutcome::Rejected(rejection) = report.outcome() else {
        panic!("version skew must publish a typed rejection outcome");
    };
    assert_eq!(
        rejection.rejection_kind(),
        SubscriptionSupportCompatibilityDecisionKind::VersionSkewRejected
    );
    assert_eq!(
        report.completed_action().envelope().verdict(),
        SubscriptionSupportOperationalVerdict::RejectedByPolicy
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_compatibility_plan_count(),
        1
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_version_skew_rejection_count(),
        1
    );
}

#[test]
fn subscription_support_portability_facade_publishes_import_bundle_and_counters() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let plan = store
        .admit_subscription_support_portability_batch(
            SupportActionId::new("support-portability:facade-import").unwrap(),
            vec![portability_basis(
                SubscriptionSupportActionOrigin::ReplicationImport,
                "facade-import",
            )],
            1,
            0,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::target_import_admitted(
                "target-store-import-admission",
                "source-identity-preservation:store-import",
                "semantic:store-portability-import",
            )
            .unwrap(),
            SupportPathClass::ImportAdmission,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();

    assert_eq!(
        store
            .subscription_support_counters()
            .support_portability_plan_count(),
        1
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_import_admission_count(),
        0
    );

    let report = store
        .publish_subscription_support_portability_consequence(plan)
        .unwrap();
    let SubscriptionSupportPortabilityOutcome::Imported(access) = report.outcome() else {
        panic!("facade import must produce admitted semantic access");
    };

    assert_eq!(
        report.participation_record().decision_kind(),
        SubscriptionSupportPortabilityDecisionKind::TargetImportAdmitted
    );
    assert_eq!(
        access.import_admission().manifest_digest(),
        report.manifest().manifest_digest()
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_action_envelope_publications(),
        1
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_import_admission_count(),
        1
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_portability_required_basis_count(),
        1
    );
}

#[test]
fn subscription_support_maintenance_facade_admits_descriptor_and_counters() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let basis = maintenance_basis("facade-rebuild");
    let retained_basis_digest = basis.basis_digest().to_string();
    let plan = store
        .admit_subscription_support_maintenance_batch(
            SupportActionId::new("support-maintenance:facade-rebuild").unwrap(),
            vec![basis],
            SubscriptionSupportMaintenanceDecision::rebuild_descriptor_admitted(
                retained_basis_digest,
            )
            .unwrap(),
            SupportPathClass::MaintenanceExecution,
            SupportProgramDensityClass::MaintenanceKeyBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();

    assert_eq!(
        plan.maintenance_receipt().batch_summary().batch_class(),
        crate::MaintenanceBatchClass::SubscriptionSupport
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_maintenance_descriptor_count(),
        1
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_maintenance_rebuild_debt_count(),
        0
    );

    let report = store
        .publish_subscription_support_maintenance_consequence(plan)
        .unwrap();

    assert_eq!(report.admissions().len(), 1);
    assert_eq!(report.descriptor_records().len(), 1);
    assert_eq!(
        report.completed_action().envelope().verdict(),
        SubscriptionSupportOperationalVerdict::RebuildRequired
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_action_envelope_publications(),
        1
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_maintenance_rebuild_debt_count(),
        1
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_action_envelope_publications(),
        1
    );
}

#[test]
fn subscription_support_maintenance_delay_report_persists_without_publishing_action() {
    let path = unique_test_store_path("worth-store-support-maintenance-delay-local-reopen");
    {
        let mut store = WORTHStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        let basis = maintenance_basis("delayed-local");
        let retained_basis_digest = basis.basis_digest().to_string();
        let plan = store
            .admit_subscription_support_maintenance_batch(
                SupportActionId::new("support-maintenance:delayed-local").unwrap(),
                vec![basis],
                SubscriptionSupportMaintenanceDecision::rebuild_descriptor_admitted(
                    retained_basis_digest,
                )
                .unwrap(),
                SupportPathClass::MaintenanceExecution,
                SupportProgramDensityClass::MaintenanceKeyBatch,
                SupportAllocationScope::FamilyLocalBatch,
                SupportActionBreadthBudget::new(4, 1024).unwrap(),
                128,
            )
            .unwrap();

        let report = store
            .report_delayed_subscription_support_maintenance(
                &plan,
                "maintenance pacing deferred support rebuild",
                SupportActionBreadthBudget::new(4, 1024).unwrap(),
                128,
            )
            .unwrap();

        assert_eq!(
            report.debt_summary().verdict(),
            SubscriptionSupportOperationalVerdict::RebuildRequired
        );
        assert_eq!(
            store
                .subscription_support_counters()
                .support_maintenance_delay_count(),
            1
        );
        assert_eq!(
            store
                .subscription_support_counters()
                .support_action_envelope_publications(),
            0
        );
    }

    let reopened = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    assert_eq!(
        reopened
            .subscription_support_counters()
            .support_maintenance_delay_count(),
        1
    );

    let raw = std::fs::read_to_string(&path).unwrap();
    let payload: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let records = payload
        .get("subscription_support_maintenance_debt_records")
        .and_then(serde_json::Value::as_object)
        .expect("maintenance debt records should persist");
    assert_eq!(records.len(), 1);
}

#[test]
fn subscription_support_maintenance_descriptor_records_survive_local_file_reopen() {
    let path = unique_test_store_path("worth-store-support-maintenance-local-reopen");
    let declaration_id = {
        let mut store = WORTHStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        let basis = maintenance_basis("local-reopen");
        let retained_basis_digest = basis.basis_digest().to_string();
        let plan = store
            .admit_subscription_support_maintenance_batch(
                SupportActionId::new("support-maintenance:local-reopen").unwrap(),
                vec![basis],
                SubscriptionSupportMaintenanceDecision::rebuild_descriptor_admitted(
                    retained_basis_digest,
                )
                .unwrap(),
                SupportPathClass::MaintenanceExecution,
                SupportProgramDensityClass::MaintenanceKeyBatch,
                SupportAllocationScope::FamilyLocalBatch,
                SupportActionBreadthBudget::new(4, 1024).unwrap(),
                128,
            )
            .unwrap();
        let declaration_id = plan.maintenance_receipt().admitted_declarations()[0]
            .declaration()
            .id()
            .clone();
        let report = store
            .publish_subscription_support_maintenance_consequence(plan)
            .unwrap();
        assert_eq!(report.descriptor_records().len(), 1);
        declaration_id
    };

    let reopened = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    let status = reopened.maintenance_status(&declaration_id).unwrap();
    assert_eq!(
        status.execution_status(),
        crate::MaintenanceExecutionStatus::Admitted
    );
    let raw = std::fs::read_to_string(&path).unwrap();
    let state: crate::backend::records::StoreState = serde_json::from_str(&raw).unwrap();
    assert_eq!(
        state
            .subscription_support_maintenance_descriptor_records
            .len(),
        1
    );
}

#[test]
fn subscription_support_maintenance_descriptor_records_survive_sqlite_reopen() {
    let path = unique_test_sqlite_path("worth-store-support-maintenance-sqlite-reopen");
    let declaration_id = {
        let mut store = WORTHStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        let basis = maintenance_basis("sqlite-reopen");
        let retained_basis_digest = basis.basis_digest().to_string();
        let plan = store
            .admit_subscription_support_maintenance_batch(
                SupportActionId::new("support-maintenance:sqlite-reopen").unwrap(),
                vec![basis],
                SubscriptionSupportMaintenanceDecision::rebuild_descriptor_admitted(
                    retained_basis_digest,
                )
                .unwrap(),
                SupportPathClass::MaintenanceExecution,
                SupportProgramDensityClass::MaintenanceKeyBatch,
                SupportAllocationScope::FamilyLocalBatch,
                SupportActionBreadthBudget::new(4, 1024).unwrap(),
                128,
            )
            .unwrap();
        let declaration_id = plan.maintenance_receipt().admitted_declarations()[0]
            .declaration()
            .id()
            .clone();
        store
            .publish_subscription_support_maintenance_consequence(plan)
            .unwrap();
        declaration_id
    };

    let reopened = WORTHStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .unwrap();
    let status = reopened.maintenance_status(&declaration_id).unwrap();
    assert_eq!(
        status.execution_status(),
        crate::MaintenanceExecutionStatus::Admitted
    );
    let connection = rusqlite::Connection::open(path).unwrap();
    let row_count: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM subscription_support_maintenance_descriptor_records",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(row_count, 1);
}

#[test]
fn local_file_subscription_support_maintenance_descriptor_drift_fails_open() {
    let path = unique_test_store_path("worth-store-support-maintenance-drift");
    {
        let mut store = WORTHStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        let basis = maintenance_basis("drift");
        let retained_basis_digest = basis.basis_digest().to_string();
        let plan = store
            .admit_subscription_support_maintenance_batch(
                SupportActionId::new("support-maintenance:drift").unwrap(),
                vec![basis],
                SubscriptionSupportMaintenanceDecision::rebuild_descriptor_admitted(
                    retained_basis_digest,
                )
                .unwrap(),
                SupportPathClass::MaintenanceExecution,
                SupportProgramDensityClass::MaintenanceKeyBatch,
                SupportAllocationScope::FamilyLocalBatch,
                SupportActionBreadthBudget::new(4, 1024).unwrap(),
                128,
            )
            .unwrap();
        store
            .publish_subscription_support_maintenance_consequence(plan)
            .unwrap();
    }

    let raw = std::fs::read_to_string(&path).unwrap();
    let mut payload: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let records = payload
        .get_mut("subscription_support_maintenance_descriptor_records")
        .and_then(serde_json::Value::as_object_mut)
        .expect("support maintenance descriptor records should persist");
    let first_record = records
        .values_mut()
        .next()
        .expect("one record should persist");
    first_record["descriptor_digest"] = serde_json::Value::String("drifted-digest".into());
    std::fs::write(&path, serde_json::to_string_pretty(&payload).unwrap()).unwrap();

    let error = WORTHStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err("descriptor drift should fail reopen");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportPublicationViolation
    );
}

#[test]
fn local_file_subscription_support_maintenance_debt_drift_fails_open() {
    let path = unique_test_store_path("worth-store-support-maintenance-debt-drift");
    {
        let mut store = WORTHStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        let basis = maintenance_basis("debt-drift");
        let retained_basis_digest = basis.basis_digest().to_string();
        let plan = store
            .admit_subscription_support_maintenance_batch(
                SupportActionId::new("support-maintenance:debt-drift").unwrap(),
                vec![basis],
                SubscriptionSupportMaintenanceDecision::rebuild_descriptor_admitted(
                    retained_basis_digest,
                )
                .unwrap(),
                SupportPathClass::MaintenanceExecution,
                SupportProgramDensityClass::MaintenanceKeyBatch,
                SupportAllocationScope::FamilyLocalBatch,
                SupportActionBreadthBudget::new(4, 1024).unwrap(),
                128,
            )
            .unwrap();
        store
            .report_delayed_subscription_support_maintenance(
                &plan,
                "maintenance debt drift fixture",
                SupportActionBreadthBudget::new(4, 1024).unwrap(),
                128,
            )
            .unwrap();
    }

    let raw = std::fs::read_to_string(&path).unwrap();
    let mut payload: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let records = payload
        .get_mut("subscription_support_maintenance_debt_records")
        .and_then(serde_json::Value::as_object_mut)
        .expect("support maintenance debt records should persist");
    let first_record = records
        .values_mut()
        .next()
        .expect("one debt record should persist");
    first_record["verdict"] = serde_json::Value::String(String::from("ExactResumePreserved"));
    std::fs::write(&path, serde_json::to_string_pretty(&payload).unwrap()).unwrap();

    let error = WORTHStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err("drifted maintenance debt report should fail reopen");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportPublicationViolation
    );
}

#[test]
fn publish_subscription_support_persists_complete_record_family() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let admitted = store
        .admit_subscription_support_declaration(raw_exact())
        .unwrap();
    let publishable = store
        .subscription_support_pipeline()
        .prepare_exact(
            admitted,
            "basis:1",
            "cursor:1",
            "checkpoint:1",
            "schema:1",
            "compatibility:1",
        )
        .unwrap();

    let published = store.publish_subscription_support(publishable).unwrap();
    let fetched = store
        .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            published.artifact_id().clone(),
        ))
        .unwrap();

    assert_eq!(
        fetched.record_set().key().artifact_id(),
        published.artifact_id().as_str()
    );
    assert_eq!(
        fetched.record_set().artifact_digest(),
        published.artifact_digest()
    );
    assert_eq!(fetched.record_set().basis_digest(), "basis:1");
    assert_eq!(fetched.record_set().cursor_digest(), "cursor:1");
    assert_eq!(fetched.record_set().checkpoint_digest(), "checkpoint:1");
    assert_eq!(fetched.record_set().schema_digest(), "schema:1");
    assert_eq!(
        fetched.record_set().compatibility_digest(),
        "compatibility:1"
    );
}

#[test]
fn subscription_support_action_publication_recovery_marks_pending_action_interrupted_after_reopen()
{
    let path = unique_test_store_path("worth-store-subscription-support-action-pending-recovery");
    let action_id = SupportActionId::new("support-retention:pending-publication-recovery").unwrap();
    {
        let mut store = WORTHStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        let executed = RawSupportProgramAction::new(
            action_id.clone(),
            retention_basis("pending-recovery"),
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        )
        .unwrap()
        .plan()
        .verify()
        .execute();
        store
            .persist_subscription_support_executed_action_for_publication(executed)
            .unwrap();
    }

    let mut reopened = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    let report = reopened
        .recover_subscription_support_action_publication(action_id.clone())
        .unwrap();
    assert_eq!(
        report.recovery_disposition(),
        SupportActionRecoveryDisposition::InterruptedBeforePublication
    );
    assert!(report.completed_action().is_none());
    assert_eq!(
        reopened
            .subscription_support_counters()
            .support_action_interrupted_recovery_count(),
        1
    );

    let raw = std::fs::read_to_string(&path).unwrap();
    let payload: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let records = payload
        .get("subscription_support_action_records")
        .and_then(serde_json::Value::as_object)
        .expect("support action records should persist");
    let record = records
        .get(action_id.as_str())
        .expect("pending action record should persist");
    assert_eq!(
        record
            .get("publication_state")
            .and_then(serde_json::Value::as_str),
        Some("InterruptedBeforePublication")
    );

    let mut reopened_again = WORTHStoreBuilder::new().local_file(path).build().unwrap();
    let second = reopened_again
        .recover_subscription_support_action_publication(action_id)
        .unwrap();
    assert_eq!(
        second.recovery_disposition(),
        SupportActionRecoveryDisposition::InterruptedBeforePublication
    );
    assert_eq!(
        reopened_again
            .subscription_support_counters()
            .support_action_interrupted_recovery_count(),
        1
    );
}

#[test]
fn subscription_support_action_publication_recovery_reopens_published_consequence_without_duplication(
) {
    let path =
        unique_test_sqlite_path("worth-store-subscription-support-action-published-recovery");
    let action_id =
        SupportActionId::new("support-retention:published-publication-recovery").unwrap();
    {
        let mut store = WORTHStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        let plan = store
            .admit_subscription_support_retention_batch(
                action_id.clone(),
                vec![retention_basis("published-recovery")],
                SubscriptionSupportRetentionDecision::retain_exact(),
                SupportPathClass::OperationalPlanning,
                SupportProgramDensityClass::FamilyLocalBatch,
                SupportAllocationScope::FamilyLocalBatch,
                SupportActionBreadthBudget::new(4, 1024).unwrap(),
                128,
            )
            .unwrap();
        store
            .publish_subscription_support_retention_consequence(plan)
            .unwrap();
    }

    let mut reopened = WORTHStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .unwrap();
    let report = reopened
        .recover_subscription_support_action_publication(action_id)
        .unwrap();
    assert_eq!(
        report.recovery_disposition(),
        SupportActionRecoveryDisposition::PublishedConsequenceRecovered
    );
    let completed = report
        .completed_action()
        .expect("published action recovery should expose completed action");
    assert_eq!(
        completed.envelope().recovery_disposition(),
        SupportActionRecoveryDisposition::PublishedConsequenceRecovered
    );
    assert_eq!(
        reopened
            .subscription_support_counters()
            .support_action_envelope_publications(),
        1
    );
    assert_eq!(
        reopened
            .subscription_support_counters()
            .support_action_interrupted_recovery_count(),
        0
    );

    let mut reopened_again = WORTHStoreBuilder::new().sqlite_file(path).build().unwrap();
    let second = reopened_again
        .recover_subscription_support_action_publication(
            SupportActionId::new("support-retention:published-publication-recovery").unwrap(),
        )
        .unwrap();
    assert_eq!(
        second.recovery_disposition(),
        SupportActionRecoveryDisposition::PublishedConsequenceRecovered
    );
    assert_eq!(
        reopened_again
            .subscription_support_counters()
            .support_action_interrupted_recovery_count(),
        0
    );
}

#[test]
fn subscription_support_translation_rejects_WORTHd_report_basis() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let plan = store
        .admit_subscription_support_retention_batch(
            SupportActionId::new("support-retention:WORTHd-translation-basis").unwrap(),
            vec![retention_basis("WORTHd-translation-basis")],
            SubscriptionSupportRetentionDecision::retain_exact(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();
    let report = store
        .publish_subscription_support_retention_consequence(plan)
        .unwrap();

    let WORTHd_basis = SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId("artifact:store-retention:WORTHd-translation-basis".into()),
        "basis:store-retention",
        "cursor:store-retention",
        "checkpoint:store-retention",
        "compatibility:WORTHd-drift",
        "portability:store-retention",
        SubscriptionSupportActionOrigin::Retention,
    )
    .unwrap();

    let error =
        SubscriptionSupportOperationalVerdictTranslationRequest::exact_from_retention_report(
            &report,
            WORTHd_basis,
        )
        .expect_err(
            "translation must reject a report basis whose digests drift from the published proof",
        );

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
}

#[test]
fn subscription_support_maintenance_debt_translation_requires_reported_basis() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let basis = maintenance_basis("delayed-exact-translation");
    let plan = store
        .admit_subscription_support_maintenance_batch(
            SupportActionId::new("support-maintenance:delayed-exact-translation").unwrap(),
            vec![basis.clone()],
            SubscriptionSupportMaintenanceDecision::refresh_descriptor_admitted(
                "maintenance refresh deferred by operator pacing",
            )
            .unwrap(),
            SupportPathClass::MaintenanceExecution,
            SupportProgramDensityClass::MaintenanceKeyBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();
    let report = store
        .report_delayed_subscription_support_maintenance(
            &plan,
            "maintenance refresh deferred by operator pacing",
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();

    let WORTHd_basis = SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        basis.artifact_id().clone(),
        "basis:store-maintenance:delayed-exact-translation",
        "cursor:store-maintenance",
        "checkpoint:store-maintenance",
        "compatibility:WORTHd-drift",
        "portability:store-maintenance",
        SubscriptionSupportActionOrigin::Maintenance,
    )
    .unwrap();

    let error =
        SubscriptionSupportOperationalVerdictTranslationRequest::exact_from_maintenance_debt_report(
            &report,
            WORTHd_basis,
        )
        .expect_err("maintenance debt translation must reject a basis the debt report did not prove");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );

    store
        .translate_subscription_support_operational_verdict(
            SubscriptionSupportOperationalVerdictTranslationRequest::exact_from_maintenance_debt_report(
                &report,
                basis,
            )
            .unwrap(),
        )
        .expect("exact delayed refresh should still translate when using the report-proven basis");
}

#[test]
fn local_file_interrupted_refresh_maintenance_work_kind_drift_fails_open() {
    let path = unique_test_store_path("worth-store-support-maintenance-work-kind-drift");
    {
        let mut store = WORTHStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        let plan = store
            .admit_subscription_support_maintenance_batch(
                SupportActionId::new("support-maintenance:restart-refresh-drift").unwrap(),
                vec![maintenance_basis("restart-refresh-drift")],
                SubscriptionSupportMaintenanceDecision::interrupted_restart_recovered(
                    crate::SupportMaintenanceWorkKind::Refresh,
                    "maintenance-restart:refresh-drift",
                )
                .unwrap(),
                SupportPathClass::MaintenanceExecution,
                SupportProgramDensityClass::MaintenanceKeyBatch,
                SupportAllocationScope::FamilyLocalBatch,
                SupportActionBreadthBudget::new(4, 1024).unwrap(),
                128,
            )
            .unwrap();
        store
            .publish_subscription_support_maintenance_consequence(plan)
            .unwrap();
    }

    let raw = std::fs::read_to_string(&path).unwrap();
    let mut payload: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let records = payload
        .get_mut("subscription_support_maintenance_descriptor_records")
        .and_then(serde_json::Value::as_object_mut)
        .expect("support maintenance descriptor records should persist");
    let first_record = records
        .values_mut()
        .next()
        .expect("one descriptor record should persist");
    first_record["work_kind"] = serde_json::Value::String(String::from("Rebuild"));
    std::fs::write(&path, serde_json::to_string_pretty(&payload).unwrap()).unwrap();

    let error = WORTHStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err("interrupted refresh descriptors must not reopen as rebuild work");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportPublicationViolation
    );
}

#[test]
fn local_file_subscription_support_action_record_drift_fails_open() {
    let path = unique_test_store_path("worth-store-subscription-support-action-record-drift");
    let action_id = SupportActionId::new("support-retention:drifted-action-record").unwrap();
    {
        let mut store = WORTHStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        let executed = RawSupportProgramAction::new(
            action_id.clone(),
            retention_basis("drifted-action"),
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        )
        .unwrap()
        .plan()
        .verify()
        .execute();
        store
            .persist_subscription_support_executed_action_for_publication(executed)
            .unwrap();
    }

    let raw = std::fs::read_to_string(&path).unwrap();
    let mut payload: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let records = payload
        .get_mut("subscription_support_action_records")
        .and_then(serde_json::Value::as_object_mut)
        .expect("support action records should persist");
    let record = records
        .get_mut(action_id.as_str())
        .expect("staged action record should persist");
    record["publication_state"] = serde_json::Value::String(format!(
        "{:?}",
        SupportActionPublicationState::PublishedConsequence
    ));
    std::fs::write(&path, serde_json::to_string_pretty(&payload).unwrap()).unwrap();

    let error = WORTHStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err("drifted action record should fail reopen");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportPublicationViolation
    );
}

#[test]
fn local_file_subscription_support_action_record_rejects_illegal_tier_recall_rebuild_state() {
    let path = unique_test_store_path("worth-store-subscription-support-tier-recall-action-drift");
    let action_id = SupportActionId::new("support-tier-recall:drifted-action-record").unwrap();
    {
        let mut store = WORTHStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        let executed = RawSupportProgramAction::new(
            action_id.clone(),
            retention_basis("tier-recall-drift"),
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        )
        .unwrap()
        .plan()
        .verify()
        .execute();
        store
            .persist_subscription_support_executed_action_for_publication(executed)
            .unwrap();
    }

    let raw = std::fs::read_to_string(&path).unwrap();
    let mut payload: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let records = payload
        .get_mut("subscription_support_action_records")
        .and_then(serde_json::Value::as_object_mut)
        .expect("support action records should persist");
    let record = records
        .get_mut(action_id.as_str())
        .expect("staged action record should persist");
    record["action_origin"] = serde_json::Value::String(String::from("TierRecall"));
    record["verdict"] = serde_json::Value::String(String::from("RebuildRequired"));
    std::fs::write(&path, serde_json::to_string_pretty(&payload).unwrap()).unwrap();

    let error = WORTHStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err("illegal tier-recall rebuild posture must fail reopen");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportPublicationViolation
    );
}

#[test]
fn fetch_subscription_support_requires_family_and_artifact_identity() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let admitted = store
        .admit_subscription_support_declaration(raw_exact())
        .unwrap();
    let publishable = store
        .subscription_support_pipeline()
        .prepare_exact(
            admitted,
            "basis:1",
            "cursor:1",
            "checkpoint:1",
            "schema:1",
            "compatibility:1",
        )
        .unwrap();
    let published = store.publish_subscription_support(publishable).unwrap();

    let error = store
        .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
            SubscriptionSupportFamilyId::new("degraded-continuation-support").unwrap(),
            SubscriptionSupportFamilyKind::DegradedContinuationSupport,
            published.artifact_id().clone(),
        ))
        .expect_err("artifact ids are not universal subscription-support fetch keys");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportPublicationViolation
    );
}

#[test]
fn duplicate_subscription_support_publication_is_idempotent_when_equivalent() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let admitted = store
        .admit_subscription_support_declaration(raw_exact())
        .unwrap();
    let publishable = store
        .subscription_support_pipeline()
        .prepare_exact(
            admitted,
            "basis:1",
            "cursor:1",
            "checkpoint:1",
            "schema:1",
            "compatibility:1",
        )
        .unwrap();

    let first = store
        .publish_subscription_support(publishable.clone())
        .unwrap();
    let second = store.publish_subscription_support(publishable).unwrap();

    assert_eq!(first.artifact_id(), second.artifact_id());
    assert_eq!(store.subscription_support_counters().duplicate_retries(), 1);
}

#[test]
fn subscription_support_fetch_reports_direct_lookup_cost() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let admitted = store
        .admit_subscription_support_declaration(raw_exact())
        .unwrap();
    let publishable = store
        .subscription_support_pipeline()
        .prepare_exact(
            admitted,
            "basis:1",
            "cursor:1",
            "checkpoint:1",
            "schema:1",
            "compatibility:1",
        )
        .unwrap();
    let published = store.publish_subscription_support(publishable).unwrap();
    let fetched = store
        .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            published.artifact_id().clone(),
        ))
        .unwrap();

    assert_eq!(fetched.fetch_report().lookup_key_count(), 1);
    assert_eq!(fetched.fetch_report().rows_read(), 1);
    assert_eq!(fetched.fetch_report().global_scan_count(), 0);
    assert!(!fetched.fetch_report().access_structure_debt());
    assert_eq!(store.subscription_support_counters().lookup_keys_used(), 1);
    assert_eq!(store.subscription_support_counters().rows_read(), 1);
}

#[test]
fn local_file_subscription_support_reopen_preserves_identity_and_digest() {
    let path = unique_test_store_path("worth-store-subscription-support-local");
    let (artifact_id, artifact_digest) = {
        let mut store = WORTHStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        let admitted = store
            .admit_subscription_support_declaration(raw_exact())
            .unwrap();
        let publishable = store
            .subscription_support_pipeline()
            .prepare_exact(
                admitted,
                "basis:1",
                "cursor:1",
                "checkpoint:1",
                "schema:1",
                "compatibility:1",
            )
            .unwrap();
        let published = store.publish_subscription_support(publishable).unwrap();
        (
            published.artifact_id().clone(),
            published.artifact_digest().to_string(),
        )
    };

    let mut reopened = WORTHStoreBuilder::new().local_file(path).build().unwrap();
    let fetched = reopened
        .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            artifact_id,
        ))
        .unwrap();

    assert_eq!(fetched.record_set().artifact_digest(), artifact_digest);
}

#[test]
fn sqlite_subscription_support_reopen_preserves_identity_and_digest() {
    let path = unique_test_sqlite_path("worth-store-subscription-support-sqlite");
    let (artifact_id, artifact_digest) = {
        let mut store = WORTHStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        let admitted = store
            .admit_subscription_support_declaration(raw_exact())
            .unwrap();
        let publishable = store
            .subscription_support_pipeline()
            .prepare_exact(
                admitted,
                "basis:1",
                "cursor:1",
                "checkpoint:1",
                "schema:1",
                "compatibility:1",
            )
            .unwrap();
        let published = store.publish_subscription_support(publishable).unwrap();
        (
            published.artifact_id().clone(),
            published.artifact_digest().to_string(),
        )
    };

    let mut reopened = WORTHStoreBuilder::new().sqlite_file(path).build().unwrap();
    let fetched = reopened
        .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            artifact_id,
        ))
        .unwrap();

    assert_eq!(fetched.record_set().artifact_digest(), artifact_digest);
}

#[test]
fn sqlite_subscription_support_reopen_classifies_exact_resume_from_fetched_evidence() {
    let path = unique_test_sqlite_path("worth-store-subscription-support-sqlite-classify-exact");
    let artifact_id = {
        let mut store = WORTHStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        let admitted = store
            .admit_subscription_support_declaration(raw_exact())
            .unwrap();
        let publishable = store
            .subscription_support_pipeline()
            .prepare_exact(
                admitted,
                "basis:1",
                "cursor:1",
                "checkpoint:1",
                "schema:1",
                "compatibility:1",
            )
            .unwrap();
        store
            .publish_subscription_support(publishable)
            .unwrap()
            .artifact_id()
            .clone()
    };

    let mut reopened = WORTHStoreBuilder::new().sqlite_file(path).build().unwrap();
    let fetched = reopened
        .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            artifact_id,
        ))
        .unwrap();
    let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, true).unwrap();
    let report = reopened
        .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
            fetched,
            evidence,
            crate::SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
        ))
        .unwrap();

    assert_eq!(
        report.classification(),
        SubscriptionResumeClassification::Exact
    );
    assert_eq!(report.primary_cause(), None);
    assert_eq!(report.cost_surface().decoded_payload_bytes(), 128);
    assert_eq!(report.cost_surface().scanned_support_rows(), 1);
    assert_eq!(
        reopened
            .subscription_support_counters()
            .exact_classifications(),
        1
    );
}

#[test]
fn sqlite_subscription_support_restart_reconstruction_is_shard_bounded() {
    let path = unique_test_sqlite_path("worth-store-subscription-support-restart-shard");
    let artifact_id = {
        let mut store = WORTHStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        let admitted = store
            .admit_subscription_support_declaration(raw_exact())
            .unwrap();
        let publishable = store
            .subscription_support_pipeline()
            .prepare_exact(
                admitted,
                "basis:1",
                "cursor:1",
                "checkpoint:1",
                "schema:1",
                "compatibility:1",
            )
            .unwrap();
        store
            .publish_subscription_support(publishable)
            .unwrap()
            .artifact_id()
            .clone()
    };

    let mut reopened = WORTHStoreBuilder::new().sqlite_file(path).build().unwrap();
    let report = reopened
        .reconstruct_subscription_support_restart_shard(
            SubscriptionSupportRestartReconstructionRequest::new(
                SubscriptionSupportRestartShard::for_family(
                    SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                    SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                ),
                8,
            )
            .unwrap(),
        )
        .unwrap();

    assert_eq!(report.support_rows_read(), 1);
    assert_eq!(report.restart_shards_touched(), 1);
    assert_eq!(report.global_scan_count(), 0);
    assert_eq!(report.reports().len(), 1);
    assert_eq!(report.reports()[0].artifact_id(), &artifact_id);
    assert_eq!(
        report.reports()[0].classification(),
        SubscriptionResumeClassification::Exact
    );
    assert_eq!(
        report.reports()[0].cost_surface().restart_shards_touched(),
        1
    );
    assert_eq!(
        report.reports()[0].cost_surface().decoded_payload_bytes(),
        0
    );
    assert_eq!(
        reopened
            .subscription_support_counters()
            .restart_reconstruction_count(),
        1
    );
    assert_eq!(
        reopened
            .subscription_support_counters()
            .restart_global_scan_count(),
        0
    );
}

#[test]
fn subscription_support_restart_reconstruction_rejects_unbounded_shard_work() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    for basis in ["basis:1", "basis:2"] {
        let admitted = store
            .admit_subscription_support_declaration(raw_exact())
            .unwrap();
        let publishable = store
            .subscription_support_pipeline()
            .prepare_exact(
                admitted,
                basis,
                format!("cursor:{basis}"),
                format!("checkpoint:{basis}"),
                "schema:1",
                "compatibility:1",
            )
            .unwrap();
        store.publish_subscription_support(publishable).unwrap();
    }

    let error = store
        .reconstruct_subscription_support_restart_shard(
            SubscriptionSupportRestartReconstructionRequest::new(
                SubscriptionSupportRestartShard::for_family(
                    SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                    SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                ),
                1,
            )
            .unwrap(),
        )
        .expect_err("restart reconstruction must reject shards over the admitted row bound");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .restart_reconstruction_count(),
        0
    );
}

#[test]
fn subscription_support_restart_reconstruction_rejects_family_kind_mismatch() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let admitted = store
        .admit_subscription_support_declaration(raw_exact())
        .unwrap();
    let publishable = store
        .subscription_support_pipeline()
        .prepare_exact(
            admitted,
            "basis:1",
            "cursor:1",
            "checkpoint:1",
            "schema:1",
            "compatibility:1",
        )
        .unwrap();
    store.publish_subscription_support(publishable).unwrap();

    let error = store
        .reconstruct_subscription_support_restart_shard(
            SubscriptionSupportRestartReconstructionRequest::new(
                SubscriptionSupportRestartShard::for_family(
                    SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                    SubscriptionSupportFamilyKind::DegradedContinuationSupport,
                ),
                8,
            )
            .unwrap(),
        )
        .expect_err("restart shard proof must include the admitted family kind");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
}

#[test]
fn subscription_support_missing_materialized_support_requires_retained_rebuild_basis() {
    let artifact_id = {
        let mut source = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let admitted = source
            .admit_subscription_support_declaration(raw_materialized())
            .unwrap();
        let publishable = source
            .subscription_support_pipeline()
            .prepare_exact(
                admitted,
                "basis:retained",
                "cursor:1",
                "checkpoint:1",
                "schema:1",
                "compatibility:1",
            )
            .unwrap();
        source
            .publish_subscription_support(publishable)
            .unwrap()
            .artifact_id()
            .clone()
    };
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let rebuild = store
        .classify_missing_subscription_support(
            SubscriptionSupportMissingSupportRecoveryRequest::new(
                SubscriptionSupportFamilyId::new("materialized-narrowing-support").unwrap(),
                SubscriptionSupportFamilyKind::MaterializedNarrowingSupport,
                SubscriptionSupportRole::NarrowingMaterialization,
                artifact_id.clone(),
                "basis:retained",
                "cursor:1",
                "checkpoint:1",
                "compatibility:1",
                "portability:1",
            )
            .unwrap()
            .with_rebuild_maintenance_admission(
                "basis:retained",
                SubscriptionSupportMissingSupportMaintenanceAdmission::new(
                    SupportActionId::new("support-maintenance:missing-recovery").unwrap(),
                    SupportActionBreadthBudget::new(1, 1024).unwrap(),
                    128,
                )
                .unwrap(),
            )
            .unwrap(),
        )
        .unwrap();
    assert_eq!(
        rebuild.classification(),
        SubscriptionResumeClassification::RebuildRequired
    );
    let maintenance_report = rebuild
        .maintenance_report()
        .expect("rebuildable missing support must admit maintenance work");
    assert_eq!(
        maintenance_report.participation_record().decision_kind(),
        crate::SubscriptionSupportMaintenanceDecisionKind::RebuildDescriptorAdmitted
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .rebuild_basis_plan_count(),
        1
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_maintenance_rebuild_debt_count(),
        1
    );

    let denied = store
        .classify_missing_subscription_support(
            SubscriptionSupportMissingSupportRecoveryRequest::new(
                SubscriptionSupportFamilyId::new("materialized-narrowing-support").unwrap(),
                SubscriptionSupportFamilyKind::MaterializedNarrowingSupport,
                SubscriptionSupportRole::NarrowingMaterialization,
                artifact_id,
                "basis:retained",
                "cursor:1",
                "checkpoint:1",
                "compatibility:1",
                "portability:1",
            )
            .unwrap(),
        )
        .unwrap();
    assert_eq!(
        denied.classification(),
        SubscriptionResumeClassification::NotResumable
    );
}

#[test]
fn subscription_support_missing_recovery_requires_cursor_and_checkpoint_evidence() {
    let artifact_id = {
        let mut source = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let admitted = source
            .admit_subscription_support_declaration(raw_materialized())
            .unwrap();
        let publishable = source
            .subscription_support_pipeline()
            .prepare_exact(
                admitted,
                "basis:retained",
                "cursor:1",
                "checkpoint:1",
                "schema:1",
                "compatibility:1",
            )
            .unwrap();
        source
            .publish_subscription_support(publishable)
            .unwrap()
            .artifact_id()
            .clone()
    };

    let error = SubscriptionSupportMissingSupportRecoveryRequest::new(
        SubscriptionSupportFamilyId::new("materialized-narrowing-support").unwrap(),
        SubscriptionSupportFamilyKind::MaterializedNarrowingSupport,
        SubscriptionSupportRole::NarrowingMaterialization,
        artifact_id,
        "basis:retained",
        "",
        "checkpoint:1",
        "compatibility:1",
        "portability:1",
    )
    .expect_err("missing-support recovery must not omit retained cursor evidence");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
}

#[test]
fn subscription_support_resume_classification_localizes_multi_drift_precedence() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let admitted = store
        .admit_subscription_support_declaration(raw_exact())
        .unwrap();
    let publishable = store
        .subscription_support_pipeline()
        .prepare_exact(
            admitted,
            "basis:1",
            "cursor:1",
            "checkpoint:1",
            "schema:1",
            "compatibility:1",
        )
        .unwrap();
    let published = store.publish_subscription_support(publishable).unwrap();
    let fetched = store
        .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            published.artifact_id().clone(),
        ))
        .unwrap();
    let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, true)
        .unwrap()
        .with_basis_digest("basis:drift")
        .unwrap()
        .with_cursor_digest("cursor:drift")
        .unwrap()
        .with_support_artifact_digest("artifact:drift")
        .unwrap();

    let report = store
        .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
            fetched,
            evidence,
            crate::SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
        ))
        .unwrap();

    assert_eq!(
        report.classification(),
        SubscriptionResumeClassification::NotResumable
    );
    assert_eq!(
        report.primary_cause(),
        Some(SubscriptionSupportDriftCause::SubscriptionSupportBasisDrift)
    );
    assert_eq!(
        report.suppressed_causes(),
        &[
            SubscriptionSupportDriftCause::SubscriptionSupportCursorDrift,
            SubscriptionSupportDriftCause::SubscriptionSupportDigestMismatch,
        ]
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .denied_classifications(),
        1
    );
}

#[test]
fn subscription_support_resume_classification_distinguishes_checkpoint_schema_and_compatibility() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let admitted = store
        .admit_subscription_support_declaration(raw_exact())
        .unwrap();
    let publishable = store
        .subscription_support_pipeline()
        .prepare_exact(
            admitted,
            "basis:1",
            "cursor:1",
            "checkpoint:1",
            "schema:1",
            "compatibility:1",
        )
        .unwrap();
    let published = store.publish_subscription_support(publishable).unwrap();
    let fetched = store
        .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            published.artifact_id().clone(),
        ))
        .unwrap();
    let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, true)
        .unwrap()
        .with_checkpoint_digest("checkpoint:drift")
        .unwrap()
        .with_schema_digest("schema:drift")
        .unwrap()
        .with_compatibility_digest("compatibility:drift")
        .unwrap();

    let report = store
        .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
            fetched,
            evidence,
            crate::SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
        ))
        .unwrap();

    assert_eq!(
        report.classification(),
        SubscriptionResumeClassification::NotResumable
    );
    assert_eq!(
        report.primary_cause(),
        Some(SubscriptionSupportDriftCause::SubscriptionSupportCompatibilityDrift)
    );
    assert_eq!(
        report.suppressed_causes(),
        &[
            SubscriptionSupportDriftCause::SubscriptionSupportSchemaDrift,
            SubscriptionSupportDriftCause::SubscriptionSupportCheckpointDrift,
        ]
    );
}

#[test]
fn subscription_support_digest_drift_classifies_rebuild_required_only_with_rebuild_plan() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let admitted = store
        .admit_subscription_support_declaration(raw_materialized())
        .unwrap();
    let publishable = store
        .subscription_support_pipeline()
        .prepare_exact(
            admitted,
            "basis:1",
            "cursor:1",
            "checkpoint:1",
            "schema:1",
            "compatibility:1",
        )
        .unwrap();
    let published = store.publish_subscription_support(publishable).unwrap();
    let fetched = store
        .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
            SubscriptionSupportFamilyId::new("materialized-narrowing-support").unwrap(),
            SubscriptionSupportFamilyKind::MaterializedNarrowingSupport,
            published.artifact_id().clone(),
        ))
        .unwrap();
    let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 256, true)
        .unwrap()
        .with_support_artifact_digest("artifact:stale")
        .unwrap()
        .with_retained_rebuild_basis_digest("basis:1")
        .unwrap();
    let plan = crate::SubscriptionSupportClassificationPlan::new(
        SubscriptionSupportPlanFamily::RebuildPlanClassificationPlan,
        SubscriptionSupportPayloadBudget::new(16 * 1024, 64).unwrap(),
        SubscriptionSupportAllocationScope::FamilyLocalScratch,
        SubscriptionSupportDensityClass::FamilyBatchClassificationDebt,
        None,
    )
    .unwrap();

    let report = store
        .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
            fetched, evidence, plan,
        ))
        .unwrap();

    assert_eq!(
        report.classification(),
        SubscriptionResumeClassification::RebuildRequired
    );
    assert_eq!(
        report.primary_cause(),
        Some(SubscriptionSupportDriftCause::SubscriptionSupportDigestMismatch)
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .rebuild_required_classifications(),
        1
    );
}

#[test]
fn subscription_support_digest_drift_without_retained_rebuild_basis_is_not_resumable() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let admitted = store
        .admit_subscription_support_declaration(raw_materialized())
        .unwrap();
    let publishable = store
        .subscription_support_pipeline()
        .prepare_exact(
            admitted,
            "basis:1",
            "cursor:1",
            "checkpoint:1",
            "schema:1",
            "compatibility:1",
        )
        .unwrap();
    let published = store.publish_subscription_support(publishable).unwrap();
    let fetched = store
        .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
            SubscriptionSupportFamilyId::new("materialized-narrowing-support").unwrap(),
            SubscriptionSupportFamilyKind::MaterializedNarrowingSupport,
            published.artifact_id().clone(),
        ))
        .unwrap();
    let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 256, true)
        .unwrap()
        .with_support_artifact_digest("artifact:stale")
        .unwrap();
    let plan = crate::SubscriptionSupportClassificationPlan::new(
        SubscriptionSupportPlanFamily::RebuildPlanClassificationPlan,
        SubscriptionSupportPayloadBudget::new(16 * 1024, 64).unwrap(),
        SubscriptionSupportAllocationScope::FamilyLocalScratch,
        SubscriptionSupportDensityClass::FamilyBatchClassificationDebt,
        None,
    )
    .unwrap();

    let report = store
        .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
            fetched, evidence, plan,
        ))
        .unwrap();

    assert_eq!(
        report.classification(),
        SubscriptionResumeClassification::NotResumable
    );
    assert_eq!(
        report.primary_cause(),
        Some(SubscriptionSupportDriftCause::SubscriptionSupportDigestMismatch)
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .denied_classifications(),
        1
    );
}

#[test]
fn subscription_support_resume_distinguishes_degraded_and_session_memory_loss() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let admitted = store
        .admit_subscription_support_declaration(raw_degraded())
        .unwrap();
    let publishable = store
        .subscription_support_pipeline()
        .prepare_exact(
            admitted,
            "basis:1",
            "cursor:1",
            "checkpoint:1",
            "schema:1",
            "compatibility:1",
        )
        .unwrap();
    let published = store.publish_subscription_support(publishable).unwrap();
    let fetched = store
        .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
            SubscriptionSupportFamilyId::new("degraded-continuation-support").unwrap(),
            SubscriptionSupportFamilyKind::DegradedContinuationSupport,
            published.artifact_id().clone(),
        ))
        .unwrap();
    let degraded_plan = crate::SubscriptionSupportClassificationPlan::new(
        SubscriptionSupportPlanFamily::DegradedResumeClassificationPlan,
        SubscriptionSupportPayloadBudget::new(16 * 1024, 64).unwrap(),
        SubscriptionSupportAllocationScope::RestartShardBatch,
        SubscriptionSupportDensityClass::RestartShardBatchClassification,
        Some("restart-shard-a".into()),
    )
    .unwrap();
    let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 64, true).unwrap();
    let degraded = store
        .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
            fetched.clone(),
            evidence,
            degraded_plan.clone(),
        ))
        .unwrap();
    assert_eq!(
        degraded.classification(),
        SubscriptionResumeClassification::Degraded
    );

    let session_loss = SubscriptionSupportResumeEvidence::matching(&fetched, 64, false).unwrap();
    let denied = store
        .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
            fetched,
            session_loss,
            degraded_plan,
        ))
        .unwrap();

    assert_eq!(
        denied.classification(),
        SubscriptionResumeClassification::NotResumable
    );
    assert_eq!(
        denied.primary_cause(),
        Some(SubscriptionSupportDriftCause::SubscriptionSupportSessionMemoryMissing)
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .degraded_classifications(),
        1
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .denied_classifications(),
        1
    );
}

#[test]
fn subscription_support_resume_rejects_cross_family_evidence() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let admitted = store
        .admit_subscription_support_declaration(raw_exact())
        .unwrap();
    let publishable = store
        .subscription_support_pipeline()
        .prepare_exact(
            admitted,
            "basis:1",
            "cursor:1",
            "checkpoint:1",
            "schema:1",
            "compatibility:1",
        )
        .unwrap();
    let published = store.publish_subscription_support(publishable).unwrap();
    let fetched = store
        .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            published.artifact_id().clone(),
        ))
        .unwrap();
    let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, true)
        .unwrap()
        .with_expected_family_kind(SubscriptionSupportFamilyKind::DegradedContinuationSupport);

    let report = store
        .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
            fetched,
            evidence,
            crate::SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
        ))
        .unwrap();

    assert_eq!(
        report.classification(),
        SubscriptionResumeClassification::NotResumable
    );
    assert_eq!(
        report.primary_cause(),
        Some(SubscriptionSupportDriftCause::SubscriptionSupportFamilyMismatch)
    );
}

#[test]
fn sqlite_subscription_support_legacy_rows_backfill_index_projections() {
    let legacy_path = unique_test_sqlite_path("worth-store-subscription-support-legacy-indexes");
    let record_set = {
        let mut source = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let admitted = source
            .admit_subscription_support_declaration(raw_exact())
            .unwrap();
        let publishable = source
            .subscription_support_pipeline()
            .prepare_exact(
                admitted,
                "basis:1",
                "cursor:1",
                "checkpoint:1",
                "schema:1",
                "compatibility:1",
            )
            .unwrap();
        let published = source.publish_subscription_support(publishable).unwrap();
        source
            .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
                SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                published.artifact_id().clone(),
            ))
            .unwrap()
            .record_set()
            .clone()
    };

    let connection = rusqlite::Connection::open(&legacy_path).unwrap();
    connection
        .execute_batch(
            "
            CREATE TABLE subscription_support_record_sets (
                storage_key TEXT PRIMARY KEY,
                family_id TEXT NOT NULL,
                artifact_id TEXT NOT NULL,
                payload_json TEXT NOT NULL
            );
            ",
        )
        .unwrap();
    connection
        .execute(
            "INSERT INTO subscription_support_record_sets \
             (storage_key, family_id, artifact_id, payload_json) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                record_set.key().storage_key(),
                record_set.key().family_id(),
                record_set.key().artifact_id(),
                serde_json::to_string(&record_set).unwrap(),
            ],
        )
        .unwrap();
    drop(connection);

    let reopened = WORTHStoreBuilder::new()
        .sqlite_file(legacy_path.clone())
        .build()
        .unwrap();
    assert!(reopened
        .subscription_support_access_structure_report()
        .has_debt());

    let connection = rusqlite::Connection::open(&legacy_path).unwrap();
    let basis_digest: String = connection
        .query_row(
            "SELECT basis_digest FROM subscription_support_record_sets",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(basis_digest, "basis:1");
}

#[test]
fn local_file_subscription_support_digest_drift_fails_open() {
    let path = unique_test_store_path("worth-store-subscription-support-drift");
    {
        let mut store = WORTHStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        let admitted = store
            .admit_subscription_support_declaration(raw_exact())
            .unwrap();
        let publishable = store
            .subscription_support_pipeline()
            .prepare_exact(
                admitted,
                "basis:1",
                "cursor:1",
                "checkpoint:1",
                "schema:1",
                "compatibility:1",
            )
            .unwrap();
        store.publish_subscription_support(publishable).unwrap();
    }

    let raw = std::fs::read_to_string(&path).unwrap();
    let mut payload: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let record_sets = payload
        .get_mut("subscription_support_record_sets")
        .and_then(serde_json::Value::as_object_mut)
        .expect("subscription support record set should persist");
    let first_record = record_sets
        .values_mut()
        .next()
        .expect("one subscription support record set should persist");
    first_record["artifact_digest"] = serde_json::Value::String(String::new());
    std::fs::write(&path, serde_json::to_string_pretty(&payload).unwrap()).unwrap();

    let error = WORTHStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err("durable subscription-support digest drift should fail open");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportPublicationViolation
    );
}

#[test]
fn sqlite_subscription_support_linkage_gap_fails_open() {
    let path = unique_test_sqlite_path("worth-store-subscription-support-linkage-gap");
    {
        let mut store = WORTHStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        let admitted = store
            .admit_subscription_support_declaration(raw_exact())
            .unwrap();
        let publishable = store
            .subscription_support_pipeline()
            .prepare_exact(
                admitted,
                "basis:1",
                "cursor:1",
                "checkpoint:1",
                "schema:1",
                "compatibility:1",
            )
            .unwrap();
        store.publish_subscription_support(publishable).unwrap();
    }

    let connection = rusqlite::Connection::open(&path).unwrap();
    connection
        .execute("DELETE FROM subscription_support_record_sets", [])
        .unwrap();

    let error = WORTHStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err("missing durable support rows should fail open");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportPublicationViolation
    );
}

#[test]
fn sqlite_subscription_support_index_projection_drift_fails_open() {
    let path = unique_test_sqlite_path("worth-store-subscription-support-index-drift");
    {
        let mut store = WORTHStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        let admitted = store
            .admit_subscription_support_declaration(raw_exact())
            .unwrap();
        let publishable = store
            .subscription_support_pipeline()
            .prepare_exact(
                admitted,
                "basis:1",
                "cursor:1",
                "checkpoint:1",
                "schema:1",
                "compatibility:1",
            )
            .unwrap();
        store.publish_subscription_support(publishable).unwrap();
    }

    let connection = rusqlite::Connection::open(&path).unwrap();
    connection
        .execute(
            "UPDATE subscription_support_record_sets SET basis_digest = 'basis:index-drift'",
            [],
        )
        .unwrap();
    drop(connection);

    let error = WORTHStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err("indexed support projections must not drift from the payload");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportPublicationViolation
    );
}

#[test]
fn sqlite_subscription_support_restart_shard_projection_drift_fails_open() {
    let path = unique_test_sqlite_path("worth-store-subscription-support-restart-shard-drift");
    {
        let mut store = WORTHStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        let admitted = store
            .admit_subscription_support_declaration(raw_exact())
            .unwrap();
        let publishable = store
            .subscription_support_pipeline()
            .prepare_exact(
                admitted,
                "basis:1",
                "cursor:1",
                "checkpoint:1",
                "schema:1",
                "compatibility:1",
            )
            .unwrap();
        store.publish_subscription_support(publishable).unwrap();
    }

    let connection = rusqlite::Connection::open(&path).unwrap();
    connection
        .execute(
            "UPDATE subscription_support_record_sets SET restart_shard = 'restart:wrong'",
            [],
        )
        .unwrap();
    drop(connection);

    let error = WORTHStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err("restart-shard projection drift must fail open");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportPublicationViolation
    );
}

#[test]
fn duplicate_subscription_support_publication_rejects_durable_identity_collision() {
    let path = unique_test_store_path("worth-store-subscription-support-collision");
    {
        let mut store = WORTHStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        let admitted = store
            .admit_subscription_support_declaration(raw_exact())
            .unwrap();
        let publishable = store
            .subscription_support_pipeline()
            .prepare_exact(
                admitted,
                "basis:1",
                "cursor:1",
                "checkpoint:1",
                "schema:1",
                "compatibility:1",
            )
            .unwrap();
        store.publish_subscription_support(publishable).unwrap();
    }

    let raw = std::fs::read_to_string(&path).unwrap();
    let mut payload: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let record_sets = payload
        .get_mut("subscription_support_record_sets")
        .and_then(serde_json::Value::as_object_mut)
        .expect("subscription support record set should persist");
    let first_record = record_sets
        .values_mut()
        .next()
        .expect("one subscription support record set should persist");
    first_record["artifact_digest"] = serde_json::Value::String("collision-digest".into());
    std::fs::write(&path, serde_json::to_string_pretty(&payload).unwrap()).unwrap();

    let mut reopened = WORTHStoreBuilder::new().local_file(path).build().unwrap();
    let admitted = reopened
        .admit_subscription_support_declaration(raw_exact())
        .unwrap();
    let publishable = reopened
        .subscription_support_pipeline()
        .prepare_exact(
            admitted,
            "basis:1",
            "cursor:1",
            "checkpoint:1",
            "schema:1",
            "compatibility:1",
        )
        .unwrap();

    let error = reopened
        .publish_subscription_support(publishable)
        .expect_err("same durable identity with different projection must reject");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportPublicationViolation
    );
    assert_eq!(
        reopened
            .subscription_support_counters()
            .identity_collisions(),
        1
    );
}

#[test]
fn sqlite_subscription_support_access_structure_debt_is_typed() {
    let path = unique_test_sqlite_path("worth-store-subscription-support-access-debt");
    let artifact_id = {
        let mut store = WORTHStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        let admitted = store
            .admit_subscription_support_declaration(raw_exact())
            .unwrap();
        let publishable = store
            .subscription_support_pipeline()
            .prepare_exact(
                admitted,
                "basis:1",
                "cursor:1",
                "checkpoint:1",
                "schema:1",
                "compatibility:1",
            )
            .unwrap();
        store
            .publish_subscription_support(publishable)
            .unwrap()
            .artifact_id()
            .clone()
    };

    let connection = rusqlite::Connection::open(&path).unwrap();
    connection
        .execute(
            "UPDATE subscription_support_access_structure_state SET verified = 0 WHERE state_id = 'first_ship'",
            [],
        )
        .unwrap();

    let mut reopened = WORTHStoreBuilder::new().sqlite_file(path).build().unwrap();
    let report = reopened.subscription_support_access_structure_report();
    assert!(report.has_debt());
    assert_eq!(report.debted(), report.required());

    let error = reopened
        .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            artifact_id,
        ))
        .expect_err("access-structure debt must not fall back to a global scan");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportPublicationViolation
    );
    assert_eq!(
        reopened
            .subscription_support_counters()
            .access_structure_debts(),
        1
    );
}

#[test]
fn sqlite_subscription_support_missing_lookup_index_marks_access_debt() {
    let path = unique_test_sqlite_path("worth-store-subscription-support-missing-index");
    let artifact_id = {
        let mut store = WORTHStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        let admitted = store
            .admit_subscription_support_declaration(raw_exact())
            .unwrap();
        let publishable = store
            .subscription_support_pipeline()
            .prepare_exact(
                admitted,
                "basis:1",
                "cursor:1",
                "checkpoint:1",
                "schema:1",
                "compatibility:1",
            )
            .unwrap();
        store
            .publish_subscription_support(publishable)
            .unwrap()
            .artifact_id()
            .clone()
    };

    let connection = rusqlite::Connection::open(&path).unwrap();
    connection
        .execute("DROP INDEX idx_subscription_support_family_artifact", [])
        .unwrap();
    drop(connection);

    let mut reopened = WORTHStoreBuilder::new().sqlite_file(path).build().unwrap();
    let report = reopened.subscription_support_access_structure_report();
    assert!(report.has_debt());
    assert_eq!(
        report.debted(),
        &[SubscriptionSupportAccessStructure::ArtifactLookupByFamilyAndArtifact]
    );

    let error = reopened
        .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            artifact_id,
        ))
        .expect_err("missing lookup index must be remembered as access debt");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportPublicationViolation
    );
    assert_eq!(
        reopened
            .subscription_support_counters()
            .access_structure_debts(),
        1
    );
}
