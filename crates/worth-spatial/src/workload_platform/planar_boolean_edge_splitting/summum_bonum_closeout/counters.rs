#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanEdgeSplitSummumBonumCloseoutCounters {
    candidate_rows: usize,
    lineage_rows: usize,
    decision_rows: usize,
    persistent_name_rows: usize,
    replay_parity_rows: usize,
    split_ledger_chains: usize,
    downstream_consumptions: usize,
    loop_reconstruction_consumptions: usize,
    endpoint_noop_decisions: usize,
    micro_interval_policy_required: usize,
    topology_products_emitted: usize,
}

impl PlanarBooleanEdgeSplitSummumBonumCloseoutCounters {
    pub(crate) fn record_candidate_rows(&mut self, rows: usize) {
        self.candidate_rows = rows;
    }
    pub(crate) fn record_lineage_rows(&mut self, rows: usize) {
        self.lineage_rows = rows;
    }
    pub(crate) fn record_decision_rows(&mut self, rows: usize) {
        self.decision_rows = rows;
    }
    pub(crate) fn record_persistent_name_rows(&mut self, rows: usize) {
        self.persistent_name_rows = rows;
    }
    pub(crate) fn record_replay_parity_rows(&mut self, rows: usize) {
        self.replay_parity_rows = rows;
    }
    pub(crate) fn record_split_ledger_chains(&mut self, rows: usize) {
        self.split_ledger_chains = rows;
    }
    pub(crate) fn record_downstream_consumption(&mut self) {
        self.downstream_consumptions += 1;
    }
    pub(crate) fn record_loop_reconstruction_consumption(&mut self) {
        self.loop_reconstruction_consumptions += 1;
    }
    pub(crate) fn record_endpoint_noop_decisions(&mut self, rows: usize) {
        self.endpoint_noop_decisions = rows;
    }
    pub(crate) fn record_micro_interval_policy_required(&mut self, rows: usize) {
        self.micro_interval_policy_required = rows;
    }
    pub(crate) fn record_topology_products_emitted(&mut self, rows: usize) {
        self.topology_products_emitted = rows;
    }

    pub fn candidate_rows(self) -> usize {
        self.candidate_rows
    }
    pub fn lineage_rows(self) -> usize {
        self.lineage_rows
    }
    pub fn decision_rows(self) -> usize {
        self.decision_rows
    }
    pub fn persistent_name_rows(self) -> usize {
        self.persistent_name_rows
    }
    pub fn replay_parity_rows(self) -> usize {
        self.replay_parity_rows
    }
    pub fn split_ledger_chains(self) -> usize {
        self.split_ledger_chains
    }
    pub fn downstream_consumptions(self) -> usize {
        self.downstream_consumptions
    }
    pub fn loop_reconstruction_consumptions(self) -> usize {
        self.loop_reconstruction_consumptions
    }
    pub fn endpoint_noop_decisions(self) -> usize {
        self.endpoint_noop_decisions
    }
    pub fn micro_interval_policy_required(self) -> usize {
        self.micro_interval_policy_required
    }
    pub fn topology_products_emitted(self) -> usize {
        self.topology_products_emitted
    }
}
