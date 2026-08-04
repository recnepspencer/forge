use super::SubscriptionSupportCounterSnapshot;

impl SubscriptionSupportCounterSnapshot {
    pub(crate) fn record_restart_reconstruction(&mut self, shards_touched: u64) {
        self.restart_reconstruction_count += 1;
        self.restart_shards_touched += shards_touched;
    }

    pub(crate) fn record_rebuild_basis_plan(&mut self) {
        self.rebuild_basis_plan_count += 1;
    }

    pub(crate) fn record_runtime_handoff(&mut self) {
        self.runtime_handoff_count += 1;
    }

    pub(crate) fn record_operational_verdict_translation(&mut self) {
        self.operational_verdict_translation_count += 1;
    }

    pub(crate) fn record_operational_verdict_translation_rejection(&mut self) {
        self.operational_verdict_translation_rejections += 1;
    }

    pub(crate) fn record_support_action_envelope_publication(&mut self) {
        self.support_action_envelope_publications += 1;
    }

    pub(crate) fn record_support_action_recovery(&mut self) {
        self.support_action_recovery_count += 1;
    }

    pub(crate) fn record_support_hot_path_rejection(&mut self) {
        self.support_hot_path_rejections += 1;
    }

    pub(crate) fn record_support_global_scan_recovery_rejection(&mut self) {
        self.support_global_scan_recovery_rejection_count += 1;
    }

    pub(crate) fn record_support_batch_receipt_reuse(&mut self) {
        self.support_batch_receipt_reuse_count += 1;
    }

    pub(crate) fn record_support_store_global_debt_rejection(&mut self) {
        self.support_store_global_debt_rejections += 1;
    }
}
