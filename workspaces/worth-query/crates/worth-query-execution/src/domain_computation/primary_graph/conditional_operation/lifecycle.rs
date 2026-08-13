use std::collections::BTreeMap;

use worth_runtime_bridge::facade::BridgeManagedClockBinding;

pub(in crate::domain_computation::primary_graph) trait WorthQueryInstalledConditionalOperation {
    fn binding_identity(&self) -> &str;
}

pub(in crate::domain_computation::primary_graph) struct WorthQueryInstalledTemporalOperation<
    Binding,
> {
    pub(super) binding: Binding,
    pub(super) managed_clock: BridgeManagedClockBinding,
}

impl<Binding> WorthQueryInstalledConditionalOperation
    for WorthQueryInstalledTemporalOperation<Binding>
{
    fn binding_identity(&self) -> &str {
        self.managed_clock.binding_identity()
    }
}

pub(in crate::domain_computation::primary_graph) struct WorthQueryConditionalOperationRegistry<
    Schema,
> {
    installed: BTreeMap<String, Box<dyn WorthQueryInstalledConditionalOperation>>,
    marker: std::marker::PhantomData<fn() -> Schema>,
}

impl<Schema> Default for WorthQueryConditionalOperationRegistry<Schema> {
    fn default() -> Self {
        Self {
            installed: BTreeMap::new(),
            marker: std::marker::PhantomData,
        }
    }
}

impl<Schema> WorthQueryConditionalOperationRegistry<Schema> {
    pub(super) fn install(
        &mut self,
        operation: Box<dyn WorthQueryInstalledConditionalOperation>,
    ) -> Result<(), ()> {
        let identity = operation.binding_identity().to_string();
        if self.installed.contains_key(&identity) {
            return Err(());
        }
        self.installed.insert(identity, operation);
        Ok(())
    }

    pub(super) fn len(&self) -> usize {
        self.installed.len()
    }
}
