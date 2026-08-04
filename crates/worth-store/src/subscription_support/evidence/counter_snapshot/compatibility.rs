use super::SubscriptionSupportCounterSnapshot;

impl SubscriptionSupportCounterSnapshot {
    pub(crate) fn record_support_compatibility_plan(&mut self, affected_entries: u64) {
        self.support_compatibility_plan_count += 1;
        self.support_compatibility_affected_entries += affected_entries;
        self.support_manifest_admission_count += 1;
        self.support_compatibility_receipt_binding_count += 1;
    }

    pub(crate) fn record_support_exact_compatible_migration(&mut self) {
        self.support_exact_compatible_migration_count += 1;
    }

    pub(crate) fn record_support_degraded_compatibility(&mut self) {
        self.support_degraded_compatibility_count += 1;
    }

    pub(crate) fn record_support_version_skew_rejection(&mut self) {
        self.support_version_skew_rejection_count += 1;
    }
}
