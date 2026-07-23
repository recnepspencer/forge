use crate::runtime::replacement::compatibility::managed_live::{
    WorthUiQueryLiveRebindEntry, WorthUiQueryLiveRebindOutcome,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiQueryLiveRebindCounters {
    bindings_planned: usize,
    preserved_binding_count: usize,
    rebound_binding_count: usize,
    retired_binding_count: usize,
    denied_binding_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryLiveRebindPlan {
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    entries: Vec<WorthUiQueryLiveRebindEntry>,
    changed_entries: Vec<WorthUiQueryLiveRebindEntry>,
    live_candidate_binding_count: usize,
    basis_digest: u64,
    counters: WorthUiQueryLiveRebindCounters,
}

impl WorthUiQueryLiveRebindCounters {
    pub(crate) fn record_entry(&mut self, outcome: &WorthUiQueryLiveRebindOutcome) {
        self.bindings_planned += 1;
        match outcome {
            WorthUiQueryLiveRebindOutcome::Preserve(_) => {
                self.preserved_binding_count += 1;
            }
            WorthUiQueryLiveRebindOutcome::Rebind(_) => {
                self.rebound_binding_count += 1;
            }
            WorthUiQueryLiveRebindOutcome::Retire(_) => {
                self.retired_binding_count += 1;
            }
            WorthUiQueryLiveRebindOutcome::Deny(_) => {
                self.denied_binding_count += 1;
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn record_preserved_binding_for_test(&mut self) {
        self.bindings_planned += 1;
        self.preserved_binding_count += 1;
    }

    pub fn bindings_planned(self) -> usize {
        self.bindings_planned
    }

    pub fn preserved_binding_count(self) -> usize {
        self.preserved_binding_count
    }

    pub fn rebound_binding_count(self) -> usize {
        self.rebound_binding_count
    }

    pub fn retired_binding_count(self) -> usize {
        self.retired_binding_count
    }

    pub fn denied_binding_count(self) -> usize {
        self.denied_binding_count
    }
}

impl WorthUiQueryLiveRebindPlan {
    pub(crate) fn new(
        active_artifact_digest: u64,
        candidate_artifact_digest: u64,
        mut entries: Vec<WorthUiQueryLiveRebindEntry>,
    ) -> Self {
        entries.sort_by(|left, right| left.identity().cmp(right.identity()));
        let mut counters = WorthUiQueryLiveRebindCounters::default();
        for entry in &entries {
            counters.record_entry(entry.outcome());
        }
        let changed_entries = entries
            .iter()
            .filter(|entry| !matches!(entry.outcome(), WorthUiQueryLiveRebindOutcome::Preserve(_)))
            .cloned()
            .collect();
        let live_candidate_binding_count =
            counters.preserved_binding_count() + counters.rebound_binding_count();
        let basis_digest = super::basis_digest::query_rebind_basis_digest(
            active_artifact_digest,
            candidate_artifact_digest,
            &entries,
            counters,
        );
        Self {
            active_artifact_digest,
            candidate_artifact_digest,
            entries,
            changed_entries,
            live_candidate_binding_count,
            basis_digest,
            counters,
        }
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.active_artifact_digest
    }

    pub fn candidate_artifact_digest(&self) -> u64 {
        self.candidate_artifact_digest
    }

    pub fn entries(&self) -> &[WorthUiQueryLiveRebindEntry] {
        &self.entries
    }

    pub(crate) fn changed_entries(&self) -> &[WorthUiQueryLiveRebindEntry] {
        &self.changed_entries
    }

    pub(crate) fn live_candidate_binding_count(&self) -> usize {
        self.live_candidate_binding_count
    }

    pub(crate) fn basis_digest(&self) -> u64 {
        self.basis_digest
    }

    pub fn counters(&self) -> WorthUiQueryLiveRebindCounters {
        self.counters
    }

    pub fn binding_for_view_binding_id(
        &self,
        view_binding_id: &str,
    ) -> Option<&WorthUiQueryLiveRebindEntry> {
        self.entries
            .binary_search_by(|entry| entry.identity().view_binding_id().cmp(view_binding_id))
            .ok()
            .map(|index| &self.entries[index])
    }
}
