use forge_store_wal::{BlobWalRecordEnvelope, BlobWalRecordKind};

use super::super::checkpoint::{checkpoint_from_parts, checkpoint_identity};
use super::super::states::{
    BlobResumeCheckpointStateKind, BlobResumeChunkAppendStarted, BlobResumeChunkIntegrityAdmitted,
    BlobResumeFrontierCheckpointed, BlobResumeRootCandidateBuilt, BlobResumeRootPublicationReady,
    BlobResumeSessionClosed,
};
use super::super::{BlobResumeCheckpoint, BlobResumeDenial};

pub(super) fn export_append_started_checkpoint(
    started: &BlobResumeChunkAppendStarted,
    record: BlobWalRecordEnvelope,
) -> Result<BlobResumeCheckpoint, BlobResumeDenial> {
    if record.identity().kind() != BlobWalRecordKind::SessionCheckpoint {
        return Err(BlobResumeDenial::WrongWalRecordKind);
    }
    Ok(checkpoint_from_parts(
        started.admitted.clone(),
        BlobResumeCheckpointStateKind::ChunkAppendStarted,
        checkpoint_identity(&started.admitted.session_id, &record, "append-started"),
        None,
        None,
        None,
        None,
        None,
    ))
}

impl BlobResumeChunkIntegrityAdmitted {
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
}

impl BlobResumeFrontierCheckpointed {
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
}

impl BlobResumeRootCandidateBuilt {
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
}

impl BlobResumeRootPublicationReady {
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