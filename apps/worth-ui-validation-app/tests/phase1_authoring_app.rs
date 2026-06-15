use worth_ui_validation_app::{
    ValidationWorkbenchApp, ValidationWorkbenchLaunch, ValidationWorkbenchSnapshot,
};

#[test]
fn validation_app_launches_phase1_authoring_sample_through_runtime_facade() {
    let app = ValidationWorkbenchApp::new(
        ValidationWorkbenchLaunch::new()
            .prepare()
            .expect("validation app should prepare from the phase 1 authoring sample"),
    );

    let snapshot = app.snapshot();
    assert_phase1_snapshot(snapshot);
}

#[test]
fn validation_app_launch_snapshot_matches_prepared_launch_snapshot() {
    let launch = ValidationWorkbenchLaunch::new()
        .prepare()
        .expect("validation app should prepare from the phase 1 authoring sample");

    assert_phase1_snapshot(launch.snapshot());
}

fn assert_phase1_snapshot(snapshot: ValidationWorkbenchSnapshot) {
    assert_eq!(snapshot.app_name(), "ShopifyAdminApp");
    assert_eq!(snapshot.workspace_name(), "AdminWorkspace");
    assert_eq!(snapshot.page_count(), 4);
    assert_eq!(snapshot.dynamic_page_count(), 2);
    assert_ne!(snapshot.artifact_digest(), 0);
    assert_ne!(snapshot.active_plan_digest(), 0);
}
