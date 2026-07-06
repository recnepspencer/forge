use super::{
    BlobRecoveredPlacementObservation, BlobRecoveredPublishedGeneration,
    BlobRecoveredReachabilityStaging, BlobRecoveredResumeSession,
    BlobRecoveryRecordCounterSnapshot, BlobRecoveryRecordSet,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobRecoveryOutcome {
    PublishedGeneration,
    ClosedResumeSessionPublishedGeneration,
    ResumeSession,
    ReachabilityStaged,
    PlacementObserved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobRecoveryReplay {
    records: BlobRecoveryRecordSet,
    counters: BlobRecoveryRecordCounterSnapshot,
}

impl BlobRecoveryReplay {
    pub fn reconstruct(records: BlobRecoveryRecordSet) -> Self {
        let counters = records.counters().with_replayed_outcome();
        Self { records, counters }
    }

    pub const fn outcome(&self) -> BlobRecoveryOutcome {
        BlobRecoveryOutcome::ClosedResumeSessionPublishedGeneration
    }

    pub const fn published_generation(&self) -> &BlobRecoveredPublishedGeneration {
        self.records.publication().published()
    }

    pub const fn resume_session(&self) -> &BlobRecoveredResumeSession {
        self.records.resume_session().session()
    }

    pub const fn reachability_staging(&self) -> &BlobRecoveredReachabilityStaging {
        self.records.manifest().reachability().staged()
    }

    pub const fn placement_observation(&self) -> &BlobRecoveredPlacementObservation {
        self.records.manifest().placement().observation()
    }

    pub const fn counters(&self) -> BlobRecoveryRecordCounterSnapshot {
        self.counters
    }
}
