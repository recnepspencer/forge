use super::worth_ui_durable_resize_reconciliation_support::initial_mounted_resize_input;

pub(crate) struct WorthUiInitialMountedReconciliationPlanner;

impl WorthUiInitialMountedReconciliationPlanner {
    pub(crate) fn reconcile(
        artifact: &crate::source::WorthUiArtifact,
        artifact_digest: u64,
    ) -> crate::runtime::WorthUiDurableStateReconciliationPlan {
        let scan =
            crate::runtime::replacement::artifact_durable_state_definition::durable_resize_definitions(
                artifact,
            );
        let mut counters = crate::runtime::WorthUiDurableStateReconciliationCounters::default();
        counters.record_initial_artifact_nodes(scan.node_visits());
        let inputs = scan
            .definitions()
            .iter()
            .map(|definition| {
                counters.record_initialized_resize_input();
                initial_mounted_resize_input(definition)
            })
            .collect();
        crate::runtime::WorthUiDurableStateReconciliationPlan::initial_mounted(
            artifact_digest,
            inputs,
            counters,
        )
    }
}
