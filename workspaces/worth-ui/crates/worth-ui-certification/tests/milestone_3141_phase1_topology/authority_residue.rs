use std::collections::BTreeSet;

#[test]
fn baseline_receipt_construction_has_one_private_host_truth_owner() {
    let inventory = super::workspace_source_inventory();
    let constructors = inventory
        .rust_files_under("crates/worth-ui-runtime/src")
        .filter(|source| source.text().contains("UiMountedSurfaceBaselineReceipt {"))
        .map(|source| source.relative_path().to_string_lossy().replace('\\', "/"))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        constructors,
        BTreeSet::from([
            "crates/worth-ui-runtime/src/mounting/host_truth/surface_lifecycle.rs".to_owned()
        ])
    );
    let owner = inventory
        .source("crates/worth-ui-runtime/src/mounting/host_truth/surface_lifecycle.rs")
        .expect("baseline response owner");
    assert!(!owner.text().contains("derive(Clone"));
    assert!(!owner.text().contains("derive(Copy"));
    assert!(!owner.text().contains("pub(crate) fn issue"));
}

#[test]
fn phase_two_close_surface_is_affine_and_bound_to_the_real_native_report() {
    let inventory = super::workspace_source_inventory();
    let outcome = inventory
        .source("crates/worth-ui-runtime/src/native_platform/outcome.rs")
        .expect("native Phase 2 outcome owner");
    assert!(outcome
        .text()
        .contains("pub struct UiNativePlatformCloseReceipt"));
    assert!(outcome
        .text()
        .contains("report: worth_ui_host_native::UiNativeEventLoopRunReport"));
    assert!(outcome
        .text()
        .contains("Closed(UiNativePlatformCloseReceipt)"));
    assert!(!outcome.text().contains("terminal_resource_count"));
    assert!(outcome.text().contains("UiNativePlatformStopReport"));
    assert!(outcome.text().contains("effect_posture"));
    assert!(outcome.text().contains("terminal_census"));
    assert!(!outcome
        .text()
        .contains("#[derive(Clone, Copy)]\npub struct UiNativePlatformCloseReceipt"));

    let state = inventory
        .source("crates/worth-ui-host-native/src/native/resource_census.rs")
        .expect("real native resource census owner");
    for field in [
        "windows",
        "surfaces",
        "adapters",
        "devices",
        "queues",
        "retained_targets",
        "registrations",
        "readback_buffers",
        "pending_submissions",
        "event_wake_registrations",
        "application_drivers",
    ] {
        assert!(state.text().contains(&format!("pub {field}: usize")));
    }
}
