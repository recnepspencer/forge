#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobCloseoutProofTopology {
    replay_bundle_bound: bool,
    transcript_replay_bound: bool,
    blob_evidence_family_bound: bool,
    heavy_qualification_family_bound: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobCloseoutProofSummary {
    checked_execution: bool,
    oracle_verdict_count: usize,
    counter_row_count: usize,
    topology: BlobCloseoutProofTopology,
}

impl BlobCloseoutProofTopology {
    pub const fn new(
        replay_bundle_bound: bool,
        transcript_replay_bound: bool,
        blob_evidence_family_bound: bool,
        heavy_qualification_family_bound: bool,
    ) -> Self {
        Self {
            replay_bundle_bound,
            transcript_replay_bound,
            blob_evidence_family_bound,
            heavy_qualification_family_bound,
        }
    }

    pub const fn checked_execution(self) -> bool {
        self.replay_bundle_bound
            && self.transcript_replay_bound
            && self.blob_evidence_family_bound
            && self.heavy_qualification_family_bound
    }
}

impl BlobCloseoutProofSummary {
    pub const fn new(
        checked_execution: bool,
        oracle_verdict_count: usize,
        counter_row_count: usize,
        topology: BlobCloseoutProofTopology,
    ) -> Self {
        Self {
            checked_execution,
            oracle_verdict_count,
            counter_row_count,
            topology,
        }
    }

    pub const fn checked_execution(self) -> bool {
        self.checked_execution
    }
    pub const fn oracle_verdict_count(self) -> usize {
        self.oracle_verdict_count
    }
    pub const fn counter_row_count(self) -> usize {
        self.counter_row_count
    }
    pub const fn topology(self) -> BlobCloseoutProofTopology {
        self.topology
    }
}
