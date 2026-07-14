use super::*;

#[test]
fn absent_probe_issues_key_and_source_bound_absence_after_real_leaf_selection() {
    let catalog = admitted_layout_bootstrap_catalog();
    let security = admitted_tenant_page_security_scope_for_layout_partition_test();
    let executed = layout_read_runtime()
        .execute_page_lookup(PageLookupRequest::new(
            &catalog,
            security.witnesses(),
            segment(7),
            page(9),
            forge_store_physical_format::PhysicalRecordSlot::from_raw(15).unwrap(),
            PreExecutionBudgetEnvelope::foreground_default(),
            ordinary_source(),
        ))
        .unwrap()
        .into_result()
        .unwrap();
    let BTreeLookupExecutionView::Absent(absence) = executed.view() else {
        panic!("missing probe must issue the absent case")
    };
    assert_eq!(absence.probe_slot().get(), 15);
    let receipt = executed.counter_receipt();
    assert_eq!(
        receipt
            .plan_binding()
            .materialization()
            .expect("executed B-tree plan retains materialization")
            .source(),
        executed
            .current_materialization()
            .materialization()
            .source(),
    );
    assert!(absence.selected_leaf().page_id().is_some());
    assert_eq!(receipt.observation(), PlannedCounterObservation::Exact);
    assert_eq!(receipt.observed().allocation_events(), 4);
    assert_eq!(receipt.observed().page_touches(), 2);
}
