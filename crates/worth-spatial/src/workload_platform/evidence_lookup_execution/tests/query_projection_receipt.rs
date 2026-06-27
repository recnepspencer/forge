use super::super::execute::execute_evidence_lookup;
use super::super::outcome::EvidenceLookupExecutionOutcome;
use super::fixtures::ExecutionSubject;

#[test]
fn missing_query_consumed_fact_receipt_does_not_execute_lookup() {
    let subject = ExecutionSubject::dense_projection_consumption();
    let selected_plan = subject.selected_plan();
    let index_product = subject.index_product(&selected_plan);

    let receipt =
        execute_evidence_lookup(&subject.execution_request(&selected_plan, &index_product))
            .expect("execution returns denial-grade receipt");

    assert_eq!(
        receipt.outcome(),
        EvidenceLookupExecutionOutcome::RequiredQuerySupport
    );
    assert_eq!(receipt.counters().evidence_candidate_count(), 0);
    assert_eq!(receipt.counters().ledger_rows_touched_count(), 0);
    assert_eq!(receipt.counters().index_rows_consumed_count(), 0);
    assert_eq!(receipt.counters().query_artifact_count(), 0);
    assert!(receipt
        .lookup_product_output()
        .evidence_receipt_digests()
        .is_empty());
}
