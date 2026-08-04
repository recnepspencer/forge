use super::SubscriptionSupportCounterSnapshot;

impl SubscriptionSupportCounterSnapshot {
    pub(crate) fn record_access_structure_debt(&mut self) {
        self.access_structure_debts += 1;
    }

    pub(crate) fn record_admitted(&mut self) {
        self.declarations_admitted += 1;
    }

    pub(crate) fn record_rejected(&mut self) {
        self.declarations_rejected += 1;
    }

    pub(crate) fn record_published(&mut self) {
        self.artifacts_published += 1;
    }

    pub(crate) fn record_fetch(&mut self, lookup_keys: u64, rows_read: u64) {
        self.artifacts_fetched += 1;
        self.lookup_keys_used += lookup_keys;
        self.rows_read += rows_read;
    }

    pub(crate) fn record_family_catalog_lookup(&mut self) {
        self.family_catalog_lookups += 1;
    }

    pub(crate) fn record_duplicate_retry(&mut self) {
        self.duplicate_retries += 1;
    }

    pub(crate) fn record_identity_collision(&mut self) {
        self.identity_collisions += 1;
    }

    pub(crate) fn record_malformed_support_record(&mut self) {
        self.malformed_support_records += 1;
    }

    pub(crate) fn record_budget_denial(&mut self) {
        self.budget_denials += 1;
        self.support_payload_budget_rejection_count += 1;
    }
}
