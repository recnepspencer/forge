use super::super::{
    retention_basis, RawSupportProgramAction, StoreErrorKind,
    SubscriptionSupportCertificationLaneKind, SubscriptionSupportCertificationLaneOutcome,
    SupportActionBreadthBudget, SupportActionId, SupportActionRecoveryDisposition,
    SupportAllocationScope, SupportBatchProofKind, SupportPathClass, SupportProgramDensityClass,
    WORTHStoreBuilder,
};
use super::evidence::CertificationMatrixEvidence;

pub(super) fn record_pipeline(evidence: &mut CertificationMatrixEvidence) {
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
    let receipt_reuse_report = bounded_pipeline
        .verify_support_batch_receipt_reuse(
            &plan,
            vec![
                SupportBatchProofKind::CompatibilityReceipt,
                SupportBatchProofKind::BasisEvidence,
                SupportBatchProofKind::CursorCheckpointEvidence,
                SupportBatchProofKind::PortabilityScopeEvidence,
            ],
        )
        .unwrap();
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
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_typed_rejection(
            SubscriptionSupportCertificationLaneKind::SupportForegroundOperationalWorkRejected,
            foreground_error.kind().clone(),
            bounded_counters.clone(),
        )
        .unwrap(),
    );
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_typed_rejection(
            SubscriptionSupportCertificationLaneKind::SupportStoreGlobalDensityRejected,
            store_global_error.kind().clone(),
            bounded_counters.clone(),
        )
        .unwrap(),
    );
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_batch_receipt_reuse_report(
            SubscriptionSupportCertificationLaneKind::SupportBatchReceiptReuseVerified,
            &receipt_reuse_report,
            bounded_counters,
        )
        .unwrap(),
    );

    let action_publication_recovery = {
        let path = crate::tests::harness::fixtures::stores::unique_test_store_path(
            "worth-store-subscription-support-certification-action-recovery",
        );
        let action_id = SupportActionId::new("support-retention:cert-crash-recovery").unwrap();
        {
            let mut store = WORTHStoreBuilder::new()
                .local_file(path.clone())
                .build()
                .unwrap();
            let executed = RawSupportProgramAction::new(
                action_id.clone(),
                retention_basis("crash-recovery"),
                crate::SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            )
            .unwrap()
            .plan()
            .verify()
            .execute();
            store
                .persist_subscription_support_executed_action_for_publication(executed)
                .unwrap();
        }
        let mut reopened = WORTHStoreBuilder::new().local_file(path).build().unwrap();
        let report = reopened
            .recover_subscription_support_action_publication(action_id)
            .unwrap();
        assert_eq!(
            report.recovery_disposition(),
            SupportActionRecoveryDisposition::InterruptedBeforePublication
        );
        let counters = reopened.subscription_support_counters();
        (report, counters)
    };
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_action_publication_recovery(
            SubscriptionSupportCertificationLaneKind::SupportActionPublicationCrashRecovered,
            &action_publication_recovery.0,
            action_publication_recovery.1,
        )
        .unwrap(),
    );

    let global_scan_recovery_forbidden = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let error = store
            .reject_subscription_support_global_scan_recovery()
            .expect_err("global scan recovery must remain forbidden");
        let counters = store.subscription_support_counters();
        (error, counters)
    };
    assert_eq!(
        global_scan_recovery_forbidden.0.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
    assert_eq!(
        global_scan_recovery_forbidden
            .1
            .support_global_scan_recovery_rejection_count(),
        1
    );
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_typed_rejection(
            SubscriptionSupportCertificationLaneKind::SupportGlobalScanRecoveryForbidden,
            global_scan_recovery_forbidden.0.kind().clone(),
            global_scan_recovery_forbidden.1,
        )
        .unwrap(),
    );
}
