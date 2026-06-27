use worth_ui_inspection::{UiInspectionScopeInventory, RUNTIME_INSPECTION_SCOPE_INVENTORY};

use crate::facade::{
    inspection_observation::WorthUiInspectionObservationState, CapabilitySnapshot,
    UiInspectionScope, UiInspectionSupportReport, WorthUiDslPackage, WorthUiHostContract,
    WorthUiRuntimeSupportInventory, PHASE3_RUNTIME_SUPPORT_INVENTORY,
};

pub(crate) struct WorthUiFacadeLifecycleBootstrap {
    inspection_scope_inventory: UiInspectionScopeInventory,
    inspection_observation: WorthUiInspectionObservationState,
    runtime_support_inventory: WorthUiRuntimeSupportInventory,
    _dsl_package: WorthUiDslPackage,
    _host_contract: WorthUiHostContract,
}

impl WorthUiFacadeLifecycleBootstrap {
    fn new(dsl_package: WorthUiDslPackage, host_contract: WorthUiHostContract) -> Self {
        Self::new_with_inspection_scope_inventory(
            dsl_package,
            host_contract,
            RUNTIME_INSPECTION_SCOPE_INVENTORY,
        )
    }

    pub(crate) fn new_with_inspection_scope_inventory(
        dsl_package: WorthUiDslPackage,
        host_contract: WorthUiHostContract,
        inspection_scope_inventory: UiInspectionScopeInventory,
    ) -> Self {
        Self {
            inspection_scope_inventory,
            inspection_observation: WorthUiInspectionObservationState::new(),
            runtime_support_inventory: PHASE3_RUNTIME_SUPPORT_INVENTORY,
            _dsl_package: dsl_package,
            _host_contract: host_contract,
        }
    }

    pub(crate) fn inspection_support_report(
        &self,
        scope: UiInspectionScope,
    ) -> UiInspectionSupportReport {
        self.inspection_observation.record_support_report();
        self.inspection_scope_inventory.support_report(scope)
    }

    pub(crate) fn inspection_closure_report(&self) -> crate::facade::UiInspectionClosureReport {
        self.inspection_scope_inventory.closure_report()
    }

    pub(crate) fn runtime_support_inventory(&self) -> &WorthUiRuntimeSupportInventory {
        &self.runtime_support_inventory
    }

    pub(crate) fn inspection_observation(&self) -> crate::facade::UiInspectionFacadeObservation {
        self.inspection_observation.snapshot()
    }

    pub(crate) fn record_inspection_query(&self) {
        self.inspection_observation.record_query();
    }

    pub(crate) fn record_unsupported_inspection_query(&self) {
        self.inspection_observation.record_unsupported_query();
    }
}

pub(crate) struct WorthUiCapabilityRegistrationFreezeCore {
    capability_snapshot: CapabilitySnapshot,
    lifecycle: WorthUiFacadeLifecycleBootstrap,
}

impl WorthUiCapabilityRegistrationFreezeCore {
    pub(crate) fn new(
        capability_snapshot: CapabilitySnapshot,
        dsl_package: WorthUiDslPackage,
        host_contract: WorthUiHostContract,
    ) -> Self {
        Self {
            capability_snapshot,
            lifecycle: WorthUiFacadeLifecycleBootstrap::new(dsl_package, host_contract),
        }
    }

    pub(crate) fn new_with_inspection_scope_inventory(
        capability_snapshot: CapabilitySnapshot,
        dsl_package: WorthUiDslPackage,
        host_contract: WorthUiHostContract,
        inspection_scope_inventory: UiInspectionScopeInventory,
    ) -> Self {
        Self {
            capability_snapshot,
            lifecycle: WorthUiFacadeLifecycleBootstrap::new_with_inspection_scope_inventory(
                dsl_package,
                host_contract,
                inspection_scope_inventory,
            ),
        }
    }

    pub(crate) fn into_parts(self) -> (CapabilitySnapshot, WorthUiFacadeLifecycleBootstrap) {
        (self.capability_snapshot, self.lifecycle)
    }
}
