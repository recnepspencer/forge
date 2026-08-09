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
fn phase_one_native_outcome_has_no_future_cleanup_or_census_surface() {
    let inventory = super::workspace_source_inventory();
    for path in [
        "crates/worth-ui-native-platform/src/outcome.rs",
        "crates/worth-ui-native-platform/src/lib.rs",
    ] {
        let source = inventory
            .source(path)
            .expect("native Phase 1 outcome owner");
        for forbidden in [
            "UiNativePlatformCloseReceipt",
            "terminal_resource_count",
            "ApplicationCleanup",
            "Closed(",
        ] {
            assert!(
                !source.text().contains(forbidden),
                "{path} exposes future cleanup surface {forbidden}"
            );
        }
    }
}
