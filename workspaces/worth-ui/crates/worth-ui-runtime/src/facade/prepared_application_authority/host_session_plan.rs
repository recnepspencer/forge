use std::fmt;
use std::rc::Rc;

use crate::facade::host::{
    WorthUiHostCapabilityReport, WorthUiHostContract, WorthUiHostKind,
    WorthUiOperationalHostAdapter,
};
use worth_ui_host_contract::WorthUiHostCapabilityObservationGeneration;

/// Prepared host posture for one application generation. Phase 10 consumes
/// this plan into operational host-session authority.
#[derive(Clone)]
pub struct WorthUiHostSessionPlan {
    contract: WorthUiHostContract,
    protocol_contract: worth_ui_host_contract::UiHostProtocolContract,
    capability_report: WorthUiHostCapabilityReport,
    mounted_frame_retention_budget: crate::mounting::UiMountedFrameRetentionBudget,
    host_observation_capacity:
        crate::host_exchange::observation_report_validation::UiHostObservationCapacity,
    adapter: Rc<dyn WorthUiOperationalHostAdapter>,
}

impl WorthUiHostSessionPlan {
    pub(crate) fn prepare<Adapter>(adapter: Adapter) -> Self
    where
        Adapter: WorthUiOperationalHostAdapter + 'static,
    {
        let contract = adapter.operational_host_contract();
        let protocol_contract = adapter.operational_protocol_contract();
        let capability_report = adapter
            .operational_capability_report()
            .with_observation_generation(WorthUiHostCapabilityObservationGeneration::new(0));
        Self {
            contract,
            protocol_contract,
            capability_report,
            mounted_frame_retention_budget: Default::default(),
            host_observation_capacity: Default::default(),
            adapter: Rc::new(adapter),
        }
    }

    pub(crate) fn set_mounted_frame_retention_budget(
        &mut self,
        budget: crate::mounting::UiMountedFrameRetentionBudget,
    ) {
        self.mounted_frame_retention_budget = budget;
    }

    pub(crate) fn mounted_frame_retention_budget(
        &self,
    ) -> crate::mounting::UiMountedFrameRetentionBudget {
        self.mounted_frame_retention_budget
    }

    pub(crate) fn set_host_observation_capacity(
        &mut self,
        capacity: crate::host_exchange::observation_report_validation::UiHostObservationCapacity,
    ) {
        self.host_observation_capacity = capacity;
    }

    pub(crate) fn host_observation_capacity(
        &self,
    ) -> crate::host_exchange::observation_report_validation::UiHostObservationCapacity {
        self.host_observation_capacity
    }

    pub fn host_kind(&self) -> WorthUiHostKind {
        self.contract.kind()
    }

    pub(crate) fn capability_report(&self) -> &WorthUiHostCapabilityReport {
        &self.capability_report
    }

    pub(crate) fn protocol_contract(&self) -> worth_ui_host_contract::UiHostProtocolContract {
        self.protocol_contract
    }

    pub(crate) fn adapter(&self) -> Rc<dyn WorthUiOperationalHostAdapter> {
        Rc::clone(&self.adapter)
    }
}

impl fmt::Debug for WorthUiHostSessionPlan {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorthUiHostSessionPlan")
            .field("contract", &self.contract)
            .field("protocol_contract", &self.protocol_contract)
            .field("capability_report", &self.capability_report)
            .field(
                "mounted_frame_retention_budget",
                &self.mounted_frame_retention_budget,
            )
            .field("host_observation_capacity", &self.host_observation_capacity)
            .finish_non_exhaustive()
    }
}

impl PartialEq for WorthUiHostSessionPlan {
    fn eq(&self, other: &Self) -> bool {
        self.contract == other.contract
            && self.protocol_contract == other.protocol_contract
            && self.capability_report == other.capability_report
            && self.mounted_frame_retention_budget == other.mounted_frame_retention_budget
            && self.host_observation_capacity == other.host_observation_capacity
    }
}

impl Eq for WorthUiHostSessionPlan {}
