use crate::tests::harness::fixtures::stores::unique_test_sqlite_path;
use crate::{
    ArtifactCompatibilityWindow, ArtifactFamilyId, ArtifactFormatVersion, ArtifactSemanticVersion,
    CompatibilityAdmissionCounters, CompatibilityAdmissionPath, CompatibilityAdmissionReceipt,
    CompatibilityManifestDigest, CompatibilityReadAdmissionOutcome, CompatibilityRejection,
    CompatibilityRejectionKind, CompatibilityRelation, ForgeStore, ForgeStoreBuilder,
    QuarantinedDecodedArtifact, RawSubscriptionSupportDeclaration, ReadCompatibilityReceipt,
    StoreErrorKind, SubscriptionResumeClassification, SubscriptionSupportAllocationScope,
    SubscriptionSupportArtifactId, SubscriptionSupportAuthority, SubscriptionSupportCatalog,
    SubscriptionSupportCertificationBundle, SubscriptionSupportCertificationLaneKind,
    SubscriptionSupportCertificationLaneOutcome, SubscriptionSupportCertificationMatrixStatus,
    SubscriptionSupportClassificationPlan, SubscriptionSupportClassificationReport,
    SubscriptionSupportCompatibilityDecision, SubscriptionSupportCounterSnapshot,
    SubscriptionSupportDensityClass, SubscriptionSupportDriftCause, SubscriptionSupportFamilyId,
    SubscriptionSupportFamilyKind, SubscriptionSupportFetchRequest,
    SubscriptionSupportMaintenanceDecision, SubscriptionSupportMissingSupportMaintenanceAdmission,
    SubscriptionSupportMissingSupportRecoveryRequest, SubscriptionSupportPayloadBudget,
    SubscriptionSupportPayloadDigest, SubscriptionSupportPlanFamily,
    SubscriptionSupportPortabilityDecision, SubscriptionSupportRestartReconstructionRequest,
    SubscriptionSupportRestartShard, SubscriptionSupportResumeEvidence,
    SubscriptionSupportResumeRequest, SubscriptionSupportRetentionDecision,
    SubscriptionSupportRole, SubscriptionSupportRuntimeHandoffRequest, SubscriptionSupportScope,
    SupportActionBreadthBudget, SupportActionId, SupportAllocationScope,
    SupportCompatibilityReceiptWitness, SupportFamilyVersionWindow, SupportPathClass,
    SupportPortabilityManifestBudget, SupportProgramDensityClass,
};

fn raw_exact() -> RawSubscriptionSupportDeclaration {
    RawSubscriptionSupportDeclaration::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportAuthority::ForgeQuery,
        "forge-query-live-v1",
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
        SubscriptionSupportAuthority::ForgeQuery,
        "forge-query-live-v1",
        SubscriptionSupportScope::from_canonical(vec!["feed:2".into()]).unwrap(),
        SubscriptionSupportPayloadDigest::new("payload:def").unwrap(),
    )
}

fn raw_materialized() -> RawSubscriptionSupportDeclaration {
    RawSubscriptionSupportDeclaration::new(
        SubscriptionSupportFamilyId::new("materialized-narrowing-support").unwrap(),
        SubscriptionSupportFamilyKind::MaterializedNarrowingSupport,
        SubscriptionSupportRole::NarrowingMaterialization,
        SubscriptionSupportAuthority::ForgeRuntimeBridge,
        "forge-runtime-bridge-v1",
        SubscriptionSupportScope::from_canonical(vec!["account:1".into(), "narrow:active".into()])
            .unwrap(),
        SubscriptionSupportPayloadDigest::new("payload:materialized").unwrap(),
    )
}

fn retention_basis(artifact_suffix: &str) -> crate::SubscriptionSupportOperationalBasis {
    crate::SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId(format!("artifact:cert-retention:{artifact_suffix}")),
        "basis:cert-retention",
        "cursor:cert-retention",
        "checkpoint:cert-retention",
        "compatibility:cert-retention",
        "portability:cert-retention",
        crate::SubscriptionSupportActionOrigin::Retention,
    )
    .unwrap()
}

fn compatibility_basis(artifact_suffix: &str) -> crate::SubscriptionSupportOperationalBasis {
    crate::SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId(format!("artifact:cert-compatibility:{artifact_suffix}")),
        "basis:cert-compatibility",
        "cursor:cert-compatibility",
        "checkpoint:cert-compatibility",
        "compatibility:manifest-v2",
        "portability:cert-compatibility",
        crate::SubscriptionSupportActionOrigin::Compatibility,
    )
    .unwrap()
}

fn portability_basis(
    action_origin: crate::SubscriptionSupportActionOrigin,
    artifact_suffix: &str,
) -> crate::SubscriptionSupportOperationalBasis {
    crate::SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId(format!("artifact:cert-portability:{artifact_suffix}")),
        "basis:cert-portability",
        "cursor:cert-portability",
        "checkpoint:cert-portability",
        "compatibility:cert-portability",
        format!("portability:cert-portability:{artifact_suffix}"),
        action_origin,
    )
    .unwrap()
}

