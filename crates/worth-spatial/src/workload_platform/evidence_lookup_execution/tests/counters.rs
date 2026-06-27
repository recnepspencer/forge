use super::super::execute::execute_evidence_lookup;
use super::super::outcome::EvidenceLookupExecutionOutcome;
use super::fixtures::ExecutionSubject;

#[test]
fn lookup_execution_counters_follow_selected_plan_and_index() {
    let subject = ExecutionSubject::sparse_event_ledger();
    let selected_plan = subject.selected_plan();
    let index_product = subject.index_product(&selected_plan);

    let receipt =
        execute_evidence_lookup(&subject.execution_request(&selected_plan, &index_product))
            .expect("execution admits");

    assert_eq!(
        receipt.outcome(),
        EvidenceLookupExecutionOutcome::IndexedHit
    );
    assert_eq!(
        receipt.counters().selected_family_count(),
        selected_plan.counters().selected_family_count()
    );
    assert_eq!(
        receipt.counters().selected_region_count(),
        selected_plan.counters().selected_spatial_region_count()
    );
    assert_eq!(receipt.counters().evidence_candidate_count(), 1);
    assert_eq!(receipt.counters().ledger_rows_touched_count(), 1);
    assert_eq!(
        receipt.counters().index_rows_consumed_count(),
        index_product.counters().selected_basis_row_count()
    );
    assert_eq!(
        receipt.counters().resident_byte_count(),
        index_product.counters().resident_byte_count()
    );
    assert_eq!(receipt.counters().indexed_hit_count(), 1);
    assert_eq!(receipt.counters().indexed_miss_count(), 0);
    assert_eq!(receipt.counters().caller_owned_scan_count(), 0);
    assert_eq!(receipt.counters().query_artifact_count(), 0);
}
