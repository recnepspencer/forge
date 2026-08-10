use crate::{SubscriptionSupportCounterSnapshot, SubscriptionSupportMaintenanceDecisionKind};

pub(super) fn record_maintenance_decision_counters(
    counters: &mut SubscriptionSupportCounterSnapshot,
    decision_kind: SubscriptionSupportMaintenanceDecisionKind,
) {
    match decision_kind {
        SubscriptionSupportMaintenanceDecisionKind::RebuildDescriptorAdmitted => {
            counters.record_support_maintenance_rebuild_descriptor();
        }
        SubscriptionSupportMaintenanceDecisionKind::RefreshDescriptorAdmitted => {
            counters.record_support_maintenance_refresh_descriptor();
        }
        SubscriptionSupportMaintenanceDecisionKind::CompatibilityMigrationDescriptorAdmitted => {
            counters.record_support_maintenance_compatibility_migration_descriptor();
        }
        SubscriptionSupportMaintenanceDecisionKind::DegradationRecoveryDescriptorAdmitted => {
            counters.record_support_maintenance_degradation_recovery_descriptor();
        }
        SubscriptionSupportMaintenanceDecisionKind::InterruptedRestartRecovered => {
            counters.record_support_maintenance_interrupted_restart_recovery();
        }
    }
}
