use worth_query_installation::facade::ApplicationSchema;
use worth_relational::facade::{history::RelationalCommitReceipt, transactions::RecordRef};

use super::lifecycle::WorthQueryConditionalOperationRegistry;
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema + 'static,
{
    /// Publishes the commit into the bounded invalidation journal and updates
    /// the derived temporal-intent index before the committing operation
    /// returns. Clock observation therefore consumes only derived state.
    pub(in crate::domain_computation::primary_graph) fn maintain_conditional_commit(
        &self,
        commit: &RelationalCommitReceipt,
        records: Vec<RecordRef>,
    ) {
        let changed_kinds = self
            .primary_provider
            .record_conditional_commit(commit, records);
        if changed_kinds.is_empty() {
            return;
        }
        let mut owners = ConditionalCommitMaintenanceOwners::take(self);
        let Some((registry, bridge)) = owners.roots_mut() else {
            // A conditional operation committing its own effect owns these
            // roots in the re-entry lane and advances its local index there.
            return;
        };
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            registry.refresh_changed_intent_kinds(self, bridge, &changed_kinds)
        }));
        match outcome {
            Ok(Ok(())) => self
                .primary_provider
                .clear_conditional_maintenance_failure(),
            Ok(Err(denial)) => self
                .primary_provider
                .record_conditional_maintenance_failure(denial.subject()),
            Err(_) => self
                .primary_provider
                .record_conditional_maintenance_failure(
                    "conditional commit-time intent maintenance panicked",
                ),
        }
    }
}

struct ConditionalCommitMaintenanceOwners<'runtime, Schema> {
    runtime: &'runtime WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    registry: Option<WorthQueryConditionalOperationRegistry<Schema>>,
    bridge: Option<worth_runtime_bridge::facade::BridgeOwnedSignalRuntime>,
}

impl<'runtime, Schema> ConditionalCommitMaintenanceOwners<'runtime, Schema> {
    fn take(runtime: &'runtime WorthQueryPrimaryGraphApplicationRuntime<Schema>) -> Self {
        let registry = Some(std::mem::take(
            &mut *runtime
                .conditional_operations
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner),
        ));
        let bridge = runtime.bridge.take_conditional_if_present();
        Self {
            runtime,
            registry,
            bridge,
        }
    }

    fn roots_mut(
        &mut self,
    ) -> Option<(
        &mut WorthQueryConditionalOperationRegistry<Schema>,
        &mut worth_runtime_bridge::facade::BridgeOwnedSignalRuntime,
    )> {
        Some((self.registry.as_mut()?, self.bridge.as_mut()?))
    }
}

impl<Schema> Drop for ConditionalCommitMaintenanceOwners<'_, Schema> {
    fn drop(&mut self) {
        if let Some(bridge) = self.bridge.take() {
            self.runtime.bridge.restore_conditional_shared(bridge);
        }
        if let Some(registry) = self.registry.take() {
            *self
                .runtime
                .conditional_operations
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner) = registry;
        }
    }
}
