use crate::declaration::{
    UiDeclaredHostCapabilityPosture, UiDeclaredMeasurementPolicyPosture, UiDeclaredPostureContract,
    UiDeclaredPostureLane, UiDeclaredQueryBindingPosture, UiDeclaredServiceUsagePosture,
    UiDeclaredTouchMeaningPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiDeclaredPosturePayload {
    contract: UiDeclaredPostureContract,
}

impl UiDeclaredPosturePayload {
    pub(crate) const fn new(contract: UiDeclaredPostureContract) -> Self {
        Self { contract }
    }

    pub(crate) const fn contract(&self) -> &UiDeclaredPostureContract {
        &self.contract
    }

    pub(crate) const fn query_binding(
        &self,
    ) -> &UiDeclaredPostureLane<UiDeclaredQueryBindingPosture> {
        self.contract.query_binding()
    }

    pub(crate) const fn service_usage(
        &self,
    ) -> &UiDeclaredPostureLane<UiDeclaredServiceUsagePosture> {
        self.contract.service_usage()
    }

    pub(crate) const fn touch_meaning(
        &self,
    ) -> &UiDeclaredPostureLane<UiDeclaredTouchMeaningPosture> {
        self.contract.touch_meaning()
    }

    pub(crate) const fn measurement_policy(
        &self,
    ) -> &UiDeclaredPostureLane<UiDeclaredMeasurementPolicyPosture> {
        self.contract.measurement_policy()
    }

    pub(crate) const fn host_capability(
        &self,
    ) -> &UiDeclaredPostureLane<UiDeclaredHostCapabilityPosture> {
        self.contract.host_capability()
    }
}
