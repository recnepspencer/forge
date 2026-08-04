use super::super::{
    classification, new, pipeline, SubscriptionSupportCertificationLaneKind,
    SubscriptionSupportCertificationLaneOutcome, SubscriptionSupportClassificationViolation,
    SubscriptionSupportCompatibilityDecision, SubscriptionSupportCompatibilityDecisionKind,
    SubscriptionSupportCompatibilityOutcome, SubscriptionSupportFamilyId,
    SubscriptionSupportFamilyKind, SubscriptionSupportOperationalVerdict,
    SubscriptionSupportPublicationPipeline, SupportActionBreadthBudget, SupportActionId,
    SupportAllocationScope, SupportFamilyVersionWindow, SupportPathClass,
    SupportProgramDensityClass,
};
use super::StoreErrorKind;
use super::{
    compatibility_basis, read_receipt_witness, rejected_read_outcome_witness, retention_basis,
};
use super::{CompatibilityRejectionKind, CompatibilityRelation};

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
