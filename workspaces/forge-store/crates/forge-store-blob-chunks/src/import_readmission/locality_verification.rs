use crate::{BlobChunkIdentity, BlobChunkSecurityMetadataWitness, StoredChunkDigest};

use super::chunk_evidence::BlobImportedChunkEvidence;
use super::counters::BlobImportReadmissionCounters;
use super::declaration::BlobImportChunkDeclaration;
use super::denial::BlobImportReadmissionDenial;

#[derive(Debug, Clone)]
pub(super) struct ImportedBlobWitnessBasis {
    pub(super) reachable_chunks: Vec<BlobChunkIdentity>,
    pub(super) stored_digest: StoredChunkDigest,
    pub(super) chunk_security_metadata: BlobChunkSecurityMetadataWitness,
}

pub(super) struct VerifiedChunkLocality {
    pub(super) local_chunks: u64,
    pub(super) witness_basis: Option<ImportedBlobWitnessBasis>,
}

pub(super) fn verify_declared_chunks(
    rows: &[BlobImportChunkDeclaration],
    current_chunks: &[BlobImportedChunkEvidence<'_>],
    readmitted_security_metadata: BlobChunkSecurityMetadataWitness,
    counters: BlobImportReadmissionCounters,
) -> Result<VerifiedChunkLocality, BlobImportReadmissionDenial> {
    let mut reachable_chunks = Vec::new();
    let mut stored_digest = None;
    let mut chunk_security_metadata = None;
    let mut local_chunks = 0_u64;
    for row in rows {
        let Some(chunk) = current_chunks
            .iter()
            .find(|chunk| chunk_matches(row, chunk))
        else {
            continue;
        };
        let leaf_security = chunk.leaf().security_metadata();
        if !same_security_scope(leaf_security, readmitted_security_metadata) {
            return Err(BlobImportReadmissionDenial::ChunkEvidenceMismatch { counters });
        }
        match chunk_security_metadata {
            Some(existing) if existing != leaf_security => {
                return Err(BlobImportReadmissionDenial::ChunkEvidenceMismatch { counters });
            }
            None => chunk_security_metadata = Some(leaf_security),
            _ => {}
        }
        stored_digest.get_or_insert_with(|| chunk.leaf().stored_digest().clone());
        reachable_chunks.push(chunk.leaf().identity().clone());
        local_chunks += 1;
    }
    let witness_basis = (local_chunks == rows.len() as u64).then(|| ImportedBlobWitnessBasis {
        reachable_chunks,
        stored_digest: stored_digest.expect("fully-local import implies stored digest"),
        chunk_security_metadata: chunk_security_metadata
            .expect("fully-local import implies security metadata"),
    });
    Ok(VerifiedChunkLocality {
        local_chunks,
        witness_basis,
    })
}

fn chunk_matches(row: &BlobImportChunkDeclaration, chunk: &BlobImportedChunkEvidence<'_>) -> bool {
    let leaf = chunk.leaf();
    leaf.ordinal().get() == row.ordinal()
        && leaf.identity().chunk_digest().as_str() == row.chunk_identity()
        && leaf.stored_digest().digest().as_str() == row.stored_digest()
        && leaf.checksum_digest().as_str() == row.checksum_digest()
        && chunk.bytes().range().len() == row.bytes()
}

fn same_security_scope(
    left: BlobChunkSecurityMetadataWitness,
    right: BlobChunkSecurityMetadataWitness,
) -> bool {
    left.key_scope() == right.key_scope()
        && left.key_version_posture() == right.key_version_posture()
        && left.tenant_scope() == right.tenant_scope()
        && left.authenticity_requirement() == right.authenticity_requirement()
}
