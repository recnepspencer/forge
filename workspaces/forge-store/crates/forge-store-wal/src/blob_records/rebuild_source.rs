use crate::{BlobWalRecordEnvelope, DurablePublicationScope};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobWalReplayRebuildWitness {
    record: BlobWalRecordEnvelope,
    counter_shape: Vec<u64>,
}

impl BlobWalReplayRebuildWitness {
    pub fn admit(record: BlobWalRecordEnvelope) -> Self {
        let counter_shape = counter_shape(&record);

        Self {
            record,
            counter_shape,
        }
    }

    pub const fn record(&self) -> &BlobWalRecordEnvelope {
        &self.record
    }
    pub fn counter_shape(&self) -> &[u64] {
        &self.counter_shape
    }
}

fn counter_shape(record: &BlobWalRecordEnvelope) -> Vec<u64> {
    let scope = match record.durable_publication().scope() {
        DurablePublicationScope::WalFrame(scope) => scope,
        DurablePublicationScope::Checkpoint(_) | DurablePublicationScope::Manifest(_) => {
            unreachable!("blob wal records admit wal-frame publication only")
        }
    };
    let mut shape = vec![
        1,
        scope.segment_id(),
        scope.generation(),
        scope.lsn_end() - scope.lsn_start(),
        scope.expected_bytes(),
    ];
    shape.sort_unstable();
    shape
}
