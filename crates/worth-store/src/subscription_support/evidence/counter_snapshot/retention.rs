use super::SubscriptionSupportCounterSnapshot;

impl SubscriptionSupportCounterSnapshot {
    pub(crate) fn record_support_retention_plan(&mut self, affected_entries: u64) {
        self.support_retention_plan_count += 1;
        self.support_retention_affected_entries += affected_entries;
    }

    pub(crate) fn record_support_retained_family(&mut self) {
        self.support_retained_family_count += 1;
    }

    pub(crate) fn record_support_reclaimed_family(&mut self) {
        self.support_reclaimed_family_count += 1;
    }

    pub(crate) fn record_support_compacted_basis(&mut self) {
        self.support_compacted_basis_count += 1;
    }

    pub(crate) fn record_support_expired_family(&mut self) {
        self.support_expired_family_count += 1;
    }

    pub(crate) fn record_support_reclaim_consequence(&mut self) {
        self.support_reclaim_consequence_count += 1;
    }

    pub(crate) fn record_support_policy_expiration(&mut self) {
        self.support_policy_expiration_count += 1;
    }
}
