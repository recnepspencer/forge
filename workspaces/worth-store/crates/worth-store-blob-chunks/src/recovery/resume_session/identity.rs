use crate::{BlobChunkSecurityMetadataWitness, BlobChunkingRuleAdmission};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobResumeSessionId {
    value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobResumeCheckpointIdentity {
    value: String,
}

impl BlobResumeSessionId {
    pub(crate) fn from_declaration(
        authority_digest: &str,
        metadata: BlobChunkSecurityMetadataWitness,
        rule: &BlobChunkingRuleAdmission,
        declared_total_bytes: u64,
    ) -> Self {
        Self {
            value: format!(
                "blob-resume-session:v1:authority={}:security={}:rule={}:bytes={}",
                authority_digest,
                metadata.receipt().receipt_id().security_scope_fingerprint(),
                rule.rule_version(),
                declared_total_bytes
            ),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl BlobResumeCheckpointIdentity {
    pub(crate) fn from_parts(
        session_id: &BlobResumeSessionId,
        wal_payload_digest: &str,
        state: &str,
    ) -> Self {
        Self {
            value: format!(
                "blob-resume-checkpoint:v1:session={}:wal={}:state={}",
                session_id.as_str(),
                wal_payload_digest,
                state
            ),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}
