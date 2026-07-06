use crate::BlobRootCandidateForPublication;

use super::super::states::{BlobResumeFrontierCheckpointed, BlobResumeRootCandidateBuilt};
use super::super::{BlobResumeCounterSnapshot, BlobResumeDenial};

impl BlobResumeFrontierCheckpointed {
    pub fn build_root_candidate(
        self,
        root_candidate: BlobRootCandidateForPublication,
    ) -> Result<BlobResumeRootCandidateBuilt, BlobResumeDenial> {
        let intent = root_candidate.intent();
        if intent.chunk_tree_root() != self.frontier.chunk_tree_root()
            || intent.logical_content_digest() != self.frontier.logical_content_digest()
        {
            return Err(BlobResumeDenial::RootCandidateMismatch);
        }
        let counters = self.counters().root_candidate();
        Ok(BlobResumeRootCandidateBuilt {
            checkpointed: self.with_counters(counters),
            root_candidate,
        })
    }
}

impl BlobResumeRootCandidateBuilt {
    pub const fn counters(&self) -> BlobResumeCounterSnapshot {
        self.checkpointed.integrity.durable.admitted.counters
    }

    pub(crate) fn with_counters(mut self, counters: BlobResumeCounterSnapshot) -> Self {
        self.checkpointed.integrity.durable.admitted.counters = counters;
        self
    }
}