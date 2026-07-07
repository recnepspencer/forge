#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BlobPublicationCounterSnapshot {
    root_candidates: u64,
    reachability_staged: u64,
    wal_records: u64,
    session_closeouts: u64,
    committed_publications: u64,
    recovered_states: u64,
    denied_promotions: u64,
    visible_observations: u64,
}

impl BlobPublicationCounterSnapshot {
    pub const fn start() -> Self {
        Self {
            root_candidates: 0,
            reachability_staged: 0,
            wal_records: 0,
            session_closeouts: 0,
            committed_publications: 0,
            recovered_states: 0,
            denied_promotions: 0,
            visible_observations: 0,
        }
    }

    pub const fn with_root_candidate(mut self) -> Self {
        self.root_candidates += 1;
        self
    }

    pub const fn with_reachability_staged(mut self) -> Self {
        self.reachability_staged += 1;
        self
    }

    pub const fn with_wal_record(mut self) -> Self {
        self.wal_records += 1;
        self
    }

    pub const fn with_session_closeout(mut self) -> Self {
        self.session_closeouts += 1;
        self
    }

    pub const fn with_committed_publication(mut self) -> Self {
        self.committed_publications += 1;
        self
    }

    pub const fn with_recovered_state(mut self) -> Self {
        self.recovered_states += 1;
        self
    }

    pub const fn with_denied_promotion(mut self) -> Self {
        self.denied_promotions += 1;
        self
    }

    pub const fn with_visible_observation(mut self) -> Self {
        self.visible_observations += 1;
        self
    }

    pub const fn root_candidates(self) -> u64 {
        self.root_candidates
    }

    pub const fn reachability_staged(self) -> u64 {
        self.reachability_staged
    }

    pub const fn wal_records(self) -> u64 {
        self.wal_records
    }

    pub const fn session_closeouts(self) -> u64 {
        self.session_closeouts
    }

    pub const fn committed_publications(self) -> u64 {
        self.committed_publications
    }

    pub const fn recovered_states(self) -> u64 {
        self.recovered_states
    }

    pub const fn denied_promotions(self) -> u64 {
        self.denied_promotions
    }

    pub const fn visible_observations(self) -> u64 {
        self.visible_observations
    }
}
