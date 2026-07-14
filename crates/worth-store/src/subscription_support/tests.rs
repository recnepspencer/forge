use super::*;
use crate::{
    failure::StoreErrorKind, ArtifactCompatibilityWindow, ArtifactFamilyId, ArtifactFormatVersion,
    ArtifactSemanticVersion, CompatibilityAdmissionCounters, CompatibilityAdmissionPath,
    CompatibilityAdmissionReceipt, CompatibilityManifestDigest, CompatibilityReadAdmissionOutcome,
    CompatibilityRejection, CompatibilityRejectionKind, CompatibilityRelation,
    QuarantinedDecodedArtifact, ReadCompatibilityReceipt,
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

fn operational_basis(
    action_origin: SubscriptionSupportActionOrigin,
) -> SubscriptionSupportOperationalBasis {
    SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId("artifact:phase-1".into()),
        "basis:phase-1",
        "cursor:phase-1",
        "checkpoint:phase-1",
        "compatibility:phase-1",
        "portability:phase-1",
        action_origin,
    )
    .unwrap()
}

fn retention_basis(artifact_suffix: &str) -> SubscriptionSupportOperationalBasis {
    SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId(format!("artifact:retention:{artifact_suffix}")),
        "basis:retention",
        "cursor:retention",
        "checkpoint:retention",
        "compatibility:retention",
        "portability:retention",
        SubscriptionSupportActionOrigin::Retention,
    )
    .unwrap()
}

fn retention_basis_for_family(
    family_id: &str,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    artifact_suffix: &str,
    action_origin: SubscriptionSupportActionOrigin,
) -> SubscriptionSupportOperationalBasis {
    SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new(family_id).unwrap(),
        family_kind,
        support_role,
        SubscriptionSupportArtifactId(format!("artifact:retention:{artifact_suffix}")),
        "basis:retention",
        "cursor:retention",
        "checkpoint:retention",
        "compatibility:retention",
        "portability:retention",
        action_origin,
    )
    .unwrap()
}

fn compatibility_basis(artifact_suffix: &str) -> SubscriptionSupportOperationalBasis {
    SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId(format!("artifact:compatibility:{artifact_suffix}")),
        "basis:compatibility",
        "cursor:compatibility",
        "checkpoint:compatibility",
        "compatibility:manifest-v2",
        "portability:compatibility",
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
        SubscriptionSupportArtifactId(format!("artifact:portability:{artifact_suffix}")),
        "basis:portability",
        "cursor:portability",
        "checkpoint:portability",
        "compatibility:portability",
        format!("portability:{artifact_suffix}"),
        action_origin,
    )
    .unwrap()
}

fn maintenance_basis(artifact_suffix: &str) -> SubscriptionSupportOperationalBasis {
    SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId(format!("artifact:maintenance:{artifact_suffix}")),
        format!("basis:maintenance:{artifact_suffix}"),
        "cursor:maintenance",
        "checkpoint:maintenance",
        "compatibility:maintenance",
        "portability:maintenance",
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
        "structural:support-compatibility",
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

#[test]
fn catalog_rejects_family_role_mismatch() {
    let raw = RawSubscriptionSupportDeclaration::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::DegradedContinuation,
        SubscriptionSupportAuthority::WorthQuery,
        "worth-query-live-v1",
        SubscriptionSupportScope::from_canonical(vec!["feed:2".into()]).unwrap(),
        SubscriptionSupportPayloadDigest::new("payload:abc").unwrap(),
    );

    let error = SubscriptionSupportCatalog::first_ship()
        .admit(raw)
        .expect_err("role mismatch should reject before publication");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportAdmissionViolation
    );
}

#[test]
fn scope_rejects_noncanonical_order() {
    let error = SubscriptionSupportScope::from_canonical(vec!["z".into(), "a".into()])
        .expect_err("noncanonical declaration scopes must reject");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportAdmissionViolation
    );
}

#[test]
fn catalog_rejects_unadmitted_upstream_authority() {
    let raw = RawSubscriptionSupportDeclaration::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportAuthority::Unadmitted("external-test".into()),
        "worth-query-live-v1",
        SubscriptionSupportScope::from_canonical(vec!["feed:2".into()]).unwrap(),
        SubscriptionSupportPayloadDigest::new("payload:abc").unwrap(),
    );

    let error = SubscriptionSupportCatalog::first_ship()
        .admit(raw)
        .expect_err("unknown upstream authority must reject before publication");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportAdmissionViolation
    );
}

#[test]
fn catalog_declares_required_access_structures() {
    let report = SubscriptionSupportCatalog::first_ship().access_structures();

    assert_eq!(
        report.required(),
        &[
            SubscriptionSupportAccessStructure::FamilyLookup,
            SubscriptionSupportAccessStructure::ArtifactLookupByFamilyAndArtifact,
            SubscriptionSupportAccessStructure::DeclarationLookup,
            SubscriptionSupportAccessStructure::BasisLookup,
            SubscriptionSupportAccessStructure::CursorLookup,
            SubscriptionSupportAccessStructure::CheckpointLookup,
            SubscriptionSupportAccessStructure::CompatibilityLookup,
            SubscriptionSupportAccessStructure::ClassificationLookup,
            SubscriptionSupportAccessStructure::RestartManifestSequence,
        ]
    );
}

#[test]
fn artifact_identity_is_deterministic_and_family_bound() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let admitted = pipeline.admit(raw_exact()).unwrap();
    let left = pipeline
        .prepare_exact(
            admitted.clone(),
            "basis:1",
            "cursor:1",
            "checkpoint:1",
            "schema:1",
            "compatibility:1",
        )
        .unwrap();
    let right = pipeline
        .prepare_exact(
            admitted,
            "basis:1",
            "cursor:1",
            "checkpoint:1",
            "schema:1",
            "compatibility:1",
        )
        .unwrap();

    assert_eq!(left.artifact_id(), right.artifact_id());
    assert!(left
        .artifact_id()
        .as_str()
        .starts_with("subscription-support:basis-bound-continuation-support:"));
}

#[test]
fn classification_precedence_keeps_suppressed_causes() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let admitted = pipeline.admit(raw_exact()).unwrap();
    let publishable = pipeline
        .prepare_exact(
            admitted,
            "basis:1",
            "cursor:1",
            "checkpoint:1",
            "schema:1",
            "compatibility:1",
        )
        .unwrap();
    let published = pipeline.publish(publishable).unwrap();
    let report = pipeline
        .classify(
            &published,
            SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
            128,
            2,
            vec![
                SubscriptionSupportDriftCause::SubscriptionSupportCursorDrift,
                SubscriptionSupportDriftCause::SubscriptionSupportCompatibilityDrift,
                SubscriptionSupportDriftCause::SubscriptionSupportSessionMemoryMissing,
            ],
        )
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
            SubscriptionSupportDriftCause::SubscriptionSupportCursorDrift,
            SubscriptionSupportDriftCause::SubscriptionSupportSessionMemoryMissing,
        ]
    );
}

#[test]
fn budget_denial_happens_before_classification() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let admitted = pipeline.admit(raw_exact()).unwrap();
    let publishable = pipeline
        .prepare_exact(
            admitted,
            "basis:1",
            "cursor:1",
            "checkpoint:1",
            "schema:1",
            "compatibility:1",
        )
        .unwrap();
    let published = pipeline.publish(publishable).unwrap();

    let error = pipeline
        .classify(
            &published,
            SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
            32 * 1024,
            2,
            Vec::new(),
        )
        .expect_err("payload over budget should not classify");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
    assert_eq!(pipeline.counters().budget_denials(), 1);
}

