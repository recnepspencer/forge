use worth_store_contracts::StableDigest;

use crate::{
    BlobChunkDedupeAdmissionDenial, BlobChunkDedupeCounterSnapshot, BlobChunkDedupeReceipt,
    BlobChunkIdentity, BlobChunkSecurityMetadataWitness,
};

use super::reference_identity::dedupe_reference_identity;
use super::registered_edge::BlobChunkRegisteredDedupeReference;
use super::released_edges::BlobChunkDedupeReferenceRelease;

#[derive(Debug, PartialEq, Eq)]
pub(super) struct BlobChunkDedupeReferenceSet {
    shared_identity: BlobChunkIdentity,
    security_metadata: BlobChunkSecurityMetadataWitness,
    counters: BlobChunkDedupeCounterSnapshot,
    edges: Vec<BlobChunkDedupeReferenceEdge>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct BlobChunkDedupeReferenceEdge {
    reference_identity: StableDigest,
    candidate_identity: BlobChunkIdentity,
    denied: bool,
}

impl BlobChunkDedupeReferenceSet {
    pub(super) fn from_first_receipt(receipt: BlobChunkDedupeReceipt) -> Self {
        let edge = BlobChunkDedupeReferenceEdge::from_receipt(&receipt, 1);
        Self {
            shared_identity: receipt.existing_identity().clone(),
            security_metadata: receipt.security_metadata(),
            counters: receipt.counters(),
            edges: vec![edge],
        }
    }

    pub(super) fn admit_receipt(
        &mut self,
        receipt: &BlobChunkDedupeReceipt,
    ) -> Result<BlobChunkRegisteredDedupeReference, BlobChunkDedupeAdmissionDenial> {
        if !self.matches(receipt.existing_identity(), receipt.security_metadata()) {
            return Err(self.edge_mismatch_denial());
        }
        self.counters = self.counters.record_reference_edge_admitted();
        let edge = BlobChunkDedupeReferenceEdge::from_receipt(receipt, self.edges.len() as u64 + 1);
        let registered = BlobChunkRegisteredDedupeReference::from_receipt_and_edge(receipt, &edge);
        self.edges.push(edge);
        Ok(registered)
    }

    pub(super) fn deny_candidate_edge(
        &mut self,
        candidate: &BlobChunkIdentity,
    ) -> Result<(), BlobChunkDedupeAdmissionDenial> {
        let Some(edge) = self
            .edges
            .iter_mut()
            .find(|edge| &edge.candidate_identity == candidate)
        else {
            return Err(self.edge_mismatch_denial());
        };
        if !edge.denied {
            edge.denied = true;
            self.counters = self.counters.record_reference_edges_denied(1);
        }
        Ok(())
    }

    pub(super) fn deny_all_edges(mut self) -> BlobChunkDedupeReferenceRelease {
        let mut newly_denied = 0;
        for edge in &mut self.edges {
            if !edge.denied {
                edge.denied = true;
                newly_denied += 1;
            }
        }
        self.counters = self.counters.record_reference_edges_denied(newly_denied);
        BlobChunkDedupeReferenceRelease::from_denied_set(self)
    }

    pub(super) fn matches(
        &self,
        identity: &BlobChunkIdentity,
        metadata: BlobChunkSecurityMetadataWitness,
    ) -> bool {
        &self.shared_identity == identity && self.security_metadata == metadata
    }

    pub(super) fn live_edges(&self) -> u64 {
        self.edges.iter().filter(|edge| !edge.denied).count() as u64
    }

    pub(super) fn denied_edges(&self) -> u64 {
        self.edges.iter().filter(|edge| edge.denied).count() as u64
    }

    pub(super) fn has_live_edges(&self) -> bool {
        self.live_edges() != 0
    }

    pub(super) const fn shared_identity(&self) -> &BlobChunkIdentity {
        &self.shared_identity
    }

    pub(super) const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub(super) const fn counters(&self) -> BlobChunkDedupeCounterSnapshot {
        self.counters
    }

    pub(super) fn released_reference_identities(&self) -> Vec<StableDigest> {
        self.edges
            .iter()
            .map(|edge| edge.reference_identity.clone())
            .collect()
    }

    fn edge_mismatch_denial(&self) -> BlobChunkDedupeAdmissionDenial {
        BlobChunkDedupeAdmissionDenial::DedupeReferenceEdgeMismatch {
            counters: self.counters.record_cross_scope_denial(),
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

    pub(super) const fn reference_identity(&self) -> &StableDigest {
        &self.reference_identity
    }
}
