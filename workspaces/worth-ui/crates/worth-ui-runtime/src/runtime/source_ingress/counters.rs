#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiSourceIngressCounters {
    raw_events_observed: usize,
    events_coalesced: usize,
    provider_reads: usize,
    source_revisions_emitted: usize,
    candidate_submissions_emitted: usize,
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

    pub fn frame_path_work(&self) -> usize {
        self.frame_path_work
    }

    pub fn active_runtime_mutations(&self) -> usize {
        self.active_runtime_mutations
    }
}
