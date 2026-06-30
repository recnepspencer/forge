use super::super::execute::execute_evidence_lookup;
use super::super::receipt::EvidenceLookupExecutionTopologySupportState;
use super::fixtures::ExecutionSubject;

#[test]
fn execution_receipt_preserves_index_posture_and_topology_summary() {
    let subject = ExecutionSubject::dense_projection_consumption();
    let selected_plan = subject.selected_plan();
    let index_product = subject.index_product(&selected_plan);
    let projection_receipt =
        crate::workload_platform::evidence_lookup_input_admission::real_projection_consumption_receipt();

    let receipt = execute_evidence_lookup(&subject.execution_request_with_projection_receipt(
        &selected_plan,
        &index_product,
        &projection_receipt,
    ))
    .expect("execution admits");

    assert_eq!(
        receipt.index_lifecycle_posture(),
        index_product.lifecycle_posture()
    );
    assert_eq!(
        receipt.index_disposal_posture(),
        index_product.disposal_posture()
    );
    assert_eq!(
        receipt.topology_support_state(),
        EvidenceLookupExecutionTopologySupportState::NotEvaluatedForUnaffectedFamily
    );
    assert_eq!(
        receipt.lookup_product_output_digest(),
        receipt.lookup_product_output().output_digest()
    );
}
