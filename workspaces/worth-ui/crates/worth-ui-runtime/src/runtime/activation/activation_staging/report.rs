use crate::runtime::{WorthUiActivationReadiness, WorthUiActivationStagingCounters};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiActivationStagingReport {
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    readiness: WorthUiActivationReadiness,
    counters: WorthUiActivationStagingCounters,
}

impl WorthUiActivationStagingReport {
    pub(crate) fn new(
        active_artifact_digest: u64,
        candidate_artifact_digest: u64,
        readiness: WorthUiActivationReadiness,
        counters: WorthUiActivationStagingCounters,
    ) -> Self {
        Self {
            active_artifact_digest,
            candidate_artifact_digest,
            readiness,
            counters,
        }
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.active_artifact_digest
    }

    pub fn candidate_artifact_digest(&self) -> u64 {
        self.candidate_artifact_digest
    }

    pub fn readiness(&self) -> WorthUiActivationReadiness {
        self.readiness
    }

    pub fn counters(&self) -> WorthUiActivationStagingCounters {
        self.counters
    }
}
