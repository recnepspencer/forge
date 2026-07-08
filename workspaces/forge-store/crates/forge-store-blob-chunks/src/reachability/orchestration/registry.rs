use crate::reachability::receipt_construction::proof_set::{
    collect_unique_reachable_chunks, exact_current_counters_for,
};
use crate::reachability::receipt_construction::{
    BlobReachabilityCanonicalSnapshot, BlobReachabilityEdgeRelease,
};
use crate::reachability::transitions::apply_dedupe_release::transition_apply_dedupe_reference_release;
use crate::reachability::transitions::classify_reclaim::classify_reclaim_for_identity;
use crate::reachability::transitions::prove_reachable::transition_prove_reachable_chunks;
use crate::reachability::transitions::release_edge::transition_release_edge;
use crate::reachability::types::{
    BlobChunkReachabilityProofSet, BlobChunkReachabilityRegistry, BlobReachabilityReclaimDecision,
};
use crate::{
    BlobChunkDedupeReferenceRelease, BlobChunkIdentity, BlobReachabilityDenial,
    BlobReachabilityEdge, BlobRetentionHold,
};

impl BlobChunkReachabilityRegistry {
    pub fn release_edge(
        &mut self,
        edge: &BlobReachabilityEdge,
    ) -> Result<BlobReachabilityEdgeRelease, BlobReachabilityDenial> {
        transition_release_edge(self, edge)
    }

    pub(crate) fn apply_registry_owned_dedupe_reference_release(
        &mut self,
        release: &BlobChunkDedupeReferenceRelease,
    ) {
        transition_apply_dedupe_reference_release(self, release);
    }

    pub fn reclaim_decision_for(
        &self,
        identity: &BlobChunkIdentity,
    ) -> BlobReachabilityReclaimDecision {
        classify_reclaim_for_identity(self, identity)
    }

    pub(crate) fn first_retention_hold_for_reclaim(&self) -> Option<BlobRetentionHold> {
        self.holds().first().map(|hold| {
            BlobRetentionHold::from_reachability_hold_kind(hold.kind(), hold.identity().as_str())
        })
    }

    pub fn prove_reachable_chunks(
        &self,
    ) -> Result<BlobChunkReachabilityProofSet, BlobReachabilityDenial> {
        transition_prove_reachable_chunks(self)
    }

    pub fn canonical_snapshot(
        &self,
    ) -> Result<BlobReachabilityCanonicalSnapshot, BlobReachabilityDenial> {
        let proof = self.prove_reachable_chunks()?;
        Ok(proof.into_canonical_snapshot(
            exact_current_counters_for(self, &collect_unique_reachable_chunks(self))
                .record_replay_convergence_check(),
        ))
    }

    pub fn counters(&self) -> crate::BlobReachabilityCounterSnapshot {
        exact_current_counters_for(self, &collect_unique_reachable_chunks(self))
    }

    pub fn orphan_candidates(&self) -> &[BlobChunkIdentity] {
        // Kept for test compatibility via proof set path; registry stores none directly.
        &[]
    }
}
