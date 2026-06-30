use super::super::identity::{lower_index_family_identity, rebuild_required_identity};
use super::fixtures::{
    admitted_index_product, selected_lookup_slice_for_plan, IndexProductSubject,
};
use crate::workload_platform::evidence_lookup_index_product::{
    reuse_evidence_lookup_index_product, EvidenceLookupIndexLifecyclePostureKind,
    EvidenceLookupIndexProductErrorKind,
};

#[test]
fn index_reuse_requires_equivalence_basis() {
    let selected_plan = IndexProductSubject::sparse_event_ledger().select_plan();
    let ledger = selected_lookup_slice_for_plan(&selected_plan);
    let prior_product = admitted_index_product(&selected_plan);

    let reused = reuse_evidence_lookup_index_product(&selected_plan, &ledger, &prior_product)
        .expect("matching basis should reuse");
    assert_eq!(
        reused.lifecycle_posture().kind(),
        EvidenceLookupIndexLifecyclePostureKind::EquivalentReuse
    );
    assert_eq!(
        reused.compiled_product_identity_digest(),
        prior_product.compiled_product_identity_digest()
    );
    assert_eq!(
        reused.equivalence_policy_identity_digest(),
        prior_product.equivalence_policy_identity_digest()
    );
    assert!(reused.reuse_decision_identity_digest().is_some());

    let mismatched_plan = IndexProductSubject::dense_projection_consumption().select_plan();
    let mismatched_ledger = selected_lookup_slice_for_plan(&mismatched_plan);
    let error =
        reuse_evidence_lookup_index_product(&mismatched_plan, &mismatched_ledger, &prior_product)
            .expect_err("mismatched plan basis must deny");
    assert_eq!(
        error.kind(),
        EvidenceLookupIndexProductErrorKind::ReusedIndexBasisMismatch
    );
    let expected_lowered_identity =
        lower_index_family_identity(&mismatched_plan, &mismatched_ledger);
    let expected_denial = rebuild_required_identity(
        expected_lowered_identity.compiled_product_identity(),
        "evidence-lookup-index-reuse-basis-mismatch",
    );
    assert_eq!(
        error.rebuild_denial_identity_digest(),
        Some(expected_denial.identity_digest())
    );
}
