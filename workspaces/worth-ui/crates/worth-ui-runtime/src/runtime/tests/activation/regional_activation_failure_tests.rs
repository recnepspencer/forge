use super::regional_activation_test_support::{
    assert_active_unchanged, assert_candidate_reclaimed, assert_commit_resource_denial,
    candidate_reclamation_probe, regional_activation_inputs, RegionalActivationInputs,
};

#[test]
fn unavailable_validation_resource_reclaims_candidate_regions_before_preflight() {
    let RegionalActivationInputs {
        mut runtime,
        pending,
        admitted_catalog,
    } = regional_activation_inputs();
    let active_before = runtime.active.active_plan();
    let observation_before = runtime.inspect_active();
    let mut candidate_probe = None;

    let denial = runtime
        .activate_admitted_allocation_catalog_with_boundary_source(
            pending,
            admitted_catalog,
            |runtime, _, candidate_plan, _| {
                let boundary = runtime.safe_frame_boundary();
                candidate_probe = Some(candidate_reclamation_probe(candidate_plan));
                let held = runtime.allocation_invalidation_index.borrow_mut();
                std::mem::forget(held);
                Ok((boundary, None))
            },
        )
        .expect_err("exclusive invalidation ownership denies validation");

    assert_commit_resource_denial(denial, 0);
    assert_candidate_reclaimed(candidate_probe);
    assert_active_unchanged(&runtime, &active_before, observation_before);
}

#[test]
fn denied_final_frame_validation_reclaims_candidate_regions_before_publication() {
    let RegionalActivationInputs {
        mut runtime,
        pending,
        admitted_catalog,
    } = regional_activation_inputs();
    let active_before = runtime.active.active_plan();
    let observation_before = runtime.inspect_active();
    let mut candidate_probe = None;

    let denial = runtime
        .activate_admitted_allocation_catalog_with_boundary_source(
            pending,
            admitted_catalog,
            |runtime, _, candidate_plan, _| {
                candidate_probe = Some(candidate_reclamation_probe(candidate_plan));
                Ok((runtime.traversal_frame_boundary_for_test(), None))
            },
        )
        .expect_err("unsafe final frame validation denies publication");
    let crate::runtime::WorthUiAllocationCatalogActivationDenial::Attempt(denial) = denial else {
        panic!("frame-validation denial retains the canonical attempt")
    };
    let crate::runtime::UiCommittedAllocationActivationDenialReason::FrameBoundary(gate) =
        denial.reason()
    else {
        panic!("unsafe frame validation retains its typed gate denial")
    };

    assert_eq!(
        gate.reason(),
        crate::runtime::WorthUiActivationGateDenialReason::UnsafeFrameBoundary
    );
    assert_eq!(denial.evidence().counters().active_successor_builds(), 0);
    assert!(denial.evidence().live_state_unchanged());
    assert_candidate_reclaimed(candidate_probe);
    assert_active_unchanged(&runtime, &active_before, observation_before);
}

#[test]
fn unavailable_publication_resource_reclaims_preflighted_candidate_regions() {
    let RegionalActivationInputs {
        mut runtime,
        pending,
        admitted_catalog,
    } = regional_activation_inputs();
    let active_before = runtime.active.active_plan();
    let observation_before = runtime.inspect_active();
    let mut candidate_probe = None;

    let denial = runtime
        .activate_admitted_allocation_catalog_with_boundary_source(
            pending,
            admitted_catalog,
            |runtime, _, candidate_plan, _| {
                let boundary = runtime.safe_frame_boundary();
                candidate_probe = Some(candidate_reclamation_probe(candidate_plan));
                let held = runtime.allocation_invalidation_index.borrow();
                std::mem::forget(held);
                Ok((boundary, None))
            },
        )
        .expect_err("shared invalidation ownership denies publication mutation");

    assert_commit_resource_denial(denial, 1);
    assert_candidate_reclaimed(candidate_probe);
    assert_active_unchanged(&runtime, &active_before, observation_before);
}

