use super::fixtures::{admitted_index_product, complete_ledger_for_plan, IndexProductSubject};
use crate::workload_platform::evidence_lookup_index_product::{
    reuse_evidence_lookup_index_product, EvidenceLookupIndexLifecyclePostureKind,
    EvidenceLookupIndexProductErrorKind,
};

#[test]
fn index_reuse_requires_equivalence_basis() {
    let selected_plan = IndexProductSubject::sparse_event_ledger().select_plan();
    let ledger = complete_ledger_for_plan(&selected_plan);
    let prior_product = admitted_index_product(&selected_plan);

    let reused = reuse_evidence_lookup_index_product(&selected_plan, &ledger, &prior_product)
        .expect("matching basis should reuse");
    assert_eq!(
        reused.lifecycle_posture().kind(),
        EvidenceLookupIndexLifecyclePostureKind::EquivalentReuse
    );

    let mismatched_plan = IndexProductSubject::dense_projection_consumption().select_plan();
    let mismatched_ledger = complete_ledger_for_plan(&mismatched_plan);
    let error =
        reuse_evidence_lookup_index_product(&mismatched_plan, &mismatched_ledger, &prior_product)
            .expect_err("mismatched plan basis must deny");
    assert_eq!(
        error.kind(),
        EvidenceLookupIndexProductErrorKind::ReusedIndexBasisMismatch
    );
}
