use worth_store_contracts::StableDigest;

use crate::{
    chunk_integrity::{stable_digest_for, stable_digest_for_bytes},
    BlobChunkByteWindow, BlobChunkContentDigest, BlobChunkIdentity, BlobChunkProofLeaf,
    BlobChunkSecurityMetadataWitness, StoredChunkDigest,
};

use super::counters::BlobImportReadmissionCounters;
use super::denial::BlobImportReadmissionDenial;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobImportedChunkEvidence<'a> {
    leaf: BlobChunkProofLeaf,
    bytes: BlobChunkByteWindow<'a>,
}

impl<'a> BlobImportedChunkEvidence<'a> {
    pub(crate) fn collect_from_leaf(
        leaf: &BlobChunkProofLeaf,
        bytes: BlobChunkByteWindow<'a>,
        counters: BlobImportReadmissionCounters,
    ) -> Result<Self, BlobImportReadmissionDenial> {
        if leaf.byte_range() != bytes.range()
            || expected_content_digest(leaf, bytes) != *leaf.content_digest()
            || expected_stored_digest(leaf) != *leaf.stored_digest()
            || expected_chunk_identity(leaf) != *leaf.identity()
        {
            return Err(BlobImportReadmissionDenial::ChunkEvidenceMismatch { counters });
        }
        Ok(Self {
            leaf: leaf.clone(),
            bytes,
        })
    }

    pub(crate) fn leaf(&self) -> &BlobChunkProofLeaf {
        &self.leaf
    }

    pub(crate) fn bytes(&self) -> BlobChunkByteWindow<'a> {
        self.bytes
    }
}

fn expected_content_digest(
    leaf: &BlobChunkProofLeaf,
    bytes: BlobChunkByteWindow<'_>,
) -> BlobChunkContentDigest {
    BlobChunkContentDigest::from_integrity_parts(stable_digest_for_bytes(
        "content",
        import_chunk_rule_version(),
        leaf.ordinal(),
        leaf.byte_range(),
        bytes.bytes(),
    ))
}

fn expected_stored_digest(leaf: &BlobChunkProofLeaf) -> StoredChunkDigest {
    StoredChunkDigest::from_declared_digest(stable_digest_for(
        "stored",
        import_chunk_rule_version(),
        leaf.ordinal(),
        leaf.byte_range(),
        leaf.checksum_digest().as_str(),
    ))
}

fn expected_chunk_identity(leaf: &BlobChunkProofLeaf) -> BlobChunkIdentity {
    BlobChunkIdentity::from_integrity_parts(stable_digest_for(
        "chunk",
        import_chunk_rule_version(),
        leaf.ordinal(),
        leaf.byte_range(),
        &identity_evidence(leaf.security_metadata(), leaf.stored_digest().digest()),
    ))
}

fn identity_evidence(
    security_metadata: BlobChunkSecurityMetadataWitness,
    stored_digest: &StableDigest,
) -> String {
    format!(
        "{}:{}:{}:{}:{}:{}",
        stored_digest.as_str(),
        security_metadata.key_scope() as u8,
        security_metadata.key_version_posture() as u8,
        security_metadata.tenant_scope() as u8,
        security_metadata
            .authenticity_requirement()
            .class()
            .map_or(0, |class| class as u8),
        security_metadata.custody_posture() as u8
    )
}

const fn import_chunk_rule_version() -> &'static str {
    "s7.fixed-size.raw-chunk.v1"
}
