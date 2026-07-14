use crate::{
    durable_phase_for_record_kind, record_kind_admits_recovery_replay, BlobWalRecordEnvelope,
    BlobWalRecordIdentity, DurablePublicationPhase, DurablePublicationScope, ReplayCursor,
    WalFrameOrderingProof, WalSegmentGeneration, WalTopologyScan,
};

use crate::{WalOperationDenial, WalOperationDenialKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedReplayTailCursor {
    cursor: ReplayCursor,
}

pub fn admit_replay_cursor(
    scan: WalTopologyScan,
    expected_generation: WalSegmentGeneration,
) -> Result<AdmittedReplayTailCursor, WalOperationDenial> {
    let cursor = scan
        .admit_replay_cursor(expected_generation)
        .map_err(|_| WalOperationDenial::new(WalOperationDenialKind::ReplayTopologyDenied))?;
    Ok(AdmittedReplayTailCursor { cursor })
}

pub fn inspect_replay_tail_record(
    cursor: &AdmittedReplayTailCursor,
    record: &BlobWalRecordEnvelope,
) -> Result<WalReplayTailRecordReport, WalOperationDenial> {
    if !record_kind_admits_recovery_replay(record.identity().kind()) {
        return Err(WalOperationDenial::new(
            WalOperationDenialKind::NonReplayTailRecord,
        ));
    }
    Ok(WalReplayTailRecordReport::from_record(cursor, record))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalReplayTailCursorReport {
    first_lsn: u64,
    end_lsn: u64,
    segment_count: usize,
    ordered_range_count: usize,
    ordering_proof: WalFrameOrderingProof,
}

impl WalReplayTailCursorReport {
    fn from_cursor(cursor: &AdmittedReplayTailCursor) -> Self {
        Self {
            first_lsn: cursor.first_lsn().get(),
            end_lsn: cursor.end_lsn().get(),
            segment_count: cursor.segments().len(),
            ordered_range_count: cursor.ordering_proof().ordered_range_count(),
            ordering_proof: cursor.ordering_proof().clone(),
        }
    }

    pub const fn first_lsn(&self) -> u64 {
        self.first_lsn
    }

    pub const fn end_lsn(&self) -> u64 {
        self.end_lsn
    }

    pub const fn segment_count(&self) -> usize {
        self.segment_count
    }

    pub const fn ordered_range_count(&self) -> usize {
        self.ordered_range_count
    }

    pub const fn ordering_proof(&self) -> &WalFrameOrderingProof {
        &self.ordering_proof
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalReplayTailRecordReport {
    identity: BlobWalRecordIdentity,
    durable_phase: DurablePublicationPhase,
    segment_id: u64,
    generation: u64,
    lsn_start: u64,
    lsn_end: u64,
    expected_bytes: u64,
}

impl WalReplayTailRecordReport {
    fn from_record(cursor: &AdmittedReplayTailCursor, record: &BlobWalRecordEnvelope) -> Self {
        let DurablePublicationScope::WalFrame(scope) = record.durable_publication().scope() else {
            unreachable!("blob wal records only admit wal-frame publication scopes")
        };
        debug_assert!(cursor.first_lsn().get() <= scope.lsn_start());
        debug_assert!(scope.lsn_end() <= cursor.end_lsn().get());
        Self {
            identity: record.identity(),
            durable_phase: durable_phase_for_record_kind(record.identity().kind()),
            segment_id: scope.segment_id(),
            generation: scope.generation(),
            lsn_start: scope.lsn_start(),
            lsn_end: scope.lsn_end(),
            expected_bytes: scope.expected_bytes(),
        }
    }

    pub const fn identity(&self) -> BlobWalRecordIdentity {
        self.identity
    }

    pub const fn durable_phase(&self) -> DurablePublicationPhase {
        self.durable_phase
    }

    pub const fn segment_id(&self) -> u64 {
        self.segment_id
    }

    pub const fn generation(&self) -> u64 {
        self.generation
    }

    pub const fn lsn_start(&self) -> u64 {
        self.lsn_start
    }

    pub const fn lsn_end(&self) -> u64 {
        self.lsn_end
    }

    pub const fn expected_bytes(&self) -> u64 {
        self.expected_bytes
    }
}

impl AdmittedReplayTailCursor {
    pub fn report(&self) -> WalReplayTailCursorReport {
        WalReplayTailCursorReport::from_cursor(self)
    }

    pub fn segments(&self) -> &[crate::ReplayCursorSegment] {
        self.cursor.segments()
    }

    pub const fn ordering_proof(&self) -> &WalFrameOrderingProof {
        self.cursor.ordering_proof()
    }

    pub fn first_lsn(&self) -> crate::LogSequenceNumber {
        self.cursor.first_lsn()
    }

    pub fn end_lsn(&self) -> crate::LogSequenceNumber {
        self.cursor.end_lsn()
    }
}
