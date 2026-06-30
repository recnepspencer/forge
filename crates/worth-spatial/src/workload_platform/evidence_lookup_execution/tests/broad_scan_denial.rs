use super::super::execute::execute_evidence_lookup;
use super::fixtures::{complete_ledger_for_plan, ExecutionSubject};
use crate::workload_platform::evidence_lookup_index_product::{
    audit_evidence_lookup_index_product_basis, EvidenceLookupIndexBasisAuditScope,
};

#[test]
fn unbounded_basis_is_rejected_before_execution_even_when_selected_stage_evidence_matches() {
    let subject = ExecutionSubject::sparse_event_ledger();
    let selected_plan = subject.selected_plan();
    let ledger = complete_ledger_for_plan(&selected_plan);
    let index_product = subject.index_product(&selected_plan);

    let receipt =
        execute_evidence_lookup(&subject.execution_request(&selected_plan, &index_product))
            .expect("bounded execution admits");

    assert!(
        receipt.counters().ledger_rows_touched_count()
            < index_product.counters().total_ledger_row_count()
    );
    assert_eq!(
        receipt
            .lookup_product_output()
            .evidence_receipt_digests()
            .len(),
        1
    );

    let error = audit_evidence_lookup_index_product_basis(
        &selected_plan,
        &ledger,
        EvidenceLookupIndexBasisAuditScope::CompleteLedgerUnbounded,
    )
    .expect_err("unbounded basis must fail before execution can start");

    assert!(
        error.counters().selected_basis_row_count()
            > index_product.counters().selected_basis_row_count()
    );
}
