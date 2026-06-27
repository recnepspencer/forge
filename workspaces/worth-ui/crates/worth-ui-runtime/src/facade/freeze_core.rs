use crate::capability::CapabilitySnapshot;
use worth_ui_dsl::WorthUiDslPackage;
use worth_ui_host_contract::WorthUiHostContract;

pub struct WorthUiCapabilityRegistrationFreezeCore {
    capability_snapshot: CapabilitySnapshot,
    dsl_package: WorthUiDslPackage,
    host_contract: WorthUiHostContract,
}

impl WorthUiCapabilityRegistrationFreezeCore {
    pub(crate) fn new(
        capability_snapshot: CapabilitySnapshot,
        dsl_package: WorthUiDslPackage,
        host_contract: WorthUiHostContract,
    ) -> Self {
        Self {
            capability_snapshot,
            dsl_package,
            host_contract,
        }
    }

    pub fn into_parts(
        self,
    ) -> (
        CapabilitySnapshot,
        WorthUiDslPackage,
        WorthUiHostContract,
    ) {
        (
            self.capability_snapshot,
            self.dsl_package,
            self.host_contract,
        )
    }
}
