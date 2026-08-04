use super::super::{
    fetched_exact_report, publish_exact, raw_degraded, raw_materialized, unique_test_sqlite_path,
    SubscriptionResumeClassification, SubscriptionSupportAllocationScope,
    SubscriptionSupportCertificationLaneKind, SubscriptionSupportCertificationLaneOutcome,
    SubscriptionSupportClassificationPlan, SubscriptionSupportDensityClass,
    SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind, SubscriptionSupportFetchRequest,
    SubscriptionSupportMissingSupportMaintenanceAdmission,
    SubscriptionSupportMissingSupportRecoveryRequest, SubscriptionSupportPayloadBudget,
    SubscriptionSupportPlanFamily, SubscriptionSupportRestartReconstructionRequest,
    SubscriptionSupportRestartShard, SubscriptionSupportResumeEvidence,
    SubscriptionSupportResumeRequest, SubscriptionSupportRole, WORTHStoreBuilder,
};
use super::evidence::CertificationMatrixEvidence;

pub(super) fn record_exact_recovery(evidence: &mut CertificationMatrixEvidence) {
    let exact_report =
        fetched_exact_report(&mut WORTHStoreBuilder::new().in_memory().build().unwrap());
    assert_eq!(
        exact_report.classification(),
        SubscriptionResumeClassification::Exact
    );
    assert_eq!(exact_report.cost_surface().decoded_payload_bytes(), 128);
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::ExactResumeControl,
            &exact_report,
        )
        .unwrap(),
    );
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::ResultCostSurfaceExact,
            &exact_report,
        )
        .unwrap(),
    );
    evidence.record_classification_report(exact_report);

    let path = unique_test_sqlite_path("worth-store-subscription-support-certification-restart");
    let artifact_id = {
        let mut store = WORTHStoreBuilder::new()
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
    let mut reopened = WORTHStoreBuilder::new().sqlite_file(path).build().unwrap();
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
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::RestartExactResume,
            &restart_report.reports()[0],
        )
        .unwrap(),
    );
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::RestartShardBoundedReconstruction,
            &restart_report.reports()[0],
        )
        .unwrap(),
    );
    evidence.record_classification_report(restart_report.reports()[0].clone());

    let mut rebuild_store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let missing_artifact_id = {
        let mut source = WORTHStoreBuilder::new().in_memory().build().unwrap();
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
                    crate::SupportActionBreadthBudget::new(1, 1024).unwrap(),
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
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_missing_support_recovery(
            SubscriptionSupportCertificationLaneKind::RebuildRequiredMissingSupport,
            &rebuild_report,
            rebuild_store.subscription_support_counters(),
        )
        .unwrap(),
    );

    let degraded_report = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
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
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_classification_report(
            SubscriptionSupportCertificationLaneKind::DegradedButRecoverable,
            &degraded_report,
        )
        .unwrap(),
    );
    evidence.record_classification_report(degraded_report);
}
