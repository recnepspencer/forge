use worth_store_wal::{BlobWalRecordEnvelope, BlobWalRecordKind};

use crate::{BlobChunkSecurityMetadataWitness, BlobChunkingRuleAdmission};

use super::{
    checkpoint::{checkpoint_from_parts, checkpoint_identity, BlobResumePublicationProgress},
    BlobResumeCheckpoint, BlobResumeCheckpointStateKind, BlobResumeChunkAppendStarted,
    BlobResumeCounterSnapshot, BlobResumeDenial, BlobResumeSessionAdmitted,
    BlobResumeSessionDeclaration, BlobResumeSessionId, BlobResumeStoreAuthority,
};

impl BlobResumeSessionDeclaration {
    pub fn new(
        security_metadata: BlobChunkSecurityMetadataWitness,
        chunking_rule: BlobChunkingRuleAdmission,
        declared_total_bytes: u64,
    ) -> Result<Self, BlobResumeDenial> {
        if declared_total_bytes == 0 {
            return Err(BlobResumeDenial::EmptyDeclaredBlob);
        }
        Ok(Self {
            security_metadata,
            chunking_rule,
            declared_total_bytes,
            counters: BlobResumeCounterSnapshot::start().declared(),
        })
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub const fn chunking_rule(&self) -> &BlobChunkingRuleAdmission {
        &self.chunking_rule
    }

    pub const fn declared_total_bytes(&self) -> u64 {
        self.declared_total_bytes
    }

    pub fn export_checkpoint(
        &self,
        authority: BlobResumeStoreAuthority,
        record: BlobWalRecordEnvelope,
    ) -> Result<BlobResumeCheckpoint, BlobResumeDenial> {
        let session_id = BlobResumeSessionId::from_declaration(
            authority.authority_digest(),
            self.security_metadata,
            &self.chunking_rule,
            self.declared_total_bytes,
        );
        let admitted = BlobResumeSessionAdmitted::admit(self.clone(), authority);
        if record.identity().kind() != BlobWalRecordKind::SessionCheckpoint {
            return Err(BlobResumeDenial::WrongWalRecordKind);
        }
        Ok(checkpoint_from_parts(
            admitted,
            BlobResumeCheckpointStateKind::SessionDeclared,
            checkpoint_identity(&session_id, &record, "declared"),
            None,
            None,
            None,
            BlobResumePublicationProgress::none(),
        ))
    }
}

impl BlobResumeSessionAdmitted {
    pub fn admit(
        declaration: BlobResumeSessionDeclaration,
        authority: BlobResumeStoreAuthority,
    ) -> Self {
        let session_id = BlobResumeSessionId::from_declaration(
            authority.authority_digest(),
            declaration.security_metadata,
            &declaration.chunking_rule,
            declaration.declared_total_bytes,
        );
        let counters = declaration.counters.admitted();
        Self {
            session_id,
            authority_digest: authority.authority_digest().to_owned(),
            declaration,
            counters,
        }
    }

    pub fn start_chunk_append(
        self,
        ordinal: crate::BlobChunkOrdinal,
    ) -> BlobResumeChunkAppendStarted {
        let counters = self.counters.append_started();
        BlobResumeChunkAppendStarted {
            admitted: self.with_counters(counters),
            ordinal,
        }
    }

    pub fn export_session_id(&self) -> &BlobResumeSessionId {
        &self.session_id
    }

    pub fn export_checkpoint(
        &self,
        record: BlobWalRecordEnvelope,
    ) -> Result<BlobResumeCheckpoint, BlobResumeDenial> {
        if record.identity().kind() != BlobWalRecordKind::SessionCheckpoint {
            return Err(BlobResumeDenial::WrongWalRecordKind);
        }
        Ok(checkpoint_from_parts(
            self.clone(),
            BlobResumeCheckpointStateKind::SessionAdmitted,
            checkpoint_identity(&self.session_id, &record, "admitted"),
            None,
            None,
            None,
            BlobResumePublicationProgress::none(),
        ))
    }

    pub fn authority_digest(&self) -> &str {
        &self.authority_digest
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.declaration.security_metadata
    }

    pub const fn chunking_rule(&self) -> &BlobChunkingRuleAdmission {
        &self.declaration.chunking_rule
    }

    pub const fn declared_total_bytes(&self) -> u64 {
        self.declaration.declared_total_bytes
    }

    pub const fn counters(&self) -> BlobResumeCounterSnapshot {
        self.counters
    }

    pub(super) fn with_counters(mut self, counters: BlobResumeCounterSnapshot) -> Self {
        self.counters = counters;
        self
    }
}
