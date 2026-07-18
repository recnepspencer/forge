use std::fmt;
use std::rc::Rc;

use crate::facade::host_observation::{
    WorthUiHostCapabilityReport, WorthUiHostContract, WorthUiHostKind,
    WorthUiOperationalHostAdapter,
};
use worth_ui_host_contract::WorthUiHostCapabilityObservationGeneration;

/// Prepared host posture for one application generation. Phase 10 consumes
/// this plan into operational host-session authority.
#[derive(Clone)]
pub struct WorthUiHostSessionPlan {
    contract: WorthUiHostContract,
    capability_report: WorthUiHostCapabilityReport,
    adapter: Rc<dyn WorthUiOperationalHostAdapter>,
}

impl WorthUiHostSessionPlan {
    pub(crate) fn prepare<Adapter>(adapter: Adapter) -> Self
    where
        Adapter: WorthUiOperationalHostAdapter + 'static,
    {
        let contract = adapter.operational_host_contract();
        let capability_report = adapter
            .operational_capability_report()
            .with_observation_generation(WorthUiHostCapabilityObservationGeneration::new(0));
        Self {
            contract,
            capability_report,
            adapter: Rc::new(adapter),
        }
    }

    pub fn host_kind(&self) -> WorthUiHostKind {
        self.contract.kind()
    }

    pub(crate) fn capability_report(&self) -> &WorthUiHostCapabilityReport {
        &self.capability_report
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
            .field("capability_report", &self.capability_report)
            .finish_non_exhaustive()
    }
}

impl PartialEq for WorthUiHostSessionPlan {
    fn eq(&self, other: &Self) -> bool {
        self.contract == other.contract && self.capability_report == other.capability_report
    }
}

impl Eq for WorthUiHostSessionPlan {}
