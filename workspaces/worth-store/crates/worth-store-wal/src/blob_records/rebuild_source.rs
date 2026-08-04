use crate::{
    BlobWalRecordEnvelope, PublicationScope, WalSecurityMetadataCarrier,
    WalSecurityMetadataEnvelope,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobWalReplayRebuildWitness {
    source: WalSecurityMetadataEnvelope<BlobWalRecordEnvelope>,
    counter_shape: Vec<u64>,
}

impl BlobWalReplayRebuildWitness {
    pub fn admit(source: WalSecurityMetadataEnvelope<BlobWalRecordEnvelope>) -> Self {
        let counter_shape = counter_shape(source.record());

        Self {
            source,
            counter_shape,
        }
    }

    pub const fn record(&self) -> &BlobWalRecordEnvelope {
        self.source.record()
    }

    pub const fn security_metadata(&self) -> WalSecurityMetadataCarrier {
        self.source.security_metadata()
    }

    pub fn counter_shape(&self) -> &[u64] {
        &self.counter_shape
    }
}

fn counter_shape(record: &BlobWalRecordEnvelope) -> Vec<u64> {
    let scope = match record.publication_declaration().scope() {
        PublicationScope::WalFrame(scope) => scope,
        PublicationScope::Checkpoint(_) | PublicationScope::Manifest(_) => {
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
