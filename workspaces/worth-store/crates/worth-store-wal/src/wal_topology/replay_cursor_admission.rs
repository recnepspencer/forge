use std::collections::BTreeSet;

use super::scan::{WalSegmentScanLifecycle, WalSegmentScanRecord};
use super::{
    ReplayCursorSegment, WalFrameOrderingProof, WalSegmentGeneration, WalTopologyDenial,
    WalTopologyDenialKind,
};

pub(crate) struct AdmittedReplayCursorSegments {
    pub(crate) segments: Vec<ReplayCursorSegment>,
    pub(crate) ordering_proof: WalFrameOrderingProof,
}

pub(crate) fn admit_replay_cursor_segments(
    records: impl IntoIterator<Item = WalSegmentScanRecord>,
    expected_generation: WalSegmentGeneration,
) -> Result<AdmittedReplayCursorSegments, WalTopologyDenial> {
    let mut scan_records: Vec<_> = records.into_iter().collect();
    if scan_records.is_empty() {
        return Err(WalTopologyDenial::new(WalTopologyDenialKind::EmptyTopology));
    }

    require_current_generation(&scan_records, expected_generation)?;
    require_unique_segments(&scan_records)?;
    scan_records.sort_by(canonical_scan_record_order);
    require_contiguous_non_overlapping_ranges(&scan_records)?;

    let segments = cursor_segments(&scan_records);
    let first_lsn = scan_records[0].lsn_range().start();
    let end_lsn = scan_records[scan_records.len() - 1]
        .lsn_range()
        .end_exclusive();
    let ordering_proof = WalFrameOrderingProof::new(
        expected_generation,
        scan_records.len(),
        scan_records.len(),
        segments.len(),
        scan_records.len().saturating_sub(1),
        first_lsn,
        end_lsn,
    );

    Ok(AdmittedReplayCursorSegments {
        segments,
        ordering_proof,
    })
}

fn require_current_generation(
    scan_records: &[WalSegmentScanRecord],
    expected_generation: WalSegmentGeneration,
) -> Result<(), WalTopologyDenial> {
    for record in scan_records {
        if record.lifecycle_posture() == WalSegmentScanLifecycle::Stale {
            return Err(WalTopologyDenial::for_segment(
                WalTopologyDenialKind::StaleSegment,
                record.segment_id(),
            ));
        }
        if record.generation() != expected_generation {
            return Err(WalTopologyDenial::generation_mismatch(
                record.segment_id(),
                expected_generation,
                record.generation(),
            ));
        }
    }
    Ok(())
}

fn require_unique_segments(scan_records: &[WalSegmentScanRecord]) -> Result<(), WalTopologyDenial> {
    let mut seen = BTreeSet::new();
    for record in scan_records {
        if !seen.insert(record.segment_id()) {
            return Err(WalTopologyDenial::for_segment(
                WalTopologyDenialKind::DuplicateSegment,
                record.segment_id(),
            ));
        }
    }
    Ok(())
}

fn require_contiguous_non_overlapping_ranges(
    scan_records: &[WalSegmentScanRecord],
) -> Result<(), WalTopologyDenial> {
    for pair in scan_records.windows(2) {
        let previous = pair[0].lsn_range();
        let observed = pair[1].lsn_range();
        if previous.start() == observed.start() {
            return Err(WalTopologyDenial::duplicate_lsn(previous, observed));
        }
        if previous.overlaps(observed) {
            return Err(WalTopologyDenial::overlapping_range(previous, observed));
        }
        if !previous.is_contiguous_with(observed) {
            return Err(WalTopologyDenial::gap(
                previous.end_exclusive(),
                observed.start(),
            ));
        }
    }
    Ok(())
}

fn canonical_scan_record_order(
    left: &WalSegmentScanRecord,
    right: &WalSegmentScanRecord,
) -> std::cmp::Ordering {
    left.lsn_range()
        .cmp(&right.lsn_range())
        .then_with(|| left.segment_id().cmp(&right.segment_id()))
        .then_with(|| left.generation().cmp(&right.generation()))
}

fn cursor_segments(scan_records: &[WalSegmentScanRecord]) -> Vec<ReplayCursorSegment> {
    let mut segments = Vec::with_capacity(scan_records.len());
    for record in scan_records {
        segments.push(ReplayCursorSegment::new(
            record.segment_id(),
            record.generation(),
            record.lsn_range(),
        ));
    }
    segments
}
