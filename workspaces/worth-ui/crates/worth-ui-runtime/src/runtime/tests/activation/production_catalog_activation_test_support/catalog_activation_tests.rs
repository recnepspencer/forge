use super::{activation_staging_inputs, runtime_with_production_catalog_activation};

#[test]
fn successful_catalog_activation_publishes_every_receipt_once() {
    let (runtime, _, _, _) = runtime_with_production_catalog_activation();
    assert_eq!(runtime.committed_allocation_scope_count_for_test(), 2);
}

#[test]
fn denied_catalog_activation_never_publishes_prepared_receipts() {
    let inputs = activation_staging_inputs();
    let (mut runtime, pending) = inputs.into_runtime_and_pending();
    assert!(runtime.durable_semantic_state().is_none());
    let (snapshot, first, second) =
        crate::runtime::tests::allocation_catalog_test_support::admitted_disjoint_planning_admissions(
            "catalog-rollback",
        );
    let admitted = snapshot
        .admit_allocation_catalog_basis_set(vec![first, second])
        .expect("graph admits complete catalog basis");
    assert_eq!(runtime.committed_allocation_scope_count_for_test(), 0);
    let denied_boundary = runtime.traversal_frame_boundary_for_test();
    let denial = runtime
        .activate_admitted_allocation_catalog_at_frame_boundary(
            pending,
            admitted,
            denied_boundary,
            None,
        )
        .expect_err("unsafe boundary denies the complete activation attempt");
    let crate::runtime::WorthUiAllocationCatalogActivationDenial::Attempt(denial) = denial else {
        panic!("post-mint denial carries canonical attempt evidence")
    };
    let crate::runtime::UiCommittedAllocationActivationDenialReason::FrameBoundary(gate_denial) =
        denial.reason()
    else {
        panic!("unsafe boundary denies during immutable transaction preflight")
    };
    assert_eq!(
        gate_denial.reason(),
        crate::runtime::WorthUiActivationGateDenialReason::UnsafeFrameBoundary
    );
    assert!(denial.evidence().live_state_unchanged());
    assert_eq!(denial.evidence().committed_row_count(), 2);
    assert_eq!(denial.evidence().counters().denial_count(), 1);
    assert_ne!(denial.attempt_identity_digest(), 0);
    assert_eq!(runtime.committed_allocation_scope_count_for_test(), 0);
    assert!(runtime.durable_semantic_state().is_none());
}

#[test]
fn catalog_activation_denies_multi_receipt_and_durable_replacement_atomically() {
    for remaining in [0, 1, 2] {
        let inputs = activation_staging_inputs();
        let (runtime, pending) = inputs.into_runtime_and_pending();
        let predecessor = runtime
            .allocation_receipt_ledger
            .position_truth_revision_for_test(remaining);
        let (snapshot, admissions) =
            crate::runtime::tests::allocation_catalog_test_support::admitted_viewport_planning_admissions(
                "catalog-authority-exhaustion",
                2,
            );
        let admitted = snapshot
            .admit_allocation_catalog_basis_set(admissions)
            .expect("two-neighborhood catalog admits");
        let denial = runtime
            .prepare_allocation_catalog_activation(&pending, admitted)
            .expect_err("exhausted catalog publication denies before activation");
        let crate::runtime::launch::UiAllocationCatalogPreparationDenial::ReceiptCommit(
            commit_outcome,
        ) = denial
        else {
            panic!("catalog exhaustion remains typed")
        };
        let crate::runtime::UiAllocationReceiptCommitOutcome::Denied(commit_denial) =
            *commit_outcome
        else {
            panic!("catalog receipt commit denial remains typed")
        };
        let crate::runtime::UiAllocationReceiptCommitDenial::AuthorityCounterExhausted(exhaustion) =
            *commit_denial
        else {
            panic!("catalog authority exhaustion remains typed")
        };
        assert_eq!(
            exhaustion.counter(),
            crate::runtime::UiAllocationAuthorityCounter::TruthRevision
        );
        assert_eq!(exhaustion.increment(), 3);
        assert_eq!(
            runtime.allocation_receipt_ledger.ledger_baseline_for_test(),
            predecessor
        );
        assert_eq!(runtime.committed_allocation_scope_count_for_test(), 0);
        assert!(runtime.durable_semantic_state().is_none());
    }
}
