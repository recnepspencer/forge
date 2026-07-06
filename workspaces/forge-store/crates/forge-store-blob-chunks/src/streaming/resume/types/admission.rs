use crate::{
    BlobChunkSecurityMetadataWitness, BlobChunkingRuleAdmission, BlobResumeSessionAdmitted,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobStreamingResumeAdmission {
    session_digest: String,
    authority_digest: String,
    security_metadata: BlobChunkSecurityMetadataWitness,
    chunking_rule: BlobChunkingRuleAdmission,
    declared_total_bytes: u64,
}

impl BlobStreamingResumeAdmission {
    pub fn from_admitted_resume_session(session: &BlobResumeSessionAdmitted) -> Self {
        Self {
            session_digest: session.export_session_id().as_str().to_owned(),
            authority_digest: session.authority_digest().to_owned(),
            security_metadata: session.security_metadata(),
            chunking_rule: session.chunking_rule().clone(),
            declared_total_bytes: session.declared_total_bytes(),
        }
    }

    pub fn session_digest(&self) -> &str {
        &self.session_digest
    }

    pub fn authority_digest(&self) -> &str {
        &self.authority_digest
    }

    pub(crate) fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub(crate) fn chunking_rule(&self) -> &BlobChunkingRuleAdmission {
        &self.chunking_rule
    }

    pub(crate) fn declared_total_bytes(&self) -> u64 {
        self.declared_total_bytes
    }
}