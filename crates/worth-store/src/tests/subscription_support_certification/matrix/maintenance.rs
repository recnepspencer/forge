use super::super::{
    maintenance_basis, SubscriptionSupportCertificationLaneKind,
    SubscriptionSupportCertificationLaneOutcome, SubscriptionSupportMaintenanceDecision,
    SupportActionBreadthBudget, SupportActionId, SupportAllocationScope, SupportPathClass,
    SupportProgramDensityClass, WORTHStoreBuilder,
};
use super::evidence::CertificationMatrixEvidence;

pub(super) fn record_maintenance(evidence: &mut CertificationMatrixEvidence) {
    let maintenance_rebuild = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
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
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_maintenance_report(
            SubscriptionSupportCertificationLaneKind::SupportMaintenanceRebuildAdmitted,
            &maintenance_rebuild.0,
            maintenance_rebuild.1.clone(),
        )
        .unwrap(),
    );
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_maintenance_report(
            SubscriptionSupportCertificationLaneKind::SupportMaintenanceKeyBatchBounded,
            &maintenance_rebuild.0,
            maintenance_rebuild.1,
        )
        .unwrap(),
    );

    let maintenance_refresh = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
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
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_maintenance_report(
            SubscriptionSupportCertificationLaneKind::SupportMaintenanceRefreshAdmitted,
            &maintenance_refresh.0,
            maintenance_refresh.1,
        )
        .unwrap(),
    );

    let maintenance_migration = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
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
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_maintenance_report(
            SubscriptionSupportCertificationLaneKind::SupportMaintenanceCompatibilityMigrationAdmitted,
            &maintenance_migration.0,
            maintenance_migration.1,
        )
        .unwrap(),
    );

    let maintenance_degradation = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
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
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_maintenance_report(
            SubscriptionSupportCertificationLaneKind::SupportMaintenanceDegradationRecoveryAdmitted,
            &maintenance_degradation.0,
            maintenance_degradation.1,
        )
        .unwrap(),
    );

    let maintenance_recovered = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
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
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_maintenance_report(
            SubscriptionSupportCertificationLaneKind::SupportMaintenanceInterruptedRestartRecovered,
            &maintenance_recovered.0,
            maintenance_recovered.1,
        )
        .unwrap(),
    );

    let maintenance_delayed = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let basis = maintenance_basis("delayed");
        let retained_basis_digest = basis.basis_digest().to_string();
        let plan = store
            .admit_subscription_support_maintenance_batch(
                SupportActionId::new("support-maintenance:cert-delayed").unwrap(),
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
            .report_delayed_subscription_support_maintenance(
                &plan,
                "maintenance lane deferred by operator pacing",
                SupportActionBreadthBudget::new(4, 1024).unwrap(),
                128,
            )
            .unwrap();
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_maintenance_debt_report(
            SubscriptionSupportCertificationLaneKind::SupportMaintenanceDelayedDebtReported,
            &maintenance_delayed.0,
            maintenance_delayed.1,
        )
        .unwrap(),
    );

    let maintenance_coalesced = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
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
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_maintenance_report(
            SubscriptionSupportCertificationLaneKind::SupportMaintenanceCoalescedRebuildAdmitted,
            &maintenance_coalesced.0,
            maintenance_coalesced.1,
        )
        .unwrap(),
    );
}
