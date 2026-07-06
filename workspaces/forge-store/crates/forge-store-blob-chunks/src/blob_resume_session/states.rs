use forge_store_physical_isolation::CurrentGenerationPhysicalReference;
use forge_store_wal::{BlobWalRecordEnvelope, BlobWalRecordKind};

use crate::{
    BlobChunkProofLeaf, BlobChunkReachabilityProofSet, BlobChunkSecurityMetadataWitness,
    BlobChunkingRuleAdmission, BlobReachabilityStaging, BlobRootCandidateForPublication,
    BlobStreamingContentFrontier,
};

use super::{
    checkpoint::{checkpoint_from_parts, checkpoint_identity},
    BlobResumeCheckpoint, BlobResumeCheckpointIdentity, BlobResumeCounterSnapshot,
    BlobResumeDenial, BlobResumeSessionId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobResumeCheckpointStateKind {
    SessionDeclared,
    SessionAdmitted,
    ChunkAppendStarted,
    ChunkBytesDurable,
    ChunkIntegrityAdmitted,
    FrontierCheckpointed,
    RootCandidateBuilt,
    RootPublicationReady,
    BlobPublished,
    SessionClosed,
    SessionAbandoned,
    SessionReclaimed,
    SessionClosedWithOrphanChunks,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobResumeSessionDeclaration {
    pub(super) security_metadata: BlobChunkSecurityMetadataWitness,
    pub(super) chunking_rule: BlobChunkingRuleAdmission,
    pub(super) declared_total_bytes: u64,
    pub(super) counters: BlobResumeCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobResumeSessionAdmitted {
    pub(super) session_id: BlobResumeSessionId,
    pub(super) authority_digest: String,
    pub(super) declaration: BlobResumeSessionDeclaration,
    pub(super) counters: BlobResumeCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobResumeChunkAppendStarted {
    pub(super) admitted: BlobResumeSessionAdmitted,
    pub(super) ordinal: crate::BlobChunkOrdinal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobResumeChunkBytesDurable {
    admitted: BlobResumeSessionAdmitted,
    ordinal: crate::BlobChunkOrdinal,
    wal_record: BlobWalRecordEnvelope,
    durable_bytes: u64,
    physical_reference: CurrentGenerationPhysicalReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobResumeChunkIntegrityAdmitted {
    durable: BlobResumeChunkBytesDurable,
    leaf: BlobChunkProofLeaf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobResumeFrontierCheckpointed {
    integrity: BlobResumeChunkIntegrityAdmitted,
    frontier: BlobStreamingContentFrontier,
    checkpoint_record: BlobWalRecordEnvelope,
    checkpoint_identity: BlobResumeCheckpointIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobResumeRootCandidateBuilt {
    checkpointed: BlobResumeFrontierCheckpointed,
    root_candidate: BlobRootCandidateForPublication,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobResumeRootPublicationReady {
    root_candidate: BlobResumeRootCandidateBuilt,
    reachability_staging: BlobReachabilityStaging,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobResumeSessionClosed {
    ready: BlobResumeRootPublicationReady,
    closeout_record: BlobWalRecordEnvelope,
}

impl BlobResumeChunkAppendStarted {
    pub fn export_checkpoint(
        &self,
        record: BlobWalRecordEnvelope,
    ) -> Result<BlobResumeCheckpoint, BlobResumeDenial> {
        if record.identity().kind() != BlobWalRecordKind::SessionCheckpoint {
            return Err(BlobResumeDenial::WrongWalRecordKind);
        }
        Ok(checkpoint_from_parts(
            self.admitted.clone(),
            BlobResumeCheckpointStateKind::ChunkAppendStarted,
            checkpoint_identity(&self.admitted.session_id, &record, "append-started"),
            None,
            None,
            None,
            None,
            None,
        ))
    }

    pub fn record_chunk_bytes_durable(
        self,
        wal_record: BlobWalRecordEnvelope,
        durable_bytes: u64,
        physical_reference: CurrentGenerationPhysicalReference,
    ) -> Result<BlobResumeChunkBytesDurable, BlobResumeDenial> {
        if wal_record.identity().kind() != BlobWalRecordKind::ChunkAppend {
            return Err(BlobResumeDenial::WrongWalRecordKind);
        }
        if durable_bytes == 0 {
            return Err(BlobResumeDenial::MissingDurableBytes);
        }
        let counters = self.admitted.counters.bytes_durable();
        Ok(BlobResumeChunkBytesDurable {
            admitted: self.admitted.with_counters(counters),
            ordinal: self.ordinal,
            wal_record,
            durable_bytes,
            physical_reference,
        })
    }
}

impl BlobResumeChunkBytesDurable {
    pub fn admit_chunk_integrity(
        self,
        leaf: BlobChunkProofLeaf,
    ) -> Result<BlobResumeChunkIntegrityAdmitted, BlobResumeDenial> {
        if leaf.ordinal() != self.ordinal {
            return Err(BlobResumeDenial::ChunkOrdinalMismatch);
        }
        if leaf.security_metadata() != self.admitted.declaration.security_metadata {
            return Err(BlobResumeDenial::ChunkSecurityScopeMismatch);
        }
        let actual_total_bytes = leaf.byte_range().end();
        if actual_total_bytes > self.durable_bytes {
            return Err(BlobResumeDenial::ChunkTailMissing {
                expected_total_bytes: actual_total_bytes,
                actual_total_bytes: self.durable_bytes,
            });
        }
        let counters = self.admitted.counters.integrity_admitted();
        Ok(BlobResumeChunkIntegrityAdmitted {
            durable: self.with_counters(counters),
            leaf,
        })
    }

    pub fn export_checkpoint(&self) -> BlobResumeCheckpoint {
        checkpoint_from_parts(
            self.admitted.clone(),
            BlobResumeCheckpointStateKind::ChunkBytesDurable,
            checkpoint_identity(&self.admitted.session_id, &self.wal_record, "bytes"),
            None,
            Some(self.physical_reference),
            None,
            None,
            None,
        )
    }

    fn with_counters(mut self, counters: BlobResumeCounterSnapshot) -> Self {
        self.admitted.counters = counters;
        self
    }
}

impl BlobResumeChunkIntegrityAdmitted {
    pub fn checkpoint_frontier(
        self,
        frontier: BlobStreamingContentFrontier,
        checkpoint_record: BlobWalRecordEnvelope,
    ) -> Result<BlobResumeFrontierCheckpointed, BlobResumeDenial> {
        if checkpoint_record.identity().kind() != BlobWalRecordKind::SessionCheckpoint {
            return Err(BlobResumeDenial::WrongWalRecordKind);
        }
        let latest = frontier
            .proof_frontier()
            .ordered_leaves()
            .last()
            .ok_or(BlobResumeDenial::FrontierMissingChunk)?;
        if latest.ordinal() != self.leaf.ordinal()
            || latest.stored_digest() != self.leaf.stored_digest()
            || frontier.proof_frontier().total_bytes()
                > self.durable.admitted.declaration.declared_total_bytes
        {
            return Err(BlobResumeDenial::FrontierMissingChunk);
        }
        let checkpoint_identity = checkpoint_identity(
            &self.durable.admitted.session_id,
            &checkpoint_record,
            "frontier",
        );
        let counters = self.durable.admitted.counters.checkpointed();
        Ok(BlobResumeFrontierCheckpointed {
            integrity: self.with_counters(counters),
            frontier,
            checkpoint_record,
            checkpoint_identity,
        })
    }

    pub fn export_checkpoint(&self) -> BlobResumeCheckpoint {
        checkpoint_from_parts(
            self.durable.admitted.clone(),
            BlobResumeCheckpointStateKind::ChunkIntegrityAdmitted,
            checkpoint_identity(
                &self.durable.admitted.session_id,
                &self.durable.wal_record,
                "integrity",
            ),
            Some(self.leaf.clone()),
            Some(self.durable.physical_reference),
            None,
            None,
            None,
        )
    }

    fn with_counters(mut self, counters: BlobResumeCounterSnapshot) -> Self {
        self.durable.admitted.counters = counters;
        self
    }
}

impl BlobResumeFrontierCheckpointed {
    pub fn build_root_candidate(
        self,
        root_candidate: BlobRootCandidateForPublication,
    ) -> Result<BlobResumeRootCandidateBuilt, BlobResumeDenial> {
        let intent = root_candidate.intent();
        if intent.chunk_tree_root() != self.frontier.chunk_tree_root()
            || intent.logical_content_digest() != self.frontier.logical_content_digest()
        {
            return Err(BlobResumeDenial::RootCandidateMismatch);
        }
        let counters = self.counters().root_candidate();
        Ok(BlobResumeRootCandidateBuilt {
            checkpointed: self.with_counters(counters),
            root_candidate,
        })
    }

    pub fn export_checkpoint(&self) -> BlobResumeCheckpoint {
        checkpoint_from_parts(
            self.integrity.durable.admitted.clone(),
            BlobResumeCheckpointStateKind::FrontierCheckpointed,
            self.checkpoint_identity.clone(),
            Some(self.integrity.leaf.clone()),
            Some(self.integrity.durable.physical_reference),
            Some(self.frontier.clone()),
            None,
            None,
        )
    }

    pub fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.integrity
            .durable
            .admitted
            .declaration
            .security_metadata
    }

    pub const fn counters(&self) -> BlobResumeCounterSnapshot {
        self.integrity.durable.admitted.counters
    }

    fn with_counters(mut self, counters: BlobResumeCounterSnapshot) -> Self {
        self.integrity.durable.admitted.counters = counters;
        self
    }
}

impl BlobResumeRootCandidateBuilt {
    pub fn stage_reachability(
        self,
        reachability: BlobChunkReachabilityProofSet,
    ) -> Result<BlobResumeRootPublicationReady, BlobResumeDenial> {
        let staging = BlobReachabilityStaging::stage(self.root_candidate.clone(), reachability)
            .map_err(|_| BlobResumeDenial::RootCandidateMismatch)?;
        let counters = self.counters().root_ready();
        Ok(BlobResumeRootPublicationReady {
            root_candidate: self.with_counters(counters),
            reachability_staging: staging,
        })
    }

    pub fn export_checkpoint(&self) -> BlobResumeCheckpoint {
        checkpoint_from_parts(
            self.checkpointed.integrity.durable.admitted.clone(),
            BlobResumeCheckpointStateKind::RootCandidateBuilt,
            self.checkpointed.checkpoint_identity.clone(),
            Some(self.checkpointed.integrity.leaf.clone()),
            Some(self.checkpointed.integrity.durable.physical_reference),
            Some(self.checkpointed.frontier.clone()),
            Some(self.root_candidate.clone()),
            None,
        )
    }

    pub const fn counters(&self) -> BlobResumeCounterSnapshot {
        self.checkpointed.integrity.durable.admitted.counters
    }

    fn with_counters(mut self, counters: BlobResumeCounterSnapshot) -> Self {
        self.checkpointed.integrity.durable.admitted.counters = counters;
        self
    }
}

impl BlobResumeRootPublicationReady {
    pub fn close_session(
        self,
        closeout_record: BlobWalRecordEnvelope,
    ) -> Result<BlobResumeSessionClosed, BlobResumeDenial> {
        if closeout_record.identity().kind() != BlobWalRecordKind::SessionCloseout {
            return Err(BlobResumeDenial::WrongWalRecordKind);
        }
        let counters = self.counters().closed();
        Ok(BlobResumeSessionClosed {
            ready: self.with_counters(counters),
            closeout_record,
        })
    }

    pub fn export_checkpoint(&self) -> BlobResumeCheckpoint {
        checkpoint_from_parts(
            self.root_candidate
                .checkpointed
                .integrity
                .durable
                .admitted
                .clone(),
            BlobResumeCheckpointStateKind::RootPublicationReady,
            self.root_candidate.checkpointed.checkpoint_identity.clone(),
            Some(self.root_candidate.checkpointed.integrity.leaf.clone()),
            Some(
                self.root_candidate
                    .checkpointed
                    .integrity
                    .durable
                    .physical_reference,
            ),
            Some(self.root_candidate.checkpointed.frontier.clone()),
            Some(self.root_candidate.root_candidate.clone()),
            Some(self.reachability_staging.clone()),
        )
    }

    pub const fn reachability_staging(&self) -> &BlobReachabilityStaging {
        &self.reachability_staging
    }

    pub const fn counters(&self) -> BlobResumeCounterSnapshot {
        self.root_candidate
            .checkpointed
            .integrity
            .durable
            .admitted
            .counters
    }

    fn with_counters(mut self, counters: BlobResumeCounterSnapshot) -> Self {
        self.root_candidate
            .checkpointed
            .integrity
            .durable
            .admitted
            .counters = counters;
        self
    }
}

impl BlobResumeSessionClosed {
    pub fn export_checkpoint(&self) -> BlobResumeCheckpoint {
        self.ready
            .export_checkpoint()
            .with_state(BlobResumeCheckpointStateKind::SessionClosed)
    }

    pub fn closeout_payload_digest(&self) -> &str {
        self.closeout_record.payload_digest()
    }
}
