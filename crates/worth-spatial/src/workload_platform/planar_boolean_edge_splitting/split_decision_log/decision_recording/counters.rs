#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanSplitDecisionLogCounters {
    decision_rows: usize,
    endpoint_decisions_recorded: usize,
    interval_subdivision_decisions_recorded: usize,
    micro_interval_policy_decisions_recorded: usize,
    coalescence_decisions_recorded: usize,
    fragment_decisions_recorded: usize,
    coverage_decisions_recorded: usize,
    persistent_name_decisions_recorded: usize,
    phase_stop_decisions_recorded: usize,
    duplicate_decision_identities_rejected: usize,
    missing_coverage_rejected: usize,
    foreign_product_denials: usize,
    lookup_index_entries: usize,
    affected_artifact_index_entries: usize,
    diagnostic_reports_emitted: usize,
    non_failure_localizations_rejected: usize,
    lookup_hits: usize,
    lookup_misses: usize,
}

impl PlanarBooleanSplitDecisionLogCounters {
    pub(crate) fn emitted_decision_row(&mut self) {
        self.decision_rows += 1;
    }
    pub(crate) fn recorded_endpoint_decision(&mut self) {
        self.endpoint_decisions_recorded += 1;
    }
    pub(crate) fn recorded_interval_subdivision_decision(&mut self) {
        self.interval_subdivision_decisions_recorded += 1;
    }
    pub(crate) fn recorded_micro_interval_policy_decision(&mut self) {
        self.micro_interval_policy_decisions_recorded += 1;
    }
    pub(crate) fn recorded_coalescence_decision(&mut self) {
        self.coalescence_decisions_recorded += 1;
    }
    pub(crate) fn recorded_fragment_decision(&mut self) {
        self.fragment_decisions_recorded += 1;
    }
    pub(crate) fn recorded_coverage_decision(&mut self) {
        self.coverage_decisions_recorded += 1;
    }
    pub(crate) fn recorded_persistent_name_decision(&mut self) {
        self.persistent_name_decisions_recorded += 1;
    }
    pub(crate) fn recorded_phase_stop_decision(&mut self) {
        self.phase_stop_decisions_recorded += 1;
    }
    pub(crate) fn rejected_duplicate_decision_identity(&mut self) {
        self.duplicate_decision_identities_rejected += 1;
    }
    pub(crate) fn rejected_missing_coverage(&mut self) {
        self.missing_coverage_rejected += 1;
    }
    pub(crate) fn rejected_foreign_product(&mut self) {
        self.foreign_product_denials += 1;
    }
    pub(crate) fn set_index_entries(&mut self, decision_entries: usize, artifact_entries: usize) {
        self.lookup_index_entries = decision_entries;
        self.affected_artifact_index_entries = artifact_entries;
    }
    pub(crate) fn emitted_diagnostic_report(&mut self) {
        self.diagnostic_reports_emitted += 1;
    }
    pub(crate) fn rejected_non_failure_localization(&mut self) {
        self.non_failure_localizations_rejected += 1;
    }
    pub(crate) fn recorded_lookup_hit(&mut self) {
        self.lookup_hits += 1;
    }
    pub(crate) fn recorded_lookup_miss(&mut self) {
        self.lookup_misses += 1;
    }
    pub fn decision_rows(self) -> usize {
        self.decision_rows
    }
    pub fn endpoint_decisions_recorded(self) -> usize {
        self.endpoint_decisions_recorded
    }
    pub fn interval_subdivision_decisions_recorded(self) -> usize {
        self.interval_subdivision_decisions_recorded
    }
    pub fn micro_interval_policy_decisions_recorded(self) -> usize {
        self.micro_interval_policy_decisions_recorded
    }
    pub fn coalescence_decisions_recorded(self) -> usize {
        self.coalescence_decisions_recorded
    }
    pub fn fragment_decisions_recorded(self) -> usize {
        self.fragment_decisions_recorded
    }
    pub fn coverage_decisions_recorded(self) -> usize {
        self.coverage_decisions_recorded
    }
    pub fn persistent_name_decisions_recorded(self) -> usize {
        self.persistent_name_decisions_recorded
    }
    pub fn phase_stop_decisions_recorded(self) -> usize {
        self.phase_stop_decisions_recorded
    }
    pub fn duplicate_decision_identities_rejected(self) -> usize {
        self.duplicate_decision_identities_rejected
    }
    pub fn missing_coverage_rejected(self) -> usize {
        self.missing_coverage_rejected
    }
    pub fn foreign_product_denials(self) -> usize {
        self.foreign_product_denials
    }
    pub fn lookup_index_entries(self) -> usize {
        self.lookup_index_entries
    }
    pub fn affected_artifact_index_entries(self) -> usize {
        self.affected_artifact_index_entries
    }
    pub fn diagnostic_reports_emitted(self) -> usize {
        self.diagnostic_reports_emitted
    }
    pub fn non_failure_localizations_rejected(self) -> usize {
        self.non_failure_localizations_rejected
    }
    pub fn lookup_hits(self) -> usize {
        self.lookup_hits
    }
    pub fn lookup_misses(self) -> usize {
        self.lookup_misses
    }
}
