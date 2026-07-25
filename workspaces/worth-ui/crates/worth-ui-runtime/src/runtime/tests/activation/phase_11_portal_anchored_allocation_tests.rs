use super::phase_11_portal_test_support::{
    committed, submit_portal_observation, submit_portal_observation_in,
};
use worth_ui_host_contract::UiPortalAnchorRectObservation;
use worth_ui_inspection::UiEvidenceAuthorityGeneration;
#[test]
fn moved_rect_preserves_identity_and_replans_only_graph_owned_portal_locality() {
    let (mut runtime, roots, active_receipt, _) =
        super::production_catalog_activation_test_support::runtime_with_portal_catalog();
    let portal_root = active_receipt.identity().graph_node_identity();
    let prior_identity = active_receipt.identity().clone();
    let activation_basis_generation = active_receipt.generation().measurement_basis_generation();
    let completion = submit_portal_observation(
        &mut runtime,
        44,
        UiPortalAnchorRectObservation {
            x: 101.0,
            y: 202.0,
            width: 30.0,
            height: 40.0,
        },
    );
    let crate::runtime::WorthUiFrameworkTurnCompletion::AllocationInvalidationsNarrowed {
        selection,
        transaction,
        ..
    } = completion
    else {
        panic!("portal movement must reach graph-owned locality: {completion:?}")
    };
    assert_eq!(selection.ordered_neighborhoods().len(), 1);
    assert_eq!(
        selection.primary().identity().root_graph_node_identity(),
        portal_root
    );
    assert!(roots
        .iter()
        .filter(|root| **root != portal_root)
        .all(|root| {
            selection
                .ordered_neighborhoods()
                .iter()
                .all(|selected| selected.identity().root_graph_node_identity() != *root)
        }));
    let committed = committed(transaction);
    let evidence = committed.evidence();
    let portal = evidence
        .portal_anchor_movements()
        .first()
        .expect("transaction retains portal movement evidence");
    assert!(matches!(
        portal.identity_transition(),
        crate::runtime::UiPortalAnchorIdentityTransition::Preserved { identity }
            if identity.target().raw() == 44
    ));
    assert_eq!(portal.neighborhood_identity_digests().len(), 1);
    assert_eq!(portal.authority_probes(), 1);
    let receipt = &committed.receipts()[0];
    assert_eq!(receipt.identity(), &prior_identity);
    assert_eq!(
        portal.receipt_identity_digest(),
        receipt.identity().identity_digest()
    );
    assert_eq!(
        portal.receipt_generation_digest(),
        receipt.generation().identity_digest()
    );
    let geometry = receipt
        .geometry_evidence()
        .portal_anchor_observation()
        .expect("committed receipt carries the successor portal observation");
    assert_eq!(
        receipt.identity().portal_anchor(),
        Some(portal.identity_transition().current())
    );
    assert_eq!(geometry.identity(), portal.identity_transition().current());
    assert_eq!(geometry.observed_bounds().x(), 101.0);
    assert_eq!(geometry.observed_bounds().y(), 202.0);
    let successor_basis_generation = receipt
        .committed_allocation()
        .measurement_basis()
        .generation();
    assert_eq!(
        receipt.generation().measurement_basis_generation(),
        successor_basis_generation
    );
    assert_ne!(successor_basis_generation, activation_basis_generation);
    assert_eq!(
        receipt
            .generation()
            .portal_evidence_generation()
            .map(UiEvidenceAuthorityGeneration::as_u64),
        Some(18)
    );
    let canonical_portal = receipt
        .committed_allocation()
        .planning_basis()
        .portal_allocation_input()
        .expect("successor portal input is canonical planning truth");
    assert_eq!(canonical_portal.observation().rect().x, 101.0);
    assert_eq!(
        receipt
            .committed_allocation()
            .planning_basis()
            .allocation_constraint_set()
            .and_then(crate::evidence::UiAllocationConstraintSet::portal_anchor_planning_input)
            .and_then(crate::evidence::UiConstraintPortalAnchorPlanningInputResult::source_generation_digest),
        Some(18)
    );
}

