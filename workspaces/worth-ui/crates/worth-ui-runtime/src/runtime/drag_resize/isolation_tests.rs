#[test]
fn same_frame_preview_is_isolated_before_terminal_commit() {
    let (mut runtime, root, input) = crate::runtime::tests::production_catalog_activation_test_support::runtime_with_durable_resize_catalog();
    let preview_extent = super::UiResizeLogicalExtent::try_from_logical_pixels(310.0).unwrap();
    let durable_extent = super::UiResizeLogicalExtent::try_from_logical_pixels(320.0).unwrap();
    let completion = runtime.execute_framework_turn(|turn| {
        turn.resize_preview(|source| {
            source
                .admit_and_submit(super::UiResizePreviewSample::new(root, preview_extent))
                .unwrap();
        });
        turn.durable_resize(|source| {
            source
                .admit_and_submit(super::UiDurableResizeCommitIntent::terminal(
                    input,
                    durable_extent,
                ))
                .unwrap();
        });
    });
    let mut host = |geometry: crate::host::UiHostPreviewPaintGeometry<'_>| {
        assert_eq!(geometry.extent(), preview_extent);
        Ok(())
    };
    let resolved = completion.resolve_preview_paint(&mut host).unwrap();
    let crate::runtime::UiPreviewPaintIsolationOutcome::Verified(isolation) = resolved.isolation()
    else {
        panic!("preview paint must preserve allocation truth")
    };
    assert_eq!(isolation.before(), isolation.after());
    assert_eq!(isolation.delta().durable_resize_mutations(), 0);
    assert_eq!(isolation.delta().committed_receipt_publications(), 0);
    assert_eq!(isolation.delta().durable_state_replacements(), 0);
    assert_eq!(isolation.durable_mutations(), 0);
    assert_eq!(isolation.committed_receipts(), 0);
    let durable = resolved
        .follow_on()
        .durable_resize_outcome()
        .expect("verified preview isolation admits durable follow-on");
    assert_eq!(durable.extent(), durable_extent);
    assert_eq!(durable.counters().durable_mutations(), 1);
    assert_eq!(durable.counters().committed_receipts(), 1);
}

#[test]
fn preview_verifies_before_delayed_durable_commit_denies_at_receipt_exhaustion() {
    let (mut runtime, root, input) = crate::runtime::tests::production_catalog_activation_test_support::runtime_with_durable_resize_catalog();
    let predecessor = runtime
        .allocation_receipt_ledger
        .position_truth_revision_for_test(1);
    let extent = super::UiResizeLogicalExtent::try_from_logical_pixels(340.0).unwrap();
    let completion = runtime.execute_framework_turn(|turn| {
        turn.resize_preview(|source| {
            source
                .admit_and_submit(super::UiResizePreviewSample::new(root, extent))
                .unwrap();
        });
        turn.durable_resize(|source| {
            source
                .admit_and_submit(super::UiDurableResizeCommitIntent::terminal(input, extent))
                .unwrap();
        });
    });
    let mut host = |_geometry: crate::host::UiHostPreviewPaintGeometry<'_>| Ok(());
    let resolved = completion.resolve_preview_paint(&mut host).unwrap();
    let crate::runtime::UiPreviewPaintIsolationOutcome::Verified(isolation) = resolved.isolation()
    else {
        panic!("preview remains isolated")
    };
    assert_eq!(isolation.committed_receipts(), 0);
    assert_eq!(isolation.durable_mutations(), 0);
    let denial = resolved
        .follow_on()
        .durable_resize_denial()
        .expect("delayed commit denies");
    let crate::runtime::UiAllocationReplanTransactionCommitDenial::AuthorityCounterExhausted(
        exhaustion,
    ) = denial.denial()
    else {
        panic!("typed exhaustion required")
    };
    assert_eq!(
        exhaustion.counter(),
        crate::runtime::UiAllocationAuthorityCounter::TruthRevision
    );
    assert_eq!(denial.counters().committed_receipts(), 0);
    assert_eq!(denial.counters().durable_mutations(), 0);
    assert_eq!(
        runtime.allocation_receipt_ledger.ledger_baseline_for_test(),
        predecessor
    );
}