#[test]
fn exact_handle_requires_exact_report() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let admitted = pipeline.admit(raw_exact()).unwrap();
    let publishable = pipeline
        .prepare_exact(
            admitted,
            "basis:1",
            "cursor:1",
            "checkpoint:1",
            "schema:1",
            "compatibility:1",
        )
        .unwrap();
    let published = pipeline.publish(publishable).unwrap();
    let report = pipeline
        .classify(
            &published,
            SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
            128,
            2,
            Vec::new(),
        )
        .unwrap();

    let handle = pipeline.exact_handle(&published, &report).unwrap();

    assert_eq!(handle.artifact_id(), published.artifact_id());
    assert_eq!(
        report.cost_surface(),
        SubscriptionSupportResultCostSurface::new(
            SubscriptionSupportPlanFamily::ExactResumeClassificationPlan,
            SubscriptionSupportDensityClass::SparseIdentityClassification,
            128,
            2,
            0,
            SubscriptionSupportAllocationScope::NoAllocation
        )
    );
}

#[test]
fn durable_records_are_materialized_from_admitted_artifacts_and_reports() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let admitted = pipeline.admit(raw_exact()).unwrap();
    let publishable = pipeline
        .prepare_exact(
            admitted,
            "basis:1",
            "cursor:1",
            "checkpoint:1",
            "schema:1",
            "compatibility:1",
        )
        .unwrap();
    let linkage_record = records::SubscriptionSupportLinkageRecord::from_publishable(&publishable);
    let published = pipeline.publish(publishable).unwrap();
    let artifact_record = records::SubscriptionSupportArtifactRecord::from_published(&published);
    let report = pipeline
        .classify(
            &published,
            SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
            128,
            2,
            Vec::new(),
        )
        .unwrap();
    let classification_record =
        records::SubscriptionSupportClassificationRecord::from_report(&report);
    let restart_record = records::SubscriptionSupportRestartRecord::new(&report, "shard-a")
        .expect("restart record shards must be explicit");

    assert_eq!(artifact_record.artifact_id(), published.artifact_id());
    assert!(serde_json::to_string(&linkage_record)
        .unwrap()
        .contains("compatibility_binding"));
    assert!(serde_json::to_string(&classification_record)
        .unwrap()
        .contains("cost_surface"));
    assert!(serde_json::to_string(&restart_record)
        .unwrap()
        .contains("shard-a"));
}

#[test]
fn resume_handles_reject_reports_for_other_artifacts() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let first = pipeline.admit(raw_exact()).unwrap();
    let first_publishable = pipeline
        .prepare_exact(
            first,
            "basis:1",
            "cursor:1",
            "checkpoint:1",
            "schema:1",
            "compatibility:1",
        )
        .unwrap();
    let first = pipeline.publish(first_publishable).unwrap();
    let second = pipeline.admit(raw_exact()).unwrap();
    let second_publishable = pipeline
        .prepare_exact(
            second,
            "basis:2",
            "cursor:2",
            "checkpoint:2",
            "schema:2",
            "compatibility:2",
        )
        .unwrap();
    let second = pipeline.publish(second_publishable).unwrap();
    let report = pipeline
        .classify(
            &first,
            SubscriptionSupportClassificationPlan::exact_sparse_identity().unwrap(),
            128,
            2,
            Vec::new(),
        )
        .unwrap();

    let error = pipeline
        .exact_handle(&second, &report)
        .expect_err("resume handles must not reuse another artifact's report");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
}

#[test]
fn placement_unavailable_is_cost_posture_not_degraded_resume_meaning() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let admitted = pipeline.admit(raw_degraded()).unwrap();
    let publishable = pipeline
        .prepare_exact(
            admitted,
            "basis:1",
            "cursor:1",
            "checkpoint:1",
            "schema:1",
            "compatibility:1",
        )
        .unwrap();
    let published = pipeline.publish(publishable).unwrap();
    let report = pipeline
        .classify(
            &published,
            SubscriptionSupportClassificationPlan::new(
                SubscriptionSupportPlanFamily::DegradedResumeClassificationPlan,
                SubscriptionSupportPayloadBudget::new(16 * 1024, 64).unwrap(),
                SubscriptionSupportAllocationScope::RestartShardBatch,
                SubscriptionSupportDensityClass::RestartShardBatchClassification,
                Some("shard-a".into()),
            )
            .unwrap(),
            128,
            2,
            vec![SubscriptionSupportDriftCause::SubscriptionSupportPlacementUnavailable],
        )
        .unwrap();

    assert_eq!(
        report.classification(),
        SubscriptionResumeClassification::Degraded
    );
    assert_eq!(
        report.cost_surface().plan_family(),
        SubscriptionSupportPlanFamily::DegradedResumeClassificationPlan
    );
}

#[test]
fn certification_bundle_is_digest_backed() {
    let catalog = SubscriptionSupportCatalog::first_ship();
    let report = SubscriptionSupportClassificationReport {
        artifact_id: SubscriptionSupportArtifactId("artifact:test".into()),
        declaration_digest: SubscriptionSupportDeclarationDigest("declaration:test".into()),
        classification: SubscriptionResumeClassification::Exact,
        primary_cause: None,
        suppressed_causes: Vec::new(),
        cost_surface: SubscriptionSupportResultCostSurface::new(
            SubscriptionSupportPlanFamily::ExactResumeClassificationPlan,
            SubscriptionSupportDensityClass::SparseIdentityClassification,
            1,
            1,
            0,
            SubscriptionSupportAllocationScope::NoAllocation,
        ),
        counter_snapshot: SubscriptionSupportCounterSnapshot::default(),
    };

    let bundle = SubscriptionSupportCertificationBundle::new(
        &catalog,
        SubscriptionSupportCounterSnapshot::default(),
        &[report],
    )
    .unwrap();

    assert_eq!(bundle.catalog_family_count(), 3);
    assert!(!bundle.classification_digest().is_empty());
    assert!(!bundle.failure_digest().is_empty());
}

#[test]
fn certification_bundle_failure_digest_changes_when_reports_fail() {
    let catalog = SubscriptionSupportCatalog::first_ship();
    let exact = SubscriptionSupportClassificationReport {
        artifact_id: SubscriptionSupportArtifactId("artifact:exact".into()),
        declaration_digest: SubscriptionSupportDeclarationDigest("declaration:exact".into()),
        classification: SubscriptionResumeClassification::Exact,
        primary_cause: None,
        suppressed_causes: Vec::new(),
        cost_surface: SubscriptionSupportResultCostSurface::new(
            SubscriptionSupportPlanFamily::ExactResumeClassificationPlan,
            SubscriptionSupportDensityClass::SparseIdentityClassification,
            1,
            1,
            0,
            SubscriptionSupportAllocationScope::NoAllocation,
        ),
        counter_snapshot: SubscriptionSupportCounterSnapshot::default(),
    };
    let denied = SubscriptionSupportClassificationReport {
        artifact_id: SubscriptionSupportArtifactId("artifact:denied".into()),
        declaration_digest: SubscriptionSupportDeclarationDigest("declaration:denied".into()),
        classification: SubscriptionResumeClassification::NotResumable,
        primary_cause: Some(SubscriptionSupportDriftCause::SubscriptionSupportBasisDrift),
        suppressed_causes: Vec::new(),
        cost_surface: SubscriptionSupportResultCostSurface::new(
            SubscriptionSupportPlanFamily::DeniedResumeClassificationPlan,
            SubscriptionSupportDensityClass::SparseIdentityClassification,
            1,
            1,
            0,
            SubscriptionSupportAllocationScope::NoAllocation,
        ),
        counter_snapshot: SubscriptionSupportCounterSnapshot::default(),
    };

    let exact_bundle = SubscriptionSupportCertificationBundle::new(
        &catalog,
        SubscriptionSupportCounterSnapshot::default(),
        &[exact.clone()],
    )
    .unwrap();
    let mixed_bundle = SubscriptionSupportCertificationBundle::new(
        &catalog,
        SubscriptionSupportCounterSnapshot::default(),
        &[exact, denied],
    )
    .unwrap();

    assert_ne!(exact_bundle.failure_digest(), mixed_bundle.failure_digest());
}

