use std::collections::HashMap;

use worth_store_recovery_physics::{
    LogSequenceNumber, ReplayCursor, WalLsnRange, WalSegmentGeneration, WalSegmentId,
    WalSegmentScanRecord, WalTopologyDenial, WalTopologyScan,
};

pub fn named_scan_orders() -> Vec<(&'static str, Vec<WalSegmentScanRecord>)> {
    vec![
        ("forward", intact_forward_scan()),
        ("reverse", intact_reverse_scan()),
        ("directory_listing", directory_listing_scan()),
        ("map_iteration", map_iteration_scan()),
        ("hostile_mixed", hostile_mixed_scan()),
    ]
}

pub fn generation(value: u64) -> WalSegmentGeneration {
    WalSegmentGeneration::new(value).expect("test generation must be valid")
}

pub fn lsn(value: u64) -> LogSequenceNumber {
    LogSequenceNumber::new(value)
}

pub fn range(start: u64, end_exclusive: u64) -> WalLsnRange {
    try_range(start, end_exclusive).expect("test range must be valid")
}

pub fn try_range(start: u64, end_exclusive: u64) -> Result<WalLsnRange, WalTopologyDenial> {
    WalLsnRange::new(lsn(start), lsn(end_exclusive))
}

pub fn segment(value: u64) -> WalSegmentId {
    WalSegmentId::new(value).expect("test segment id must be valid")
}

pub fn scan_record(
    segment_id: u64,
    generation: WalSegmentGeneration,
    start: u64,
    end_exclusive: u64,
) -> WalSegmentScanRecord {
    WalSegmentScanRecord::current(segment(segment_id), generation, range(start, end_exclusive))
}

pub fn stale_scan_record(
    segment_id: u64,
    generation: WalSegmentGeneration,
    start: u64,
    end_exclusive: u64,
) -> WalSegmentScanRecord {
    WalSegmentScanRecord::stale(segment(segment_id), generation, range(start, end_exclusive))
}

pub fn intact_forward_scan() -> Vec<WalSegmentScanRecord> {
    let generation = generation(7);
    vec![
        scan_record(11, generation, 0, 2),
        scan_record(12, generation, 2, 4),
        scan_record(13, generation, 4, 6),
    ]
}

fn intact_reverse_scan() -> Vec<WalSegmentScanRecord> {
    let mut records = intact_forward_scan();
    records.reverse();
    records
}

pub fn directory_listing_scan() -> Vec<WalSegmentScanRecord> {
    let generation = generation(7);
    vec![
        scan_record(13, generation, 4, 6),
        scan_record(11, generation, 0, 2),
        scan_record(12, generation, 2, 4),
    ]
}

pub fn hostile_mixed_scan() -> Vec<WalSegmentScanRecord> {
    let generation = generation(7);
    vec![
        scan_record(12, generation, 2, 4),
        scan_record(13, generation, 4, 6),
        scan_record(11, generation, 0, 2),
    ]
}

pub fn map_iteration_scan() -> Vec<WalSegmentScanRecord> {
    let mut discovered = HashMap::new();
    for record in hostile_mixed_scan() {
        discovered.insert(100 - record.segment_id().get(), record);
    }
    discovered.into_values().collect()
}

pub fn admit_cursor(records: Vec<WalSegmentScanRecord>) -> Result<ReplayCursor, WalTopologyDenial> {
    WalTopologyScan::from_segment_scan(records).admit_replay_cursor(generation(7))
}
