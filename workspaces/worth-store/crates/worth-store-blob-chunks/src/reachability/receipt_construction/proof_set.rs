use crate::reachability::counters::BlobReachabilityCounterSnapshot;
use crate::reachability::types::{BlobChunkReachabilityProofSet, BlobChunkReachabilityRegistry};
use crate::BlobChunkIdentity;

pub(crate) fn collect_unique_reachable_chunks(
    registry: &BlobChunkReachabilityRegistry,
) -> Vec<BlobChunkIdentity> {
    let mut chunks = Vec::new();
    for edge in registry.edges() {
        if !chunks.iter().any(|chunk| chunk == edge.chunk_identity()) {
            chunks.push(edge.chunk_identity().clone());
        }
    }
    chunks.sort_by(|left, right| {
        left.chunk_digest()
            .as_str()
            .cmp(right.chunk_digest().as_str())
    });
    chunks
}

pub(crate) fn collect_orphan_candidates(
    _registry: &BlobChunkReachabilityRegistry,
) -> Vec<BlobChunkIdentity> {
    Vec::new()
}

pub(crate) fn exact_current_counters_for(
    registry: &BlobChunkReachabilityRegistry,
    reachable_chunks: &[BlobChunkIdentity],
) -> BlobReachabilityCounterSnapshot {
    registry
        .stored_counters()
        .with_current_reference_edges(
            registry.edges().len() as u64,
            registry
                .edges()
                .iter()
                .filter(|edge| edge.is_dedupe())
                .count() as u64,
        )
        .with_current_protected_holds(registry.holds().len() as u64)
        .with_reachable_chunks(reachable_chunks.len() as u64)
        .with_orphan_candidates(collect_orphan_candidates(registry).len() as u64)
}

pub(crate) fn construct_proof_set(
    registry: &BlobChunkReachabilityRegistry,
) -> BlobChunkReachabilityProofSet {
    let first_edge = registry
        .edges()
        .first()
        .expect("proof_set is only called after nonempty edge proof");
    let authority = registry
        .authority()
        .expect("nonempty reachability proof has authority");
    let reachable_chunks = collect_unique_reachable_chunks(registry);
    let counters = exact_current_counters_for(registry, &reachable_chunks);
    BlobChunkReachabilityProofSet::construct(
        crate::reachability::types::BlobReachabilityProofSetParts {
            authority,
            reachable_chunks,
            stored_digest: first_edge.stored_digest().clone(),
            security_metadata: first_edge.security_metadata(),
            reference_edges: registry
                .edges()
                .iter()
                .map(|edge| edge.identity().clone())
                .collect(),
            protected_holds: registry
                .holds()
                .iter()
                .map(|hold| hold.identity().clone())
                .collect(),
            orphan_candidates: collect_orphan_candidates(registry),
            counters,
        },
    )
}
