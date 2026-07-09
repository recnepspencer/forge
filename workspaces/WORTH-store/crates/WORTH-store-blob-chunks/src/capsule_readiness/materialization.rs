use crate::{
    chunk_integrity::stable_digest_for_bytes, BlobChunkIdentity, BlobChunkReachabilityProofSet,
    BlobObjectClassification, BlobObjectId, BlobStreamingReadObservation,
    BlobStreamingReadObservedChunk, BlobStreamingVerifiedRead, ChunkTreeRoot, LogicalContentDigest,
};

use super::counters::BlobCapsuleReadinessCounters;
use super::denial::BlobCapsuleReadinessDenial;
use super::readiness::{BlobCapsuleMaterializationAuthority, ClassifiedBlobCapsuleSlice};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterializedBlobCapsuleChunk {
    pub(super) chunk_identity: BlobChunkIdentity,
    pub(super) observed: BlobStreamingReadObservedChunk,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedBlobCapsuleMaterialization {
    pub(super) verified_read: BlobStreamingVerifiedRead,
    pub(super) chunks: Vec<MaterializedBlobCapsuleChunk>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterializedBlobCapsuleBundle {
    pub(super) object_id: BlobObjectId,
    pub(super) generation: crate::BlobGeneration,
    pub(super) chunk_tree_root: ChunkTreeRoot,
    pub(super) logical_content_digest: LogicalContentDigest,
    pub(super) classification: BlobObjectClassification,
    pub(super) materialized_chunks: Vec<MaterializedBlobCapsuleChunk>,
    pub(super) declared_bytes: u64,
    pub(super) placement_scope: worth_store_security::StoreSecurityScopeIdentity,
    pub(super) reachability_fingerprint: String,
    pub(super) counters: BlobCapsuleReadinessCounters,
}

pub(super) fn reachability_fingerprint(reachability: &BlobChunkReachabilityProofSet) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for chunk in reachability.reachable_chunks() {
        hash = hash_bytes(hash, chunk.chunk_digest().as_str().as_bytes());
    }
    for edge in reachability.reference_edges() {
        hash = hash_bytes(hash, edge.as_str().as_bytes());
    }
    for hold in reachability.protected_holds() {
        hash = hash_bytes(hash, hold.as_str().as_bytes());
    }
    format!("s7:capsule-reachability:{hash:016x}")
}

pub(super) fn admit_materialized_capsule_read(
    authority: &BlobCapsuleMaterializationAuthority,
    classified: &ClassifiedBlobCapsuleSlice,
    verified_read: BlobStreamingVerifiedRead,
    observations: impl IntoIterator<Item = BlobStreamingReadObservation>,
) -> Result<PreparedBlobCapsuleMaterialization, BlobCapsuleReadinessDenial> {
    if authority.object_id() != verified_read.object_id()
        || authority.generation() != verified_read.generation()
        || authority.chunk_tree_root() != verified_read.chunk_tree_root()
        || authority.logical_content_digest() != verified_read.logical_content_digest()
    {
        return Err(BlobCapsuleReadinessDenial::DigestOnlyChunkReference {
            counters: classified.planned.counters.record_denied_chunk(),
        });
    }

    let mut materialized_chunks = Vec::new();
    let mut selected = classified.planned.selected_leaves.iter().peekable();
    for observation in observations {
        let Some(expected_leaf) = selected.peek() else {
            break;
        };
        let observed = match observation {
            BlobStreamingReadObservation::Chunk(observed) => observed,
            BlobStreamingReadObservation::ColdUnavailable { ordinal, .. } => {
                if ordinal != expected_leaf.ordinal() {
                    continue;
                }
                return Err(BlobCapsuleReadinessDenial::ColdPlacementUnavailable {
                    counters: classified.planned.counters.record_denied_chunk(),
                });
            }
        };
        if observed.ordinal() != expected_leaf.ordinal() {
            continue;
        }
        let leaf = selected
            .next()
            .expect("peeked selected leaf must still be present");
        if observed.byte_range() != leaf.byte_range() {
            return Err(BlobCapsuleReadinessDenial::MissingChunk {
                ordinal: leaf.ordinal().get(),
                counters: classified.planned.counters.record_denied_chunk(),
            });
        }
        let recomputed_content_digest = stable_digest_for_bytes(
            "content",
            "s7.fixed-size.raw-chunk.v1",
            leaf.ordinal(),
            leaf.byte_range(),
            observed.payload().payload_bytes(),
        );
        if &recomputed_content_digest != leaf.content_digest().digest() {
            return Err(BlobCapsuleReadinessDenial::DigestOnlyChunkReference {
                counters: classified.planned.counters.record_denied_chunk(),
            });
        }
        materialized_chunks.push(MaterializedBlobCapsuleChunk {
            chunk_identity: leaf.identity().clone(),
            observed,
        });
    }
    if let Some(leaf) = selected.next() {
        return Err(BlobCapsuleReadinessDenial::MissingChunk {
            ordinal: leaf.ordinal().get(),
            counters: classified.planned.counters.record_denied_chunk(),
        });
    }
    Ok(PreparedBlobCapsuleMaterialization {
        verified_read,
        chunks: materialized_chunks,
    })
}

pub(super) fn validate_materialized_chunks(
    classified: &ClassifiedBlobCapsuleSlice,
    materialized: &PreparedBlobCapsuleMaterialization,
) -> Result<(), BlobCapsuleReadinessDenial> {
    if materialized.chunks.len() != classified.planned.selected_leaves.len() {
        return Err(BlobCapsuleReadinessDenial::MissingChunk {
            ordinal: classified.planned.selected_leaves.len() as u64,
            counters: classified.planned.counters.record_denied_chunk(),
        });
    }
    for (leaf, chunk) in classified
        .planned
        .selected_leaves
        .iter()
        .zip(&materialized.chunks)
    {
        if chunk.observed.byte_range() != leaf.byte_range() {
            return Err(BlobCapsuleReadinessDenial::MissingChunk {
                ordinal: leaf.ordinal().get(),
                counters: classified.planned.counters.record_denied_chunk(),
            });
        }
        if chunk.chunk_identity != *leaf.identity() {
            return Err(BlobCapsuleReadinessDenial::DigestOnlyChunkReference {
                counters: classified.planned.counters.record_denied_chunk(),
            });
        }
    }
    Ok(())
}

pub(super) fn readiness_digest(
    object_id: &BlobObjectId,
    generation: crate::BlobGeneration,
    chunk_tree_root: &ChunkTreeRoot,
    logical_content_digest: &LogicalContentDigest,
    materialized_chunks: &[MaterializedBlobCapsuleChunk],
    declared_bytes: u64,
    reachability_fingerprint: &str,
) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    hash = hash_bytes(hash, object_id.digest().as_str().as_bytes());
    hash = hash_bytes(hash, &generation.sequence().to_le_bytes());
    hash = hash_bytes(hash, chunk_tree_root.digest().as_str().as_bytes());
    hash = hash_bytes(hash, logical_content_digest.digest().as_str().as_bytes());
    hash = hash_bytes(hash, &declared_bytes.to_le_bytes());
    hash = hash_bytes(hash, reachability_fingerprint.as_bytes());
    for chunk in materialized_chunks {
        hash = hash_bytes(
            hash,
            chunk.chunk_identity.chunk_digest().as_str().as_bytes(),
        );
        hash = hash_bytes(hash, &chunk.observed.byte_range().start().to_le_bytes());
        hash = hash_bytes(hash, chunk.observed.payload().payload_bytes());
    }
    format!("s7:capsule-readiness:{hash:016x}")
}

fn hash_bytes(mut hash: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}
