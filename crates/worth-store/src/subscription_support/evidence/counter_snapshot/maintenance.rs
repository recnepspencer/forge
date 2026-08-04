use super::SubscriptionSupportCounterSnapshot;

impl SubscriptionSupportCounterSnapshot {
    pub(crate) fn record_support_maintenance_plan(
        &mut self,
        descriptor_count: u64,
        coalesced_duplicate_count: u64,
    ) {
        self.support_maintenance_descriptor_count += descriptor_count;
        self.support_maintenance_coalesced_duplicate_count += coalesced_duplicate_count;
    }

    pub(crate) fn record_support_maintenance_delay_report(&mut self) {
        self.support_maintenance_delay_count += 1;
    }

    pub(crate) fn record_support_maintenance_rebuild_descriptor(&mut self) {
        self.support_maintenance_rebuild_debt_count += 1;
    }

    pub(crate) fn record_support_maintenance_refresh_descriptor(&mut self) {
        self.support_maintenance_refresh_count += 1;
    }

    pub(crate) fn record_support_maintenance_compatibility_migration_descriptor(&mut self) {
        self.support_maintenance_compatibility_migration_count += 1;
    }

    pub(crate) fn record_support_maintenance_degradation_recovery_descriptor(&mut self) {
        self.support_maintenance_degradation_recovery_count += 1;
    }

    pub(crate) fn record_support_maintenance_interrupted_restart_recovery(&mut self) {
        self.support_maintenance_interrupted_restart_recovery_count += 1;
    }
}
