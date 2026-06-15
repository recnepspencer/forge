use worth_ui_validation_app::{
    ValidationDynamicPageRequest, ValidationStaticPageId, ValidationWorkbenchLaunch,
    ValidationWorkspaceShell,
};

#[test]
fn shell_restore_snapshot_rehydrates_geometry_and_navigation_on_fresh_launch() {
    let launch = ValidationWorkbenchLaunch::new()
        .prepare()
        .expect("validation launch should prepare");
    let mut shell = ValidationWorkspaceShell::from_launch(launch);

    shell.set_rail_width(301.0);
    shell.set_inspector_width(377.0);
    shell.set_status_height(144.0);
    let baseline_snapshot = shell.snapshot();

    shell.select_static_page(ValidationStaticPageId::Products);
    shell
        .open_dynamic_page(
            ValidationDynamicPageRequest::product_detail("P-1001")
                .expect("product detail request should be valid"),
        )
        .expect("product detail request should open");
    let restore_snapshot = shell.capture_restore_snapshot();

    let restored_launch = ValidationWorkbenchLaunch::new()
        .prepare()
        .expect("fresh validation launch should prepare");
    let restored_shell = ValidationWorkspaceShell::from_launch_with_restore_snapshot(
        restored_launch,
        restore_snapshot.clone(),
    );

    assert_eq!(restored_shell.capture_restore_snapshot(), restore_snapshot);
    assert_eq!(restored_shell.rail_width(), 301.0);
    assert_eq!(restored_shell.inspector_width(), 377.0);
    assert_eq!(restored_shell.status_height(), 144.0);
    assert_eq!(restored_shell.snapshot(), baseline_snapshot);
}
