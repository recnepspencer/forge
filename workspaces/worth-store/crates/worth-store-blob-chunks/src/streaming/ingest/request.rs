use crate::{
    BlobChunkSecurityMetadataWitness, BlobChunkSecurityScope, BlobChunkingRuleAdmission,
    BlobStreamingIngestDenial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobStreamingWindow {
    max_resident_bytes: u64,
}

impl BlobStreamingWindow {
    pub const fn bounded(max_resident_bytes: u64) -> Result<Self, BlobStreamingIngestDenial> {
        if max_resident_bytes == 0 {
            return Err(BlobStreamingIngestDenial::EmptyStreamingWindow);
        }
        Ok(Self { max_resident_bytes })
    }

    pub const fn max_resident_bytes(self) -> u64 {
        self.max_resident_bytes
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlobStreamingIngestRequest {
    security_scope: BlobChunkSecurityScope,
    rule: BlobChunkingRuleAdmission,
    declared_total_bytes: u64,
}

impl BlobStreamingIngestRequest {
    pub const fn new(
        security_scope: BlobChunkSecurityScope,
        rule: BlobChunkingRuleAdmission,
        declared_total_bytes: u64,
    ) -> Result<Self, BlobStreamingIngestDenial> {
        if declared_total_bytes == 0 {
            return Err(BlobStreamingIngestDenial::EmptyDeclaredObject);
        }
        Ok(Self {
            security_scope,
            rule,
            declared_total_bytes,
        })
    }

    pub const fn rule(&self) -> &BlobChunkingRuleAdmission {
        &self.rule
    }

    pub(crate) const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_scope.metadata()
    }

    pub const fn declared_total_bytes(&self) -> u64 {
        self.declared_total_bytes
    }

    pub(crate) fn into_parts(self) -> (BlobChunkSecurityScope, BlobChunkingRuleAdmission, u64) {
        (self.security_scope, self.rule, self.declared_total_bytes)
    }
}
