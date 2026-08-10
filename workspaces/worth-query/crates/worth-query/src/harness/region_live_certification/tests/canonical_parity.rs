use super::super::row_catalog::REJECTION_ROW_SPECS;
use super::super::MilestoneFivePointOneLiveCertificationAdapter;
use super::row_lookup::{canonical_row, rejection_row};
use crate::harness::live_certification::{LiveCertificationMatrix, LiveFailureClass};

#[test]
fn region_live_canonical_rows_preserve_counter_parity() {
    let matrix = MilestoneFivePointOneLiveCertificationAdapter::
        region_scoped_live_narrowing_and_stream_contract_test();

    for row in &matrix.rows {
        if row.row_name == "broad-vs-region-narrowing-control" {
            continue;
        }
        assert_eq!(
            row.control_lane.counter_snapshot, row.hostile_lane.counter_snapshot,
            "canonical row {} drifted between control and hostile counters",
            row.row_name
        );
        assert_eq!(
            row.control_lane.counter_snapshot, row.parity_lane.counter_snapshot,
            "canonical row {} drifted between control and parity counters",
            row.row_name
        );
    }

    let broad_vs_region = canonical_row(&matrix, "broad-vs-region-narrowing-control");
    assert_eq!(
        broad_vs_region
            .control_lane
            .counter_snapshot
            .locality_region_match_count(),
        0
    );
    assert_eq!(
        broad_vs_region
            .control_lane
            .counter_snapshot
            .locality_work_avoided_by_region_narrowing_count(),
        0
    );
    assert_eq!(
        broad_vs_region
            .hostile_lane
            .counter_snapshot
            .locality_region_match_count(),
        1
    );
    assert_eq!(
        broad_vs_region
            .hostile_lane
            .counter_snapshot
            .locality_work_avoided_by_region_narrowing_count(),
        1
    );
    assert_eq!(
        broad_vs_region.hostile_lane.counter_snapshot,
        broad_vs_region.parity_lane.counter_snapshot,
        "broad-vs-region row should preserve exact locality counters between hostile and parity lanes"
    );
}

#[test]
fn region_live_rejection_rows_preserve_control_and_taxonomy() {
    let matrix = MilestoneFivePointOneLiveCertificationAdapter::
        region_scoped_live_narrowing_and_stream_contract_test();

    for row in &matrix.rejection_rows {
        assert_eq!(
            row.control_lane.counter_snapshot, row.parity_lane.counter_snapshot,
            "rejection row {} drifted between control and parity counters",
            row.row_name
        );
    }

    for spec in REJECTION_ROW_SPECS {
        assert_rejection_failure_class(&matrix, spec.row_name, spec.failure_class);
    }
}

fn assert_rejection_failure_class(
    matrix: &LiveCertificationMatrix,
    row_name: &str,
    expected_failure_class: LiveFailureClass,
) {
    let row = rejection_row(matrix, row_name);
    assert_eq!(
        row.hostile_lane.failure_class, expected_failure_class,
        "rejection row {row_name} drifted to the wrong failure class"
    );
    assert!(
        !row.hostile_lane.failure_digest.is_empty(),
        "rejection row {row_name} should carry a typed failure digest"
    );
}
