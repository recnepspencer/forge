use std::ops::Range;

use super::super::super::{
    CompletedRecordScan, ExternalRecordScanCursor, PhysicalRecordId, RecordScanCounterSnapshot,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScannedPhysicalRecord {
    record: PhysicalRecordId,
    payload: Option<Range<usize>>,
    declared_payload_bytes: u64,
}

impl ScannedPhysicalRecord {
    pub(super) const fn new(
        record: PhysicalRecordId,
        payload: Option<Range<usize>>,
        declared_payload_bytes: u64,
    ) -> Self {
        Self {
            record,
            payload,
            declared_payload_bytes,
        }
    }

    pub const fn record_id(&self) -> PhysicalRecordId {
        self.record
    }

    pub const fn declared_payload_bytes(&self) -> u64 {
        self.declared_payload_bytes
    }

    pub const fn payload_is_deferred(&self) -> bool {
        self.payload.is_none()
    }
}

pub struct RecordScanBatch<'scratch> {
    bytes: &'scratch [u8],
    records: Vec<ScannedPhysicalRecord>,
    end_cursor: ExternalRecordScanCursor,
    completed: bool,
    observation: RecordScanCounterSnapshot,
}

impl<'scratch> RecordScanBatch<'scratch> {
    pub(super) fn new(
        bytes: &'scratch [u8],
        records: Vec<ScannedPhysicalRecord>,
        end_cursor: ExternalRecordScanCursor,
        completed: bool,
        observation: RecordScanCounterSnapshot,
    ) -> Self {
        Self {
            bytes,
            records,
            end_cursor,
            completed,
            observation,
        }
    }

    pub fn records(&self) -> &[ScannedPhysicalRecord] {
        &self.records
    }

    pub fn payload(&self, index: usize) -> Option<&'scratch [u8]> {
        self.records
            .get(index)
            .and_then(|record| record.payload.clone())
            .map(|range| &self.bytes[range])
    }

    pub const fn end_cursor(&self) -> ExternalRecordScanCursor {
        self.end_cursor
    }

    pub const fn is_complete(&self) -> bool {
        self.completed
    }

    pub const fn observation(&self) -> RecordScanCounterSnapshot {
        self.observation
    }
}

pub enum RecordScanOutcome<'scratch> {
    Batch(RecordScanBatch<'scratch>),
    Completed(CompletedRecordScan),
}
