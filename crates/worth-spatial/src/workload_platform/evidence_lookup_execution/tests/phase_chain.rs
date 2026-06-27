use super::super::error::EvidenceLookupExecutionErrorKind;
use super::super::execute::execute_evidence_lookup;
use super::fixtures::ExecutionSubject;

#[test]
fn mismatched_selected_plan_and_index_product_fail_before_execution() {
    let sparse_subject = ExecutionSubject::sparse_event_ledger();
    let dense_subject = ExecutionSubject::dense_projection_consumption();
    let sparse_plan = sparse_subject.selected_plan();
    let dense_plan = dense_subject.selected_plan();
    let dense_index_product = dense_subject.index_product(&dense_plan);

    let error = execute_evidence_lookup(
        &sparse_subject.execution_request(&sparse_plan, &dense_index_product),
    )
    .expect_err("phase chain mismatch must fail before execution");

    assert_eq!(
        error.kind(),
        EvidenceLookupExecutionErrorKind::PlanIndexDigestMismatch
    );
}