#[test]
fn changed_target_replaces_anchor_identity_without_host_owned_retargeting() {
    let (mut runtime, _, active_receipt, _) =
        super::production_catalog_activation_test_support::runtime_with_portal_catalog();
    let prior_identity = active_receipt.identity().clone();
    let completion = submit_portal_observation(
        &mut runtime,
        45,
        UiPortalAnchorRectObservation {
            x: 1.0,
            y: 2.0,
            width: 3.0,
            height: 4.0,
        },
    );
    let crate::runtime::WorthUiFrameworkTurnCompletion::AllocationInvalidationsNarrowed {
        transaction,
        ..
    } = completion
    else {
        panic!("target replacement remains a runtime allocation transition: {completion:?}")
    };
    let committed = committed(transaction);
    let evidence = committed.evidence();
    assert!(matches!(
        evidence.portal_anchor_movements()[0].identity_transition(),
        crate::runtime::UiPortalAnchorIdentityTransition::TargetReplaced { prior, current }
            if prior.target().raw() == 44 && current.target().raw() == 45
    ));
    assert_eq!(
        committed.receipts()[0]
            .identity()
            .portal_anchor()
            .map(|identity| identity.target().raw()),
        Some(45)
    );
    assert_ne!(committed.receipts()[0].identity(), &prior_identity);
}

#[test]
fn repeated_portal_churn_advances_the_committed_binding_baseline() {
    let (mut runtime, _, _, _) =
        super::production_catalog_activation_test_support::runtime_with_portal_catalog();
    let first = submit_portal_observation_in(
        &mut runtime,
        45,
        UiPortalAnchorRectObservation {
            x: 1.0,
            y: 2.0,
            width: 3.0,
            height: 4.0,
        },
        UiEvidenceAuthorityGeneration::new(18),
        crate::host::UiPortalAnchorCoordinateSpacePosture::PortalLayer,
    );
    let crate::runtime::WorthUiFrameworkTurnCompletion::AllocationInvalidationsNarrowed {
        transaction,
        ..
    } = first
    else {
        panic!("first portal successor must commit: {first:?}")
    };
    let first = committed(transaction);
    assert_eq!(
        first.receipts()[0]
            .identity()
            .portal_anchor()
            .unwrap()
            .target()
            .raw(),
        45
    );
    let succession = first
        .portal_binding_succession()
        .expect("committed portal churn retains typed binding succession");
    assert_eq!(succession.counters().consequences_visited(), 1);
    assert_eq!(succession.counters().binding_lookups(), 1);
    assert_eq!(succession.counters().receipt_lookups(), 1);
    assert_eq!(succession.counters().binding_replacements(), 1);
    assert_eq!(
        succession.lineage()[0]
            .predecessor_evidence_generation()
            .as_u64(),
        17
    );
    assert_eq!(
        succession.lineage()[0]
            .successor_evidence_generation()
            .as_u64(),
        18
    );
    let report = crate::evidence::measurement::projection::fact_test_support::capability_report(77);
    let stale =
        crate::evidence::measurement::projection::fact_test_support::host_result_portal_anchor_at(
            981,
            45,
            [9.0, 9.0, 9.0, 9.0],
            &report,
            UiEvidenceAuthorityGeneration::new(18),
        );
    assert!(matches!(
        runtime
            .allocation_invalidation_index
            .borrow()
            .portal_movement(&stale),
        Err(
            crate::runtime::invalidation_narrowing::UiPortalMovementLookupDenial::SuccessorBasis(
                crate::runtime::UiPortalAnchorSuccessorDenial::StaleEvidenceGeneration
            )
        )
    ));

    let second = submit_portal_observation_in(
        &mut runtime,
        46,
        UiPortalAnchorRectObservation {
            x: 5.0,
            y: 6.0,
            width: 7.0,
            height: 8.0,
        },
        UiEvidenceAuthorityGeneration::new(19),
        crate::host::UiPortalAnchorCoordinateSpacePosture::Viewport,
    );
    let crate::runtime::WorthUiFrameworkTurnCompletion::AllocationInvalidationsNarrowed {
        transaction,
        ..
    } = second
    else {
        panic!("second portal successor must use the committed baseline: {second:?}")
    };
    let second = committed(transaction);
    let transition = second.evidence().portal_anchor_movements()[0].identity_transition();
    assert!(matches!(
        transition,
        crate::runtime::UiPortalAnchorIdentityTransition::TargetReplaced { prior, current }
            if prior.target().raw() == 45 && current.target().raw() == 46
    ));
    assert_eq!(
        second.receipts()[0]
            .generation()
            .portal_evidence_generation()
            .unwrap()
            .as_u64(),
        19
    );
}

