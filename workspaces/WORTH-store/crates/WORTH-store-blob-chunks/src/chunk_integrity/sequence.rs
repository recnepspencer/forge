use worth_store_contracts::StableDigest;
use worth_store_physical_format::PhysicalChunkPayloadIntegrityWitness;

use crate::{
    chunk_integrity::stable_digest_for, BlobChunkByteWindow, BlobChunkContentDigest,
    BlobChunkIdentity, BlobChunkIntegrityCounterSnapshot, BlobChunkIntegrityDenial,
    BlobChunkIntegrityProof, BlobChunkOrdinal, BlobChunkSecurityMetadataWitness,
    BlobChunkSecurityScope, BlobChunkingRuleAdmission, ChunkTreeRoot, LogicalContentDigest,
    S7BlobChunkSecurityHandoff, StoredChunkDigest,
};

#[derive(Debug, PartialEq, Eq)]
pub struct BlobChunkSequenceAdmission {
    security_scope: BlobChunkSecurityScope,
    rule: BlobChunkingRuleAdmission,
    declared_total_bytes: u64,
    next_offset: u64,
    next_ordinal: BlobChunkOrdinal,
    proof_frontier: Option<BlobChunkProofFrontier>,
    counters: BlobChunkIntegrityCounterSnapshot,
}

impl BlobChunkSequenceAdmission {
    pub fn start(
        security_scope: BlobChunkSecurityScope,
        rule: BlobChunkingRuleAdmission,
        declared_total_bytes: u64,
    ) -> Result<Self, BlobChunkIntegrityDenial> {
        Self::new(security_scope, rule, declared_total_bytes)
    }

    pub fn start_from_handoff(
        security: S7BlobChunkSecurityHandoff,
        rule: BlobChunkingRuleAdmission,
        declared_total_bytes: u64,
    ) -> Result<Self, BlobChunkIntegrityDenial> {
        Self::new(
            BlobChunkSecurityScope::from_s7_handoff(security),
            rule,
            declared_total_bytes,
        )
    }

    fn new(
        security_scope: BlobChunkSecurityScope,
        rule: BlobChunkingRuleAdmission,
        declared_total_bytes: u64,
    ) -> Result<Self, BlobChunkIntegrityDenial> {
        if declared_total_bytes == 0 {
            return Err(BlobChunkIntegrityDenial::EmptyByteWindow);
        }

        Ok(Self {
            security_scope,
            rule,
            declared_total_bytes,
            next_offset: 0,
            next_ordinal: BlobChunkOrdinal::first(),
            proof_frontier: None,
            counters: BlobChunkIntegrityCounterSnapshot::start(),
        })
    }

    pub fn push_payload(
        mut self,
        start: u64,
        payload: PhysicalChunkPayloadIntegrityWitness,
    ) -> Result<Self, BlobChunkIntegrityDenial> {
        let window = BlobChunkByteWindow::borrowed(start, payload.payload_bytes())?;
        if window.range().start() != self.next_offset {
            return Err(BlobChunkIntegrityDenial::UnexpectedWindowOffset {
                expected: self.next_offset,
                actual: window.range().start(),
                counters: self.counters.record_order_denial(),
            });
        }
        if window.range().end() > self.declared_total_bytes {
            return Err(BlobChunkIntegrityDenial::DuplicateOrReorderedChunk {
                counters: self.counters.record_order_denial(),
            });
        }
        if window.range().end() < self.declared_total_bytes
            && window.range().len() != self.rule.chunk_size().bytes()
        {
            return Err(BlobChunkIntegrityDenial::NonCanonicalInteriorChunk {
                counters: self.counters.record_order_denial(),
            });
        }

        let proof = BlobChunkIntegrityProof::admit(
            self.next_ordinal,
            window,
            payload.checksum().clone(),
            &self.rule,
            self.security_scope.metadata(),
            self.counters,
        )?;
        self.counters = proof.counters().record_checksum_computed();
        self.next_offset = proof.byte_range().end();
        self.next_ordinal = self.next_ordinal.next();
        self.proof_frontier = Some(match self.proof_frontier {
            Some(frontier) => frontier.advance(proof, payload.payload_bytes()),
            None => BlobChunkProofFrontier::first(proof, payload.payload_bytes()),
        });
        Ok(self)
    }

