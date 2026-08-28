use super::*;

#[test]
fn admitted_extent_evidence_can_be_acquired_without_offset_authority() {
    let (mut runtime, _, result, query, receipt, _, _, _) =
        crate::runtime::tests::production_catalog_activation_test_support::runtime_with_scroll_catalog();
    let mut query_owner = None;
    let mut host_owner = None;
    let dispatcher_before = runtime.allocation_frame_dispatcher_counters();
    let truth_before = runtime.allocation_receipt_ledger.truth_revision();
    let completion = runtime.execute_framework_turn(|turn| {
        turn.scroll_extent(|source| {
            query_owner = source.acquire_settled_query_owner(&query, &receipt).ok();
            host_owner = source.acquire_host_owner(&result, &receipt).ok();
        });
    });
    drop(completion);
    let query_owner = query_owner.expect("admitted Query extent owner");
    let host_owner = host_owner.expect("admitted host extent owner");
    assert_eq!(query_owner.authority_probes(), 3);
    assert_eq!(host_owner.authority_probes(), 2);
    assert_eq!(query_owner.target().target(), host_owner.target().target());
    assert_ne!(
        query_owner.target().owner_identity(),
        host_owner.target().owner_identity()
    );
    assert_eq!(
        runtime
            .allocation_frame_dispatcher_counters()
            .ingress_count(),
        dispatcher_before.ingress_count()
    );
    assert_eq!(
        runtime.allocation_receipt_ledger.truth_revision(),
        truth_before
    );
}

#[test]
fn unrelated_receipt_cannot_smuggle_host_or_query_extent_ownership() {
    let (mut runtime, _, result, query, _, unrelated_receipt, _, _) =
        crate::runtime::tests::production_catalog_activation_test_support::runtime_with_scroll_catalog();
    let mut host = None;
    let mut query_owner = None;
    let completion = runtime.execute_framework_turn(|turn| {
        turn.scroll_extent(|source| {
            host = Some(source.acquire_host_owner(&result, &unrelated_receipt));
            query_owner = Some(source.acquire_settled_query_owner(&query, &unrelated_receipt));
        });
    });
    drop(completion);
    assert_eq!(
        host.unwrap(),
        Err(crate::runtime::UiScrollOwnerAcquisitionDenial::ReceiptNotActive)
    );
    assert_eq!(
        query_owner.unwrap(),
        Err(crate::runtime::UiScrollOwnerAcquisitionDenial::ReceiptNotActive)
    );
}

#[test]
fn ordinary_neighborhood_cannot_mint_scroll_ownership() {
    let basis = crate::runtime::tests::allocation_planning_test_support::admitted_measurement_basis(
        "phase10-non-scroll",
    );
    let neighborhood =
        crate::runtime::tests::allocation_planning_test_support::admitted_allocation_neighborhood(
            "phase10-non-scroll",
        );
    let planning_basis =
        crate::runtime::WorthUiAllocationPlanningBasis::new(basis, neighborhood, None);
    assert!(UiAdmittedScrollPlanningAuthority::seal(&planning_basis)
        .expect("ordinary non-scroll posture is not malformed")
        .is_none());
}
