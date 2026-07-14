#[path = "cursor_assertions.rs"]
mod cursor_assertions;
#[path = "denial_assertions.rs"]
mod denial_assertions;
#[path = "../../../support/recovery/wal_topology/scan_fixtures.rs"]
mod scan_fixtures;

use cursor_assertions::{assert_cursor_topology, assert_ordering_proof, assert_same_cursor_order};
use denial_assertions::{
    assert_denial_has_no_context, assert_gap_denial, assert_generation_denial,
    assert_range_pair_denial, assert_segment_denial,
};
use scan_fixtures::{
    admit_cursor, directory_listing_scan, generation, hostile_mixed_scan, intact_forward_scan,
    map_iteration_scan, named_scan_orders, range, scan_record, stale_scan_record, try_range,
};
use worth_store_wal::{WalSegmentGeneration, WalSegmentId, WalTopologyDenialKind};

#[test]
fn independent_segment_scans_produce_the_same_ordered_replay_cursor() {
    let mut admitted = named_scan_orders().into_iter().map(|(name, records)| {
        (
            name,
            admit_cursor(records)
                .unwrap_or_else(|denial| panic!("{name} scan should admit topology: {denial:?}")),
        )
    });
    let (_, canonical) = admitted.next().expect("canonical scan should exist");

    assert_cursor_topology(&canonical, &[(11, 0, 2), (12, 2, 4), (13, 4, 6)]);
    assert_ordering_proof(&canonical, 3, 3, 3, 2, 0, 6);

    for (name, cursor) in admitted {
        assert_same_cursor_order(&canonical, &cursor, name);
    }
}

#[test]
fn insertion_directory_and_map_iteration_order_cannot_control_cursor_order() {
    let insertion = admit_cursor(intact_forward_scan()).unwrap();
    let directory_listing = admit_cursor(directory_listing_scan()).unwrap();
    let map_iteration = admit_cursor(map_iteration_scan()).unwrap();
    let hostile_permutation = admit_cursor(hostile_mixed_scan()).unwrap();

    assert_same_cursor_order(&insertion, &directory_listing, "directory listing");
    assert_same_cursor_order(&insertion, &map_iteration, "map iteration");
    assert_same_cursor_order(&insertion, &hostile_permutation, "hostile permutation");
    assert_cursor_topology(&map_iteration, &[(11, 0, 2), (12, 2, 4), (13, 4, 6)]);
}

#[test]
fn replay_cursor_keeps_wide_ranges_as_segment_topology_not_expanded_frames() {
    let cursor = admit_cursor(vec![
        scan_record(11, generation(7), 0, 1_000_000),
        scan_record(12, generation(7), 1_000_000, 2_500_000),
        scan_record(13, generation(7), 2_500_000, 4_000_000),
    ])
    .unwrap();

    assert_eq!(cursor.segments().len(), 3);
    assert_cursor_topology(
        &cursor,
        &[
            (11, 0, 1_000_000),
            (12, 1_000_000, 2_500_000),
            (13, 2_500_000, 4_000_000),
        ],
    );
    assert_ordering_proof(&cursor, 3, 3, 3, 2, 0, 4_000_000);
}

#[test]
fn wal_lsn_ranges_define_contiguity_at_half_open_boundaries() {
    let previous = range(0, 2);
    let adjacent = range(2, 4);
    let gapped = range(3, 5);
    let overlapping = range(1, 3);

    assert!(previous.is_contiguous_with(adjacent));
    assert!(!previous.is_contiguous_with(gapped));
    assert!(!previous.is_contiguous_with(overlapping));
}

#[test]
fn wal_topology_denies_gaps_before_replay_cursor_exists() {
    let generation = generation(7);
    let result = admit_cursor(vec![
        scan_record(11, generation, 0, 2),
        scan_record(12, generation, 3, 5),
    ]);

    assert_gap_denial(result, 2, 3);
}

#[test]
fn wal_topology_denies_empty_candidate_sets_before_cursor_construction() {
    assert_denial_has_no_context(
        admit_cursor(Vec::new()),
        WalTopologyDenialKind::EmptyTopology,
    );
}

#[test]
fn wal_topology_denies_duplicate_segments_before_cursor_construction() {
    let generation = generation(7);
    let result = admit_cursor(vec![
        scan_record(11, generation, 0, 2),
        scan_record(11, generation, 2, 4),
    ]);

    assert_segment_denial(result, WalTopologyDenialKind::DuplicateSegment, 11);
}

#[test]
fn wal_topology_denies_duplicate_lsn_ranges_before_cursor_construction() {
    let generation = generation(7);
    let result = admit_cursor(vec![
        scan_record(11, generation, 0, 2),
        scan_record(12, generation, 0, 2),
    ]);

    assert_range_pair_denial(result, WalTopologyDenialKind::DuplicateLsn, (0, 2), (0, 2));
}

#[test]
fn wal_topology_denies_overlapping_lsn_ranges_before_cursor_construction() {
    let generation = generation(7);
    let result = admit_cursor(vec![
        scan_record(11, generation, 0, 3),
        scan_record(12, generation, 2, 4),
    ]);

    assert_range_pair_denial(
        result,
        WalTopologyDenialKind::OverlappingRange,
        (0, 3),
        (2, 4),
    );
}

#[test]
fn wal_topology_denies_stale_and_wrong_generation_segments() {
    let current_generation = generation(7);
    assert_segment_denial(
        admit_cursor(vec![stale_scan_record(11, current_generation, 0, 2)]),
        WalTopologyDenialKind::StaleSegment,
        11,
    );
    assert_generation_denial(
        admit_cursor(vec![scan_record(12, generation(9), 0, 2)]),
        12,
        7,
        9,
    );
}

#[test]
fn wal_topology_denies_invalid_identity_generation_and_range_construction() {
    assert_denial_has_no_context(
        WalSegmentId::new(0).map(|_| ()),
        WalTopologyDenialKind::EmptySegmentId,
    );
    assert_denial_has_no_context(
        WalSegmentGeneration::new(0).map(|_| ()),
        WalTopologyDenialKind::InvalidSegmentGeneration,
    );
    assert_denial_has_no_context(
        try_range(2, 2).map(|_| ()),
        WalTopologyDenialKind::EmptyRange,
    );
    assert_denial_has_no_context(
        try_range(3, 2).map(|_| ()),
        WalTopologyDenialKind::InvertedRange,
    );
}
