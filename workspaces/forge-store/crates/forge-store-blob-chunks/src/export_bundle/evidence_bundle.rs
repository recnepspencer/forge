use super::counters::BlobExportEvidenceCounts;
use forge_foundational::CanonicalDerivedDigest;
use forge_store_contracts::StableDigest;

use crate::LogicalContentDigest;

use super::chunk_bytes::BlobExportedChunkBytes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobExportDigestEvidence {
    logical_content_digest: LogicalContentDigest,
    export_bundle_digest: CanonicalDerivedDigest,
    declaration_digest: StableDigest,
    declared_chunk_count: u64,
    declared_total_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobExportOfflineChunkDeclaration {
    ordinal: u64,
    chunk_identity: String,
    stored_digest: String,
    checksum_digest: String,
    bytes: u64,
}

impl BlobExportDigestEvidence {
    pub(crate) fn new(
        logical_content_digest: LogicalContentDigest,
        export_bundle_digest: CanonicalDerivedDigest,
        declarations: &[BlobExportOfflineChunkDeclaration],
    ) -> Self {
        Self {
            logical_content_digest,
            export_bundle_digest,
            declaration_digest: declaration_digest(declarations),
            declared_chunk_count: declarations.len() as u64,
            declared_total_bytes: declarations
                .iter()
                .map(BlobExportOfflineChunkDeclaration::bytes)
                .sum(),
        }
    }

    pub fn logical_content_digest(&self) -> &LogicalContentDigest {
        &self.logical_content_digest
    }

    pub fn export_bundle_digest(&self) -> &CanonicalDerivedDigest {
        &self.export_bundle_digest
    }

    pub fn declaration_digest(&self) -> &StableDigest {
        &self.declaration_digest
    }

    pub const fn declared_chunk_count(&self) -> u64 {
        self.declared_chunk_count
    }

    pub const fn declared_total_bytes(&self) -> u64 {
        self.declared_total_bytes
    }

    pub const fn evidence_item_count(&self) -> u64 {
        BlobExportEvidenceCounts::DIGEST_EVIDENCE_ITEMS
    }
}

impl BlobExportOfflineChunkDeclaration {
    pub(crate) fn from_collected_chunk(chunk: &BlobExportedChunkBytes<'_>) -> Self {
        Self {
            ordinal: chunk.leaf().ordinal().get(),
            chunk_identity: chunk.leaf().identity().chunk_digest().as_str().to_owned(),
            stored_digest: chunk.leaf().stored_digest().digest().as_str().to_owned(),
            checksum_digest: chunk.leaf().checksum_digest().as_str().to_owned(),
            bytes: chunk.bytes().range().len(),
        }
    }

    pub const fn ordinal(&self) -> u64 {
        self.ordinal
    }

    pub fn chunk_identity(&self) -> &str {
        &self.chunk_identity
    }

    pub fn stored_digest(&self) -> &str {
        &self.stored_digest
    }

    pub fn checksum_digest(&self) -> &str {
        &self.checksum_digest
    }

    pub const fn bytes(&self) -> u64 {
        self.bytes
    }
}

pub(crate) fn declaration_digest(
    declarations: &[BlobExportOfflineChunkDeclaration],
) -> StableDigest {
    let mut hash = stable_hash_bytes(0xcbf2_9ce4_8422_2325, b"phase19.export.declarations");
    let total_bytes: u64 = declarations
        .iter()
        .map(BlobExportOfflineChunkDeclaration::bytes)
        .sum();
    for declaration in declarations {
        hash = stable_hash_u64(hash, declaration.ordinal());
        hash = stable_hash_bytes(hash, declaration.chunk_identity().as_bytes());
        hash = stable_hash_bytes(hash, declaration.stored_digest().as_bytes());
        hash = stable_hash_bytes(hash, declaration.checksum_digest().as_bytes());
        hash = stable_hash_u64(hash, declaration.bytes());
    }
    hash = stable_hash_u64(hash, total_bytes);
    hash = stable_hash_u64(hash, declarations.len() as u64);
    StableDigest::new(format!("s7:export-declarations:{hash:016x}"))
        .expect("declaration digest is nonempty")
}

fn stable_hash_u64(hash: u64, value: u64) -> u64 {
    stable_hash_bytes(hash, &value.to_le_bytes())
}

fn stable_hash_bytes(mut hash: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}
