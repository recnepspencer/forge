use super::row_lookup::rejection_row;
use crate::harness::live_certification::LiveCertificationMatrix;

pub(super) fn assert_rejection_expectation(
    matrix: &LiveCertificationMatrix,
    expectation: &super::super::row_catalog::RejectionRowSpec,
) {
    let row = rejection_row(matrix, expectation.row_name);
    assert_eq!(row.hostile_lane.failure_class, expectation.failure_class);
    assert_eq!(row.control_lane.family, expectation.control_family);
    assert!(
        row.hostile_lane
            .failure_digest
            .contains(expectation.failure_digest_fragment),
        "rejection row {} should carry failure digest fragment {}",
        expectation.row_name,
        expectation.failure_digest_fragment
    );

    match expectation.row_name {
        "collection-cross-partition-denied" | "forbidden-locality-widening" => {
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .locality_widening_denial_count(),
                1
            );
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .locality_widening_budget_cross_count(),
                1
            );
        }
        "unsupported-locality-family" => {
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .locality_unsupported_family_rejection_count(),
                1
            );
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .live_invalid_promotion_rejection_count(),
                0
            );
        }
        "unsupported-locality-predicate" => {
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .locality_unsupported_predicate_rejection_count(),
                1
            );
        }
        "unsupported-stream-consumer-contract" => {
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .stream_contract_denial_count(),
                1
            );
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .stream_member_width_budget_cross_count(),
                0
            );
        }
        "raw-partition-event-leakage-forbidden" => {
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .locality_widening_denial_count(),
                1
            );
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .locality_widening_budget_cross_count(),
                1
            );
        }
        "raw-stream-member-forbidden" | "raw-stream-member-leakage-forbidden" => {
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .stream_contract_denial_count(),
                1
            );
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .stream_member_width_budget_cross_count(),
                0
            );
        }
        "forbidden-broad-success-lane" => {
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .locality_breadth_budget_cross_count(),
                1
            );
        }
        "forbidden-stream-width-overflow-success" => {
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .stream_contract_denial_count(),
                1
            );
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .stream_member_width_budget_cross_count(),
                1
            );
        }
        "forbidden-stream-window-overflow-success" => {
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .stream_contract_denial_count(),
                1
            );
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .stream_window_width_budget_cross_count(),
                1
            );
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .stream_member_width_budget_cross_count(),
                0
            );
        }
        "bridge-slice-incompatibility-denied" => {
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .locality_bridge_slice_incompatibility_count(),
                1
            );
        }
        _ => panic!(
            "missing rejection row expectation handler for {}",
            expectation.row_name
        ),
    }
}
