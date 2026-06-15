use worth_ui::facade::WorthUiActiveRuntimeObservation;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationRunSummary {
    selected_scenario: &'static str,
    latest_run_receipt: Option<&'static str>,
    runtime_observation: WorthUiActiveRuntimeObservation,
}

impl ValidationRunSummary {
    pub fn new(
        selected_scenario: &'static str,
        latest_run_receipt: Option<&'static str>,
        runtime_observation: WorthUiActiveRuntimeObservation,
    ) -> Self {
        Self {
            selected_scenario,
            latest_run_receipt,
            runtime_observation,
        }
    }

    pub fn selected_scenario(&self) -> &'static str {
        self.selected_scenario
    }

    pub fn latest_run_receipt(&self) -> Option<&'static str> {
        self.latest_run_receipt
    }

    pub fn runtime_observation(&self) -> WorthUiActiveRuntimeObservation {
        self.runtime_observation
    }
}
