use forge_store_contracts::StableDigest;

mod accessors;

use crate::dedupe::verification::BlobChunkCollisionVerificationReceipt;
use crate::{
    BlobChunkDedupeAdmissionDenial,
    BlobChunkDedupeCounterSnapshot, BlobChunkDedupeDigestRewriteBasis,
    BlobChunkDedupeIndexPartition, BlobChunkDedupeReceipt, BlobChunkIdentity,
    BlobChunkSecurityMetadataWitness,
};

#[derive(Debug, PartialEq, Eq)]
pub struct BlobChunkDedupeReferenceSet {
    shared_identity: BlobChunkIdentity,
    security_metadata: BlobChunkSecurityMetadataWitness,
    counters: BlobChunkDedupeCounterSnapshot,
    edges: Vec<BlobChunkDedupeReferenceEdge>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlobChunkDedupeReferenceRegistry {
    sets: Vec<BlobChunkDedupeReferenceSet>,
    index_partitions: Vec<StableDigest>,
    digest_rewrites: Vec<StableDigest>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BlobChunkDedupeReferenceEdge {
    reference_identity: StableDigest,
    candidate_identity: BlobChunkIdentity,
    denied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkRegisteredDedupeReference {
    reference_identity: StableDigest,
    shared_identity: BlobChunkIdentity,
    candidate_identity: BlobChunkIdentity,
    content_digest: StableDigest,
    security_metadata: BlobChunkSecurityMetadataWitness,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkDedupeReferenceRelease {
    shared_identity: BlobChunkIdentity,
    security_metadata: BlobChunkSecurityMetadataWitness,
    counters: BlobChunkDedupeCounterSnapshot,
    released_edges: u64,
    released_reference_identities: Vec<StableDigest>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobChunkDedupeReclaimDecision {
    ReclaimPermitted(BlobChunkDedupeReferenceRelease),
    ReclaimDenied(BlobChunkDedupeCounterSnapshot),
}

impl BlobChunkDedupeReferenceSet {
    pub(crate) fn from_receipt(receipt: BlobChunkDedupeReceipt) -> Self {
        let shared_identity = receipt.existing_identity().clone();
        let edge = BlobChunkDedupeReferenceEdge::from_receipt(&receipt, 1);
        Self {
            shared_identity,
            security_metadata: receipt.security_metadata(),
            counters: receipt.counters(),
            edges: vec![edge],
        }
    }

    pub(crate) fn admit_receipt(
        &mut self,
        receipt: &BlobChunkDedupeReceipt,
    ) -> Result<BlobChunkRegisteredDedupeReference, BlobChunkDedupeAdmissionDenial> {
        if receipt.existing_identity() != &self.shared_identity
            || receipt.security_metadata() != self.security_metadata
        {
            return Err(
                BlobChunkDedupeAdmissionDenial::DedupeReferenceEdgeMismatch {
                    counters: self.counters.record_cross_scope_denial(),
                },
            );
        }
        self.counters = self.counters.record_reference_edge_admitted();
        let ordinal = self.edges.len() as u64 + 1;
        let edge = BlobChunkDedupeReferenceEdge::from_receipt(receipt, ordinal);
        let registered = BlobChunkRegisteredDedupeReference::from_receipt_and_edge(receipt, &edge);
        self.edges.push(edge);
        Ok(registered)
    }

    pub const fn existing_identity(&self) -> &BlobChunkIdentity {
        &self.shared_identity
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub fn live_edges(&self) -> u64 {
        self.edges.iter().filter(|edge| !edge.denied).count() as u64
    }

    pub fn denied_edges(&self) -> u64 {
        self.edges.iter().filter(|edge| edge.denied).count() as u64
    }

    pub fn deny_all_edges(mut self) -> BlobChunkDedupeReferenceRelease {
        let mut newly_denied = 0;
        for edge in &mut self.edges {
            if !edge.denied {
                edge.denied = true;
                newly_denied += 1;
            }
        }
        let denied_edges = self.denied_edges();
        BlobChunkDedupeReferenceRelease {
            shared_identity: self.shared_identity,
            security_metadata: self.security_metadata,
            counters: self.counters.record_reference_edges_denied(newly_denied),
            released_edges: denied_edges,
            released_reference_identities: self
                .edges
                .into_iter()
                .map(|edge| edge.reference_identity)
                .collect(),
        }
    }

    pub fn reclaim_decision(&self) -> BlobChunkDedupeReclaimDecision {
        if self.live_edges() == 0 {
            BlobChunkDedupeReclaimDecision::ReclaimPermitted(BlobChunkDedupeReferenceRelease {
                shared_identity: self.shared_identity.clone(),
                security_metadata: self.security_metadata,
                counters: self.counters,
                released_edges: self.denied_edges(),
                released_reference_identities: self
                    .edges
                    .iter()
                    .map(|edge| edge.reference_identity.clone())
                    .collect(),
            })
        } else {
            BlobChunkDedupeReclaimDecision::ReclaimDenied(
                self.counters.record_reclaim_blocked_by_reference_edge(),
            )
        }
    }

    fn deny_candidate_edge(
        &mut self,
        candidate: &BlobChunkIdentity,
    ) -> Result<(), BlobChunkDedupeAdmissionDenial> {
        let Some(edge) = self
            .edges
            .iter_mut()
            .find(|edge| &edge.candidate_identity == candidate)
        else {
            return Err(
                BlobChunkDedupeAdmissionDenial::DedupeReferenceEdgeMismatch {
                    counters: self.counters.record_cross_scope_denial(),
                },
            );
        };
        if !edge.denied {
            edge.denied = true;
            self.counters = self.counters.record_reference_edges_denied(1);
        }
        Ok(())
    }
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
        let registered = BlobChunkRegisteredDedupeReference::from_receipt(&receipt);
        self.sets
            .push(BlobChunkDedupeReferenceSet::from_receipt(receipt));
        Ok(registered)
    }

    pub fn partition_index_after_collision(
        &mut self,
        receipt: &BlobChunkCollisionVerificationReceipt,
        partition_basis: StableDigest,
    ) -> BlobChunkDedupeIndexPartition {
        self.index_partitions.push(partition_basis.clone());
        BlobChunkDedupeIndexPartition::from_executed_partition(receipt, partition_basis)
    }

    pub fn rewrite_chunk_under_new_digest_basis(
        &mut self,
        receipt: &BlobChunkCollisionVerificationReceipt,
        rewrite_basis: StableDigest,
    ) -> BlobChunkDedupeDigestRewriteBasis {
        self.digest_rewrites.push(rewrite_basis.clone());
        BlobChunkDedupeDigestRewriteBasis::from_executed_rewrite(receipt, rewrite_basis)
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
        self.find_set(identity, metadata)
            .map(BlobChunkDedupeReferenceSet::reclaim_decision)
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
        let position = self.sets.iter().position(|set| {
            set.existing_identity() == identity && set.security_metadata() == metadata
        })?;
        Some(self.sets.remove(position).deny_all_edges())
    }

    fn find_set(
        &self,
        identity: &BlobChunkIdentity,
        metadata: BlobChunkSecurityMetadataWitness,
    ) -> Option<&BlobChunkDedupeReferenceSet> {
        self.sets
            .iter()
            .find(|set| set.existing_identity() == identity && set.security_metadata() == metadata)
    }

    fn find_set_mut(
        &mut self,
        identity: &BlobChunkIdentity,
        metadata: BlobChunkSecurityMetadataWitness,
    ) -> Option<&mut BlobChunkDedupeReferenceSet> {
        self.sets
            .iter_mut()
            .find(|set| set.existing_identity() == identity && set.security_metadata() == metadata)
    }
}

impl BlobChunkRegisteredDedupeReference {
    fn from_receipt(receipt: &BlobChunkDedupeReceipt) -> Self {
        let edge = BlobChunkDedupeReferenceEdge::from_receipt(receipt, 1);
        Self::from_receipt_and_edge(receipt, &edge)
    }

    fn from_receipt_and_edge(
        receipt: &BlobChunkDedupeReceipt,
        edge: &BlobChunkDedupeReferenceEdge,
    ) -> Self {
        Self {
            reference_identity: edge.reference_identity.clone(),
            shared_identity: receipt.existing_identity().clone(),
            candidate_identity: receipt.candidate_identity().clone(),
            content_digest: receipt.content_digest().clone(),
            security_metadata: receipt.security_metadata(),
        }
    }
}

impl BlobChunkDedupeReferenceEdge {
    fn from_receipt(receipt: &BlobChunkDedupeReceipt, ordinal: u64) -> Self {
        Self {
            reference_identity: dedupe_reference_identity(receipt, ordinal),
            candidate_identity: receipt.candidate_identity().clone(),
            denied: false,
        }
    }
}

fn dedupe_reference_identity(receipt: &BlobChunkDedupeReceipt, ordinal: u64) -> StableDigest {
    StableDigest::new(format!(
        "s7.dedupe.ref:{}:{}:{}:{}",
        receipt.existing_identity().chunk_digest().as_str(),
        receipt.candidate_identity().chunk_digest().as_str(),
        receipt.content_digest().as_str(),
        ordinal
    ))
    .expect("dedupe reference identity is nonempty")
}