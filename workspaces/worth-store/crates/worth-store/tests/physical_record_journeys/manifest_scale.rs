#[path = "manifest_scale/evidence.rs"]
mod evidence;
#[path = "manifest_scale/world.rs"]
mod world;

#[test]
fn bounded_scale_identity_format_and_policy_courtroom() {
    let observations = [1_u16, 9, 65].map(world::observe_scale_world);
    assert_eq!(observations.map(|value| value.routing_level), [0, 3, 6]);
    assert_eq!(
        observations.map(|value| value.point_blocks),
        [2, 5, 8],
        "C5_PREDICATE:locate-open-scale"
    );
    assert!(observations.iter().all(|value| {
        value.point_allocations == 16_384
            && value.scan_allocations >= value.point_allocations
            && value.scan_allocations < 65_536
    }));
    assert!(observations
        .windows(2)
        .all(|pair| pair[0].open_reads == pair[1].open_reads
            && pair[0].open_bytes == pair[1].open_bytes));
    assert!(observations
        .windows(2)
        .all(|pair| pair[1].whole_blocks > pair[0].whole_blocks
            && pair[1].point_blocks - pair[0].point_blocks
                < pair[1].whole_blocks - pair[0].whole_blocks));
    assert!(observations.iter().all(|value| {
        value.point_comparisons >= value.point_blocks
            && value.point_comparisons <= value.point_blocks.saturating_mul(2)
            && value.scan_records == u64::from(value.record_count)
            && value.scan_payload_bytes == u64::from(value.record_count) * 100
    }));
    assert!(observations.iter().all(|value| {
        value.point_work >= value.point_blocks + value.point_pages
            && value.point_work
                <= value
                    .point_blocks
                    .saturating_add(value.point_pages)
                    .saturating_mul(2)
            && value.point_faults <= value.point_work
            && value.scan_work >= value.scan_frames
            && value.scan_work <= value.scan_frames.saturating_mul(2)
            && value.scan_faults <= value.scan_work
            && value.signal_clock_advance == 0
            && value.signal_invalidation_delta == 0
    }));

    assert!(observations.iter().all(|value| value.invalid_worlds == 5));
    super::scale_policy_evolution::prove();
}

#[derive(Clone, Copy)]
struct ScaleObservation {
    pub(super) record_count: u16,
    pub(super) routing_level: u16,
    pub(super) whole_blocks: u64,
    pub(super) point_blocks: u64,
    pub(super) point_pages: u64,
    pub(super) point_comparisons: u64,
    pub(super) point_work: u64,
    pub(super) point_faults: u64,
    pub(super) open_reads: u64,
    pub(super) open_bytes: u64,
    pub(super) scan_records: u64,
    pub(super) scan_payload_bytes: u64,
    pub(super) scan_blocks: u64,
    pub(super) scan_frames: u64,
    pub(super) scan_work: u64,
    pub(super) scan_faults: u64,
    pub(super) signal_clock_advance: u64,
    pub(super) signal_invalidation_delta: u64,
    pub(super) point_allocations: usize,
    pub(super) scan_allocations: usize,
    pub(super) invalid_worlds: u8,
}