#[test]
fn coordinate_space_change_replaces_identity_and_commits_the_typed_posture() {
    let (mut runtime, _, active_receipt, _) =
        super::production_catalog_activation_test_support::runtime_with_portal_catalog();
    let prior_identity = active_receipt.identity().clone();
    let completion = submit_portal_observation_in(
        &mut runtime,
        44,
        UiPortalAnchorRectObservation {
            x: 1.0,
            y: 2.0,
            width: 3.0,
            height: 4.0,
        },
        UiEvidenceAuthorityGeneration::new(18),
        crate::host::UiPortalAnchorCoordinateSpacePosture::Viewport,
    );
    let crate::runtime::WorthUiFrameworkTurnCompletion::AllocationInvalidationsNarrowed {
        transaction,
        ..
    } = completion
    else {
        panic!("coordinate-space replacement must enter allocation: {completion:?}")
    };
    let committed = committed(transaction);
    let evidence = committed.evidence();
    let movement = &evidence.portal_anchor_movements()[0];
    assert!(matches!(
        movement.identity_transition(),
        crate::runtime::UiPortalAnchorIdentityTransition::CoordinateSpaceReplaced { prior, current }
            if prior.coordinate_space() == crate::evidence::UiMeasurementCoordinateSpace::PortalLayer
                && current.coordinate_space() == crate::evidence::UiMeasurementCoordinateSpace::Viewport
    ));
    let observation = committed.receipts()[0]
        .geometry_evidence()
        .portal_anchor_observation()
        .expect("receipt carries replacement coordinate posture");
    assert_eq!(
        observation.observed_bounds().coordinate_space(),
        crate::evidence::UiMeasurementCoordinateSpace::Viewport
    );
    assert_ne!(committed.receipts()[0].identity(), &prior_identity);
}

#[test]
fn stale_portal_observation_denies_before_any_receipt_mutation() {
    let (mut runtime, _, _, _) =
        super::production_catalog_activation_test_support::runtime_with_portal_catalog();
    let before = runtime.allocation_receipt_ledger.ledger_baseline_for_test();
    let completion = submit_portal_observation_in(
        &mut runtime,
        44,
        UiPortalAnchorRectObservation {
            x: 9.0,
            y: 9.0,
            width: 9.0,
            height: 9.0,
        },
        UiEvidenceAuthorityGeneration::new(16),
        crate::host::UiPortalAnchorCoordinateSpacePosture::PortalLayer,
    );
    let crate::runtime::WorthUiFrameworkTurnCompletion::AllocationInvalidationNarrowingDenied {
        rejection,
    } = completion
    else {
        panic!("stale portal evidence must deny atomically: {completion:?}")
    };
    assert_eq!(
        rejection.denial(),
        crate::runtime::UiAllocationInvalidationNarrowingDenial::PortalAnchorEvidenceStale {
            ordinal: 0
        }
    );
    assert_eq!(
        runtime.allocation_receipt_ledger.ledger_baseline_for_test(),
        before
    );
}
