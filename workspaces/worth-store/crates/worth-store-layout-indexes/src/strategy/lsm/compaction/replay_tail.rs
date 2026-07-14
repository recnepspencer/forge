use worth_store_wal::BlobWalRecordEnvelope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LsmCompactionReplayTail {
    value: BlobWalRecordEnvelope,
    generation: BlobWalRecordEnvelope,
    tombstone: BlobWalRecordEnvelope,
}

impl LsmCompactionReplayTail {
    pub(in crate::strategy::lsm) fn issue(
        value: BlobWalRecordEnvelope,
        generation: BlobWalRecordEnvelope,
        tombstone: BlobWalRecordEnvelope,
    ) -> Self {
        Self {
            value,
            generation,
            tombstone,
        }
    }

    pub const fn value(&self) -> &BlobWalRecordEnvelope {
        &self.value
    }

    pub const fn generation(&self) -> &BlobWalRecordEnvelope {
        &self.generation
    }

    pub const fn tombstone(&self) -> &BlobWalRecordEnvelope {
        &self.tombstone
    }
}
