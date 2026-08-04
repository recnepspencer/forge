use super::SubscriptionSupportCounterSnapshot;

impl SubscriptionSupportCounterSnapshot {
    pub(crate) fn record_support_portability_plan(
        &mut self,
        manifest_entries: u64,
        required_basis_count: u64,
        omitted_support_count: u64,
    ) {
        self.support_portability_plan_count += 1;
        self.support_portability_manifest_entries += manifest_entries;
        self.support_portability_required_basis_count += required_basis_count;
        self.support_portability_omitted_support_count += omitted_support_count;
    }

    pub(crate) fn record_support_replication_inclusion(&mut self, included_support_count: u64) {
        self.support_replication_inclusion_count += included_support_count;
    }

    pub(crate) fn record_support_replication_omission(&mut self, omitted_support_count: u64) {
        self.support_replication_omission_count += omitted_support_count;
    }

    pub(crate) fn record_support_import_admission(&mut self) {
        self.support_import_admission_count += 1;
    }

    pub(crate) fn record_support_import_rejection(&mut self) {
        self.support_import_rejection_count += 1;
    }

    pub(crate) fn record_support_capsule_manifest_budget_denial(&mut self) {
        self.support_capsule_manifest_budget_denial_count += 1;
        self.support_payload_budget_rejection_count += 1;
    }
}
