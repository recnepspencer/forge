use super::activation_staging_test_support::activation_staging_inputs;
use super::lane_change_activation_test_support::lane_change_activation_inputs;

fn ordinary_inputs() -> (
    crate::runtime::WorthUiRuntime,
    crate::runtime::WorthUiPendingActivation,
    crate::graph::UiAdmittedAllocationCatalogBasisSet,
) {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let (snapshot, first, second) =
        crate::runtime::tests::allocation_catalog_test_support::admitted_disjoint_planning_admissions(
            "frame-validation.ordinary",
        );
    let admitted = snapshot
        .admit_allocation_catalog_basis_set(vec![first, second])
        .expect("graph admits complete frame-validation catalog");
    (runtime, pending, admitted)
}

#[test]
fn safe_boundary_is_sealed_inside_the_canonical_receipt() {
    let (mut runtime, pending, admitted) = ordinary_inputs();
    let boundary = runtime.safe_frame_boundary();
    let receipt = runtime
        .activate_admitted_allocation_catalog_at_frame_boundary(pending, admitted, boundary, None)
        .expect("safe boundary activates");

    assert_eq!(receipt.boundary_frame_epoch(), boundary.frame_epoch());
    assert_eq!(receipt.readiness_frame_epoch(), boundary.frame_epoch());
    assert_eq!(
        receipt
            .activation_gate_receipt()
            .counters()
            .boundary_check_count(),
        4
    );
}

#[test]
fn successful_activation_publishes_one_complete_region_bundle() {
    let (mut runtime, pending, admitted) = ordinary_inputs();
    let predecessor = runtime.active.active_plan();
    let predecessor_observation = runtime.inspect_active();
    let boundary = runtime.safe_frame_boundary();
    runtime
        .activate_admitted_allocation_catalog_at_frame_boundary(pending, admitted, boundary, None)
        .expect("regional successor activates");
    let successor = runtime.active.active_plan();
    let successor_plan = successor.exact_plan();
    let evidence = successor_plan.regional_evidence();

    assert!(successor_plan.region_count() > 0);
    assert!(!successor_plan.has_reconstructive_flat_projection());
    assert_eq!(
        evidence.predecessor_artifact_digest(),
        Some(predecessor_observation.artifact_digest())
    );
    assert_eq!(
        evidence.candidate_artifact_digest(),
        runtime.inspect_active().artifact_digest()
    );
    assert_eq!(
        evidence.affected_region_count(),
        evidence.transitions().len()
    );
    for transition in evidence.transitions() {
        assert!(evidence
            .transition_for_region(transition.region_identity())
            .is_some());
    }
    for identity in predecessor.exact_plan().canonical_region_identities() {
        if evidence.transition_for_region(&identity).is_none() {
            assert!(predecessor
                .exact_plan()
                .shares_exact_region_storage_with(successor_plan, &identity,));
        }
    }
}

#[test]
fn unsafe_boundary_denial_is_canonical_and_non_mutating() {
    let (mut runtime, pending, admitted) = ordinary_inputs();
    let active_before = runtime.inspect_active();
    let dispatcher_before = runtime.allocation_frame_dispatcher_state();
    let boundary = runtime.traversal_frame_boundary_for_test();
    let denial = runtime
        .activate_admitted_allocation_catalog_at_frame_boundary(pending, admitted, boundary, None)
        .expect_err("unsafe boundary denies");
    let crate::runtime::WorthUiAllocationCatalogActivationDenial::Attempt(denial) = denial else {
        panic!("post-mint boundary denial carries canonical attempt evidence")
    };
    let crate::runtime::UiCommittedAllocationActivationDenialReason::FrameBoundary(gate) =
        denial.reason()
    else {
        panic!("boundary denial retains its typed gate reason")
    };
    assert_eq!(
        gate.reason(),
        crate::runtime::WorthUiActivationGateDenialReason::UnsafeFrameBoundary
    );
    assert!(denial.evidence().live_state_unchanged());
    assert_eq!(runtime.inspect_active(), active_before);
    assert_eq!(
        runtime.allocation_frame_dispatcher_state(),
        dispatcher_before
    );
}

#[test]
fn future_boundary_denies_before_frame_resource_acquisition() {
    let (mut runtime, pending, admitted) = ordinary_inputs();
    let active_before = runtime.inspect_active();
    let future_epoch = runtime
        .frame_epoch()
        .checked_next()
        .expect("fixture epoch has a successor");
    let boundary = runtime.safe_frame_boundary_for_epoch_for_test(future_epoch);
    let denial = runtime
        .activate_admitted_allocation_catalog_at_frame_boundary(pending, admitted, boundary, None)
        .expect_err("future boundary denies");
    let crate::runtime::WorthUiAllocationCatalogActivationDenial::Attempt(denial) = denial else {
        panic!("post-mint boundary denial carries canonical attempt evidence")
    };
    let crate::runtime::UiCommittedAllocationActivationDenialReason::FrameBoundary(gate) =
        denial.reason()
    else {
        panic!("future boundary retains its typed reason")
    };
    assert_eq!(
        gate.reason(),
        crate::runtime::WorthUiActivationGateDenialReason::FutureFrameEpochMismatch
    );
    assert_eq!(denial.evidence().counters().frame_replacement_checks(), 0);
    assert_eq!(runtime.inspect_active(), active_before);
}

#[test]
fn lane_change_without_parity_denies_through_the_ordinary_route() {
    let mut fixture = lane_change_activation_inputs();
    let active_before = fixture.runtime.inspect_active();
    let boundary = fixture.runtime.safe_frame_boundary();
    let denial = fixture
        .runtime
        .activate_admitted_allocation_catalog_at_frame_boundary(
            fixture.pending,
            fixture.admitted_catalog,
            boundary,
            None,
        )
        .expect_err("lane change requires parity evidence");
    let crate::runtime::WorthUiAllocationCatalogActivationDenial::Attempt(denial) = denial else {
        panic!("validation denial carries canonical attempt evidence")
    };
    let crate::runtime::UiCommittedAllocationActivationDenialReason::Validation(gate) =
        denial.reason()
    else {
        panic!("missing parity is a typed validation denial")
    };
    assert_eq!(
        gate.reason(),
        crate::runtime::WorthUiActivationGateDenialReason::MissingLaneParityReport
    );
    assert_eq!(fixture.runtime.inspect_active(), active_before);
}

#[test]
fn lane_change_parity_is_consumed_by_the_same_move_only_attempt() {
    let fixture = lane_change_activation_inputs();
    let super::lane_change_activation_test_support::LaneChangeActivationInputs {
        mut runtime,
        pending,
        admitted_catalog,
        node_plan,
        narrowing,
        query_comparison,
        query_rebind,
    } = fixture;
    let receipt = runtime
        .activate_admitted_allocation_catalog_with_boundary_source(
            pending,
            admitted_catalog,
            |runtime, _, candidate_plan, _planning| {
                let parity = runtime
                    .certify_lane_meaning_parity(
                        &node_plan,
                        &narrowing,
                        candidate_plan,
                        candidate_plan,
                        &query_comparison,
                        Some(&query_rebind),
                    )
                    .map_err(|_| {
                        crate::runtime::WorthUiAllocationCatalogActivationDenial::CertificationBoundary("test boundary")
                    })?;
                Ok((runtime.safe_frame_boundary(), Some(parity)))
            },
        )
        .expect("certified lane change activates through the ordinary route");

    assert!(receipt.lane_changed_node_count() > 0);
    assert!(receipt.lane_parity_semantic_reference_digest().is_some());
}
