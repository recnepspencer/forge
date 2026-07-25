use worth_ui_query_binding::certification::WorthUiInstalledQueryTestFixture;

#[test]
fn per_frame_settlement_resolution_is_pointer_stable_and_allocation_free() {
    let mut fixture = WorthUiInstalledQueryTestFixture::new("settlement-reference-cost");
    let mut binding = fixture.binding_plan().prepare_downstream_state();
    let projection = fixture.settle_snapshot();
    let reference = projection.installed_reference().clone();
    let derived_fact_address =
        projection.fact() as *const worth_ui_query_binding::WorthUiSettledSnapshotFact;
    let retained = binding
        .admit_settled_snapshot(projection)
        .expect("the exact projection admits into its binding owner");

    assert_eq!(std::sync::Arc::as_ptr(&retained), derived_fact_address);

    let mut resolved = None;
    let allocations = allocation_counter::measure(|| {
        resolved = Some(
            binding
                .settled_snapshot_fact_reference_for(&reference)
                .expect("the retained binding resolves by its exact indexed reference"),
        );
    });
    let resolved = resolved.expect("the allocation observer executes the lookup");

    assert_eq!(allocations.count_total, 0);
    assert!(std::sync::Arc::ptr_eq(&retained, &resolved));
}
