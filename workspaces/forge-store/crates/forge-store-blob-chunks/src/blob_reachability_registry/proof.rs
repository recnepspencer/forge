use crate::{
    BlobChunkIdentity, BlobChunkReachabilityProofSet, BlobChunkReachabilityRegistry,
    BlobReachabilityCounterSnapshot, BlobReachabilityDenial,
};

impl BlobChunkReachabilityRegistry {
    pub fn prove_reachable_chunks(
        &self,
    ) -> Result<BlobChunkReachabilityProofSet, BlobReachabilityDenial> {
        if self.edges.is_empty() {
            return Err(BlobReachabilityDenial::EmptyReferenceProofRejected {
                counters: self.counters.record_empty_proof_denial(),
            });
        }
        Ok(self.proof_set())
    }

    pub fn canonical_snapshot(
        &self,
    ) -> Result<crate::BlobReachabilityCanonicalSnapshot, BlobReachabilityDenial> {
        let proof = self.prove_reachable_chunks()?;
        Ok(proof.into_canonical_snapshot(
            self.exact_current_counters()
                .record_replay_convergence_check(),
        ))
    }

    pub fn counters(&self) -> BlobReachabilityCounterSnapshot {
        self.exact_current_counters()
    }

    fn proof_set(&self) -> BlobChunkReachabilityProofSet {
        let first_edge = self
            .edges
            .first()
            .expect("proof_set is only called after nonempty edge proof");
        let authority = self
            .authority
            .clone()
            .expect("nonempty reachability proof has authority");
        let reachable_chunks = self.unique_reachable_chunks();
        BlobChunkReachabilityProofSet {
            authority,
            stored_digest: first_edge.stored_digest().clone(),
            security_metadata: first_edge.security_metadata(),
            reference_edges: self
                .edges
                .iter()
                .map(|edge| edge.identity().clone())
                .collect(),
            protected_holds: self
                .holds
                .iter()
                .map(|hold| hold.identity().clone())
                .collect(),
            orphan_candidates: self.orphan_candidates(),
            counters: self.exact_current_counters_for(&reachable_chunks),
            reachable_chunks,
        }
    }

    fn unique_reachable_chunks(&self) -> Vec<BlobChunkIdentity> {
        let mut chunks = Vec::new();
        for edge in &self.edges {
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

    fn orphan_candidates(&self) -> Vec<BlobChunkIdentity> {
        Vec::new()
    }

    fn exact_current_counters(&self) -> BlobReachabilityCounterSnapshot {
        let reachable_chunks = self.unique_reachable_chunks();
        self.exact_current_counters_for(&reachable_chunks)
    }

    fn exact_current_counters_for(
        &self,
        reachable_chunks: &[BlobChunkIdentity],
    ) -> BlobReachabilityCounterSnapshot {
        self.counters
            .with_current_reference_edges(
                self.edges.len() as u64,
                self.edges.iter().filter(|edge| edge.is_dedupe()).count() as u64,
            )
            .with_current_protected_holds(self.holds.len() as u64)
            .with_reachable_chunks(reachable_chunks.len() as u64)
            .with_orphan_candidates(self.orphan_candidates().len() as u64)
    }
}
