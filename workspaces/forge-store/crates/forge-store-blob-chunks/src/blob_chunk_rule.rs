use crate::BlobChunkIntegrityDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobChunkSize {
    bytes: u64,
}

impl BlobChunkSize {
    pub const fn from_bytes(bytes: u64) -> Result<Self, BlobChunkIntegrityDenial> {
        if bytes == 0 {
            return Err(BlobChunkIntegrityDenial::EmptyChunkingRule);
        }
        Ok(Self { bytes })
    }

    pub const fn bytes(self) -> u64 {
        self.bytes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkingRuleAdmission {
    chunk_size: BlobChunkSize,
    rule_version: &'static str,
}

impl BlobChunkingRuleAdmission {
    pub const fn fixed_size(chunk_size: BlobChunkSize) -> Result<Self, BlobChunkIntegrityDenial> {
        Ok(Self {
            chunk_size,
            rule_version: "s7.fixed-size.raw-chunk.v1",
        })
    }

    pub const fn chunk_size(&self) -> BlobChunkSize {
        self.chunk_size
    }

    pub const fn rule_version(&self) -> &'static str {
        self.rule_version
    }
}
