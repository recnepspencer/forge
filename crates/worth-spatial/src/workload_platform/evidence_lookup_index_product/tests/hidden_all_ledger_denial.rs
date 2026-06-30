use super::fixtures::{complete_ledger_for_plan, IndexProductSubject};
use crate::workload_platform::evidence_lookup_index_product::{
    admit_evidence_lookup_index_product, audit_evidence_lookup_index_product_basis,
    EvidenceLookupIndexBasisAuditScope, EvidenceLookupIndexProductErrorKind,
};

#[test]
fn hidden_all_evidence_index_fails_index_contract() {
    let selected_plan = IndexProductSubject::sparse_event_ledger().select_plan();
    let ledger = complete_ledger_for_plan(&selected_plan);
    let bounded = admit_evidence_lookup_index_product(&selected_plan, &ledger)
        .expect("bounded selected-scope basis should admit");

    let error = audit_evidence_lookup_index_product_basis(
        &selected_plan,
        &ledger,
        EvidenceLookupIndexBasisAuditScope::CompleteLedgerUnbounded,
    )
    .expect_err("all-ledger basis must deny on the production audit boundary");

    assert_eq!(
        error.kind(),
        EvidenceLookupIndexProductErrorKind::LedgerBasisExceedsSelectedScope
    );
    assert!(
        error.counters().selected_basis_row_count() > bounded.counters().selected_basis_row_count()
    );
    assert!(error.counters().resident_byte_count() > bounded.counters().resident_byte_count());
}
