use super::super::{
    classification, new, pipeline, records, SubscriptionResumeClassification,
    SubscriptionSupportAllocationScope, SubscriptionSupportArtifactRecord,
    SubscriptionSupportClassificationPlan, SubscriptionSupportClassificationRecord,
    SubscriptionSupportClassificationViolation, SubscriptionSupportDensityClass,
    SubscriptionSupportDriftCause, SubscriptionSupportLinkageRecord,
    SubscriptionSupportPayloadBudget, SubscriptionSupportPlanFamily,
    SubscriptionSupportPublicationPipeline, SubscriptionSupportRestartRecord,
    SubscriptionSupportResultCostSurface,
};
use super::StoreErrorKind;
use super::{raw_degraded, raw_exact};

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
