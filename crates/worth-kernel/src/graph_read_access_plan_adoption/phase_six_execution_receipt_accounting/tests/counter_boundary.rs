use crate::graph_read_access_plan_adoption::{
    WorthGraphReadAccessCounterAccountingStatus, WorthGraphReadAccessSourceCounterProofKind,
};

use super::production_phase_six_closeout;

#[test]
fn receipt_counters_match_touched_breadth_for_bounded_slice() {
    let closeout = production_phase_six_closeout();
    let first_counter_row = closeout
        .counter_accounting_report()
        .rows()
        .iter()
        .find(|row| {
            row.status() == WorthGraphReadAccessCounterAccountingStatus::QueryCountersAccounted
        })
        .expect("executed vertical slice counter row should exist");

    assert_eq!(
        first_counter_row.status(),
        WorthGraphReadAccessCounterAccountingStatus::QueryCountersAccounted
    );
    assert!(first_counter_row.planned_access_step_count() > 0);
    assert_eq!(
        first_counter_row.consumed_access_step_count(),
        first_counter_row.planned_access_step_count() * first_counter_row.executor_entry_count()
    );
    assert!(first_counter_row.candidate_root_count() > 0);
    assert_eq!(
        first_counter_row.source_counter_proof().kind(),
        WorthGraphReadAccessSourceCounterProofKind::PhaseFourQueryReceipt
    );
    assert!(first_counter_row
        .source_counter_proof()
        .source_counter_digest()
        .is_some());
    assert_eq!(
        first_counter_row.candidate_root_count(),
        first_counter_row
            .source_counter_proof()
            .candidate_root_count()
    );
    assert_eq!(
        first_counter_row.touched_node_count(),
        first_counter_row
            .source_counter_proof()
            .touched_node_count()
    );
    assert_eq!(
        first_counter_row.touched_edge_count(),
        first_counter_row
            .source_counter_proof()
            .touched_edge_count()
    );
    assert_eq!(
        first_counter_row.frontier_width(),
        first_counter_row.source_counter_proof().frontier_width()
    );
    assert_eq!(
        first_counter_row.resident_byte_count(),
        first_counter_row
            .source_counter_proof()
            .resident_byte_count()
    );
    assert_eq!(
        first_counter_row.touched_node_count(),
        first_counter_row.candidate_root_count() + first_counter_row.frontier_width()
    );
    assert!(first_counter_row.touched_edge_count() >= first_counter_row.frontier_width());
    assert_eq!(
        first_counter_row.visited_breadth(),
        first_counter_row.dedup_breadth()
    );
    assert!(first_counter_row.resident_byte_count() > 0);
    assert_eq!(0, first_counter_row.local_work_count());
    assert_eq!(0, first_counter_row.fallback_count());
    assert_eq!(0, first_counter_row.caller_owned_work().total_count());
    assert_eq!(
        0,
        first_counter_row
            .source_counter_proof()
            .streaming_page_count()
    );
    assert_eq!(
        0,
        first_counter_row
            .source_counter_proof()
            .streaming_emitted_row_count()
    );
}

#[test]
fn explicit_counter_gaps_are_not_reported_as_zero_query_counters() {
    let closeout = production_phase_six_closeout();

    assert!(closeout
        .counter_accounting_report()
        .rows()
        .iter()
        .all(|row| row.status().is_accounted_or_explicit_gap()));
    assert_eq!(
        closeout
            .counter_accounting_report()
            .rows()
            .iter()
            .filter(|row| {
                row.status()
                    == WorthGraphReadAccessCounterAccountingStatus::CounterGapRequiresQueryReceiptSurface
            })
            .count(),
        closeout
            .counter_accounting_report()
            .explicit_counter_gap_count()
    );
}
