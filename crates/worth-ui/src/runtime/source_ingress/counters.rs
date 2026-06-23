#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiSourceIngressCounters {
    raw_events_observed: usize,
    events_coalesced: usize,
    provider_reads: usize,
    source_revisions_emitted: usize,
    candidate_submissions_emitted: usize,
    observed_modules: usize,
    parsed_modules: usize,
    authored_declarations_inspected: usize,
    authored_declarations_touched: usize,
    semantic_slices_emitted: usize,
    frame_path_work: usize,
    active_runtime_mutations: usize,
}

impl WorthUiSourceIngressCounters {
    pub(crate) fn observe_event(&mut self) {
        self.raw_events_observed += 1;
    }

    pub(crate) fn coalesce_event(&mut self) {
        self.events_coalesced += 1;
    }

    pub(crate) fn record_provider_read(&mut self) {
        self.provider_reads += 1;
    }

    pub(crate) fn emit_revision(&mut self) {
        self.source_revisions_emitted += 1;
    }

    pub(crate) fn emit_candidate_submission(&mut self) {
        self.candidate_submissions_emitted += 1;
    }

    pub(crate) fn record_observed_modules(&mut self, count: usize) {
        self.observed_modules = count;
    }

    pub(crate) fn record_parsed_modules(&mut self, count: usize) {
        self.parsed_modules = count;
    }

    pub(crate) fn record_authored_declarations_inspected(&mut self, count: usize) {
        self.authored_declarations_inspected = count;
    }

    pub(crate) fn record_authored_declarations_touched(&mut self, count: usize) {
        self.authored_declarations_touched = count;
    }

    pub(crate) fn record_semantic_slices_emitted(&mut self, count: usize) {
        self.semantic_slices_emitted = count;
    }

    pub fn raw_events_observed(&self) -> usize {
        self.raw_events_observed
    }

    pub fn events_coalesced(&self) -> usize {
        self.events_coalesced
    }

    pub fn provider_reads(&self) -> usize {
        self.provider_reads
    }

    pub fn source_revisions_emitted(&self) -> usize {
        self.source_revisions_emitted
    }

    pub fn candidate_submissions_emitted(&self) -> usize {
        self.candidate_submissions_emitted
    }

    pub fn observed_modules(&self) -> usize {
        self.observed_modules
    }

    pub fn parsed_modules(&self) -> usize {
        self.parsed_modules
    }

    pub fn authored_declarations_inspected(&self) -> usize {
        self.authored_declarations_inspected
    }

    pub fn authored_declarations_touched(&self) -> usize {
        self.authored_declarations_touched
    }

    pub fn semantic_slices_emitted(&self) -> usize {
        self.semantic_slices_emitted
    }

    pub fn frame_path_work(&self) -> usize {
        self.frame_path_work
    }

    pub fn active_runtime_mutations(&self) -> usize {
        self.active_runtime_mutations
    }
}