#[test]
fn ledger_predecessor_drift_reclaims_candidate_regions_without_active_publication() {
    let RegionalActivationInputs {
        mut runtime,
        pending,
        admitted_catalog,
    } = regional_activation_inputs();
    let active_before = runtime.active.active_plan();
    let observation_before = runtime.inspect_active();
    let mut candidate_probe = None;

    let denial = runtime
        .activate_admitted_allocation_catalog_with_boundary_source(
            pending,
            admitted_catalog,
            |runtime, _, candidate_plan, _| {
                let boundary = runtime.safe_frame_boundary();
                candidate_probe = Some(candidate_reclamation_probe(candidate_plan));
                let _intervening_truth = runtime
                    .allocation_receipt_ledger
                    .position_truth_revision_for_test(100);
                Ok((boundary, None))
            },
        )
        .expect_err("intervening ledger truth denies stale publication");
    let crate::runtime::WorthUiAllocationCatalogActivationDenial::Attempt(denial) = denial else {
        panic!("ledger drift denial retains the canonical attempt")
    };

    assert!(matches!(
        denial.reason(),
        crate::runtime::UiCommittedAllocationActivationDenialReason::LedgerPredecessorMismatch
    ));
    assert!(denial.evidence().live_state_unchanged());
    assert_candidate_reclaimed(candidate_probe);
    assert_active_unchanged(&runtime, &active_before, observation_before);
}

#[test]
fn unavailable_frame_commit_reclaims_candidate_regions_without_active_publication() {
    let RegionalActivationInputs {
        mut runtime,
        pending,
        admitted_catalog,
    } = regional_activation_inputs();
    let active_before = runtime.active.active_plan();
    let observation_before = runtime.inspect_active();
    let mut candidate_probe = None;

    let denial = runtime
        .activate_admitted_allocation_catalog_with_boundary_source(
            pending,
            admitted_catalog,
            |runtime, _, candidate_plan, _| {
                let boundary = runtime.safe_frame_boundary();
                candidate_probe = Some(candidate_reclamation_probe(candidate_plan));
                runtime.shutdown_allocation_frame_dispatcher();
                Ok((boundary, None))
            },
        )
        .expect_err("a stopped frame dispatcher denies the final fallible acquisition");
    let crate::runtime::WorthUiAllocationCatalogActivationDenial::Attempt(denial) = denial else {
        panic!("frame-resource denial retains the canonical attempt")
    };

    assert!(matches!(
        denial.reason(),
        crate::runtime::UiCommittedAllocationActivationDenialReason::FrameReplacement(_)
    ));
    assert!(denial.evidence().live_state_unchanged());
    assert_candidate_reclaimed(candidate_probe);
    assert_active_unchanged(&runtime, &active_before, observation_before);
}

#[test]
fn every_late_fallible_boundary_preserves_active_truth_and_reclaims_candidate_storage() {
    #[derive(Clone, Copy, Debug)]
    enum Interruption {
        Certification,
        FrameBoundary,
        LedgerLineage,
        FrameDispatcher,
    }

    for interruption in [
        Interruption::Certification,
        Interruption::FrameBoundary,
        Interruption::LedgerLineage,
        Interruption::FrameDispatcher,
    ] {
        let RegionalActivationInputs {
            mut runtime,
            pending,
            admitted_catalog,
        } = regional_activation_inputs();
        let active_before = runtime.active.active_plan();
        let observation_before = runtime.inspect_active();
        let mut candidate_probe = None;
        let result = runtime.activate_admitted_allocation_catalog_with_boundary_source(
            pending,
            admitted_catalog,
            |runtime, _, candidate_plan, _| {
                candidate_probe = Some(candidate_reclamation_probe(candidate_plan));
                match interruption {
                    Interruption::Certification => Err(
                        crate::runtime::WorthUiAllocationCatalogActivationDenial::CertificationBoundary(
                            "interruption matrix",
                        ),
                    ),
                    Interruption::FrameBoundary => {
                        Ok((runtime.traversal_frame_boundary_for_test(), None))
                    }
                    Interruption::LedgerLineage => {
                        let _ = runtime
                            .allocation_receipt_ledger
                            .position_truth_revision_for_test(100);
                        Ok((runtime.safe_frame_boundary(), None))
                    }
                    Interruption::FrameDispatcher => {
                        runtime.shutdown_allocation_frame_dispatcher();
                        Ok((runtime.safe_frame_boundary(), None))
                    }
                }
            },
        );

        assert!(
            result.is_err(),
            "{interruption:?} must interrupt publication"
        );
        assert_candidate_reclaimed(candidate_probe);
        assert_active_unchanged(&runtime, &active_before, observation_before);
    }
}
