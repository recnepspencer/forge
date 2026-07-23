use crate::runtime::{
    WorthUiActiveRuntimeObservation, WorthUiIdentityStateQueryCertificationCounters,
    WorthUiIdentityStateQueryCertificationDenial,
    WorthUiIdentityStateQueryCertificationDenialReason,
    WorthUiIdentityStateQueryCertificationScenario, WorthUiQueryDriftCertification,
    WorthUiStateCarryForwardReceipt, WorthUiStateLifecycleReceipt, WorthUiStateQueryResidueScan,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiIdentityStateCertification {
    scenario_name: String,
    active_observation: WorthUiActiveRuntimeObservation,
    state_receipts: Vec<WorthUiStateLifecycleReceipt>,
    carry_forward_receipts: Vec<WorthUiStateCarryForwardReceipt>,
    query_drift: WorthUiQueryDriftCertification,
    residue_scan: WorthUiStateQueryResidueScan,
    counters: WorthUiIdentityStateQueryCertificationCounters,
}

impl WorthUiIdentityStateCertification {
    pub(crate) fn certify(
        scenario: WorthUiIdentityStateQueryCertificationScenario,
        active_observation: WorthUiActiveRuntimeObservation,
        residue_scan: WorthUiStateQueryResidueScan,
    ) -> Result<Self, WorthUiIdentityStateQueryCertificationDenial> {
        let mut counters = WorthUiIdentityStateQueryCertificationCounters::default();
        if scenario.is_empty() {
            return Err(denial(
                WorthUiIdentityStateQueryCertificationDenialReason::EmptyScenario,
                counters,
            ));
        }

        let mut state_receipts = Vec::new();
        let mut carry_forward_receipts = Vec::new();
        for step in scenario.state_steps() {
            counters.record_state_step();
            crate::runtime::host_observation::identity_state_query_certification::state_certification::certify_state_step_receipts(
                step,
                &active_observation,
                &mut counters,
                &mut state_receipts,
                &mut carry_forward_receipts,
            )?;
        }

        let mut rebind_plans = Vec::new();
        for step in scenario.query_steps() {
            counters.record_query_step();
            crate::runtime::host_observation::identity_state_query_certification::query_step_certification::certify_query_rebind_step(
                step,
                &active_observation,
                &mut counters,
            )?;
            rebind_plans.push(step.rebind_plan().clone());
        }

        let query_drift = WorthUiQueryDriftCertification::new(rebind_plans);
        if scenario.strict_residue_scan() && !residue_scan.is_clean() {
            return Err(denial(
                WorthUiIdentityStateQueryCertificationDenialReason::StateQueryResidue {
                    label: scenario.name().to_owned(),
                },
                counters,
            ));
        }
        counters.record_residue_scan();

        Ok(Self {
            scenario_name: scenario.name().to_owned(),
            active_observation,
            state_receipts,
            carry_forward_receipts,
            query_drift,
            residue_scan,
            counters,
        })
    }

    pub fn scenario_name(&self) -> &str {
        &self.scenario_name
    }

    pub fn active_observation(&self) -> &WorthUiActiveRuntimeObservation {
        &self.active_observation
    }

    pub fn state_receipts(&self) -> &[WorthUiStateLifecycleReceipt] {
        &self.state_receipts
    }

    pub fn carry_forward_receipts(&self) -> &[WorthUiStateCarryForwardReceipt] {
        &self.carry_forward_receipts
    }

    pub fn query_drift(&self) -> &WorthUiQueryDriftCertification {
        &self.query_drift
    }

    pub fn residue_scan(&self) -> &WorthUiStateQueryResidueScan {
        &self.residue_scan
    }

    pub fn counters(&self) -> WorthUiIdentityStateQueryCertificationCounters {
        self.counters
    }
}

fn denial(
    reason: WorthUiIdentityStateQueryCertificationDenialReason,
    counters: WorthUiIdentityStateQueryCertificationCounters,
) -> WorthUiIdentityStateQueryCertificationDenial {
    WorthUiIdentityStateQueryCertificationDenial::new(reason, counters)
}
