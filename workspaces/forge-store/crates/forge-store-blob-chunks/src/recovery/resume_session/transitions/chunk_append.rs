use forge_store_physical_isolation::CurrentGenerationPhysicalReference;
use forge_store_wal::{BlobWalRecordEnvelope, BlobWalRecordKind};

use crate::BlobChunkProofLeaf;

use super::super::checkpoint::{checkpoint_from_parts, checkpoint_identity};
use super::super::states::{
    BlobResumeCheckpointStateKind, BlobResumeChunkAppendStarted, BlobResumeChunkBytesDurable,
    BlobResumeChunkIntegrityAdmitted,
};
use super::super::{BlobResumeCheckpoint, BlobResumeCounterSnapshot, BlobResumeDenial};

impl BlobResumeChunkAppendStarted {
    pub fn export_checkpoint(
        &self,
        record: BlobWalRecordEnvelope,
    ) -> Result<BlobResumeCheckpoint, BlobResumeDenial> {
        super::checkpoint_export::export_append_started_checkpoint(self, record)
    }

    pub fn record_chunk_bytes_durable(
        self,
        wal_record: BlobWalRecordEnvelope,
        durable_bytes: u64,
        physical_reference: CurrentGenerationPhysicalReference,
    ) -> Result<BlobResumeChunkBytesDurable, BlobResumeDenial> {
        if wal_record.identity().kind() != BlobWalRecordKind::ChunkAppend {
            return Err(BlobResumeDenial::WrongWalRecordKind);
        }
        if durable_bytes == 0 {
            return Err(BlobResumeDenial::MissingDurableBytes);
        }
        let counters = self.admitted.counters.bytes_durable();
        Ok(BlobResumeChunkBytesDurable {
            admitted: self.admitted.with_counters(counters),
            ordinal: self.ordinal,
            wal_record,
            durable_bytes,
            physical_reference,
        })
    }
}

impl BlobResumeChunkBytesDurable {
    pub fn admit_chunk_integrity(
        self,
        leaf: BlobChunkProofLeaf,
    ) -> Result<BlobResumeChunkIntegrityAdmitted, BlobResumeDenial> {
        if leaf.ordinal() != self.ordinal {
            return Err(BlobResumeDenial::ChunkOrdinalMismatch);
        }
        if leaf.security_metadata() != self.admitted.declaration.security_metadata {
            return Err(BlobResumeDenial::ChunkSecurityScopeMismatch);
        }
        let actual_total_bytes = leaf.byte_range().end();
        if actual_total_bytes > self.durable_bytes {
            return Err(BlobResumeDenial::ChunkTailMissing {
                expected_total_bytes: actual_total_bytes,
                actual_total_bytes: self.durable_bytes,
            });
        }
        let counters = self.admitted.counters.integrity_admitted();
        Ok(BlobResumeChunkIntegrityAdmitted {
            durable: self.with_counters(counters),
            leaf,
        })
    }

    pub fn export_checkpoint(&self) -> BlobResumeCheckpoint {
        checkpoint_from_parts(
            self.admitted.clone(),
            BlobResumeCheckpointStateKind::ChunkBytesDurable,
            checkpoint_identity(&self.admitted.session_id, &self.wal_record, "bytes"),
            None,
            Some(self.physical_reference),
            None,
            None,
            None,
        )
    }

    fn with_counters(mut self, counters: BlobResumeCounterSnapshot) -> Self {
        self.admitted.counters = counters;
        self
    }
}