#[test]
fn phase_1_illegal_hot_path_does_not_claim_payload_budget_rejection() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let error = pipeline
        .admit_support_program_path(
            SupportPathClass::ForegroundResume,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 16).unwrap(),
            2,
            128,
        )
        .expect_err("illegal hot path must reject before budget accounting");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
    assert_eq!(pipeline.counters().support_hot_path_rejections(), 1);
    assert_eq!(
        pipeline.counters().support_payload_budget_rejection_count(),
        0
    );
    assert_eq!(pipeline.counters().budget_denials(), 0);
}

#[test]
fn phase_1_operational_verdicts_translate_through_proof_witnesses() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let input = pipeline
        .translate_operational_verdict(
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            operational_basis(SubscriptionSupportActionOrigin::Retention),
            None,
            None,
        )
        .unwrap();

    assert_eq!(
        input.classification(),
        SubscriptionResumeClassification::Exact
    );
    assert_eq!(
        input.operational_verdict(),
        SubscriptionSupportOperationalVerdict::ExactResumePreserved
    );
    assert_eq!(
        pipeline.counters().operational_verdict_translation_count(),
        1
    );

    let rebuild_error = pipeline
        .translate_operational_verdict(
            SubscriptionSupportOperationalVerdict::RebuildRequired,
            operational_basis(SubscriptionSupportActionOrigin::Maintenance),
            None,
            None,
        )
        .expect_err("rebuild-required verdicts require maintenance admission proof");

    assert_eq!(
        rebuild_error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
    assert_eq!(
        pipeline
            .counters()
            .operational_verdict_translation_rejections(),
        1
    );

    let missing_portability = SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId("artifact:missing-portability".into()),
        "basis:phase-1",
        "cursor:phase-1",
        "checkpoint:phase-1",
        "compatibility:phase-1",
        "",
        SubscriptionSupportActionOrigin::ReplicationImport,
    )
    .expect_err("exact operational bases must include portability proof");

    assert_eq!(
        missing_portability.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
}

#[test]
fn phase_1_support_actions_complete_only_after_publication() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let raw_action = RawSupportProgramAction::new(
        SupportActionId::new("support-action:phase-1").unwrap(),
        operational_basis(SubscriptionSupportActionOrigin::Compatibility),
        SubscriptionSupportOperationalVerdict::DegradedResumePreserved,
    )
    .unwrap();

    let completed = pipeline
        .publish_support_consequence(
            raw_action.plan().verify().execute(),
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
        )
        .unwrap()
        .complete();

    assert_eq!(
        completed.envelope().verdict(),
        SubscriptionSupportOperationalVerdict::DegradedResumePreserved
    );
    assert_eq!(
        completed.publication_witness().action_id().as_str(),
        "support-action:phase-1"
    );
    assert_eq!(
        pipeline.counters().support_action_envelope_publications(),
        1
    );
}

#[test]
fn phase_1_performance_paths_reject_hot_path_operational_work() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let budget = SupportActionBreadthBudget::new(4, 1024).unwrap();
    let plan = pipeline
        .admit_support_program_path(
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            budget,
            2,
            128,
        )
        .unwrap();

    assert_eq!(plan.path_class(), SupportPathClass::OperationalPlanning);
    assert_eq!(pipeline.counters().support_batch_receipt_reuse_count(), 0);
    let receipt = pipeline.reuse_support_batch_receipt(&plan).unwrap();
    assert_eq!(
        receipt.density_class(),
        SupportProgramDensityClass::FamilyLocalBatch
    );
    assert_eq!(pipeline.counters().support_batch_receipt_reuse_count(), 1);

    let hot_path_error = pipeline
        .admit_support_program_path(
            SupportPathClass::ForegroundResume,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            budget,
            1,
            64,
        )
        .expect_err("foreground resume must reject operational work");

    assert_eq!(
        hot_path_error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
    assert_eq!(pipeline.counters().support_hot_path_rejections(), 1);
}

#[test]
fn phase_2_retention_batch_publishes_exact_survival_before_completion() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let plan = pipeline
        .admit_support_retention_batch(
            SupportActionId::new("support-retention:exact").unwrap(),
            vec![retention_basis("exact-1"), retention_basis("exact-2")],
            SubscriptionSupportRetentionDecision::retain_exact(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            256,
        )
        .unwrap();

    assert_eq!(plan.affected_set().affected_count(), 2);
    assert_eq!(pipeline.counters().support_retention_plan_count(), 1);
    assert_eq!(pipeline.counters().support_retention_affected_entries(), 2);

    let report = pipeline
        .publish_support_retention_consequence(plan)
        .expect("retention completion must publish a support consequence");

    assert_eq!(
        report.survival_witness().verdict(),
        SubscriptionSupportOperationalVerdict::ExactResumePreserved
    );
    assert_eq!(report.survival_witness().affected_count(), 2);
    assert_eq!(report.retention_record().affected_count(), 2);
    assert_eq!(
        report.retention_record().decision_kind(),
        SubscriptionSupportRetentionDecisionKind::RetainExact
    );
    assert_eq!(
        report.retention_record().affected_set_digest(),
        report.survival_witness().affected_set_digest()
    );
    assert!(matches!(
        report.materialization(),
        SubscriptionSupportRetentionMaterialization::Retained(_)
    ));
    assert_eq!(
        report.completed_action().envelope().action_origin(),
        SubscriptionSupportActionOrigin::Retention
    );
    assert_eq!(
        pipeline.counters().support_action_envelope_publications(),
        1
    );
}

