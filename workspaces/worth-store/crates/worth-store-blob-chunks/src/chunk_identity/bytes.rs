use crate::{BlobChunkIntegrityDenial, BlobChunkingRuleAdmission};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlobChunkOrdinal(u64);

impl BlobChunkOrdinal {
    pub const fn first() -> Self {
        Self(0)
    }

    pub const fn next(self) -> Self {
        Self(self.0 + 1)
    }

    pub const fn get(self) -> u64 {
        self.0
    }

    pub(crate) const fn from_raw(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobChunkByteRange {
    start: u64,
    len: u64,
}

impl BlobChunkByteRange {
    pub const fn new(start: u64, len: u64) -> Result<Self, BlobChunkIntegrityDenial> {
        if len == 0 {
            return Err(BlobChunkIntegrityDenial::EmptyByteWindow);
        }
        Ok(Self { start, len })
    }

    pub const fn start(self) -> u64 {
        self.start
    }

    pub const fn len(self) -> u64 {
        self.len
    }

    pub const fn is_empty(self) -> bool {
        self.len == 0
    }

    pub const fn end(self) -> u64 {
        self.start + self.len
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobChunkByteWindow<'a> {
    range: BlobChunkByteRange,
    bytes: &'a [u8],
}

impl<'a> BlobChunkByteWindow<'a> {
    pub fn borrowed(start: u64, bytes: &'a [u8]) -> Result<Self, BlobChunkIntegrityDenial> {
        let range = BlobChunkByteRange::new(start, bytes.len() as u64)?;
        Ok(Self { range, bytes })
    }

    pub(crate) fn validate_against_rule(
        &self,
        rule: &BlobChunkingRuleAdmission,
    ) -> Result<(), BlobChunkIntegrityDenial> {
        if self.range.len() > rule.chunk_size().bytes() {
            return Err(BlobChunkIntegrityDenial::WindowExceedsChunkRule);
        }
        Ok(())
    }

    pub const fn range(&self) -> BlobChunkByteRange {
        self.range
    }

    pub const fn bytes(&self) -> &'a [u8] {
        self.bytes
    }
}
