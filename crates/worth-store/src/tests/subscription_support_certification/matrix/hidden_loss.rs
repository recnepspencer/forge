use super::super::{
    compatibility_basis, maintenance_basis, portability_basis, read_receipt_witness,
    retention_basis, CompatibilityRelation, StoreErrorKind,
    SubscriptionSupportCertificationLaneKind, SubscriptionSupportCertificationLaneOutcome,
    SubscriptionSupportCompatibilityDecision, SubscriptionSupportMaintenanceDecision,
    SubscriptionSupportOperationalVerdictTranslationRequest,
    SubscriptionSupportPortabilityDecision, SubscriptionSupportRetentionDecision,
    SupportActionBreadthBudget, SupportActionId, SupportAllocationScope, SupportPathClass,
    SupportPortabilityManifestBudget, SupportProgramDensityClass, WORTHStoreBuilder,
};
use super::evidence::CertificationMatrixEvidence;

pub(super) fn record_hidden_loss(evidence: &mut CertificationMatrixEvidence) {
    let hidden_exact_loss_counters = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();

        let retention_plan = store
            .admit_subscription_support_retention_batch(
                SupportActionId::new("support-retention:cert-hidden-exact").unwrap(),
                vec![retention_basis("hidden-exact-retention")],
                SubscriptionSupportRetentionDecision::expire_by_policy(
                    "policy-expired:hidden-exact",
                )
                .unwrap(),
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
        let retention_error = store
            .translate_subscription_support_operational_verdict(
                SubscriptionSupportOperationalVerdictTranslationRequest::exact_from_retention_report(
                    &retention_report,
                    retention_basis("hidden-exact-retention"),
                )
                .unwrap(),
            )
            .expect_err("expired retention support must not translate to exact");
        assert_eq!(
            retention_error.kind(),
            &StoreErrorKind::SubscriptionSupportClassificationViolation
        );

        let compatibility_plan = store
            .admit_subscription_support_compatibility_batch(
                SupportActionId::new("support-compatibility:cert-hidden-exact").unwrap(),
                vec![compatibility_basis("hidden-exact-compatibility")],
                read_receipt_witness(CompatibilityRelation::AdapterRequired),
                "semantic:hidden-exact-compatibility",
                SubscriptionSupportCompatibilityDecision::degraded_compatibility(
                    "compatibility drift hidden exact guard",
                )
                .unwrap(),
                SupportPathClass::OperationalPlanning,
                SupportProgramDensityClass::FamilyLocalBatch,
                SupportAllocationScope::FamilyLocalBatch,
                SupportActionBreadthBudget::new(4, 1024).unwrap(),
                128,
            )
            .unwrap();
        let compatibility_report = store
            .publish_subscription_support_compatibility_consequence(compatibility_plan)
            .unwrap();
        let compatibility_error = store
            .translate_subscription_support_operational_verdict(
                SubscriptionSupportOperationalVerdictTranslationRequest::exact_from_compatibility_report(
                    &compatibility_report,
                    compatibility_basis("hidden-exact-compatibility"),
                )
                .unwrap(),
            )
            .expect_err("degraded compatibility support must not translate to exact");
        assert_eq!(
            compatibility_error.kind(),
            &StoreErrorKind::SubscriptionSupportClassificationViolation
        );

        let omitted_id = portability_basis(
            crate::SubscriptionSupportActionOrigin::ReplicationExport,
            "hidden-exact-portability-b",
        )
        .artifact_id()
        .clone();
        let portability_plan = store
            .admit_subscription_support_portability_batch(
                SupportActionId::new("support-portability:cert-hidden-exact").unwrap(),
                vec![
                    portability_basis(
                        crate::SubscriptionSupportActionOrigin::ReplicationExport,
                        "hidden-exact-portability-a",
                    ),
                    portability_basis(
                        crate::SubscriptionSupportActionOrigin::ReplicationExport,
                        "hidden-exact-portability-b",
                    ),
                ],
                1,
                1,
                SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
                SubscriptionSupportPortabilityDecision::partial_scope_omission(
                    vec![omitted_id],
                    "hidden exact portability omission",
                )
                .unwrap(),
                SupportPathClass::ReplicationExport,
                SupportProgramDensityClass::PortabilityScopeBatch,
                SupportAllocationScope::PortabilityManifest,
                SupportActionBreadthBudget::new(4, 1024).unwrap(),
                128,
            )
            .unwrap();
        let portability_report = store
            .publish_subscription_support_portability_consequence(portability_plan)
            .unwrap();
        let portability_error = store
            .translate_subscription_support_operational_verdict(
                SubscriptionSupportOperationalVerdictTranslationRequest::exact_from_portability_report(
                    &portability_report,
                    portability_basis(
                        crate::SubscriptionSupportActionOrigin::ReplicationExport,
                        "hidden-exact-portability-a",
                    ),
                )
                .unwrap(),
            )
            .expect_err("partial portability omission must not translate to exact");
        assert_eq!(
            portability_error.kind(),
            &StoreErrorKind::SubscriptionSupportClassificationViolation
        );

        let maintenance_plan = store
            .admit_subscription_support_maintenance_batch(
                SupportActionId::new("support-maintenance:cert-hidden-exact").unwrap(),
                vec![maintenance_basis("hidden-exact-maintenance")],
                SubscriptionSupportMaintenanceDecision::degradation_recovery_descriptor_admitted(
                    "maintenance hidden exact guard",
                )
                .unwrap(),
                SupportPathClass::MaintenanceExecution,
                SupportProgramDensityClass::MaintenanceKeyBatch,
                SupportAllocationScope::FamilyLocalBatch,
                SupportActionBreadthBudget::new(4, 1024).unwrap(),
                128,
            )
            .unwrap();
        let maintenance_report = store
            .publish_subscription_support_maintenance_consequence(maintenance_plan)
            .unwrap();
        let maintenance_error = store
            .translate_subscription_support_operational_verdict(
                SubscriptionSupportOperationalVerdictTranslationRequest::exact_from_maintenance_report(
                    &maintenance_report,
                    maintenance_basis("hidden-exact-maintenance"),
                )
                .unwrap(),
            )
            .expect_err("degraded maintenance recovery must not translate to exact");
        assert_eq!(
            maintenance_error.kind(),
            &StoreErrorKind::SubscriptionSupportClassificationViolation
        );

        let counters = store.subscription_support_counters();
        assert_eq!(counters.operational_verdict_translation_rejections(), 4);
        assert_eq!(counters.support_hidden_exact_loss_count(), 0);
        counters
    };
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_typed_rejection(
            SubscriptionSupportCertificationLaneKind::SupportHiddenExactLossForbidden,
            StoreErrorKind::SubscriptionSupportClassificationViolation,
            hidden_exact_loss_counters,
        )
        .unwrap(),
    );
}
