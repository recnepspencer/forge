use forge_store_wal::{BlobWalRecordEnvelope, BlobWalRecordKind};

use crate::BlobStreamingContentFrontier;

use super::super::checkpoint::checkpoint_identity;
use super::super::states::{BlobResumeChunkIntegrityAdmitted, BlobResumeFrontierCheckpointed};
use super::super::{BlobResumeCounterSnapshot, BlobResumeDenial};

impl BlobResumeChunkIntegrityAdmitted {
    pub fn checkpoint_frontier(
        self,
        frontier: BlobStreamingContentFrontier,
        checkpoint_record: BlobWalRecordEnvelope,
    ) -> Result<BlobResumeFrontierCheckpointed, BlobResumeDenial> {
        if checkpoint_record.identity().kind() != BlobWalRecordKind::SessionCheckpoint {
            return Err(BlobResumeDenial::WrongWalRecordKind);
        }
        let latest = frontier
            .proof_frontier()
            .ordered_leaves()
            .last()
            .ok_or(BlobResumeDenial::FrontierMissingChunk)?;
        if latest.ordinal() != self.leaf.ordinal()
            || latest.stored_digest() != self.leaf.stored_digest()
            || frontier.proof_frontier().total_bytes()
                > self.durable.admitted.declaration.declared_total_bytes
        {
            return Err(BlobResumeDenial::FrontierMissingChunk);
        }
        let checkpoint_identity = checkpoint_identity(
            &self.durable.admitted.session_id,
            &checkpoint_record,
            "frontier",
        );
        let counters = self.durable.admitted.counters.checkpointed();
        Ok(BlobResumeFrontierCheckpointed {
            integrity: self.with_counters(counters),
            frontier,
            checkpoint_record,
            checkpoint_identity,
        })
    }

    fn with_counters(mut self, counters: BlobResumeCounterSnapshot) -> Self {
        self.durable.admitted.counters = counters;
        self
    }
}

impl BlobResumeFrontierCheckpointed {
    pub fn security_metadata(&self) -> crate::BlobChunkSecurityMetadataWitness {
        self.integrity
            .durable
            .admitted
            .declaration
            .security_metadata
    }

    pub const fn counters(&self) -> BlobResumeCounterSnapshot {
        self.integrity.durable.admitted.counters
    }

    pub(crate) fn with_counters(mut self, counters: BlobResumeCounterSnapshot) -> Self {
        self.integrity.durable.admitted.counters = counters;
        self
    }
}
