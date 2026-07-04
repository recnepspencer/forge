use crate::workload_platform::evidence_lookup_index_product::tests::fixtures::{
    admitted_index_product, selected_lookup_slice_for_plan, IndexProductSubject,
};
use crate::workload_platform::evidence_lookup_reuse_decision::{
    decide_evidence_lookup_index_reuse, execute_evidence_lookup_index_reuse,
    EvidenceLookupIndexReuseExecutionInput, EvidenceLookupIndexReuseResolution,
    EvidenceLookupReuseDecisionPosture, EvidenceLookupReuseMismatchLocus,
};

#[test]
fn reuse_decision_binds_identity_and_family_chain() {
    let selected_plan = IndexProductSubject::sparse_event_ledger().select_plan();
    let ledger = selected_lookup_slice_for_plan(&selected_plan);
    let prior_product = admitted_index_product(&selected_plan);
    let current_input = EvidenceLookupIndexReuseExecutionInput::lower(&selected_plan, &ledger)
        .expect("reuse input");

    let decision = decide_evidence_lookup_index_reuse(&current_input, &prior_product);

    assert_eq!(
        decision.posture(),
        EvidenceLookupReuseDecisionPosture::ReuseAdmitted
    );
    assert_eq!(
        decision.compiled_product_identity_digest(),
        prior_product.compiled_product_identity_digest()
    );
    assert_eq!(
        decision.equivalence_policy_identity_digest(),
        prior_product.equivalence_policy_identity_digest()
    );
    assert_eq!(
        decision.selected_equivalence_family_identity(),
        prior_product.selected_equivalence_family_identity()
    );
    assert!(decision.reuse_decision_identity_digest().is_some());

    let reused =
        execute_evidence_lookup_index_reuse(decision.clone(), &current_input, &prior_product)
            .expect("reuse execution");
    let EvidenceLookupIndexReuseResolution::Reused {
        product: reused, ..
    } = reused
    else {
        panic!("expected reused resolution");
    };
    assert_eq!(
        reused.reuse_decision_identity_digest(),
        decision.reuse_decision_identity_digest()
    );
}

#[test]
fn rebuild_required_is_first_class_not_fallback() {
    let selected_plan = IndexProductSubject::dense_projection_consumption().select_plan();
    let ledger = selected_lookup_slice_for_plan(&selected_plan);
    let selected_product = admitted_index_product(&selected_plan);
    let prior_product = selected_product
        .clone()
        .with_test_selected_reuse_basis_identity_digest("phase-9-forged-selected-reuse-basis");
    let current_input = EvidenceLookupIndexReuseExecutionInput::lower(&selected_plan, &ledger)
        .expect("rebuild-required input");

    let decision = decide_evidence_lookup_index_reuse(&current_input, &prior_product);

    assert_eq!(
        decision.posture(),
        EvidenceLookupReuseDecisionPosture::AdvisoryMatchRequiresRebuild
    );
    let denial = decision
        .rebuild_denial()
        .expect("rebuild-required decision must carry typed denial");
    assert_eq!(
        denial.mismatch_loci(),
        &[EvidenceLookupReuseMismatchLocus::SelectedReuseBasisIdentity]
    );

    let rebuilt =
        execute_evidence_lookup_index_reuse(decision.clone(), &current_input, &prior_product)
            .expect("rebuild execution");
    let EvidenceLookupIndexReuseResolution::Rebuilt {
        product: rebuilt, ..
    } = rebuilt
    else {
        panic!("expected rebuilt resolution");
    };
    assert_eq!(rebuilt.reuse_decision_identity_digest(), None);
    assert_eq!(
        rebuilt.selected_reuse_basis_identity_digest(),
        selected_product.selected_reuse_basis_identity_digest()
    );
}

#[test]
fn reuse_denial_localizes_mismatch_locus() {
    let selected_plan = IndexProductSubject::dense_projection_consumption().select_plan();
    let ledger = selected_lookup_slice_for_plan(&selected_plan);
    let prior_product = admitted_index_product(&selected_plan)
        .with_test_selected_equivalence_family_identity(
            "spatial.selected-equivalence.retained-replay-semantic-parity",
        );
    let current_input = EvidenceLookupIndexReuseExecutionInput::lower(&selected_plan, &ledger)
        .expect("denied input");

    let decision = decide_evidence_lookup_index_reuse(&current_input, &prior_product);

    assert_eq!(
        decision.posture(),
        EvidenceLookupReuseDecisionPosture::Denied
    );
    let denial = decision
        .rebuild_denial()
        .expect("denied decision must carry denial");
    assert_eq!(
        denial.mismatch_loci(),
        &[EvidenceLookupReuseMismatchLocus::SelectedEquivalenceFamilyIdentity]
    );
    let denied =
        execute_evidence_lookup_index_reuse(decision.clone(), &current_input, &prior_product)
            .expect("denied execution");
    let EvidenceLookupIndexReuseResolution::Denied {
        denial: carried_denial,
        ..
    } = denied
    else {
        panic!("expected denied resolution");
    };
    assert_eq!(carried_denial, denial.clone());
}

#[test]
fn reuse_counters_are_semantic_not_generic() {
    let selected_plan = IndexProductSubject::sparse_event_ledger().select_plan();
    let ledger = selected_lookup_slice_for_plan(&selected_plan);
    let prior_product = admitted_index_product(&selected_plan);
    let current_input = EvidenceLookupIndexReuseExecutionInput::lower(&selected_plan, &ledger)
        .expect("reuse input");

    let decision = decide_evidence_lookup_index_reuse(&current_input, &prior_product);

    assert_eq!(decision.counters().compared_basis_dimension_count(), 8);
    assert_eq!(
        decision.product_counters().selected_basis_row_count(),
        prior_product.counters().selected_basis_row_count()
    );
    assert_eq!(
        decision.product_counters().topology_receipt_ref_count(),
        prior_product.counters().topology_receipt_ref_count()
    );
    assert_eq!(
        decision.product_counters().query_support_row_count(),
        prior_product.counters().query_support_row_count()
    );
}