    pub fn finish(self) -> Result<AdmittedBlobChunkSequence, BlobChunkIntegrityDenial> {
        if self.next_offset != self.declared_total_bytes {
            return Err(BlobChunkIntegrityDenial::MissingTailChunk {
                expected_total_bytes: self.declared_total_bytes,
                actual_total_bytes: self.next_offset,
                counters: self.counters.record_order_denial(),
            });
        }

        let chunk_count = self.next_ordinal.get();
        let counters = self.counters.record_sequence_finalized(chunk_count);
        let proof_frontier = self
            .proof_frontier
            .expect("nonempty finalized sequence has a proof frontier");
        let chunk_tree_root = proof_frontier.chunk_tree_root();
        let logical_content_digest = proof_frontier.logical_content_digest();
        let chunk_identity_summary = proof_frontier.chunk_identity_summary();

        Ok(AdmittedBlobChunkSequence {
            proof_frontier,
            chunk_identity_summary,
            chunk_tree_root,
            logical_content_digest,
            counters,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedBlobChunkSequence {
    proof_frontier: BlobChunkProofFrontier,
    chunk_identity_summary: StableDigest,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    counters: BlobChunkIntegrityCounterSnapshot,
}

impl AdmittedBlobChunkSequence {
    pub const fn proof_frontier(&self) -> &BlobChunkProofFrontier {
        &self.proof_frontier
    }

    pub const fn first_chunk(&self) -> &BlobChunkIntegrityProof {
        self.proof_frontier.first_chunk()
    }

    pub const fn chunk_identity_summary(&self) -> &StableDigest {
        &self.chunk_identity_summary
    }

    pub(crate) const fn chunk_tree_root(&self) -> &ChunkTreeRoot {
        &self.chunk_tree_root
    }

    pub(crate) const fn logical_content_digest(&self) -> &LogicalContentDigest {
        &self.logical_content_digest
    }

    pub const fn counters(&self) -> BlobChunkIntegrityCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkProofFrontier {
    first_chunk: BlobChunkIntegrityProof,
    latest_chunk: BlobChunkIntegrityProof,
    chunk_count: u64,
    total_bytes: u64,
    chunk_identity_basis: u64,
    chunk_tree_basis: u64,
    logical_content_basis: u64,
    ordered_leaves: Vec<BlobChunkProofLeaf>,
}

impl BlobChunkProofFrontier {
    fn first(proof: BlobChunkIntegrityProof, bytes: &[u8]) -> Self {
        let chunk_tree_basis = accumulate_chunk_proof(accumulator_seed("chunk-tree-root"), &proof);
        let chunk_identity_basis = accumulate_bytes(
            accumulator_seed("chunk-identities"),
            proof.identity().chunk_digest().as_str().as_bytes(),
        );
        let logical_content_basis = accumulate_bytes(accumulator_seed("logical-content"), bytes);
        let ordered_leaf = BlobChunkProofLeaf::from_proof(&proof);
        Self {
            total_bytes: proof.byte_range().end(),
            first_chunk: proof.clone(),
            latest_chunk: proof,
            chunk_count: 1,
            chunk_identity_basis,
            chunk_tree_basis,
            logical_content_basis,
            ordered_leaves: vec![ordered_leaf],
        }
    }

    fn advance(mut self, proof: BlobChunkIntegrityProof, bytes: &[u8]) -> Self {
        let chunk_identity_basis = accumulate_bytes(
            self.chunk_identity_basis,
            proof.identity().chunk_digest().as_str().as_bytes(),
        );
        let chunk_tree_basis = accumulate_chunk_proof(self.chunk_tree_basis, &proof);
        let logical_content_basis = accumulate_bytes(self.logical_content_basis, bytes);
        self.ordered_leaves
            .push(BlobChunkProofLeaf::from_proof(&proof));
        Self {
            first_chunk: self.first_chunk,
            total_bytes: proof.byte_range().end(),
            latest_chunk: proof,
            chunk_count: self.chunk_count + 1,
            chunk_identity_basis,
            chunk_tree_basis,
            logical_content_basis,
            ordered_leaves: self.ordered_leaves,
        }
    }

    pub const fn first_chunk(&self) -> &BlobChunkIntegrityProof {
        &self.first_chunk
    }

    pub fn first_leaf(&self) -> &BlobChunkProofLeaf {
        &self.ordered_leaves[0]
    }

    pub const fn latest_chunk(&self) -> &BlobChunkIntegrityProof {
        &self.latest_chunk
    }

    pub const fn chunk_count(&self) -> u64 {
        self.chunk_count
    }

    pub const fn total_bytes(&self) -> u64 {
        self.total_bytes
    }

    pub fn ordered_leaves(&self) -> &[BlobChunkProofLeaf] {
        &self.ordered_leaves
    }

    pub(crate) fn chunk_identity_summary(&self) -> StableDigest {
        accumulated_digest(
            "chunk-identities",
            self.chunk_identity_basis,
            self.total_bytes,
            self.chunk_count,
        )
    }

    pub(crate) fn chunk_tree_root(&self) -> ChunkTreeRoot {
        ChunkTreeRoot::from_declared_digest(accumulated_digest(
            "chunk-tree-root",
            self.chunk_tree_basis,
            self.total_bytes,
            self.chunk_count,
        ))
    }

    pub(crate) fn logical_content_digest(&self) -> LogicalContentDigest {
        LogicalContentDigest::from_declared_digest(accumulated_logical_digest(
            "logical-content",
            self.logical_content_basis,
            self.total_bytes,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkProofLeaf {
    ordinal: BlobChunkOrdinal,
    byte_range: crate::BlobChunkByteRange,
    identity: BlobChunkIdentity,
    stored_digest: StoredChunkDigest,
    content_digest: BlobChunkContentDigest,
    checksum_digest: StableDigest,
    security_metadata: BlobChunkSecurityMetadataWitness,
}

impl BlobChunkProofLeaf {
    fn from_proof(proof: &BlobChunkIntegrityProof) -> Self {
        Self {
            ordinal: proof.ordinal(),
            byte_range: proof.byte_range(),
            identity: proof.identity().clone(),
            stored_digest: proof.stored_digest().clone(),
            content_digest: proof.content_digest().clone(),
            checksum_digest: proof.checksum().checksum().digest().clone(),
            security_metadata: proof.security_metadata(),
        }
    }

    pub const fn ordinal(&self) -> BlobChunkOrdinal {
        self.ordinal
    }

    pub const fn byte_range(&self) -> crate::BlobChunkByteRange {
        self.byte_range
    }

    pub const fn identity(&self) -> &BlobChunkIdentity {
        &self.identity
    }

    pub const fn stored_digest(&self) -> &StoredChunkDigest {
        &self.stored_digest
    }

    pub const fn content_digest(&self) -> &BlobChunkContentDigest {
        &self.content_digest
    }

    pub const fn checksum_digest(&self) -> &StableDigest {
        &self.checksum_digest
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }
}

fn accumulated_digest(
    lane: &str,
    accumulator: u64,
    total_bytes: u64,
    chunk_count: u64,
) -> StableDigest {
    let evidence = format!("{accumulator:016x}:{total_bytes}:{chunk_count}");
    stable_digest_for(
        lane,
        "s7.sequence.v1",
        BlobChunkOrdinal::first(),
        crate::BlobChunkByteRange::new(chunk_count, evidence.len() as u64)
            .expect("finalized sequence has nonempty evidence"),
        &evidence,
    )
}

fn accumulated_logical_digest(lane: &str, accumulator: u64, total_bytes: u64) -> StableDigest {
    let evidence = format!("{accumulator:016x}:{total_bytes}");
    stable_digest_for(
        lane,
        "s7.sequence.logical.v1",
        BlobChunkOrdinal::first(),
        crate::BlobChunkByteRange::new(1, evidence.len() as u64)
            .expect("finalized logical evidence has nonempty evidence"),
        &evidence,
    )
}

fn accumulate_chunk_proof(hash: u64, proof: &BlobChunkIntegrityProof) -> u64 {
    let hash = accumulate_u64(hash, proof.ordinal().get());
    let hash = accumulate_u64(hash, proof.byte_range().start());
    let hash = accumulate_u64(hash, proof.byte_range().len());
    let hash = accumulate_bytes(hash, proof.identity().chunk_digest().as_str().as_bytes());
    let hash = accumulate_bytes(hash, proof.stored_digest().digest().as_str().as_bytes());
    accumulate_bytes(
        hash,
        proof.checksum().checksum().digest().as_str().as_bytes(),
    )
}

fn accumulator_seed(lane: &str) -> u64 {
    accumulate_bytes(0xcbf2_9ce4_8422_2325, lane.as_bytes())
}

fn accumulate_u64(hash: u64, value: u64) -> u64 {
    accumulate_bytes(hash, &value.to_le_bytes())
}

fn accumulate_bytes(mut hash: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}
