#![forbid(unsafe_code)]

mod large_record_streaming_envelope;

use forge_store_contracts::StableDigest;
pub use large_record_streaming_envelope::{
    LargeRecordStreamingEnvelope, LargeRecordStreamingEnvelopeDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkIdentity {
    content_digest: StableDigest,
}

impl BlobChunkIdentity {
    pub const fn from_digest(content_digest: StableDigest) -> Self {
        Self { content_digest }
    }
}
