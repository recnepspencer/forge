use worth_ui_query_binding::{
    UiCollectionProjectionWorkCounters, WorthUiCollectionQueryWorkInspection,
};

use crate::collection_projection::support::{
    measure_changed_row_work, CollectionChangeCostEvidence,
};

#[test]
fn one_and_sixteen_changes_have_exact_work_independent_of_projection_width() {
    let one = measure_changed_row_work(1, 1);
    let medium_one = measure_changed_row_work(1_024, 1);
    let medium_sixteen = measure_changed_row_work(1_024, 16);

    assert_change_cost(one, 1, 1, 0);
    assert_change_cost(medium_one, 1_024, 1, 1);
    assert_change_cost(medium_sixteen, 1_024, 16, 1);
}

fn assert_change_cost(
    evidence: CollectionChangeCostEvidence,
    cardinality: usize,
    changed_rows: usize,
    continuation_operations: usize,
) {
    assert_eq!(evidence.cardinality(), cardinality);
    assert_eq!(evidence.changed_rows(), changed_rows);
    assert_ui_work(evidence.ui(), changed_rows, continuation_operations);
    assert_query_work(evidence.query(), changed_rows);
}

fn assert_ui_work(
    work: UiCollectionProjectionWorkCounters,
    changed_rows: usize,
    continuation_operations: usize,
) {
    assert_eq!(work.rows_visited(), changed_rows);
    assert_eq!(work.selected_key_accesses(), changed_rows);
    assert_eq!(work.indexed_row_lookups(), changed_rows);
    assert_eq!(work.native_values_materialized(), changed_rows);
    assert_eq!(work.continuation_operations(), continuation_operations);
    assert_eq!(work.unrelated_width_scans(), 0);
}

fn assert_query_work(work: WorthUiCollectionQueryWorkInspection, changed_rows: usize) {
    assert_eq!(work.native_facts_materialized(), changed_rows);
    assert_eq!(work.entity_point_lookups(), changed_rows);
    assert_eq!(work.full_collection_scans(), 0);
    assert_eq!(work.unrelated_consumer_scans(), 0);
}
