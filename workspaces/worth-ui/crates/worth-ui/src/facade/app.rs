use worth_ui_runtime::{
    facade::{
        phase3_unsupported_receipt, CapabilitySnapshot, UiInspectionQuery, UiInspectionReceipt,
        UiInspectionScopeInventory, WorthUiDslPackage, WorthUiHostContract,
        WorthUiRuntimeSupportInventory, WorthUiRuntimeSupportInventoryFields,
        WorthUiRuntimeSupportRow,
    },
    WorthUiCapabilityRegistrationFreezeCore,
};

struct WorthUiFacadeLifecycleState {
    _dsl_package: WorthUiDslPackage,
    _host_contract: WorthUiHostContract,
    inspection_scope_inventory: UiInspectionScopeInventory,
    runtime_support_inventory: WorthUiRuntimeSupportInventory,
}

/// Worth UI application after capability registration has frozen.
pub struct WorthUiApp {
    capability_snapshot: CapabilitySnapshot,
    lifecycle: WorthUiFacadeLifecycleState,
}

impl WorthUiApp {
    pub(crate) fn from_freeze_core(core: WorthUiCapabilityRegistrationFreezeCore) -> Self {
        let (capability_snapshot, dsl_package, host_contract) = core.into_parts();
        let lifecycle = WorthUiFacadeLifecycleState {
            _dsl_package: dsl_package,
            _host_contract: host_contract,
            inspection_scope_inventory: UiInspectionScopeInventory::phase3_runtime_defaults(),
            runtime_support_inventory: WorthUiRuntimeSupportInventory::new(
                WorthUiRuntimeSupportInventoryFields {
                    dsl_package: WorthUiRuntimeSupportRow::new("dsl_package"),
                    inspection: WorthUiRuntimeSupportRow::new("inspection"),
                    query_binding: WorthUiRuntimeSupportRow::new("query_binding"),
                    host_contract: WorthUiRuntimeSupportRow::new("host_contract"),
                },
            ),
        };

        Self {
            capability_snapshot,
            lifecycle,
        }
    }

    /// Inspect the immutable capability snapshot owned by this app.
    pub fn capabilities(&self) -> &worth_ui_runtime::facade::CapabilitySnapshot {
        &self.capability_snapshot
    }

    /// Enter the runtime-owned inspection surface through one formal facade lane.
    pub fn inspect(&self, query: UiInspectionQuery) -> UiInspectionReceipt {
        phase3_unsupported_receipt(query)
    }

    pub fn inspection_scope_inventory(&self) -> &UiInspectionScopeInventory {
        &self.lifecycle.inspection_scope_inventory
    }

    pub fn runtime_support_inventory(&self) -> &WorthUiRuntimeSupportInventory {
        &self.lifecycle.runtime_support_inventory
    }
}