fn maintenance_basis(artifact_suffix: &str) -> crate::SubscriptionSupportOperationalBasis {
    crate::SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId(format!("artifact:cert-maintenance:{artifact_suffix}")),
        format!("basis:cert-maintenance:{artifact_suffix}"),
        "cursor:cert-maintenance",
        "checkpoint:cert-maintenance",
        "compatibility:cert-maintenance",
        "portability:cert-maintenance",
        crate::SubscriptionSupportActionOrigin::Maintenance,
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

fn read_receipt_witness(relation: CompatibilityRelation) -> SupportCompatibilityReceiptWitness {
    let family_id = ArtifactFamilyId::new("basis-bound-continuation-support");
    let manifest_digest = compatibility_manifest_digest(&family_id);
    let receipt = ReadCompatibilityReceipt::new(CompatibilityAdmissionReceipt::new(
        family_id,
        manifest_digest,
        "support-registry:snapshot",
        "support-manifest:frontier",
        ArtifactSemanticVersion::new(1),
        ArtifactSemanticVersion::new(1),
        CompatibilityAdmissionPath::BatchRead,
        relation,
    ));
    SupportCompatibilityReceiptWitness::from_read_receipt(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        &receipt,
    )
    .unwrap()
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
        "structural:cert-support-compatibility",
        "support compatibility rejection fixture",
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

fn publish_exact(
    store: &mut ForgeStore,
    basis: &str,
    cursor: &str,
    checkpoint: &str,
) -> SubscriptionSupportArtifactId {
    let admitted = store
        .admit_subscription_support_declaration(raw_exact())
        .unwrap();
    let publishable = store
        .subscription_support_pipeline()
        .prepare_exact(
            admitted,
            basis,
            cursor,
            checkpoint,
            "schema:1",
            "compatibility:1",
        )
        .unwrap();
    store
        .publish_subscription_support(publishable)
        .unwrap()
        .artifact_id()
        .clone()
}

fn fetched_exact_report(store: &mut ForgeStore) -> SubscriptionSupportClassificationReport {
    let artifact_id = publish_exact(store, "basis:1", "cursor:1", "checkpoint:1");
    let fetched = store
        .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            artifact_id,
        ))
        .unwrap();
    let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, true).unwrap();
    store
        .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
            fetched,
            evidence,
            SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
        ))
        .unwrap()
}

