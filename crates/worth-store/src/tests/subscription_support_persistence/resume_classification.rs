use super::{
    StoreErrorKind, SubscriptionResumeClassification, SubscriptionSupportAllocationScope,
    SubscriptionSupportDensityClass, SubscriptionSupportDriftCause, SubscriptionSupportFamilyId,
    SubscriptionSupportFamilyKind, SubscriptionSupportFetchRequest,
    SubscriptionSupportMissingSupportRecoveryRequest, SubscriptionSupportPayloadBudget,
    SubscriptionSupportPlanFamily, SubscriptionSupportResumeEvidence,
    SubscriptionSupportResumeRequest, SubscriptionSupportRole, WORTHStoreBuilder,
};

use super::{raw_degraded, raw_exact, raw_materialized};

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
