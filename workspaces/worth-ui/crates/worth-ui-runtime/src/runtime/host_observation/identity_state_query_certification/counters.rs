use crate::runtime::{WorthUiDurableStateReconciliationOutcome, WorthUiQueryLiveRebindOutcome};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiIdentityStateQueryCertificationCounters {
    state_steps_certified: usize,
    query_steps_certified: usize,
    ambiguous_identity_denial_count: usize,
    state_receipt_count: usize,
    state_carry_forward_count: usize,
    state_replacement_count: usize,
    state_drop_count: usize,
    state_recreate_count: usize,
    query_binding_count: usize,
    query_preserved_binding_count: usize,
    query_rebound_binding_count: usize,
    query_retired_binding_count: usize,
    query_denied_binding_count: usize,
    residue_scan_count: usize,
}

impl WorthUiIdentityStateQueryCertificationCounters {
    pub(crate) fn record_state_step(&mut self) {
        self.state_steps_certified += 1;
    }

    pub(crate) fn record_query_step(&mut self) {
        self.query_steps_certified += 1;
    }

    pub(crate) fn record_ambiguous_identity_denial(&mut self) {
        self.ambiguous_identity_denial_count += 1;
    }

    pub(crate) fn record_state_receipt(
        &mut self,
        outcome: WorthUiDurableStateReconciliationOutcome,
    ) {
        self.state_receipt_count += 1;
        match outcome {
            WorthUiDurableStateReconciliationOutcome::CarryForward => {
                self.state_carry_forward_count += 1;
            }
            WorthUiDurableStateReconciliationOutcome::Replace => self.state_replacement_count += 1,
            WorthUiDurableStateReconciliationOutcome::Drop => self.state_drop_count += 1,
            WorthUiDurableStateReconciliationOutcome::Recreate => self.state_recreate_count += 1,
        }
    }

    pub(crate) fn record_query_binding(&mut self, outcome: &WorthUiQueryLiveRebindOutcome) {
        self.query_binding_count += 1;
        match outcome {
            WorthUiQueryLiveRebindOutcome::Preserve(_) => self.query_preserved_binding_count += 1,
            WorthUiQueryLiveRebindOutcome::Rebind(_) => self.query_rebound_binding_count += 1,
            WorthUiQueryLiveRebindOutcome::Retire(_) => self.query_retired_binding_count += 1,
            WorthUiQueryLiveRebindOutcome::Deny(_) => self.query_denied_binding_count += 1,
        }
    }

    pub(crate) fn record_residue_scan(&mut self) {
        self.residue_scan_count += 1;
    }

    pub fn state_steps_certified(self) -> usize {
        self.state_steps_certified
    }

    pub fn query_steps_certified(self) -> usize {
        self.query_steps_certified
    }

    pub fn ambiguous_identity_denial_count(self) -> usize {
        self.ambiguous_identity_denial_count
    }

    pub fn state_receipt_count(self) -> usize {
        self.state_receipt_count
    }

    pub fn state_carry_forward_count(self) -> usize {
        self.state_carry_forward_count
    }

    pub fn state_replacement_count(self) -> usize {
        self.state_replacement_count
    }

    pub fn state_drop_count(self) -> usize {
        self.state_drop_count
    }

    pub fn state_recreate_count(self) -> usize {
        self.state_recreate_count
    }

    pub fn query_binding_count(self) -> usize {
        self.query_binding_count
    }

    pub fn query_denied_binding_count(self) -> usize {
        self.query_denied_binding_count
    }

    pub fn residue_scan_count(self) -> usize {
        self.residue_scan_count
    }
}
