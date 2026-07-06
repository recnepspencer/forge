use forge_store_wal::{BlobWalRecordEnvelope, BlobWalRecordKind};

use crate::BlobChunkReachabilityProofSet;

use super::super::states::{
    BlobResumeRootCandidateBuilt, BlobResumeRootPublicationReady, BlobResumeSessionClosed,
};
use super::super::{BlobResumeCounterSnapshot, BlobResumeDenial};
use crate::BlobReachabilityStaging;

impl BlobResumeRootCandidateBuilt {
    pub fn stage_reachability(
        self,
        reachability: BlobChunkReachabilityProofSet,
    ) -> Result<BlobResumeRootPublicationReady, BlobResumeDenial> {
        let staging = BlobReachabilityStaging::stage(self.root_candidate.clone(), reachability)
            .map_err(|_| BlobResumeDenial::RootCandidateMismatch)?;
        let counters = self.counters().root_ready();
        Ok(BlobResumeRootPublicationReady {
            root_candidate: self.with_counters(counters),
            reachability_staging: staging,
        })
    }
}

impl BlobResumeRootPublicationReady {
    pub fn close_session(
        self,
        closeout_record: BlobWalRecordEnvelope,
    ) -> Result<BlobResumeSessionClosed, BlobResumeDenial> {
        if closeout_record.identity().kind() != BlobWalRecordKind::SessionCloseout {
            return Err(BlobResumeDenial::WrongWalRecordKind);
        }
        let counters = self.counters().closed();
        Ok(BlobResumeSessionClosed {
            ready: self.with_counters(counters),
            closeout_record,
        })
    }

    pub const fn reachability_staging(&self) -> &BlobReachabilityStaging {
        &self.reachability_staging
    }

    pub const fn counters(&self) -> BlobResumeCounterSnapshot {
        self.root_candidate
            .checkpointed
            .integrity
            .durable
            .admitted
            .counters
    }

    fn with_counters(mut self, counters: BlobResumeCounterSnapshot) -> Self {
        self.root_candidate
            .checkpointed
            .integrity
            .durable
            .admitted
            .counters = counters;
        self
    }
}