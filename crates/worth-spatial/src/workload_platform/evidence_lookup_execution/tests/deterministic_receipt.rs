use super::super::execute::execute_evidence_lookup;
use super::fixtures::ExecutionSubject;
use crate::workload_platform::evidence_lookup_input_admission::real_projection_consumption_receipt;

#[test]
fn lookup_execution_receipt_is_deterministic() {
    let subject = ExecutionSubject::dense_projection_consumption();
    let selected_plan = subject.selected_plan();
    let index_product = subject.index_product(&selected_plan);
    let left_projection_receipt = real_projection_consumption_receipt();
    let right_projection_receipt = real_projection_consumption_receipt();

    let left = execute_evidence_lookup(&subject.execution_request_with_projection_receipt(
        &selected_plan,
        &index_product,
        &left_projection_receipt,
    ))
    .expect("left execution admits");
    let right = execute_evidence_lookup(&subject.execution_request_with_projection_receipt(
        &selected_plan,
        &index_product,
        &right_projection_receipt,
    ))
    .expect("right execution admits");

    assert_eq!(
        left.execution_receipt_digest(),
        right.execution_receipt_digest()
    );
    assert_eq!(
        left.lookup_product_output().output_digest(),
        right.lookup_product_output().output_digest()
    );
    assert_eq!(left.counter_digest(), right.counter_digest());
}
