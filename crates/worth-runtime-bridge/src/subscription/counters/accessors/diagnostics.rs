use super::super::BridgeSubscriptionCounters;

impl BridgeSubscriptionCounters {
    pub fn diagnostics_bundle_count(&self) -> usize {
        self.values.diagnostics_bundle_count
    }

    pub fn subscription_diagnostics_reference_emit_count(&self) -> usize {
        self.values.subscription_diagnostics_reference_emit_count
    }

    pub fn subscription_rich_diagnostics_hot_path_materialization_count(&self) -> usize {
        self.values
            .subscription_rich_diagnostics_hot_path_materialization_count
    }
}
