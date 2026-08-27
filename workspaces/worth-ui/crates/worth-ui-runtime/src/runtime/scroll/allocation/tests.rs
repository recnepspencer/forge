use super::*;

fn ordinary_activated_target() -> UiActivatedScrollOwner {
    let (mut runtime, _, result, _, receipt, _, _, _) = crate::runtime::tests::production_catalog_activation_test_support::runtime_with_scroll_catalog();
    let mut target = None;
    let _ = runtime.execute_framework_turn(|turn| {
        turn.scroll_offset(|source| target = source.acquire_host_owner(&result, &receipt).ok());
    });
    target.expect("ordinary framework turn activates admitted scroll contract")
}

#[test]
fn pointer_rate_offsets_are_allocation_inert() {
    let (mut runtime, _, result, _, receipt, _, _, _) = crate::runtime::tests::production_catalog_activation_test_support::runtime_with_scroll_catalog();
    let mut target = None;
    let _ = runtime.execute_framework_turn(|turn| {
        turn.scroll_offset(|source| target = source.acquire_host_owner(&result, &receipt).ok());
    });
    let target = target.expect("framework turn issues exact scroll projection capability");
    assert_eq!(target.authority_probes(), 2);
    let counters = runtime
        .allocation_invalidation_index
        .borrow()
        .scroll_binding_counters();
    assert_eq!(counters.context_reads, 2);
    assert_eq!(counters.target_probes, 2);
    assert_eq!(counters.bindings_sealed, 2);
    assert_eq!(counters.duplicate_probes, 0);
    let dispatcher_before = runtime.allocation_frame_dispatcher_counters();
    let durable_before = runtime.durable_semantic_state();
    let truth_before = runtime.allocation_receipt_ledger.truth_revision();
    for sample in 0..1_000 {
        let input = UiProjectedScrollOffset::logical(target.clone(), 0.0, sample as f32).unwrap();
        let mut outcome = None;
        let _completion = runtime.execute_framework_turn(|turn| {
            turn.scroll_offset(|source| outcome = Some(source.project(input)));
        });
        let projected = outcome.unwrap().expect("activated scroll offset projects");
        assert_eq!(projected.projection_generation(), sample + 1);
        assert_eq!(projected.allocation_invalidations(), 0);
        assert_eq!(projected.committed_receipts(), 0);
    }
    assert_eq!(
        runtime
            .allocation_frame_dispatcher_counters()
            .ingress_count(),
        dispatcher_before.ingress_count()
    );
    assert_eq!(runtime.durable_semantic_state(), durable_before);
    assert_eq!(
        runtime.allocation_receipt_ledger.truth_revision(),
        truth_before
    );
    assert_eq!(runtime.scroll_offset_projection.generation(), 1_000);
}

#[test]
fn query_extent_owner_is_acquired_from_admitted_receipt() {
    let (mut runtime, _, result, query, receipt, _, _, _) = crate::runtime::tests::production_catalog_activation_test_support::runtime_with_scroll_catalog();
    let mut owner = None;
    let mut host_owner = None;
    let _ = runtime.execute_framework_turn(|turn| {
        turn.scroll_offset(|source| {
            owner = source.acquire_settled_query_owner(&query, &receipt).ok();
            host_owner = source.acquire_host_owner(&result, &receipt).ok();
        });
    });
    let owner = owner.expect("active Query settlement acquires its scroll owner");
    let host_owner = host_owner.expect("active host witness acquires its scroll owner");
    assert_eq!(owner.authority_probes(), 3);
    assert_eq!(owner.target().target(), host_owner.target().target());
    assert_ne!(
        owner.target().owner_identity(),
        host_owner.target().owner_identity()
    );
    let before = runtime.allocation_receipt_ledger.truth_revision();
    for sample in 0..1_000 {
        let offset = UiProjectedScrollOffset::logical(owner.clone(), sample as f32, 0.0).unwrap();
        let mut outcome = None;
        let _ = runtime.execute_framework_turn(|turn| {
            turn.scroll_offset(|source| outcome = Some(source.project(offset)));
        });
        let outcome = outcome.unwrap().expect("Query-owned offset projects");
        assert_eq!(outcome.allocation_invalidations(), 0);
        assert_eq!(outcome.committed_receipts(), 0);
    }
    assert_eq!(runtime.allocation_receipt_ledger.truth_revision(), before);
}

#[test]
fn unrelated_committed_receipt_cannot_smuggle_host_or_query_ownership() {
    let (mut runtime, _, result, query, _, unrelated_receipt, _, _) =
        crate::runtime::tests::production_catalog_activation_test_support::runtime_with_scroll_catalog();
    let mut host = None;
    let mut query_owner = None;
    let _ = runtime.execute_framework_turn(|turn| {
        turn.scroll_offset(|source| {
            host = Some(source.acquire_host_owner(&result, &unrelated_receipt));
            query_owner = Some(source.acquire_settled_query_owner(&query, &unrelated_receipt));
        });
    });
    assert_eq!(
        host.unwrap(),
        Err(crate::runtime::UiScrollOwnerAcquisitionDenial::ReceiptNotActive)
    );
    assert_eq!(
        query_owner.unwrap(),
        Err(crate::runtime::UiScrollOwnerAcquisitionDenial::ReceiptNotActive)
    );
    assert_eq!(runtime.scroll_offset_projection.generation(), 0);
}

#[test]
fn unknown_offset_target_denies_without_projection_mutation() {
    let inputs =
        crate::runtime::tests::activation_staging_test_support::activation_staging_inputs();
    let (mut runtime, _) = inputs.into_runtime_and_pending();
    let input = UiProjectedScrollOffset::logical(ordinary_activated_target(), 0.0, 1.0).unwrap();
    let mut outcome = None;
    let _ = runtime.execute_framework_turn(|turn| {
        turn.scroll_offset(|source| outcome = Some(source.project(input)));
    });
    assert_eq!(
        outcome.unwrap(),
        Err(UiProjectedScrollOffsetDenial::TargetNotActivated)
    );
    assert_eq!(runtime.scroll_offset_projection.generation(), 0);
}

#[test]
fn non_finite_offset_denies_before_projection() {
    assert_eq!(
        UiProjectedScrollOffset::logical(ordinary_activated_target(), f32::NAN, 0.0),
        Err(UiProjectedScrollOffsetDenial::NonFinite)
    );
}

#[test]
fn projection_generation_exhaustion_is_typed_and_does_not_replace_truth() {
    let (mut runtime, _, result, _, receipt, _, _, _) =
        crate::runtime::tests::production_catalog_activation_test_support::runtime_with_scroll_catalog();
    let mut owner = None;
    let _ = runtime.execute_framework_turn(|turn| {
        turn.scroll_offset(|source| owner = source.acquire_host_owner(&result, &receipt).ok());
    });
    runtime
        .scroll_offset_projection
        .exhaust_generation_for_test();
    let truth_before = runtime.allocation_receipt_ledger.truth_revision();
    let offset = UiProjectedScrollOffset::logical(
        owner.expect("ordinary activation seals the host scroll owner"),
        1.0,
        2.0,
    )
    .expect("finite offset is structurally valid");
    let mut outcome = None;
    let _ = runtime.execute_framework_turn(|turn| {
        turn.scroll_offset(|source| outcome = Some(source.project(offset)));
    });
    assert_eq!(
        outcome.expect("projection was attempted"),
        Err(UiProjectedScrollOffsetDenial::ProjectionGenerationExhausted)
    );
    assert_eq!(
        runtime.allocation_receipt_ledger.truth_revision(),
        truth_before
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
