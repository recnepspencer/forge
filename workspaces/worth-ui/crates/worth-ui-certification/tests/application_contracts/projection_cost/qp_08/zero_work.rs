use worth_ui::facade::app::{
    UiMountedFrameOutcome, UiMountedFrameRequest, UiPresentationDeadline, WorthUi,
};
use worth_ui_test_support::WorthUiActiveSessionCertificationExt;

#[test]
fn query_free_turn_has_zero_query_and_content_work() {
    let app = WorthUi::app()
        .bind_certification_host_adapter(
            worth_ui_host_contract::UiCertificationHostBindingGrant::for_certification(),
            worth_ui_host_headless::WorthUiHeadlessHost,
        )
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .expect("Query-free application prepares");
    let mut session = app.launch().expect("Query-free application launches");
    let outcome = session
        .execute_mounted_frame(
            UiMountedFrameRequest::all_bound_surfaces(),
            UiPresentationDeadline::at_tick(1),
            0,
            |_| {},
        )
        .unwrap_or_else(|_| panic!("Query-free mounted-frame route executes"));
    let UiMountedFrameOutcome::RejectedBeforeEffects(rejection) = outcome else {
        panic!("the empty Query-free world stops before host effects");
    };
    assert_zero_mount_work(rejection.cost_report());

    let query = session.inspect_query_state_residue();
    assert!(!query.query_installed());
    assert_eq!(query.scanned_query_bindings(), 0);
    assert_eq!(query.scanned_plan_query_links(), 0);
    assert_eq!(query.scanned_settled_snapshots(), 0);
    assert_eq!(query.scanned_live_resources(), 0);
    assert_eq!(query.operation_live_subsystem_construction_count(), 0);
    assert_eq!(query.operation_live_succession_operation_count(), 0);
    assert!(query.is_clean());
    let _ = session.shutdown();
}

fn assert_zero_mount_work(cost: worth_ui_runtime::facade::mounted::UiMountCostReport) {
    assert_eq!(cost.initial_mounted_instances(), 0);
    assert_eq!(cost.changed_mounted_instances(), 0);
    assert_eq!(cost.index_entries_touched(), 0);
    assert_eq!(cost.replaced_batch_rows(), 0);
    assert_eq!(cost.replaced_batch_bytes(), 0);
    assert_eq!(cost.surface_instance_pairs(), 0);
    assert_eq!(cost.changed_binding_generations(), 0);
    assert_eq!(cost.adapter().presented_surfaces(), 0);
}
