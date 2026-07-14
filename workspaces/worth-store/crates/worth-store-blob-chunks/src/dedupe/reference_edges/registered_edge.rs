use worth_store_contracts::StableDigest;

use crate::{BlobChunkDedupeReceipt, BlobChunkIdentity, BlobChunkSecurityMetadataWitness};

use super::reference_set::BlobChunkDedupeReferenceEdge;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkRegisteredDedupeReference {
    reference_identity: StableDigest,
    shared_identity: BlobChunkIdentity,
    candidate_identity: BlobChunkIdentity,
    content_digest: StableDigest,
    security_metadata: BlobChunkSecurityMetadataWitness,
}

impl BlobChunkRegisteredDedupeReference {
    pub(super) fn from_first_receipt(receipt: &BlobChunkDedupeReceipt) -> Self {
        Self {
            reference_identity: super::reference_identity::dedupe_reference_identity(receipt, 1),
            shared_identity: receipt.existing_identity().clone(),
            candidate_identity: receipt.candidate_identity().clone(),
            content_digest: receipt.content_digest().clone(),
            security_metadata: receipt.security_metadata(),
        }
    }

    pub(super) fn from_receipt_and_edge(
        receipt: &BlobChunkDedupeReceipt,
        edge: &BlobChunkDedupeReferenceEdge,
    ) -> Self {
        Self {
            reference_identity: edge.reference_identity().clone(),
            shared_identity: receipt.existing_identity().clone(),
            candidate_identity: receipt.candidate_identity().clone(),
            content_digest: receipt.content_digest().clone(),
            security_metadata: receipt.security_metadata(),
        }
    }

    pub(crate) const fn reference_identity(&self) -> &StableDigest {
        &self.reference_identity
    }

    pub const fn shared_identity(&self) -> &BlobChunkIdentity {
        &self.shared_identity
    }

    pub const fn candidate_identity(&self) -> &BlobChunkIdentity {
        &self.candidate_identity
    }

    pub const fn content_digest(&self) -> &StableDigest {
        &self.content_digest
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub fn contains_chunk_identity(&self, chunk_identity: &BlobChunkIdentity) -> bool {
        &self.shared_identity == chunk_identity || &self.candidate_identity == chunk_identity
    }
}
