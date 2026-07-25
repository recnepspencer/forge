use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordWriteSourceError {
    ProducerRejected,
}

pub trait RecordWriteSource: Send {
    fn declared_length(&self) -> u64;
    fn read_next(&mut self, target: &mut [u8]) -> Result<usize, RecordWriteSourceError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordStreamFailureKind {
    ServingRequiresInspection,
    ProducerRejected,
    SourceEndedEarly,
    SourceExceededDeclaredLength,
    InvalidTransferCount,
    Backend,
    ArtifactDamaged,
    FormatMismatch,
    StalePlacement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordStreamFailure {
    kind: RecordStreamFailureKind,
    completed: Range<u64>,
    media_effect_possible: bool,
}

impl RecordStreamFailure {
    pub(in crate::physical_runtime::record_serving) const fn before_media_write(
        kind: RecordStreamFailureKind,
        completed_bytes: u64,
    ) -> Self {
        Self {
            kind,
            completed: 0..completed_bytes,
            media_effect_possible: false,
        }
    }
    pub(in crate::physical_runtime::record_serving) const fn after_media_write(
        kind: RecordStreamFailureKind,
        completed_bytes: u64,
    ) -> Self {
        Self {
            kind,
            completed: 0..completed_bytes,
            media_effect_possible: true,
        }
    }
    pub(in crate::physical_runtime::record_serving) const fn during_read(
        kind: RecordStreamFailureKind,
        completed_bytes: u64,
    ) -> Self {
        Self::before_media_write(kind, completed_bytes)
    }
    pub(in crate::physical_runtime::record_serving) const fn requires_inspection(&self) -> bool {
        self.media_effect_possible
    }
    pub const fn kind(&self) -> RecordStreamFailureKind {
        self.kind
    }
    pub fn completed_range(&self) -> Range<u64> {
        self.completed.clone()
    }
}

pub(in crate::physical_runtime::record_serving) struct OwnedRecordSource {
    bytes: Vec<u8>,
    offset: usize,
}

impl OwnedRecordSource {
    pub(in crate::physical_runtime::record_serving) const fn new(bytes: Vec<u8>) -> Self {
        Self { bytes, offset: 0 }
    }
}

impl RecordWriteSource for OwnedRecordSource {
    fn declared_length(&self) -> u64 {
        self.bytes.len() as u64
    }
    fn read_next(&mut self, target: &mut [u8]) -> Result<usize, RecordWriteSourceError> {
        let count = target
            .len()
            .min(self.bytes.len().saturating_sub(self.offset));
        target[..count].copy_from_slice(&self.bytes[self.offset..self.offset + count]);
        self.offset += count;
        Ok(count)
    }
}
