#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanLoopReconstructionLedgerCounters {
    identity_rows_consumed: usize,
    decision_rows_consumed: usize,
    propagated_name_rows_consumed: usize,
    propagated_signature_rows_consumed: usize,
    ledger_rows_emitted: usize,
    downstream_identities_emitted: usize,
    request_identity_mismatch_denials: usize,
    split_ledger_lineage_mismatch_denials: usize,
    missing_tracked_loop_denials: usize,
    missing_role_outcome_denials: usize,
    missing_degenerate_outcome_denials: usize,
    missing_decision_trace_denials: usize,
}

impl PlanarBooleanLoopReconstructionLedgerCounters {
    pub(crate) fn consumed_identity_row(&mut self) {
        self.identity_rows_consumed += 1;
    }

    pub(crate) fn consumed_decision_row(&mut self) {
        self.decision_rows_consumed += 1;
    }

    pub(crate) fn consumed_propagated_name_row(&mut self) {
        self.propagated_name_rows_consumed += 1;
    }

    pub(crate) fn consumed_propagated_signature_row(&mut self) {
        self.propagated_signature_rows_consumed += 1;
    }

    pub(crate) fn emitted_ledger_row(&mut self) {
        self.ledger_rows_emitted += 1;
    }

    pub(crate) fn emitted_downstream_identity(&mut self) {
        self.downstream_identities_emitted += 1;
    }

    pub(crate) fn denied_request_identity_mismatch(&mut self) {
        self.request_identity_mismatch_denials += 1;
    }

    pub(crate) fn denied_split_ledger_lineage_mismatch(&mut self) {
        self.split_ledger_lineage_mismatch_denials += 1;
    }

    pub(crate) fn denied_missing_tracked_loop(&mut self) {
        self.missing_tracked_loop_denials += 1;
    }

    pub(crate) fn denied_missing_role_outcome(&mut self) {
        self.missing_role_outcome_denials += 1;
    }

    pub(crate) fn denied_missing_degenerate_outcome(&mut self) {
        self.missing_degenerate_outcome_denials += 1;
    }

    pub(crate) fn denied_missing_decision_trace(&mut self) {
        self.missing_decision_trace_denials += 1;
    }

    pub fn identity_rows_consumed(self) -> usize {
        self.identity_rows_consumed
    }

    pub fn decision_rows_consumed(self) -> usize {
        self.decision_rows_consumed
    }

    pub fn propagated_name_rows_consumed(self) -> usize {
        self.propagated_name_rows_consumed
    }

    pub fn propagated_signature_rows_consumed(self) -> usize {
        self.propagated_signature_rows_consumed
    }

    pub fn ledger_rows_emitted(self) -> usize {
        self.ledger_rows_emitted
    }

    pub fn downstream_identities_emitted(self) -> usize {
        self.downstream_identities_emitted
    }

    pub fn request_identity_mismatch_denials(self) -> usize {
        self.request_identity_mismatch_denials
    }

    pub fn split_ledger_lineage_mismatch_denials(self) -> usize {
        self.split_ledger_lineage_mismatch_denials
    }

    pub fn missing_tracked_loop_denials(self) -> usize {
        self.missing_tracked_loop_denials
    }

    pub fn missing_role_outcome_denials(self) -> usize {
        self.missing_role_outcome_denials
    }

    pub fn missing_degenerate_outcome_denials(self) -> usize {
        self.missing_degenerate_outcome_denials
    }

    pub fn missing_decision_trace_denials(self) -> usize {
        self.missing_decision_trace_denials
    }
}
