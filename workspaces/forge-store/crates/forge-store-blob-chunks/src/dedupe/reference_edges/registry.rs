use forge_store_contracts::StableDigest;

use crate::dedupe::verification::BlobChunkCollisionVerificationReceipt;
use crate::{
    BlobChunkDedupeAdmissionDenial, BlobChunkDedupeCounterSnapshot,
    BlobChunkDedupeDigestRewriteBasis, BlobChunkDedupeIndexPartition, BlobChunkDedupeReceipt,
    BlobChunkIdentity, BlobChunkSecurityMetadataWitness,
};

use super::reclaim_decision::{classify_reclaim, BlobChunkDedupeReclaimDecision};
use super::reference_set::BlobChunkDedupeReferenceSet;
use super::registered_edge::BlobChunkRegisteredDedupeReference;
use super::released_edges::BlobChunkDedupeReferenceRelease;

#[derive(Debug, PartialEq, Eq)]
pub struct BlobChunkDedupeReferenceRegistry {
    sets: Vec<BlobChunkDedupeReferenceSet>,
    index_partitions: Vec<StableDigest>,
    digest_rewrites: Vec<StableDigest>,
}

impl BlobChunkDedupeReferenceRegistry {
    pub fn new_store_owned() -> Self {
        Self {
            sets: Vec::new(),
            index_partitions: Vec::new(),
            digest_rewrites: Vec::new(),
        }
    }

    pub fn admit_receipt(
        &mut self,
        receipt: BlobChunkDedupeReceipt,
    ) -> Result<BlobChunkRegisteredDedupeReference, BlobChunkDedupeAdmissionDenial> {
        if let Some(set) =
            self.find_set_mut(receipt.existing_identity(), receipt.security_metadata())
        {
            return set.admit_receipt(&receipt);
        }
        let registered = BlobChunkRegisteredDedupeReference::from_first_receipt(&receipt);
        self.sets
            .push(BlobChunkDedupeReferenceSet::from_first_receipt(receipt));
        Ok(registered)
    }

    pub fn partition_index_after_collision(
        &mut self,
        receipt: &BlobChunkCollisionVerificationReceipt,
        partition_basis: StableDigest,
    ) -> BlobChunkDedupeIndexPartition {
        self.index_partitions.push(partition_basis.clone());
        super::collision_partition::executed_collision_partition(receipt, partition_basis)
    }

    pub fn rewrite_chunk_under_new_digest_basis(
        &mut self,
        receipt: &BlobChunkCollisionVerificationReceipt,
        rewrite_basis: StableDigest,
    ) -> BlobChunkDedupeDigestRewriteBasis {
        self.digest_rewrites.push(rewrite_basis.clone());
        super::digest_rewrite::executed_digest_rewrite(receipt, rewrite_basis)
    }

    pub fn live_edges_for(
        &self,
        identity: &BlobChunkIdentity,
        metadata: BlobChunkSecurityMetadataWitness,
    ) -> Option<u64> {
        self.find_set(identity, metadata)
            .map(BlobChunkDedupeReferenceSet::live_edges)
    }

    pub fn reclaim_decision_for(
        &self,
        identity: &BlobChunkIdentity,
        metadata: BlobChunkSecurityMetadataWitness,
    ) -> Option<BlobChunkDedupeReclaimDecision> {
        self.find_set(identity, metadata).map(classify_reclaim)
    }

    pub fn deny_candidate_edge_for(
        &mut self,
        identity: &BlobChunkIdentity,
        candidate: &BlobChunkIdentity,
        metadata: BlobChunkSecurityMetadataWitness,
    ) -> Result<(), BlobChunkDedupeAdmissionDenial> {
        let Some(set) = self.find_set_mut(identity, metadata) else {
            return Err(
                BlobChunkDedupeAdmissionDenial::DedupeReferenceEdgeMismatch {
                    counters: BlobChunkDedupeCounterSnapshot::start().record_cross_scope_denial(),
                },
            );
        };
        set.deny_candidate_edge(candidate)
    }

    pub fn deny_all_edges_for(
        &mut self,
        identity: &BlobChunkIdentity,
        metadata: BlobChunkSecurityMetadataWitness,
    ) -> Option<BlobChunkDedupeReferenceRelease> {
        let position = self
            .sets
            .iter()
            .position(|set| set.matches(identity, metadata))?;
        Some(self.sets.remove(position).deny_all_edges())
    }

    fn find_set(
        &self,
        identity: &BlobChunkIdentity,
        metadata: BlobChunkSecurityMetadataWitness,
    ) -> Option<&BlobChunkDedupeReferenceSet> {
        self.sets.iter().find(|set| set.matches(identity, metadata))
    }

    fn find_set_mut(
        &mut self,
        identity: &BlobChunkIdentity,
        metadata: BlobChunkSecurityMetadataWitness,
    ) -> Option<&mut BlobChunkDedupeReferenceSet> {
        self.sets
            .iter_mut()
            .find(|set| set.matches(identity, metadata))
    }
}
