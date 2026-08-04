use super::super::{
    maintenance_basis, portability_basis, publish_exact, retention_basis, unique_test_sqlite_path,
    StoreErrorKind, SubscriptionSupportActionOrigin, SubscriptionSupportCertificationLaneKind,
    SubscriptionSupportCertificationLaneOutcome, SubscriptionSupportClassificationPlan,
    SubscriptionSupportCounterSnapshot, SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
    SubscriptionSupportFetchRequest, SubscriptionSupportMaintenanceDecision,
    SubscriptionSupportPortabilityDecision, SubscriptionSupportResumeEvidence,
    SubscriptionSupportResumeRequest, SubscriptionSupportRetentionDecision,
    SupportActionBreadthBudget, SupportActionId, SupportAllocationScope, SupportPathClass,
    SupportPortabilityManifestBudget, SupportProgramDensityClass, WORTHStoreBuilder,
};
use super::evidence::CertificationMatrixEvidence;

pub(super) fn record_budgets(evidence: &mut CertificationMatrixEvidence) {
    let oversized_payload_error = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
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

        let path_budget_error = store
            .admit_subscription_support_program_path(
                SupportPathClass::OperationalPlanning,
                SupportProgramDensityClass::FamilyLocalBatch,
                SupportAllocationScope::FamilyLocalBatch,
                SupportActionBreadthBudget::new(4, 64).unwrap(),
                2,
                128,
            )
            .expect_err("oversized support path must reject before admission");
        assert_eq!(
            path_budget_error.kind(),
            &StoreErrorKind::SubscriptionSupportClassificationViolation
        );

        let manifest_budget_error = store
            .admit_subscription_support_portability_batch(
                SupportActionId::new("support-portability:cert-manifest-budget").unwrap(),
                vec![
                    portability_basis(
                        SubscriptionSupportActionOrigin::ReplicationExport,
                        "manifest-budget-a",
                    ),
                    portability_basis(
                        SubscriptionSupportActionOrigin::ReplicationExport,
                        "manifest-budget-b",
                    ),
                ],
                2,
                0,
                SupportPortabilityManifestBudget::new(1, 64).unwrap(),
                SubscriptionSupportPortabilityDecision::full_scope_replication(
                    "identity-preserved:manifest-budget",
                    "identity-preserved:manifest-budget",
                )
                .unwrap(),
                SupportPathClass::OperationalPlanning,
                SupportProgramDensityClass::PortabilityScopeBatch,
                SupportAllocationScope::PortabilityManifest,
                SupportActionBreadthBudget::new(4, 1024).unwrap(),
                128,
            )
            .expect_err("oversized portability manifest must reject before materialization");
        assert_eq!(
            manifest_budget_error.kind(),
            &StoreErrorKind::SubscriptionSupportClassificationViolation
        );

        let plan = store
            .admit_subscription_support_retention_batch(
                SupportActionId::new("support-retention:cert-envelope-budget").unwrap(),
                vec![retention_basis("envelope-budget")],
                SubscriptionSupportRetentionDecision::retain_exact(),
                SupportPathClass::OperationalPlanning,
                SupportProgramDensityClass::FamilyLocalBatch,
                SupportAllocationScope::FamilyLocalBatch,
                SupportActionBreadthBudget::new(4, 16).unwrap(),
                8,
            )
            .unwrap();
        let envelope_budget_error = store
            .publish_subscription_support_retention_consequence(plan)
            .expect_err("oversized publication envelope must reject before materialization");
        assert_eq!(
            envelope_budget_error.kind(),
            &StoreErrorKind::SubscriptionSupportClassificationViolation
        );

        let delayed_plan = store
            .admit_subscription_support_maintenance_batch(
                SupportActionId::new("support-maintenance:cert-operator-budget").unwrap(),
                vec![maintenance_basis("operator-budget")],
                SubscriptionSupportMaintenanceDecision::refresh_descriptor_admitted(
                    "operator budget refresh",
                )
                .unwrap(),
                SupportPathClass::MaintenanceExecution,
                SupportProgramDensityClass::MaintenanceKeyBatch,
                SupportAllocationScope::FamilyLocalBatch,
                SupportActionBreadthBudget::new(4, 1024).unwrap(),
                128,
            )
            .unwrap();
        let operator_report_budget_error = store
            .report_delayed_subscription_support_maintenance(
                &delayed_plan,
                "operator-report-budget",
                SupportActionBreadthBudget::new(4, 32).unwrap(),
                128,
            )
            .expect_err("oversized operator report must reject before materialization");
        assert_eq!(
            operator_report_budget_error.kind(),
            &StoreErrorKind::SubscriptionSupportClassificationViolation
        );

        (error, store.subscription_support_counters())
    };
    assert_eq!(
        oversized_payload_error.0.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
    assert_eq!(oversized_payload_error.1.budget_denials(), 4);
    assert_eq!(
        oversized_payload_error
            .1
            .support_payload_budget_rejection_count(),
        5
    );
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_typed_rejection(
            SubscriptionSupportCertificationLaneKind::OversizedPayloadRejectedBeforeDecode,
            oversized_payload_error.0.kind().clone(),
            oversized_payload_error.1,
        )
        .unwrap(),
    );

    let access_debt_outcome = {
        let path = unique_test_sqlite_path("worth-store-subscription-support-certification-debt");
        let artifact_id = {
            let mut store = WORTHStoreBuilder::new()
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
        let mut reopened = WORTHStoreBuilder::new().sqlite_file(path).build().unwrap();
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
    evidence.record_lane_outcome(access_debt_outcome);

    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_typed_rejection(
            SubscriptionSupportCertificationLaneKind::DecodedRowPublicationRejected,
            StoreErrorKind::SubscriptionSupportPublicationViolation,
            SubscriptionSupportCounterSnapshot::default(),
        )
        .unwrap(),
    );
}