#[test]
fn durable_subscription_support_resume_contract_phase_6a_matrix_is_machine_checkable() {
    let mut classification_reports = Vec::new();
    let mut lane_outcomes = Vec::new();

    let exact_report =
        fetched_exact_report(&mut ForgeStoreBuilder::new().in_memory().build().unwrap());
    assert_eq!(
        exact_report.classification(),
        SubscriptionResumeClassification::Exact
    );
    assert_eq!(exact_report.cost_surface().decoded_payload_bytes(), 128);
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::ExactResumeControl,
            &exact_report,
        )
        .unwrap(),
    );
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::ResultCostSurfaceExact,
            &exact_report,
        )
        .unwrap(),
    );
    classification_reports.push(exact_report);

    let path = unique_test_sqlite_path("forge-store-subscription-support-certification-restart");
    let artifact_id = {
        let mut store = ForgeStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        publish_exact(
            &mut store,
            "basis:restart",
            "cursor:restart",
            "checkpoint:restart",
        )
    };
    let mut reopened = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    let restart_report = reopened
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
    assert_eq!(restart_report.reports().len(), 1);
    assert_eq!(restart_report.reports()[0].artifact_id(), &artifact_id);
    assert_eq!(restart_report.global_scan_count(), 0);
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::RestartExactResume,
            &restart_report.reports()[0],
        )
        .unwrap(),
    );
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::RestartShardBoundedReconstruction,
            &restart_report.reports()[0],
        )
        .unwrap(),
    );
    classification_reports.push(restart_report.reports()[0].clone());

    let mut rebuild_store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let missing_artifact_id = {
        let mut source = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let admitted = source
            .admit_subscription_support_declaration(raw_materialized())
            .unwrap();
        let publishable = source
            .subscription_support_pipeline()
            .prepare_exact(
                admitted,
                "basis:retained",
                "cursor:retained",
                "checkpoint:retained",
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
    let rebuild_report = rebuild_store
        .classify_missing_subscription_support(
            SubscriptionSupportMissingSupportRecoveryRequest::new(
                SubscriptionSupportFamilyId::new("materialized-narrowing-support").unwrap(),
                SubscriptionSupportFamilyKind::MaterializedNarrowingSupport,
                SubscriptionSupportRole::NarrowingMaterialization,
                missing_artifact_id,
                "basis:retained",
                "cursor:retained",
                "checkpoint:retained",
                "compatibility:1",
                "portability:1",
            )
            .unwrap()
            .with_rebuild_maintenance_admission(
                "basis:retained",
                SubscriptionSupportMissingSupportMaintenanceAdmission::new(
                    crate::SupportActionId::new("support-maintenance:certification-rebuild")
                        .unwrap(),
                    crate::SupportActionBreadthBudget::new(1, 256).unwrap(),
                    128,
                )
                .unwrap(),
            )
            .unwrap(),
        )
        .unwrap();
    assert_eq!(
        rebuild_report.classification(),
        SubscriptionResumeClassification::RebuildRequired
    );
    assert!(rebuild_report.maintenance_report().is_some());
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_missing_support_recovery(
            SubscriptionSupportCertificationLaneKind::RebuildRequiredMissingSupport,
            &rebuild_report,
            rebuild_store.subscription_support_counters(),
        )
        .unwrap(),
    );

    let degraded_report = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let admitted = store
            .admit_subscription_support_declaration(raw_degraded())
            .unwrap();
        let publishable = store
            .subscription_support_pipeline()
            .prepare_exact(
                admitted,
                "basis:degraded",
                "cursor:degraded",
                "checkpoint:degraded",
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
        let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 64, true).unwrap();
        let plan = SubscriptionSupportClassificationPlan::new(
            SubscriptionSupportPlanFamily::DegradedResumeClassificationPlan,
            SubscriptionSupportPayloadBudget::new(16 * 1024, 64).unwrap(),
            SubscriptionSupportAllocationScope::RestartShardBatch,
            SubscriptionSupportDensityClass::RestartShardBatchClassification,
            Some("restart-shard:degraded".into()),
        )
        .unwrap();
        store
            .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
                fetched, evidence, plan,
            ))
            .unwrap()
    };
    assert_eq!(
        degraded_report.classification(),
        SubscriptionResumeClassification::Degraded
    );
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::DegradedButRecoverable,
            &degraded_report,
        )
        .unwrap(),
    );
    classification_reports.push(degraded_report);

    let basis_drift_report = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let artifact_id = publish_exact(&mut store, "basis:control", "cursor:1", "checkpoint:1");
        let fetched = store
            .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
                SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                artifact_id,
            ))
            .unwrap();
        let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, true)
            .unwrap()
            .with_basis_digest("basis:drift")
            .unwrap();
        store
            .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
                fetched,
                evidence,
                SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
            ))
            .unwrap()
    };
    assert_eq!(
        basis_drift_report.primary_cause(),
        Some(SubscriptionSupportDriftCause::SubscriptionSupportBasisDrift)
    );
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::NotResumableBasisDrift,
            &basis_drift_report,
        )
        .unwrap(),
    );
    classification_reports.push(basis_drift_report);

    let cursor_drift_report = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let artifact_id = publish_exact(&mut store, "basis:cursor", "cursor:1", "checkpoint:1");
        let fetched = store
            .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
                SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                artifact_id,
            ))
            .unwrap();
        let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, true)
            .unwrap()
            .with_cursor_digest("cursor:drift")
            .unwrap();
        store
            .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
                fetched,
                evidence,
                SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
            ))
            .unwrap()
    };
    assert_eq!(
        cursor_drift_report.primary_cause(),
        Some(SubscriptionSupportDriftCause::SubscriptionSupportCursorDrift)
    );
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::NotResumableCursorDrift,
            &cursor_drift_report,
        )
        .unwrap(),
    );
    classification_reports.push(cursor_drift_report);

    let support_digest_drift_report = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let artifact_id = publish_exact(&mut store, "basis:support", "cursor:1", "checkpoint:1");
        let fetched = store
            .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
                SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                artifact_id,
            ))
            .unwrap();
        let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, true)
            .unwrap()
            .with_support_artifact_digest("artifact:drift")
            .unwrap();
        store
            .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
                fetched,
                evidence,
                SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
            ))
            .unwrap()
    };
    assert_eq!(
        support_digest_drift_report.primary_cause(),
        Some(SubscriptionSupportDriftCause::SubscriptionSupportDigestMismatch)
    );
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::SupportDigestDrift,
            &support_digest_drift_report,
        )
        .unwrap(),
    );
    classification_reports.push(support_digest_drift_report);

    let compatibility_drift_report = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let artifact_id =
            publish_exact(&mut store, "basis:compat-only", "cursor:1", "checkpoint:1");
        let fetched = store
            .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
                SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                artifact_id,
            ))
            .unwrap();
        let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, true)
            .unwrap()
            .with_compatibility_digest("compatibility:drift")
            .unwrap();
        store
            .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
                fetched,
                evidence,
                SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
            ))
            .unwrap()
    };
    assert_eq!(
        compatibility_drift_report.primary_cause(),
        Some(SubscriptionSupportDriftCause::SubscriptionSupportCompatibilityDrift)
    );
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::CompatibilityDrift,
            &compatibility_drift_report,
        )
        .unwrap(),
    );
    classification_reports.push(compatibility_drift_report);

    let cross_family_reuse_report = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let artifact_id =
            publish_exact(&mut store, "basis:cross-family", "cursor:1", "checkpoint:1");
        let fetched = store
            .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
                SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                artifact_id,
            ))
            .unwrap();
        let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, true)
            .unwrap()
            .with_expected_family_kind(SubscriptionSupportFamilyKind::MaterializedNarrowingSupport);
        store
            .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
                fetched,
                evidence,
                SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
            ))
            .unwrap()
    };
    assert_eq!(
        cross_family_reuse_report.primary_cause(),
        Some(SubscriptionSupportDriftCause::SubscriptionSupportFamilyMismatch)
    );
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::CrossFamilyReuseRejected,
            &cross_family_reuse_report,
        )
        .unwrap(),
    );
    classification_reports.push(cross_family_reuse_report);

    let basis_precedence_report = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let artifact_id = publish_exact(&mut store, "basis:precedence", "cursor:1", "checkpoint:1");
        let fetched = store
            .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
                SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                artifact_id,
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
        store
            .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
                fetched,
                evidence,
                SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
            ))
            .unwrap()
    };
    assert_eq!(
        basis_precedence_report.primary_cause(),
        Some(SubscriptionSupportDriftCause::SubscriptionSupportBasisDrift)
    );
    assert_eq!(
        basis_precedence_report.suppressed_causes(),
        &[
            SubscriptionSupportDriftCause::SubscriptionSupportCursorDrift,
            SubscriptionSupportDriftCause::SubscriptionSupportDigestMismatch
        ]
    );
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::MultiDriftBasisPrecedence,
            &basis_precedence_report,
        )
        .unwrap(),
    );
    classification_reports.push(basis_precedence_report);

    let cursor_only_error = StoreErrorKind::SubscriptionSupportClassificationViolation;
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_typed_rejection(
            SubscriptionSupportCertificationLaneKind::CursorOnlyExactResumeRejected,
            cursor_only_error,
            SubscriptionSupportCounterSnapshot::default(),
        )
        .unwrap(),
    );

    let session_loss_report = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let artifact_id = publish_exact(&mut store, "basis:session", "cursor:1", "checkpoint:1");
        let fetched = store
            .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
                SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                artifact_id,
            ))
            .unwrap();
        let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, false).unwrap();
        store
            .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
                fetched,
                evidence,
                SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
            ))
            .unwrap()
    };
    assert_eq!(
        session_loss_report.primary_cause(),
        Some(SubscriptionSupportDriftCause::SubscriptionSupportSessionMemoryMissing)
    );
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::SessionMemoryLossNonAuthoritative,
            &session_loss_report,
        )
        .unwrap(),
    );
    classification_reports.push(session_loss_report);

    let tier_recall_report = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let artifact_id = publish_exact(&mut store, "basis:tier", "cursor:1", "checkpoint:1");
        let fetched = store
            .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
                SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                artifact_id,
            ))
            .unwrap();
        let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, true)
            .unwrap()
            .with_placement_unavailable();
        store
            .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
                fetched,
                evidence,
                SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
            ))
            .unwrap()
    };
    assert_eq!(
        tier_recall_report.primary_cause(),
        Some(SubscriptionSupportDriftCause::SubscriptionSupportPlacementUnavailable)
    );
    assert_eq!(
        tier_recall_report.classification(),
        SubscriptionResumeClassification::Exact
    );
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::TierRecallCostOnly,
            &tier_recall_report,
        )
        .unwrap(),
    );
    classification_reports.push(tier_recall_report);

    let runtime_handoff_report = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let artifact_id = publish_exact(&mut store, "basis:handoff", "cursor:1", "checkpoint:1");
        let report = store
            .handoff_subscription_support_runtime(
                SubscriptionSupportRuntimeHandoffRequest::new(
                    SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                    SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                    artifact_id,
                    "runtime:source",
                    "runtime:target",
                )
                .unwrap(),
            )
            .unwrap();
        assert!(!report.delivery_session_memory_persisted());
        assert_eq!(
            store
                .subscription_support_counters()
                .runtime_handoff_count(),
            1
        );
        report
    };
    assert_eq!(
        runtime_handoff_report.durable_report().classification(),
        SubscriptionResumeClassification::Exact
    );
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::RuntimeHandoffEquivalence,
            runtime_handoff_report.durable_report(),
        )
        .unwrap(),
    );
    classification_reports.push(runtime_handoff_report.durable_report().clone());

    let unknown_authority_error = {
        let raw = RawSubscriptionSupportDeclaration::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            SubscriptionSupportRole::ExactContinuation,
            SubscriptionSupportAuthority::Unadmitted("phase-5b-hostile".into()),
            "forge-query-live-v1",
            SubscriptionSupportScope::from_canonical(vec!["feed:2".into()]).unwrap(),
            SubscriptionSupportPayloadDigest::new("payload:abc").unwrap(),
        );
        SubscriptionSupportCatalog::first_ship()
            .admit(raw)
            .expect_err("unknown upstream authority must reject before publication")
    };
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_typed_rejection(
            SubscriptionSupportCertificationLaneKind::UnknownUpstreamAuthorityRejected,
            unknown_authority_error.kind().clone(),
            SubscriptionSupportCounterSnapshot::default(),
        )
        .unwrap(),
    );

    let non_canonical_error =
        SubscriptionSupportScope::from_canonical(vec!["z".into(), "a".into()])
            .expect_err("non-canonical support scope must reject before identity calculation");
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_typed_rejection(
            SubscriptionSupportCertificationLaneKind::NonCanonicalScopeRejected,
            non_canonical_error.kind().clone(),
            SubscriptionSupportCounterSnapshot::default(),
        )
        .unwrap(),
    );

    let unsupported_family_error = {
        let raw = RawSubscriptionSupportDeclaration::new(
            SubscriptionSupportFamilyId::new("unsupported-subscription-support-family").unwrap(),
            SubscriptionSupportFamilyKind::ExtensionDefinedSupport,
            SubscriptionSupportRole::ExactContinuation,
            SubscriptionSupportAuthority::ForgeQuery,
            "forge-query-live-v1",
            SubscriptionSupportScope::from_canonical(vec!["feed:2".into()]).unwrap(),
            SubscriptionSupportPayloadDigest::new("payload:abc").unwrap(),
        );
        SubscriptionSupportCatalog::first_ship()
            .admit(raw)
            .expect_err("unsupported family identity must reject before publication")
    };
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_typed_rejection(
            SubscriptionSupportCertificationLaneKind::UnsupportedFamilyKindRejected,
            unsupported_family_error.kind().clone(),
            SubscriptionSupportCounterSnapshot::default(),
        )
        .unwrap(),
    );

    let compatibility_precedence_report = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let artifact_id = publish_exact(&mut store, "basis:compat", "cursor:1", "checkpoint:1");
        let fetched = store
            .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
                SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                artifact_id,
            ))
            .unwrap();
        let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, true)
            .unwrap()
            .with_compatibility_digest("compatibility:drift")
            .unwrap()
            .with_support_artifact_digest("artifact:drift")
            .unwrap();
        store
            .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
                fetched,
                evidence,
                SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
            ))
            .unwrap()
    };
    assert_eq!(
        compatibility_precedence_report.primary_cause(),
        Some(SubscriptionSupportDriftCause::SubscriptionSupportCompatibilityDrift)
    );
    assert_eq!(
        compatibility_precedence_report.suppressed_causes(),
        &[SubscriptionSupportDriftCause::SubscriptionSupportDigestMismatch]
    );
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::MultiDriftCompatibilityPrecedence,
            &compatibility_precedence_report,
        )
        .unwrap(),
    );
    classification_reports.push(compatibility_precedence_report);

    let missing_rebuild_basis_report = {
        let missing_artifact_id = {
            let mut source = ForgeStoreBuilder::new().in_memory().build().unwrap();
            let admitted = source
                .admit_subscription_support_declaration(raw_materialized())
                .unwrap();
            let publishable = source
                .subscription_support_pipeline()
                .prepare_exact(
                    admitted,
                    "basis:missing",
                    "cursor:missing",
                    "checkpoint:missing",
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
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let report = store
            .classify_missing_subscription_support(
                SubscriptionSupportMissingSupportRecoveryRequest::new(
                    SubscriptionSupportFamilyId::new("materialized-narrowing-support").unwrap(),
                    SubscriptionSupportFamilyKind::MaterializedNarrowingSupport,
                    SubscriptionSupportRole::NarrowingMaterialization,
                    missing_artifact_id,
                    "basis:missing",
                    "cursor:missing",
                    "checkpoint:missing",
                    "compatibility:1",
                    "portability:1",
                )
                .unwrap(),
            )
            .unwrap();
        assert_eq!(
            report.classification(),
            SubscriptionResumeClassification::NotResumable
        );
        (report, store.subscription_support_counters())
    };
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_missing_support_recovery(
            SubscriptionSupportCertificationLaneKind::RebuildBasisMissingNotResumable,
            &missing_rebuild_basis_report.0,
            missing_rebuild_basis_report.1,
        )
        .unwrap(),
    );

    let batch_debt_report = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let admitted = store
            .admit_subscription_support_declaration(raw_materialized())
            .unwrap();
        let publishable = store
            .subscription_support_pipeline()
            .prepare_exact(
                admitted,
                "basis:batch",
                "cursor:batch",
                "checkpoint:batch",
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
        let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, true)
            .unwrap()
            .with_support_artifact_digest("artifact:drift")
            .unwrap();
        let plan = SubscriptionSupportClassificationPlan::new(
            SubscriptionSupportPlanFamily::DeniedResumeClassificationPlan,
            SubscriptionSupportPayloadBudget::new(16 * 1024, 64).unwrap(),
            SubscriptionSupportAllocationScope::FamilyLocalScratch,
            SubscriptionSupportDensityClass::FamilyBatchClassificationDebt,
            None,
        )
        .unwrap();
        store
            .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
                fetched, evidence, plan,
            ))
            .unwrap()
    };
    assert_eq!(
        batch_debt_report.cost_surface().density_class(),
        SubscriptionSupportDensityClass::FamilyBatchClassificationDebt
    );
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::BatchClassificationDebt,
            &batch_debt_report,
        )
        .unwrap(),
    );
    classification_reports.push(batch_debt_report);

    let oversized_payload_error = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let artifact_id = publish_exact(&mut store, "basis:budget", "cursor:1", "checkpoint:1");
        let fetched = store
            .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
                SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                artifact_id,
            ))
            .unwrap();
        let evidence =
            SubscriptionSupportResumeEvidence::matching(&fetched, 32 * 1024, true).unwrap();
        let error = store
            .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
                fetched,
                evidence,
                SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
            ))
            .expect_err("oversized payload must reject before classification");
        (error, store.subscription_support_counters())
    };
    assert_eq!(
        oversized_payload_error.0.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
    assert_eq!(oversized_payload_error.1.budget_denials(), 1);
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_typed_rejection(
            SubscriptionSupportCertificationLaneKind::OversizedPayloadRejectedBeforeDecode,
            oversized_payload_error.0.kind().clone(),
            oversized_payload_error.1,
        )
        .unwrap(),
    );

    let access_debt_outcome = {
        let path = unique_test_sqlite_path("forge-store-subscription-support-certification-debt");
        let artifact_id = {
            let mut store = ForgeStoreBuilder::new()
                .sqlite_file(path.clone())
                .build()
                .unwrap();
            publish_exact(&mut store, "basis:debt", "cursor:debt", "checkpoint:debt")
        };
        let connection = rusqlite::Connection::open(&path).unwrap();
        connection
            .execute("DROP INDEX idx_subscription_support_family_artifact", [])
            .unwrap();
        drop(connection);
        let mut reopened = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
        let report = reopened.subscription_support_access_structure_report();
        assert!(report.has_debt());
        let error = reopened
            .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
                SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                artifact_id,
            ))
            .expect_err("access-debt lane must not hide a global scan");
        assert_eq!(
            error.kind(),
            &StoreErrorKind::SubscriptionSupportPublicationViolation
        );
        SubscriptionSupportCertificationLaneOutcome::from_access_structure_debt(
            SubscriptionSupportCertificationLaneKind::BackendAccessStructureDebt,
            &report,
            reopened.subscription_support_counters(),
        )
        .unwrap()
    };
    lane_outcomes.push(access_debt_outcome);

    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_typed_rejection(
            SubscriptionSupportCertificationLaneKind::DecodedRowPublicationRejected,
            StoreErrorKind::SubscriptionSupportPublicationViolation,
            SubscriptionSupportCounterSnapshot::default(),
        )
        .unwrap(),
    );

    let compatibility_exact = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let plan = store
            .admit_subscription_support_compatibility_batch(
                SupportActionId::new("support-compatibility:cert-exact").unwrap(),
                vec![
                    compatibility_basis("exact-a"),
                    compatibility_basis("exact-b"),
                ],
                read_receipt_witness(CompatibilityRelation::Native),
                "semantic:cert-compatibility:exact",
                SubscriptionSupportCompatibilityDecision::exact_compatible_migration(
                    "classifier-equivalence:cert-v1-v2",
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
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_compatibility_report(
            SubscriptionSupportCertificationLaneKind::SupportCompatibilityExactMigration,
            &compatibility_exact.0,
            compatibility_exact.1,
        )
        .unwrap(),
    );

    let compatibility_degraded = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let plan = store
            .admit_subscription_support_compatibility_batch(
                SupportActionId::new("support-compatibility:cert-degraded").unwrap(),
                vec![compatibility_basis("degraded")],
                read_receipt_witness(CompatibilityRelation::AdapterRequired),
                "semantic:cert-compatibility:degraded",
                SubscriptionSupportCompatibilityDecision::degraded_compatibility(
                    "classifier equivalence weakened by removed cursor hint",
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
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_compatibility_report(
            SubscriptionSupportCertificationLaneKind::SupportCompatibilityDegraded,
            &compatibility_degraded.0,
            compatibility_degraded.1,
        )
        .unwrap(),
    );

    for (lane, decision) in [
        (
            SubscriptionSupportCertificationLaneKind::SupportCompatibilityOldReaderRejected,
            SubscriptionSupportCompatibilityDecision::old_reader_rejected(1, 2).unwrap(),
        ),
        (
            SubscriptionSupportCertificationLaneKind::SupportCompatibilityUnknownFamilyRejected,
            SubscriptionSupportCompatibilityDecision::unknown_family_rejected(
                SubscriptionSupportFamilyId::new("unknown-support-family").unwrap(),
            ),
        ),
        (
            SubscriptionSupportCertificationLaneKind::SupportCompatibilityVersionSkewRejected,
            SubscriptionSupportCompatibilityDecision::version_skew_rejected(
                "payload version has no admitted support reader",
            )
            .unwrap(),
        ),
    ] {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let plan = store
            .admit_subscription_support_compatibility_batch(
                SupportActionId::new(format!("support-compatibility:{lane:?}")).unwrap(),
                vec![compatibility_basis("reject")],
                rejected_read_outcome_witness(
                    CompatibilityRejectionKind::ReaderCapabilityUnsupported,
                ),
                "semantic:cert-compatibility:reject",
                decision,
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
        lane_outcomes.push(
            SubscriptionSupportCertificationLaneOutcome::from_compatibility_report(
                lane,
                &report,
                store.subscription_support_counters(),
            )
            .unwrap(),
        );
    }

    let retention_exact = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let plan = store
            .admit_subscription_support_retention_batch(
                SupportActionId::new("support-retention:cert-exact").unwrap(),
                vec![retention_basis("exact-a"), retention_basis("exact-b")],
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
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_retention_report(
            SubscriptionSupportCertificationLaneKind::SupportRetentionExactPreserved,
            &retention_exact.0,
            retention_exact.1.clone(),
        )
        .unwrap(),
    );
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_retention_report(
            SubscriptionSupportCertificationLaneKind::SupportFamilyLocalBatchBounded,
            &retention_exact.0,
            retention_exact.1,
        )
        .unwrap(),
    );

    let retention_compacted = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let plan = store
            .admit_subscription_support_retention_batch(
                SupportActionId::new("support-retention:cert-compacted").unwrap(),
                vec![retention_basis("compact-a"), retention_basis("compact-b")],
                SubscriptionSupportRetentionDecision::compact_exact("compacted-basis:cert")
                    .unwrap(),
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
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_retention_report(
            SubscriptionSupportCertificationLaneKind::SupportRetentionCompactedExact,
            &retention_compacted.0,
            retention_compacted.1,
        )
        .unwrap(),
    );

    let retention_reclaimed = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let plan = store
            .admit_subscription_support_retention_batch(
                SupportActionId::new("support-retention:cert-reclaim").unwrap(),
                vec![retention_basis("reclaim")],
                SubscriptionSupportRetentionDecision::reclaim_with_rebuild(
                    "basis:cert-reclaim",
                    "maintenance:key:cert-reclaim",
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
            .publish_subscription_support_retention_consequence(plan)
            .unwrap();
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_retention_report(
            SubscriptionSupportCertificationLaneKind::SupportRetentionReclaimedRebuildable,
            &retention_reclaimed.0,
            retention_reclaimed.1,
        )
        .unwrap(),
    );

    let retention_expired = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let plan = store
            .admit_subscription_support_retention_batch(
                SupportActionId::new("support-retention:cert-expired").unwrap(),
                vec![retention_basis("expired")],
                SubscriptionSupportRetentionDecision::expire_by_policy("policy-expired:cert")
                    .unwrap(),
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
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_retention_report(
            SubscriptionSupportCertificationLaneKind::SupportRetentionExpiredByPolicy,
            &retention_expired.0,
            retention_expired.1,
        )
        .unwrap(),
    );

    let portability_full = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let plan = store
            .admit_subscription_support_portability_batch(
                SupportActionId::new("support-portability:cert-full").unwrap(),
                vec![
                    portability_basis(
                        crate::SubscriptionSupportActionOrigin::ReplicationExport,
                        "full-a",
                    ),
                    portability_basis(
                        crate::SubscriptionSupportActionOrigin::ReplicationExport,
                        "full-b",
                    ),
                ],
                2,
                0,
                SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
                SubscriptionSupportPortabilityDecision::full_scope_replication(
                    "identity-preservation:cert-full",
                    "identity-preservation:cert-full",
                )
                .unwrap(),
                SupportPathClass::ReplicationExport,
                SupportProgramDensityClass::PortabilityScopeBatch,
                SupportAllocationScope::PortabilityManifest,
                SupportActionBreadthBudget::new(4, 1024).unwrap(),
                128,
            )
            .unwrap();
        let report = store
            .publish_subscription_support_portability_consequence(plan)
            .unwrap();
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_portability_report(
            SubscriptionSupportCertificationLaneKind::SupportPortabilityFullScopeReplicated,
            &portability_full.0,
            portability_full.1.clone(),
        )
        .unwrap(),
    );
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_portability_report(
            SubscriptionSupportCertificationLaneKind::SupportPortabilityScopeBatchBounded,
            &portability_full.0,
            portability_full.1,
        )
        .unwrap(),
    );

    let portability_partial = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let omitted_id = portability_basis(
            crate::SubscriptionSupportActionOrigin::ReplicationExport,
            "partial-b",
        )
        .artifact_id()
        .clone();
        let plan = store
            .admit_subscription_support_portability_batch(
                SupportActionId::new("support-portability:cert-partial").unwrap(),
                vec![
                    portability_basis(
                        crate::SubscriptionSupportActionOrigin::ReplicationExport,
                        "partial-a",
                    ),
                    portability_basis(
                        crate::SubscriptionSupportActionOrigin::ReplicationExport,
                        "partial-b",
                    ),
                ],
                1,
                1,
                SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
                SubscriptionSupportPortabilityDecision::partial_scope_omission(
                    vec![omitted_id],
                    "partial scope export omitted one artifact",
                )
                .unwrap(),
                SupportPathClass::ReplicationExport,
                SupportProgramDensityClass::PortabilityScopeBatch,
                SupportAllocationScope::PortabilityManifest,
                SupportActionBreadthBudget::new(4, 1024).unwrap(),
                128,
            )
            .unwrap();
        let report = store
            .publish_subscription_support_portability_consequence(plan)
            .unwrap();
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_portability_report(
            SubscriptionSupportCertificationLaneKind::SupportPortabilityPartialOmission,
            &portability_partial.0,
            portability_partial.1,
        )
        .unwrap(),
    );

    let portability_import = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let plan = store
            .admit_subscription_support_portability_batch(
                SupportActionId::new("support-portability:cert-import").unwrap(),
                vec![portability_basis(
                    crate::SubscriptionSupportActionOrigin::ReplicationImport,
                    "import",
                )],
                1,
                0,
                SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
                SubscriptionSupportPortabilityDecision::target_import_admitted(
                    "target-import:cert",
                    "identity-preservation:cert",
                    "semantic:cert-import",
                )
                .unwrap(),
                SupportPathClass::ImportAdmission,
                SupportProgramDensityClass::PortabilityScopeBatch,
                SupportAllocationScope::PortabilityManifest,
                SupportActionBreadthBudget::new(4, 1024).unwrap(),
                128,
            )
            .unwrap();
        let report = store
            .publish_subscription_support_portability_consequence(plan)
            .unwrap();
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_portability_report(
            SubscriptionSupportCertificationLaneKind::SupportPortabilityImportAdmitted,
            &portability_import.0,
            portability_import.1,
        )
        .unwrap(),
    );

    let portability_missing_basis = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let basis_a = portability_basis(
            crate::SubscriptionSupportActionOrigin::ReplicationImport,
            "missing-a",
        );
        let basis_b = portability_basis(
            crate::SubscriptionSupportActionOrigin::ReplicationImport,
            "missing-b",
        );
        let plan = store
            .admit_subscription_support_portability_batch(
                SupportActionId::new("support-portability:cert-missing-basis").unwrap(),
                vec![basis_a.clone(), basis_b.clone()],
                2,
                0,
                SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
                SubscriptionSupportPortabilityDecision::target_import_missing_basis_not_resumable(
                    "target-import-missing:cert",
                    vec![basis_a.artifact_id().clone()],
                    "missing exact imported basis",
                )
                .unwrap(),
                SupportPathClass::ImportAdmission,
                SupportProgramDensityClass::PortabilityScopeBatch,
                SupportAllocationScope::PortabilityManifest,
                SupportActionBreadthBudget::new(4, 1024).unwrap(),
                128,
            )
            .unwrap();
        let report = store
            .publish_subscription_support_portability_consequence(plan)
            .unwrap();
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_portability_report(
            SubscriptionSupportCertificationLaneKind::SupportPortabilityImportMissingBasisNotResumable,
            &portability_missing_basis.0,
            portability_missing_basis.1,
        )
        .unwrap(),
    );

    let maintenance_rebuild = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let basis = maintenance_basis("rebuild");
        let retained_basis_digest = basis.basis_digest().to_string();
        let plan = store
            .admit_subscription_support_maintenance_batch(
                SupportActionId::new("support-maintenance:cert-rebuild").unwrap(),
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
            .publish_subscription_support_maintenance_consequence(plan)
            .unwrap();
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_maintenance_report(
            SubscriptionSupportCertificationLaneKind::SupportMaintenanceRebuildAdmitted,
            &maintenance_rebuild.0,
            maintenance_rebuild.1.clone(),
        )
        .unwrap(),
    );
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_maintenance_report(
            SubscriptionSupportCertificationLaneKind::SupportMaintenanceKeyBatchBounded,
            &maintenance_rebuild.0,
            maintenance_rebuild.1,
        )
        .unwrap(),
    );

    let maintenance_refresh = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let plan = store
            .admit_subscription_support_maintenance_batch(
                SupportActionId::new("support-maintenance:cert-refresh").unwrap(),
                vec![maintenance_basis("refresh")],
                SubscriptionSupportMaintenanceDecision::refresh_descriptor_admitted(
                    "refresh support snapshot projection",
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
            .publish_subscription_support_maintenance_consequence(plan)
            .unwrap();
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_maintenance_report(
            SubscriptionSupportCertificationLaneKind::SupportMaintenanceRefreshAdmitted,
            &maintenance_refresh.0,
            maintenance_refresh.1,
        )
        .unwrap(),
    );

    let maintenance_migration = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let plan = store
            .admit_subscription_support_maintenance_batch(
                SupportActionId::new("support-maintenance:cert-migration").unwrap(),
                vec![maintenance_basis("migration")],
                SubscriptionSupportMaintenanceDecision::compatibility_migration_descriptor_admitted(
                    "compatibility-migration:cert",
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
            .publish_subscription_support_maintenance_consequence(plan)
            .unwrap();
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_maintenance_report(
            SubscriptionSupportCertificationLaneKind::SupportMaintenanceCompatibilityMigrationAdmitted,
            &maintenance_migration.0,
            maintenance_migration.1,
        )
        .unwrap(),
    );

    let maintenance_degradation = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let plan = store
            .admit_subscription_support_maintenance_batch(
                SupportActionId::new("support-maintenance:cert-degradation").unwrap(),
                vec![maintenance_basis("degradation")],
                SubscriptionSupportMaintenanceDecision::degradation_recovery_descriptor_admitted(
                    "degraded continuation support recovered with weakened posture",
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
            .publish_subscription_support_maintenance_consequence(plan)
            .unwrap();
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_maintenance_report(
            SubscriptionSupportCertificationLaneKind::SupportMaintenanceDegradationRecoveryAdmitted,
            &maintenance_degradation.0,
            maintenance_degradation.1,
        )
        .unwrap(),
    );

    let maintenance_recovered = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let plan = store
            .admit_subscription_support_maintenance_batch(
                SupportActionId::new("support-maintenance:cert-restart").unwrap(),
                vec![maintenance_basis("restart")],
                SubscriptionSupportMaintenanceDecision::interrupted_restart_recovered(
                    crate::SupportMaintenanceWorkKind::Rebuild,
                    "maintenance-restart:descriptor-recovered",
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
            .publish_subscription_support_maintenance_consequence(plan)
            .unwrap();
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_maintenance_report(
            SubscriptionSupportCertificationLaneKind::SupportMaintenanceInterruptedRestartRecovered,
            &maintenance_recovered.0,
            maintenance_recovered.1,
        )
        .unwrap(),
    );

    let maintenance_coalesced = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
        let basis = maintenance_basis("coalesced");
        let retained_basis_digest = basis.basis_digest().to_string();
        let plan = store
            .admit_subscription_support_maintenance_batch(
                SupportActionId::new("support-maintenance:cert-coalesced").unwrap(),
                vec![basis.clone(), basis],
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
            .publish_subscription_support_maintenance_consequence(plan)
            .unwrap();
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_maintenance_report(
            SubscriptionSupportCertificationLaneKind::SupportMaintenanceCoalescedRebuildAdmitted,
            &maintenance_coalesced.0,
            maintenance_coalesced.1,
        )
        .unwrap(),
    );

    let mut bounded_pipeline = crate::SubscriptionSupportPublicationPipeline::first_ship();
    let budget = SupportActionBreadthBudget::new(4, 1024).unwrap();
    let plan = bounded_pipeline
        .admit_support_program_path(
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            budget,
            2,
            128,
        )
        .unwrap();
    bounded_pipeline.reuse_support_batch_receipt(&plan).unwrap();
    let foreground_error = bounded_pipeline
        .admit_support_program_path(
            SupportPathClass::ForegroundResume,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            budget,
            1,
            64,
        )
        .expect_err("foreground resume must reject operational work");
    let store_global_error = bounded_pipeline
        .admit_support_program_path(
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::StoreGlobalDebt,
            SupportAllocationScope::FamilyLocalBatch,
            budget,
            2,
            128,
        )
        .expect_err("store-global density must remain debt");
    let bounded_counters = bounded_pipeline.counters();
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_typed_rejection(
            SubscriptionSupportCertificationLaneKind::SupportForegroundOperationalWorkRejected,
            foreground_error.kind().clone(),
            bounded_counters.clone(),
        )
        .unwrap(),
    );
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_typed_rejection(
            SubscriptionSupportCertificationLaneKind::SupportStoreGlobalDensityRejected,
            store_global_error.kind().clone(),
            bounded_counters.clone(),
        )
        .unwrap(),
    );
    lane_outcomes.push(
        SubscriptionSupportCertificationLaneOutcome::from_counter_snapshot(
            SubscriptionSupportCertificationLaneKind::SupportBatchReceiptReuseVerified,
            bounded_counters,
        )
        .unwrap(),
    );

    let bundle = SubscriptionSupportCertificationBundle::from_lane_outcomes(
        &SubscriptionSupportCatalog::first_ship(),
        SubscriptionSupportCounterSnapshot::default(),
        &classification_reports,
        lane_outcomes,
    )
    .unwrap();

    let matrix = bundle
        .matrix()
        .expect("Phase 6A bundle must carry a matrix");
    assert_eq!(
        matrix.status(),
        SubscriptionSupportCertificationMatrixStatus::Phase6AOperationalParticipationComplete
    );
    assert_eq!(
        matrix.lane_outcomes().len(),
        SubscriptionSupportCertificationLaneKind::phase_6a_required().len() + 6
    );
    assert_eq!(bundle.catalog_family_count(), 3);
    assert!(!bundle.truth_digest().is_empty());
    assert!(!bundle.artifact_digest().is_empty());
    assert!(!bundle.subscription_support_digest().is_empty());
    assert!(!bundle.replay_digest().is_empty());
    assert!(!bundle.diagnostics_digest().is_empty());
    assert!(!bundle.counter_digest().is_empty());
}

#[test]
fn durable_subscription_support_certification_matrix_rejects_missing_phase_5b_floor() {
    let report = fetched_exact_report(&mut ForgeStoreBuilder::new().in_memory().build().unwrap());
    let error = SubscriptionSupportCertificationBundle::from_lane_outcomes(
        &SubscriptionSupportCatalog::first_ship(),
        SubscriptionSupportCounterSnapshot::default(),
        std::slice::from_ref(&report),
        vec![
            SubscriptionSupportCertificationLaneOutcome::from_classification_report(
                SubscriptionSupportCertificationLaneKind::ExactResumeControl,
                &report,
            )
            .unwrap(),
        ],
    )
    .expect_err("certification must reject matrices that miss the Phase 5B floor");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
}

#[test]
fn durable_subscription_support_certification_matrix_rejects_mislabeled_lane_evidence() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let artifact_id = publish_exact(&mut store, "basis:mislabeled", "cursor:1", "checkpoint:1");
    let fetched = store
        .fetch_subscription_support(SubscriptionSupportFetchRequest::new(
            SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            artifact_id,
        ))
        .unwrap();
    let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 128, true)
        .unwrap()
        .with_support_artifact_digest("artifact:drift")
        .unwrap();
    let report = store
        .classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
            fetched,
            evidence,
            SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
        ))
        .unwrap();

    let error = SubscriptionSupportCertificationBundle::from_lane_outcomes(
        &SubscriptionSupportCatalog::first_ship(),
        SubscriptionSupportCounterSnapshot::default(),
        std::slice::from_ref(&report),
        vec![
            SubscriptionSupportCertificationLaneOutcome::from_classification_report(
                SubscriptionSupportCertificationLaneKind::NotResumableCursorDrift,
                &report,
            )
            .unwrap(),
        ],
    )
    .expect_err("certification must reject a support-digest report mislabeled as cursor drift");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
}

#[test]
fn durable_subscription_support_certification_matrix_rejects_mislabeled_retention_lane() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let plan = store
        .admit_subscription_support_retention_batch(
            SupportActionId::new("support-retention:mislabeled").unwrap(),
            vec![retention_basis("mislabeled")],
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

    let error = SubscriptionSupportCertificationLaneOutcome::from_retention_report(
        SubscriptionSupportCertificationLaneKind::SupportRetentionExpiredByPolicy,
        &report,
        store.subscription_support_counters(),
    )
    .and_then(|lane| {
        SubscriptionSupportCertificationBundle::from_lane_outcomes(
            &SubscriptionSupportCatalog::first_ship(),
            SubscriptionSupportCounterSnapshot::default(),
            &[],
            vec![lane],
        )
    })
    .expect_err("retained support cannot masquerade as expired policy");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
}

#[test]
fn subscription_support_runtime_handoff_requires_distinct_runtime_owners() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let artifact_id = publish_exact(&mut store, "basis:handoff", "cursor:1", "checkpoint:1");

    let error = SubscriptionSupportRuntimeHandoffRequest::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        artifact_id,
        "runtime:same",
        "runtime:same",
    )
    .expect_err("handoff must not collapse source and target runtime owners");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
}
