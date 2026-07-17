use crate::{
    BlobChunkDedupeAdmissionDenial, BlobChunkDedupeReceipt, BlobChunkDedupeReferenceRegistry,
    BlobChunkDedupeReferenceRelease, BlobChunkIdentity, BlobChunkProofLeaf,
    BlobChunkReachabilityProofSet, BlobChunkReachabilityRegistry, BlobChunkSecurityMetadataWitness,
    BlobGenerationPublished, BlobReachabilityDenial, BlobReachabilityEdge,
    BlobReachabilityReclaimDecision,
};

#[derive(Debug, PartialEq, Eq)]
pub struct BlobChunkReferenceAccountingRegistry {
    dedupe: BlobChunkDedupeReferenceRegistry,
    reachability: BlobChunkReachabilityRegistry,
}

impl BlobChunkReferenceAccountingRegistry {
    pub fn new_store_owned() -> Self {
        Self {
            dedupe: BlobChunkDedupeReferenceRegistry::new_store_owned(),
            reachability: BlobChunkReachabilityRegistry::new_store_owned(),
        }
    }

    pub fn admit_dedupe_reference(
        &mut self,
        receipt: BlobChunkDedupeReceipt,
        published: &BlobGenerationPublished,
        leaf: &BlobChunkProofLeaf,
    ) -> Result<(), BlobChunkReferenceAccountingDenial> {
        let registered = self.dedupe.admit_receipt(receipt)?;
        let edge = BlobReachabilityEdge::dedupe_shared_reference(&registered, published, leaf)?;
        self.reachability.admit_edge(edge)?;
        Ok(())
    }

    pub fn deny_all_dedupe_edges_for(
        &mut self,
        identity: &BlobChunkIdentity,
        metadata: BlobChunkSecurityMetadataWitness,
    ) -> Option<BlobChunkDedupeReferenceRelease> {
        let release = self.dedupe.deny_all_edges_for(identity, metadata)?;
        self.reachability
            .apply_registry_owned_dedupe_reference_release(&release);
        Some(release)
    }

    pub fn prove_reachable_chunks(
        &self,
    ) -> Result<BlobChunkReachabilityProofSet, BlobReachabilityDenial> {
        self.reachability.prove_reachable_chunks()
    }

    pub fn reclaim_decision_for(
        &self,
        identity: &BlobChunkIdentity,
    ) -> BlobReachabilityReclaimDecision {
        self.reachability.reclaim_decision_for(identity)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobChunkReferenceAccountingDenial {
    Dedupe(Box<BlobChunkDedupeAdmissionDenial>),
    Reachability(BlobReachabilityDenial),
}

impl From<BlobChunkDedupeAdmissionDenial> for BlobChunkReferenceAccountingDenial {
    fn from(value: BlobChunkDedupeAdmissionDenial) -> Self {
        Self::Dedupe(Box::new(value))
    }
}

impl From<BlobReachabilityDenial> for BlobChunkReferenceAccountingDenial {
    fn from(value: BlobReachabilityDenial) -> Self {
        Self::Reachability(value)
    }
}
