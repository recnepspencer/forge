use crate::runtime::{
    WorthUiDurableStateReconciliationDenial, WorthUiDurableStateReconciliationPlan,
    WorthUiNodeReplacementPlan, WorthUiQueryBindingDriftDenialKind, WorthUiQueryLiveRebindPlan,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiIdentityStateQueryCertificationScenario {
    name: String,
    state_steps: Vec<WorthUiStateCertificationScenarioStep>,
    query_steps: Vec<WorthUiQueryDriftCertificationScenarioStep>,
    strict_residue_scan: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiStateCertificationScenarioStep {
    label: String,
    node_plan: WorthUiNodeReplacementPlan,
    reconciliation: WorthUiStateCertificationScenarioStepReconciliation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiStateCertificationScenarioStepReconciliation {
    Plan(WorthUiDurableStateReconciliationPlan),
    Denial(WorthUiDurableStateReconciliationDenial),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryDriftCertificationScenarioStep {
    label: String,
    rebind_plan: WorthUiQueryLiveRebindPlan,
    expected_denial: Option<WorthUiQueryBindingDriftDenialKind>,
    ui_local_status_probe: bool,
}

impl WorthUiIdentityStateQueryCertificationScenario {
    pub fn named(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            state_steps: Vec::new(),
            query_steps: Vec::new(),
            strict_residue_scan: false,
        }
    }

    pub fn with_state_reconciliation_plan(
        mut self,
        label: impl Into<String>,
        node_plan: WorthUiNodeReplacementPlan,
        reconciliation_plan: WorthUiDurableStateReconciliationPlan,
    ) -> Self {
        self.state_steps
            .push(WorthUiStateCertificationScenarioStep {
                label: label.into(),
                node_plan,
                reconciliation: WorthUiStateCertificationScenarioStepReconciliation::Plan(
                    reconciliation_plan,
                ),
            });
        self
    }

    pub fn with_state_reconciliation_denial(
        mut self,
        label: impl Into<String>,
        node_plan: WorthUiNodeReplacementPlan,
        denial: WorthUiDurableStateReconciliationDenial,
    ) -> Self {
        self.state_steps
            .push(WorthUiStateCertificationScenarioStep {
                label: label.into(),
                node_plan,
                reconciliation: WorthUiStateCertificationScenarioStepReconciliation::Denial(denial),
            });
        self
    }

    pub fn with_query_rebind_plan(
        mut self,
        label: impl Into<String>,
        rebind_plan: WorthUiQueryLiveRebindPlan,
    ) -> Self {
        self.query_steps
            .push(WorthUiQueryDriftCertificationScenarioStep::new(
                label,
                rebind_plan,
            ));
        self
    }

    pub fn with_query_rebind_plan_expecting_denial(
        mut self,
        label: impl Into<String>,
        rebind_plan: WorthUiQueryLiveRebindPlan,
        expected_denial: WorthUiQueryBindingDriftDenialKind,
    ) -> Self {
        self.query_steps.push(
            WorthUiQueryDriftCertificationScenarioStep::new(label, rebind_plan)
                .expecting_denial(expected_denial),
        );
        self
    }

    pub fn with_ui_local_query_status_probe(
        mut self,
        label: impl Into<String>,
        rebind_plan: WorthUiQueryLiveRebindPlan,
    ) -> Self {
        self.query_steps.push(
            WorthUiQueryDriftCertificationScenarioStep::new(label, rebind_plan)
                .with_ui_local_status_probe(),
        );
        self
    }

    pub fn with_strict_residue_scan(mut self) -> Self {
        self.strict_residue_scan = true;
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn state_steps(&self) -> &[WorthUiStateCertificationScenarioStep] {
        &self.state_steps
    }

    pub fn query_steps(&self) -> &[WorthUiQueryDriftCertificationScenarioStep] {
        &self.query_steps
    }

    pub fn strict_residue_scan(&self) -> bool {
        self.strict_residue_scan
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.state_steps.is_empty() && self.query_steps.is_empty()
    }
}

impl WorthUiStateCertificationScenarioStep {
    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn node_plan(&self) -> &WorthUiNodeReplacementPlan {
        &self.node_plan
    }

    pub fn reconciliation(&self) -> &WorthUiStateCertificationScenarioStepReconciliation {
        &self.reconciliation
    }
}

impl WorthUiQueryDriftCertificationScenarioStep {
    fn new(label: impl Into<String>, rebind_plan: WorthUiQueryLiveRebindPlan) -> Self {
        Self {
            label: label.into(),
            rebind_plan,
            expected_denial: None,
            ui_local_status_probe: false,
        }
    }

    fn expecting_denial(mut self, expected_denial: WorthUiQueryBindingDriftDenialKind) -> Self {
        self.expected_denial = Some(expected_denial);
        self
    }

    fn with_ui_local_status_probe(mut self) -> Self {
        self.ui_local_status_probe = true;
        self
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn rebind_plan(&self) -> &WorthUiQueryLiveRebindPlan {
        &self.rebind_plan
    }

    pub fn expected_denial(&self) -> Option<WorthUiQueryBindingDriftDenialKind> {
        self.expected_denial
    }

    pub fn ui_local_status_probe(&self) -> bool {
        self.ui_local_status_probe
    }
}
