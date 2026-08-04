use super::super::{
    catalog, classification, new, pipeline, RawSupportProgramAction,
    SubscriptionResumeClassification, SubscriptionSupportActionOrigin,
    SubscriptionSupportAllocationScope, SubscriptionSupportArtifactId, SubscriptionSupportCatalog,
    SubscriptionSupportCertificationBundle, SubscriptionSupportClassificationReport,
    SubscriptionSupportClassificationViolation, SubscriptionSupportCounterSnapshot,
    SubscriptionSupportDeclarationDigest, SubscriptionSupportDensityClass,
    SubscriptionSupportDriftCause, SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
    SubscriptionSupportOperationalBasis, SubscriptionSupportOperationalVerdict,
    SubscriptionSupportPlanFamily, SubscriptionSupportPublicationPipeline,
    SubscriptionSupportResultCostSurface, SubscriptionSupportRole, SupportActionBreadthBudget,
    SupportActionId, SupportAllocationScope, SupportPathClass, SupportProgramDensityClass,
};
use super::operational_basis;
use super::StoreErrorKind;

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