#[test]
fn phase_2_retention_materializes_degraded_compacted_and_expired_lanes() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let degraded_plan = pipeline
        .admit_support_retention_batch(
            SupportActionId::new("support-retention:degraded").unwrap(),
            vec![retention_basis("degraded")],
            SubscriptionSupportRetentionDecision::retain_degraded("cursor lineage was weakened")
                .unwrap(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();
    assert_eq!(pipeline.counters().support_retained_family_count(), 0);
    assert_eq!(pipeline.counters().support_compacted_basis_count(), 0);
    assert_eq!(pipeline.counters().support_expired_family_count(), 0);
    let degraded_report = pipeline
        .publish_support_retention_consequence(degraded_plan)
        .unwrap();
    assert_eq!(
        degraded_report.survival_witness().verdict(),
        SubscriptionSupportOperationalVerdict::DegradedResumePreserved
    );
    let SubscriptionSupportRetentionMaterialization::Retained(retained) =
        degraded_report.materialization()
    else {
        panic!("degraded retention must materialize retained support");
    };
    assert_eq!(
        retained.decision_kind(),
        SubscriptionSupportRetentionDecisionKind::RetainDegraded
    );
    assert_eq!(
        retained.weakened_condition(),
        Some("cursor lineage was weakened")
    );

    let compacted_plan = pipeline
        .admit_support_retention_batch(
            SupportActionId::new("support-retention:compacted").unwrap(),
            vec![
                retention_basis("compacted-1"),
                retention_basis("compacted-2"),
            ],
            SubscriptionSupportRetentionDecision::compact_exact("compacted-basis:digest").unwrap(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            256,
        )
        .unwrap();
    let compacted_report = pipeline
        .publish_support_retention_consequence(compacted_plan)
        .unwrap();
    let SubscriptionSupportRetentionMaterialization::Compacted(compacted) =
        compacted_report.materialization()
    else {
        panic!("compacted decision must materialize compacted support basis");
    };
    assert_eq!(compacted.compacted_basis_digest(), "compacted-basis:digest");
    assert_eq!(
        compacted_report.retention_record().decision_kind(),
        SubscriptionSupportRetentionDecisionKind::CompactExact
    );

    let expired_plan = pipeline
        .admit_support_retention_batch(
            SupportActionId::new("support-retention:expired").unwrap(),
            vec![retention_basis("expired")],
            SubscriptionSupportRetentionDecision::expire_by_policy("policy window expired")
                .unwrap(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();
    let expired_report = pipeline
        .publish_support_retention_consequence(expired_plan)
        .unwrap();
    let SubscriptionSupportRetentionMaterialization::Expired(expired) =
        expired_report.materialization()
    else {
        panic!("policy expiration must materialize expired support set");
    };
    assert_eq!(expired.policy_reason(), "policy window expired");
    assert_eq!(
        expired_report.survival_witness().verdict(),
        SubscriptionSupportOperationalVerdict::RejectedByPolicy
    );

    assert_eq!(pipeline.counters().support_retained_family_count(), 1);
    assert_eq!(pipeline.counters().support_compacted_basis_count(), 1);
    assert_eq!(pipeline.counters().support_expired_family_count(), 1);
    assert_eq!(pipeline.counters().support_policy_expiration_count(), 1);
}

#[test]
fn phase_2_reclaim_distinguishes_rebuildable_and_non_resumable_loss() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let rebuild_plan = pipeline
        .admit_support_retention_batch(
            SupportActionId::new("support-reclaim:rebuild").unwrap(),
            vec![retention_basis("rebuild")],
            SubscriptionSupportRetentionDecision::reclaim_with_rebuild(
                "retained-rebuild-basis:1",
                "maintenance-admission:1",
            )
            .unwrap(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();
    let rebuild_consequence = pipeline
        .publish_support_reclaim_consequence(rebuild_plan)
        .unwrap();

    assert_eq!(
        rebuild_consequence.survival_witness().verdict(),
        SubscriptionSupportOperationalVerdict::RebuildRequired
    );
    assert_eq!(
        rebuild_consequence.retention_record().decision_kind(),
        SubscriptionSupportRetentionDecisionKind::ReclaimWithRebuild
    );
    assert_eq!(
        rebuild_consequence
            .reclaimed_artifacts()
            .retained_rebuild_basis_digest(),
        Some("retained-rebuild-basis:1")
    );
    assert_eq!(
        rebuild_consequence
            .reclaimed_artifacts()
            .maintenance_admission_key(),
        Some("maintenance-admission:1")
    );
    assert!(matches!(
        rebuild_consequence
            .survival_witness()
            .affected_set_digest()
            .as_str(),
        digest if !digest.is_empty()
    ));

    let denied_plan = pipeline
        .admit_support_retention_batch(
            SupportActionId::new("support-reclaim:not-resumable").unwrap(),
            vec![retention_basis("not-resumable")],
            SubscriptionSupportRetentionDecision::reclaim_without_rebuild(
                "retained rebuild basis was reclaimed",
            )
            .unwrap(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();
    let denied_consequence = pipeline
        .publish_support_reclaim_consequence(denied_plan)
        .unwrap();

    assert_eq!(
        denied_consequence.survival_witness().verdict(),
        SubscriptionSupportOperationalVerdict::NotResumable
    );
    assert_eq!(
        denied_consequence.retention_record().decision_kind(),
        SubscriptionSupportRetentionDecisionKind::ReclaimWithoutRebuild
    );
    assert_eq!(
        denied_consequence
            .reclaimed_artifacts()
            .missing_rebuild_basis_reason(),
        Some("retained rebuild basis was reclaimed")
    );
    assert_eq!(pipeline.counters().support_reclaim_consequence_count(), 2);
    assert_eq!(pipeline.counters().support_reclaimed_family_count(), 2);
}

#[test]
fn phase_2_retention_rejects_mixed_family_origin_and_non_reclaim_completion() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let mixed_family_error = pipeline
        .admit_support_retention_batch(
            SupportActionId::new("support-retention:mixed-family").unwrap(),
            vec![
                retention_basis("family-a"),
                retention_basis_for_family(
                    "degraded-continuation-support",
                    SubscriptionSupportFamilyKind::DegradedContinuationSupport,
                    SubscriptionSupportRole::DegradedContinuation,
                    "family-b",
                    SubscriptionSupportActionOrigin::Retention,
                ),
            ],
            SubscriptionSupportRetentionDecision::retain_exact(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("retention affected sets must be family-local");
    assert_eq!(
        mixed_family_error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );

    let mixed_origin_error = pipeline
        .admit_support_retention_batch(
            SupportActionId::new("support-retention:mixed-origin").unwrap(),
            vec![
                retention_basis("origin-a"),
                retention_basis_for_family(
                    "basis-bound-continuation-support",
                    SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                    SubscriptionSupportRole::ExactContinuation,
                    "origin-b",
                    SubscriptionSupportActionOrigin::Compatibility,
                ),
            ],
            SubscriptionSupportRetentionDecision::retain_exact(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("retention affected sets must reject non-retention-origin bases");
    assert_eq!(
        mixed_origin_error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );

    let retain_plan = pipeline
        .admit_support_retention_batch(
            SupportActionId::new("support-retention:not-reclaim").unwrap(),
            vec![retention_basis("not-reclaim")],
            SubscriptionSupportRetentionDecision::retain_exact(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();
    let non_reclaim_error = pipeline
        .publish_support_reclaim_consequence(retain_plan)
        .expect_err("retain decisions cannot complete through reclaim consequence API");
    assert_eq!(
        non_reclaim_error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
}

#[test]
fn phase_2_retention_rejects_hot_path_and_store_global_sweeps() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let budget = SupportActionBreadthBudget::new(4, 1024).unwrap();

    let hot_path_error = pipeline
        .admit_support_retention_batch(
            SupportActionId::new("support-retention:hot-path").unwrap(),
            vec![retention_basis("hot-path")],
            SubscriptionSupportRetentionDecision::retain_exact(),
            SupportPathClass::ForegroundResume,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            budget,
            128,
        )
        .expect_err("foreground resume cannot run retention support planning");

    assert_eq!(
        hot_path_error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
    assert_eq!(pipeline.counters().support_hot_path_rejections(), 1);

    let store_global_error = pipeline
        .admit_support_retention_batch(
            SupportActionId::new("support-retention:global").unwrap(),
            vec![retention_basis("global")],
            SubscriptionSupportRetentionDecision::retain_exact(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::StoreGlobalDebt,
            SupportAllocationScope::FamilyLocalBatch,
            budget,
            128,
        )
        .expect_err("store-global support retention sweeps are explicit debt");

    assert_eq!(
        store_global_error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
    assert_eq!(
        pipeline.counters().support_store_global_debt_rejections(),
        1
    );
}

#[test]
fn phase_3_exact_compatibility_migration_requires_manifest_admission() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let plan = pipeline
        .admit_support_compatibility_batch(
            SupportActionId::new("support-compatibility:exact").unwrap(),
            vec![
                compatibility_basis("exact-a"),
                compatibility_basis("exact-b"),
            ],
            read_receipt_witness(CompatibilityRelation::Native),
            "semantic:compatibility:v2",
            SubscriptionSupportCompatibilityDecision::exact_compatible_migration(
                "classifier-equivalence:v1-v2",
            )
            .unwrap(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();

    assert_eq!(
        plan.manifest_admission().compatibility_digest(),
        "compatibility:manifest-v2"
    );
    assert_eq!(
        plan.semantic_access().admission_witness().manifest_digest(),
        plan.manifest_admission()
            .compatibility_receipt()
            .manifest_digest()
    );
    let receipt = pipeline
        .reuse_support_batch_receipt(plan.path_plan())
        .unwrap();
    assert_eq!(receipt.affected_entries(), 2);

    let report = pipeline
        .publish_support_compatibility_consequence(plan)
        .unwrap();
    let SubscriptionSupportCompatibilityOutcome::ExactMigrated(migration) = report.outcome() else {
        panic!("exact compatibility decision must materialize exact migration");
    };
    assert_eq!(
        migration.classifier_equivalence_digest(),
        "classifier-equivalence:v1-v2"
    );
    assert_eq!(
        report.participation_record().decision_kind(),
        SubscriptionSupportCompatibilityDecisionKind::ExactCompatibleMigration
    );
    assert_eq!(
        report.completed_action().envelope().verdict(),
        SubscriptionSupportOperationalVerdict::ExactResumePreserved
    );

    assert_eq!(pipeline.counters().support_compatibility_plan_count(), 1);
    assert_eq!(
        pipeline.counters().support_compatibility_affected_entries(),
        2
    );
    assert_eq!(pipeline.counters().support_manifest_admission_count(), 1);
    assert_eq!(
        pipeline
            .counters()
            .support_compatibility_receipt_binding_count(),
        1
    );
    assert_eq!(
        pipeline
            .counters()
            .support_exact_compatible_migration_count(),
        1
    );
    assert_eq!(pipeline.counters().support_batch_receipt_reuse_count(), 1);
}

#[test]
fn phase_3_degraded_and_rejected_compatibility_are_typed_outcomes() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let degraded_plan = pipeline
        .admit_support_compatibility_batch(
            SupportActionId::new("support-compatibility:degraded").unwrap(),
            vec![compatibility_basis("degraded")],
            read_receipt_witness(CompatibilityRelation::AdapterRequired),
            "semantic:compatibility:degraded",
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
    let degraded_report = pipeline
        .publish_support_compatibility_consequence(degraded_plan)
        .unwrap();
    let SubscriptionSupportCompatibilityOutcome::Degraded(degraded) = degraded_report.outcome()
    else {
        panic!("degraded compatibility decision must materialize degraded posture");
    };
    assert_eq!(
        degraded.drift_reason(),
        "classifier equivalence weakened by removed cursor hint"
    );
    assert_eq!(
        degraded_report.completed_action().envelope().verdict(),
        SubscriptionSupportOperationalVerdict::DegradedResumePreserved
    );

    let rejected_plan = pipeline
        .admit_support_compatibility_batch(
            SupportActionId::new("support-compatibility:old-reader").unwrap(),
            vec![compatibility_basis("old-reader")],
            rejected_read_outcome_witness(CompatibilityRejectionKind::ReaderCapabilityUnsupported),
            "semantic:compatibility:old-reader",
            SubscriptionSupportCompatibilityDecision::old_reader_rejected(1, 2).unwrap(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();
    let rejected_report = pipeline
        .publish_support_compatibility_consequence(rejected_plan)
        .unwrap();
    let SubscriptionSupportCompatibilityOutcome::Rejected(rejection) = rejected_report.outcome()
    else {
        panic!("old-reader compatibility must materialize typed rejection");
    };
    assert_eq!(
        rejection.rejection_kind(),
        SubscriptionSupportCompatibilityDecisionKind::OldReaderRejected
    );
    assert_eq!(
        rejection.milestone12_rejection_kind(),
        Some(CompatibilityRejectionKind::ReaderCapabilityUnsupported)
    );
    assert_eq!(
        rejected_report.completed_action().envelope().verdict(),
        SubscriptionSupportOperationalVerdict::RejectedByPolicy
    );
    assert_eq!(
        pipeline.counters().support_degraded_compatibility_count(),
        1
    );
    assert_eq!(
        pipeline.counters().support_version_skew_rejection_count(),
        1
    );
}

#[test]
fn phase_3_compatibility_rejects_wrong_origin_hot_path_and_bad_window() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let wrong_origin = pipeline
        .admit_support_compatibility_batch(
            SupportActionId::new("support-compatibility:wrong-origin").unwrap(),
            vec![retention_basis("wrong-origin")],
            rejected_read_outcome_witness(CompatibilityRejectionKind::MissingCompatibilityEdge),
            "semantic:compatibility:wrong-origin",
            SubscriptionSupportCompatibilityDecision::version_skew_rejected(
                "payload outside admitted reader window",
            )
            .unwrap(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("compatibility batches must reject retention-origin bases");
    assert_eq!(
        wrong_origin.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );

    let hot_path = pipeline
        .admit_support_compatibility_batch(
            SupportActionId::new("support-compatibility:hot-path").unwrap(),
            vec![compatibility_basis("hot-path")],
            read_receipt_witness(CompatibilityRelation::Native),
            "semantic:compatibility:hot-path",
            SubscriptionSupportCompatibilityDecision::exact_compatible_migration(
                "classifier-equivalence:hot-path",
            )
            .unwrap(),
            SupportPathClass::ForegroundRead,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("foreground read paths cannot run compatibility migration");
    assert_eq!(
        hot_path.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
    assert_eq!(pipeline.counters().support_hot_path_rejections(), 1);

    let bad_window = SupportFamilyVersionWindow::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        3,
        2,
    )
    .expect_err("version windows must be ordered");
    assert_eq!(
        bad_window.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
}

#[test]
fn phase_3_compatibility_rejects_decisions_not_backed_by_milestone_12_receipts() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let exact_from_adapter = pipeline
        .admit_support_compatibility_batch(
            SupportActionId::new("support-compatibility:exact-from-adapter").unwrap(),
            vec![compatibility_basis("exact-from-adapter")],
            read_receipt_witness(CompatibilityRelation::AdapterRequired),
            "semantic:compatibility:adapter",
            SubscriptionSupportCompatibilityDecision::exact_compatible_migration(
                "classifier-equivalence:adapter",
            )
            .unwrap(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("adapter-required Milestone 12 receipts cannot claim exact support migration");
    assert_eq!(
        exact_from_adapter.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );

    let rejected_from_accepted_receipt = pipeline
        .admit_support_compatibility_batch(
            SupportActionId::new("support-compatibility:rejected-from-accepted").unwrap(),
            vec![compatibility_basis("rejected-from-accepted")],
            read_receipt_witness(CompatibilityRelation::Native),
            "semantic:compatibility:accepted",
            SubscriptionSupportCompatibilityDecision::version_skew_rejected(
                "accepted receipt cannot support rejection",
            )
            .unwrap(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("accepted Milestone 12 receipts cannot support support rejection");
    assert_eq!(
        rejected_from_accepted_receipt.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
}

#[test]
fn phase_3_unknown_family_rejection_and_certification_rows_are_typed() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let plan = pipeline
        .admit_support_compatibility_batch(
            SupportActionId::new("support-compatibility:unknown-family").unwrap(),
            vec![compatibility_basis("unknown-family")],
            rejected_read_outcome_witness(CompatibilityRejectionKind::FamilyMismatch),
            "semantic:compatibility:unknown-family",
            SubscriptionSupportCompatibilityDecision::unknown_family_rejected(
                SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
            ),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();
    let report = pipeline
        .publish_support_compatibility_consequence(plan)
        .unwrap();
    let lane = SubscriptionSupportCertificationLaneOutcome::from_compatibility_report(
        SubscriptionSupportCertificationLaneKind::SupportCompatibilityUnknownFamilyRejected,
        &report,
        pipeline.counters(),
    )
    .unwrap();

    assert_eq!(
        lane.lane(),
        SubscriptionSupportCertificationLaneKind::SupportCompatibilityUnknownFamilyRejected
    );
    assert_eq!(lane.classification(), None);
    assert_eq!(
        pipeline.counters().support_version_skew_rejection_count(),
        1
    );
}

#[test]
fn phase_4_full_scope_replication_preserves_support_identity() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let plan = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:full-replication").unwrap(),
            vec![
                portability_basis(SubscriptionSupportActionOrigin::ReplicationExport, "full-a"),
                portability_basis(SubscriptionSupportActionOrigin::ReplicationExport, "full-b"),
            ],
            2,
            0,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::full_scope_replication(
                "support-identity:full",
                "support-identity:full",
            )
            .unwrap(),
            SupportPathClass::ReplicationExport,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            256,
        )
        .unwrap();

    assert_eq!(plan.footprint().included_support_count(), 2);
    assert_eq!(plan.footprint().omitted_support_count(), 0);
    assert_eq!(plan.manifest().manifest_entry_count(), 2);

    let report = pipeline
        .publish_support_portability_consequence(plan)
        .unwrap();
    let SubscriptionSupportPortabilityOutcome::FullScopeReplicated(bundle) = report.outcome()
    else {
        panic!("full-scope portability must materialize a replicated support bundle");
    };

    assert_eq!(bundle.preserved_count(), 2);
    assert_eq!(
        report.completed_action().envelope().verdict(),
        SubscriptionSupportOperationalVerdict::ExactResumePreserved
    );
    assert_eq!(
        report.participation_record().decision_kind(),
        SubscriptionSupportPortabilityDecisionKind::FullScopeReplication
    );
    assert_eq!(pipeline.counters().support_portability_plan_count(), 1);
    assert_eq!(
        pipeline.counters().support_portability_manifest_entries(),
        2
    );
    assert_eq!(pipeline.counters().support_replication_inclusion_count(), 2);
}

#[test]
fn phase_4_partial_replication_omission_cannot_report_exact_support() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let omitted = SubscriptionSupportArtifactId("artifact:portability:partial-b".into());
    let plan = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:partial").unwrap(),
            vec![
                portability_basis(
                    SubscriptionSupportActionOrigin::ReplicationExport,
                    "partial-a",
                ),
                portability_basis(
                    SubscriptionSupportActionOrigin::ReplicationExport,
                    "partial-b",
                ),
            ],
            1,
            1,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::partial_scope_omission(
                vec![omitted.clone()],
                "target capsule omits cold support artifact",
            )
            .unwrap(),
            SupportPathClass::ReplicationExport,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            256,
        )
        .unwrap();

    let report = pipeline
        .publish_support_portability_consequence(plan)
        .unwrap();
    let SubscriptionSupportPortabilityOutcome::PartialScopeOmitted(omission) = report.outcome()
    else {
        panic!("partial portability must publish an omission report");
    };

    assert_eq!(omission.omitted_count(), 1);
    assert_eq!(
        report.completed_action().envelope().verdict(),
        SubscriptionSupportOperationalVerdict::DegradedResumePreserved
    );
    assert_ne!(
        report.participation_record().verdict(),
        SubscriptionSupportOperationalVerdict::ExactResumePreserved
    );
    assert_eq!(
        pipeline
            .counters()
            .support_portability_omitted_support_count(),
        1
    );
    assert_eq!(pipeline.counters().support_replication_omission_count(), 1);
}

#[test]
fn phase_4_portability_rejects_identity_drift_and_invalid_omission_ids() {
    let identity_drift = SubscriptionSupportPortabilityDecision::full_scope_replication(
        "source-support-identity",
        "target-support-identity",
    )
    .expect_err(
        "full-scope replication must prove identity preservation, not just name identities",
    );
    assert_eq!(
        identity_drift.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );

    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let foreign_omission = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:foreign-omission").unwrap(),
            vec![
                portability_basis(
                    SubscriptionSupportActionOrigin::ReplicationExport,
                    "foreign-a",
                ),
                portability_basis(
                    SubscriptionSupportActionOrigin::ReplicationExport,
                    "foreign-b",
                ),
            ],
            1,
            1,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::partial_scope_omission(
                vec![SubscriptionSupportArtifactId(
                    "artifact:portability:not-in-scope".into(),
                )],
                "invalid omission report",
            )
            .unwrap(),
            SupportPathClass::ReplicationExport,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("partial omission reports must name artifacts from the admitted scope");
    assert_eq!(
        foreign_omission.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );

    let duplicate_id = SubscriptionSupportArtifactId("artifact:portability:duplicate-a".into());
    let duplicate_omission = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:duplicate-omission").unwrap(),
            vec![
                portability_basis(
                    SubscriptionSupportActionOrigin::ReplicationExport,
                    "duplicate-a",
                ),
                portability_basis(
                    SubscriptionSupportActionOrigin::ReplicationExport,
                    "duplicate-b",
                ),
            ],
            0,
            2,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::partial_scope_omission(
                vec![duplicate_id.clone(), duplicate_id],
                "duplicate omission report",
            )
            .unwrap(),
            SupportPathClass::ReplicationExport,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("partial omission reports must not duplicate omitted artifacts");
    assert_eq!(
        duplicate_omission.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
}

#[test]
fn phase_4_target_import_requires_admission_before_semantic_access() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let plan = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:import").unwrap(),
            vec![portability_basis(
                SubscriptionSupportActionOrigin::ReplicationImport,
                "import-a",
            )],
            1,
            0,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::target_import_admitted(
                "target-import-admission",
                "source-identity-preservation:import-a",
                "imported-support-semantic",
            )
            .unwrap(),
            SupportPathClass::ImportAdmission,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();

    let report = pipeline
        .publish_support_portability_consequence(plan)
        .unwrap();
    let SubscriptionSupportPortabilityOutcome::Imported(access) = report.outcome() else {
        panic!("target import admission must produce semantic access");
    };

    assert_eq!(
        access.import_admission().manifest_digest(),
        report.manifest().manifest_digest()
    );
    assert_eq!(
        access
            .import_admission()
            .source_identity_preservation_digest(),
        Some("source-identity-preservation:import-a")
    );
    assert_eq!(
        access.imported_semantic_digest(),
        "imported-support-semantic"
    );
    assert_eq!(
        report.completed_action().envelope().action_origin(),
        SubscriptionSupportActionOrigin::ReplicationImport
    );
    assert_eq!(pipeline.counters().support_import_admission_count(), 1);
}

#[test]
fn phase_4_capsule_import_missing_basis_reports_not_resumable_without_semantic_access() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let present_basis =
        SubscriptionSupportArtifactId("artifact:portability:missing-basis-a".into());
    let plan = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:missing-basis-import").unwrap(),
            vec![
                portability_basis(
                    SubscriptionSupportActionOrigin::ReplicationImport,
                    "missing-basis-a",
                ),
                portability_basis(
                    SubscriptionSupportActionOrigin::ReplicationImport,
                    "missing-basis-b",
                ),
            ],
            2,
            0,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::target_import_missing_basis_not_resumable(
                "target-import-admission:missing-basis",
                vec![present_basis],
                "capsule omitted required basis evidence for one imported support artifact",
            )
            .unwrap(),
            SupportPathClass::ImportAdmission,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();

    assert_eq!(plan.manifest().manifest_entry_count(), 2);
    assert_eq!(plan.manifest().required_basis_count(), 1);

    let report = pipeline
        .publish_support_portability_consequence(plan)
        .unwrap();
    let SubscriptionSupportPortabilityOutcome::ImportedNotResumable(denial) = report.outcome()
    else {
        panic!("missing import basis must publish a typed not-resumable import report");
    };

    assert_eq!(denial.missing_basis_count(), 1);
    assert_eq!(
        denial.import_admission().manifest_digest(),
        report.manifest().manifest_digest()
    );
    assert_eq!(
        denial
            .import_admission()
            .source_identity_preservation_digest(),
        None
    );
    assert_eq!(
        report.completed_action().envelope().verdict(),
        SubscriptionSupportOperationalVerdict::NotResumable
    );
    assert_eq!(
        report.participation_record().decision_kind(),
        SubscriptionSupportPortabilityDecisionKind::TargetImportMissingBasisNotResumable
    );
    assert_eq!(
        pipeline
            .counters()
            .support_portability_required_basis_count(),
        1
    );
    assert_eq!(pipeline.counters().support_import_admission_count(), 1);
}

#[test]
fn phase_4_capsule_import_missing_basis_rejects_WORTHd_basis_membership() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let foreign_basis = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:missing-basis-foreign").unwrap(),
            vec![
                portability_basis(
                    SubscriptionSupportActionOrigin::ReplicationImport,
                    "basis-foreign-a",
                ),
                portability_basis(
                    SubscriptionSupportActionOrigin::ReplicationImport,
                    "basis-foreign-b",
                ),
            ],
            2,
            0,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::target_import_missing_basis_not_resumable(
                "target-import-admission:foreign-basis",
                vec![SubscriptionSupportArtifactId(
                    "artifact:portability:not-in-import-scope".into(),
                )],
                "WORTHd basis evidence",
            )
            .unwrap(),
            SupportPathClass::ImportAdmission,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("import basis evidence must be scope-local");
    assert_eq!(
        foreign_basis.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );

    let duplicate_basis_id =
        SubscriptionSupportArtifactId("artifact:portability:basis-dup-a".into());
    let duplicate_basis = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:missing-basis-duplicate").unwrap(),
            vec![
                portability_basis(
                    SubscriptionSupportActionOrigin::ReplicationImport,
                    "basis-dup-a",
                ),
                portability_basis(
                    SubscriptionSupportActionOrigin::ReplicationImport,
                    "basis-dup-b",
                ),
            ],
            2,
            0,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::target_import_missing_basis_not_resumable(
                "target-import-admission:duplicate-basis",
                vec![duplicate_basis_id.clone(), duplicate_basis_id],
                "duplicate basis evidence",
            )
            .unwrap(),
            SupportPathClass::ImportAdmission,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("import basis evidence must not duplicate artifact ids");
    assert_eq!(
        duplicate_basis.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
}

#[test]
fn phase_4_portability_rejects_oversized_manifest_before_materialization() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let error = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:oversized").unwrap(),
            vec![
                portability_basis(SubscriptionSupportActionOrigin::ReplicationExport, "big-a"),
                portability_basis(SubscriptionSupportActionOrigin::ReplicationExport, "big-b"),
            ],
            2,
            0,
            SupportPortabilityManifestBudget::new(1, 64).unwrap(),
            SubscriptionSupportPortabilityDecision::full_scope_replication(
                "support-identity:oversized",
                "support-identity:oversized",
            )
            .unwrap(),
            SupportPathClass::ReplicationExport,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("oversized capsule support manifests must reject before payload decode");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
    assert_eq!(pipeline.counters().support_portability_plan_count(), 0);
    assert_eq!(
        pipeline
            .counters()
            .support_capsule_manifest_budget_denial_count(),
        1
    );
}

#[test]
fn phase_4_unsupported_family_portability_rejects_typed() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let plan = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:unsupported-family").unwrap(),
            vec![portability_basis(
                SubscriptionSupportActionOrigin::ReplicationImport,
                "unsupported-family",
            )],
            0,
            1,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::unsupported_family_rejected(
                "target store does not admit this support family",
            )
            .unwrap(),
            SupportPathClass::ImportAdmission,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();
    let report = pipeline
        .publish_support_portability_consequence(plan)
        .unwrap();
    let SubscriptionSupportPortabilityOutcome::Rejected(rejection) = report.outcome() else {
        panic!("unsupported support portability must publish a typed rejection");
    };

    assert_eq!(
        rejection.rejection_kind(),
        SubscriptionSupportPortabilityDecisionKind::UnsupportedFamilyRejected
    );
    assert_eq!(
        report.completed_action().envelope().verdict(),
        SubscriptionSupportOperationalVerdict::RejectedByPolicy
    );
    assert_eq!(pipeline.counters().support_import_rejection_count(), 1);
}

#[test]
fn phase_4_portability_rejects_wrong_origin_hot_path_and_non_portability_density() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let wrong_origin = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:wrong-origin").unwrap(),
            vec![retention_basis("wrong-origin")],
            1,
            0,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::full_scope_replication(
                "support-identity:wrong-origin",
                "support-identity:wrong-origin",
            )
            .unwrap(),
            SupportPathClass::ReplicationExport,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("portability affected sets cannot consume retention-origin bases");
    assert_eq!(
        wrong_origin.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );

    let hot_path = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:hot-path").unwrap(),
            vec![portability_basis(
                SubscriptionSupportActionOrigin::ReplicationExport,
                "hot-path",
            )],
            1,
            0,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::full_scope_replication(
                "support-identity:hot-path",
                "support-identity:hot-path",
            )
            .unwrap(),
            SupportPathClass::ForegroundRead,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("foreground reads cannot plan support portability");
    assert_eq!(
        hot_path.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
    assert_eq!(pipeline.counters().support_hot_path_rejections(), 1);

    let import_decision_on_export_origin = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:import-on-export").unwrap(),
            vec![portability_basis(
                SubscriptionSupportActionOrigin::ReplicationExport,
                "import-on-export",
            )],
            1,
            0,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::target_import_admitted(
                "target-import-admission",
                "source-identity-preservation:import-on-export",
                "imported-semantic",
            )
            .unwrap(),
            SupportPathClass::ImportAdmission,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("import decisions cannot ride export-origin affected sets");
    assert_eq!(
        import_decision_on_export_origin.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );

    let export_decision_on_import_origin = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:export-on-import").unwrap(),
            vec![portability_basis(
                SubscriptionSupportActionOrigin::ReplicationImport,
                "export-on-import",
            )],
            1,
            0,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::full_scope_replication(
                "support-identity:export-on-import",
                "support-identity:export-on-import",
            )
            .unwrap(),
            SupportPathClass::ReplicationExport,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("export decisions cannot ride import-origin affected sets");
    assert_eq!(
        export_decision_on_import_origin.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );

    let wrong_density = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:wrong-density").unwrap(),
            vec![portability_basis(
                SubscriptionSupportActionOrigin::ReplicationExport,
                "wrong-density",
            )],
            1,
            0,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::full_scope_replication(
                "support-identity:wrong-density",
                "support-identity:wrong-density",
            )
            .unwrap(),
            SupportPathClass::ReplicationExport,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("portability plans require portability-scope density");
    assert_eq!(
        wrong_density.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
}

#[test]
fn phase_5_maintenance_rebuild_descriptor_is_admitted_and_coalesced_by_key() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let basis = maintenance_basis("rebuild");
    let retained_basis_digest = basis.basis_digest().to_string();
    let plan = pipeline
        .admit_support_maintenance_batch(
            SupportActionId::new("support-maintenance:rebuild").unwrap(),
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

    assert_eq!(plan.affected_set().affected_count(), 2);
    assert_eq!(plan.descriptors().len(), 1);
    assert_eq!(plan.coalesced_duplicate_count(), 1);
    assert_eq!(
        plan.maintenance_receipt().batch_summary().batch_class(),
        crate::MaintenanceBatchClass::SubscriptionSupport
    );
    assert_eq!(
        plan.descriptors()[0].work_kind(),
        SupportMaintenanceWorkKind::Rebuild
    );

    let report = pipeline
        .publish_support_maintenance_consequence(plan)
        .unwrap();

    assert_eq!(report.admissions().len(), 1);
    assert_eq!(report.descriptor_records().len(), 1);
    assert_eq!(
        report.completed_action().envelope().verdict(),
        SubscriptionSupportOperationalVerdict::RebuildRequired
    );
    assert_eq!(
        report.descriptor_records()[0].declaration_id(),
        report.admissions()[0].declaration_id()
    );
    assert_eq!(
        report.participation_record().decision_kind(),
        SubscriptionSupportMaintenanceDecisionKind::RebuildDescriptorAdmitted
    );
    assert_eq!(report.participation_record().coalesced_duplicate_count(), 1);
    assert_eq!(
        pipeline.counters().support_maintenance_descriptor_count(),
        1
    );
    assert_eq!(
        pipeline
            .counters()
            .support_maintenance_coalesced_duplicate_count(),
        1
    );
    assert_eq!(
        pipeline.counters().support_maintenance_rebuild_debt_count(),
        1
    );
}

#[test]
fn phase_5_maintenance_rejects_missing_basis_wrong_path_and_wrong_density() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let missing_basis = pipeline
        .admit_support_maintenance_batch(
            SupportActionId::new("support-maintenance:missing-basis").unwrap(),
            vec![maintenance_basis("missing-basis")],
            SubscriptionSupportMaintenanceDecision::rebuild_descriptor_admitted(
                "basis:maintenance:other",
            )
            .unwrap(),
            SupportPathClass::MaintenanceExecution,
            SupportProgramDensityClass::MaintenanceKeyBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("rebuild descriptors must consume matching retained basis evidence");
    assert_eq!(
        missing_basis.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );

    let hot_path = pipeline
        .admit_support_maintenance_batch(
            SupportActionId::new("support-maintenance:hot-path").unwrap(),
            vec![maintenance_basis("hot-path")],
            SubscriptionSupportMaintenanceDecision::refresh_descriptor_admitted(
                "stale support refresh",
            )
            .unwrap(),
            SupportPathClass::ForegroundRead,
            SupportProgramDensityClass::MaintenanceKeyBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("foreground reads cannot admit support maintenance work");
    assert_eq!(
        hot_path.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
    assert_eq!(pipeline.counters().support_hot_path_rejections(), 1);

    let wrong_density = pipeline
        .admit_support_maintenance_batch(
            SupportActionId::new("support-maintenance:wrong-density").unwrap(),
            vec![maintenance_basis("wrong-density")],
            SubscriptionSupportMaintenanceDecision::refresh_descriptor_admitted(
                "stale support refresh",
            )
            .unwrap(),
            SupportPathClass::MaintenanceExecution,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("support maintenance must use maintenance-key density");
    assert_eq!(
        wrong_density.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
}

#[test]
fn phase_5_maintenance_refresh_migration_degradation_and_restart_publish_typed_posture() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();

    let refresh = pipeline
        .admit_support_maintenance_batch(
            SupportActionId::new("support-maintenance:refresh").unwrap(),
            vec![maintenance_basis("refresh")],
            SubscriptionSupportMaintenanceDecision::refresh_descriptor_admitted(
                "support refresh keeps exact posture",
            )
            .unwrap(),
            SupportPathClass::MaintenanceExecution,
            SupportProgramDensityClass::MaintenanceKeyBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();
    let refresh = pipeline
        .publish_support_maintenance_consequence(refresh)
        .unwrap();
    assert_eq!(
        refresh.completed_action().envelope().verdict(),
        SubscriptionSupportOperationalVerdict::ExactResumePreserved
    );
    assert_eq!(pipeline.counters().support_maintenance_refresh_count(), 1);

    let migration = pipeline
        .admit_support_maintenance_batch(
            SupportActionId::new("support-maintenance:migration").unwrap(),
            vec![maintenance_basis("migration")],
            SubscriptionSupportMaintenanceDecision::compatibility_migration_descriptor_admitted(
                "compatibility-migration:exact",
            )
            .unwrap(),
            SupportPathClass::MaintenanceExecution,
            SupportProgramDensityClass::MaintenanceKeyBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();
    let migration = pipeline
        .publish_support_maintenance_consequence(migration)
        .unwrap();
    assert_eq!(
        migration.participation_record().decision_kind(),
        SubscriptionSupportMaintenanceDecisionKind::CompatibilityMigrationDescriptorAdmitted
    );
    assert_eq!(
        pipeline
            .counters()
            .support_maintenance_compatibility_migration_count(),
        1
    );

    let degradation = pipeline
        .admit_support_maintenance_batch(
            SupportActionId::new("support-maintenance:degradation").unwrap(),
            vec![maintenance_basis("degradation")],
            SubscriptionSupportMaintenanceDecision::degradation_recovery_descriptor_admitted(
                "degraded support recovery remains degraded",
            )
            .unwrap(),
            SupportPathClass::MaintenanceExecution,
            SupportProgramDensityClass::MaintenanceKeyBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();
    let degradation = pipeline
        .publish_support_maintenance_consequence(degradation)
        .unwrap();
    assert_eq!(
        degradation.completed_action().envelope().verdict(),
        SubscriptionSupportOperationalVerdict::DegradedResumePreserved
    );
    assert_eq!(
        pipeline
            .counters()
            .support_maintenance_degradation_recovery_count(),
        1
    );

    let recovered = pipeline
        .admit_support_maintenance_batch(
            SupportActionId::new("support-maintenance:restart").unwrap(),
            vec![maintenance_basis("restart")],
            SubscriptionSupportMaintenanceDecision::interrupted_restart_recovered(
                SupportMaintenanceWorkKind::Rebuild,
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
    assert!(recovered.descriptors()[0]
        .descriptor()
        .recovered_from_restart());
    let recovered = pipeline
        .publish_support_maintenance_consequence(recovered)
        .unwrap();
    assert_eq!(
        recovered.participation_record().decision_kind(),
        SubscriptionSupportMaintenanceDecisionKind::InterruptedRestartRecovered
    );
    assert_eq!(
        pipeline
            .counters()
            .support_maintenance_interrupted_restart_recovery_count(),
        1
    );
}

#[test]
fn phase_5_maintenance_delay_reports_debt_without_mutating_truth() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let basis = maintenance_basis("delayed");
    let retained_basis_digest = basis.basis_digest().to_string();
    let plan = pipeline
        .admit_support_maintenance_batch(
            SupportActionId::new("support-maintenance:delayed").unwrap(),
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

    let report = pipeline
        .report_delayed_support_maintenance(
            &plan,
            "maintenance lane deferred by batch pacing",
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();

    assert_eq!(
        report.debt_summary().verdict(),
        SubscriptionSupportOperationalVerdict::RebuildRequired
    );
    assert_eq!(
        report.debt_summary().work_kind(),
        SupportMaintenanceWorkKind::Rebuild
    );
    assert_eq!(
        report.debt_summary().delay_reason(),
        "maintenance lane deferred by batch pacing"
    );
    assert_eq!(report.admissions().len(), 1);
    assert_eq!(
        report.cost_surface().allocation_scope(),
        crate::SubscriptionSupportAllocationScope::OperatorReport
    );
    assert_eq!(pipeline.counters().support_maintenance_delay_count(), 1);
    assert_eq!(
        pipeline.counters().support_action_envelope_publications(),
        0
    );
}
