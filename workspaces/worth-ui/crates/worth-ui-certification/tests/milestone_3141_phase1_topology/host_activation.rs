use std::collections::BTreeSet;

#[test]
fn phase_one_removes_product_host_replacement_and_forged_native_host_lanes() {
    let inventory = super::workspace_source_inventory();
    let consumption = inventory
        .source("crates/worth-ui-runtime/src/mounting/presentation/consumption_view.rs")
        .expect("mounted consumption view");
    assert!(!consumption.text().contains("fn projection("));
    assert!(!consumption.text().contains("UiMountedProjectionView"));
    assert_host_neutral_builder(&inventory);
    assert_exact_host_transitions(&inventory);
    assert_native_platform_binding(&inventory);
}

fn assert_host_neutral_builder(
    inventory: &worth_ui_certification::topology::WorkspaceSourceInventory,
) {
    for path in [
        "crates/worth-ui-runtime/src/facade/entry/app_builder.rs",
        "crates/worth-ui-runtime/src/facade/entry/app_builder/freeze.rs",
    ] {
        let builder = inventory
            .source(path)
            .expect("host-neutral application builder");
        for forbidden in [
            "fn with_host<",
            "fn bind_",
            "UiApplicationBuilderDefaultHost",
            "UiApplicationHostBound",
            "UiApplicationHostUnbound",
            "WorthUiHostSessionPlan",
            "worth_ui_host_",
            "bind_native_platform_host",
            "UiNativePlatformBindingGrant",
        ] {
            assert!(
                !builder.text().contains(forbidden),
                "{path} retains {forbidden}"
            );
        }
    }
}

fn assert_exact_host_transitions(
    inventory: &worth_ui_certification::topology::WorkspaceSourceInventory,
) {
    let entry_root = "crates/worth-ui-runtime/src/facade/entry";
    for (file, host) in [
        (
            "certification_application_transition.rs",
            "WorthUiHeadlessRecorder",
        ),
        ("legacy_egui_application_transition.rs", "WorthUiHostEgui"),
    ] {
        let source = inventory
            .source(format!("{entry_root}/{file}"))
            .expect("exact host transition");
        assert!(source.text().contains("application.bind_exact_host"));
        assert!(source.text().contains(host));
    }
    let homes = inventory
        .rust_files_under(entry_root)
        .filter(|source| source.text().contains("bind_exact_host"))
        .map(|source| source.relative_path().to_string_lossy().replace('\\', "/"))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        homes,
        BTreeSet::from([
            format!("{entry_root}/certification_application_transition.rs"),
            format!("{entry_root}/host_neutral_app.rs"),
            format!("{entry_root}/legacy_egui_application_transition.rs"),
        ])
    );
    let owner = inventory
        .source(format!("{entry_root}/host_neutral_app.rs"))
        .unwrap();
    assert_private_authority_fields(owner.text());
    assert_exact_host_import_homes(inventory, entry_root);
}

fn assert_exact_host_import_homes(
    inventory: &worth_ui_certification::topology::WorkspaceSourceInventory,
    entry_root: &str,
) {
    for (host_crate, exact_home) in [
        (
            "worth_ui_host_headless",
            format!("{entry_root}/certification_application_transition.rs"),
        ),
        (
            "worth_ui_host_egui",
            format!("{entry_root}/legacy_egui_application_transition.rs"),
        ),
    ] {
        let observed = inventory
            .rust_files_under(entry_root)
            .filter(|source| source.text().contains(host_crate))
            .map(|source| source.relative_path().to_string_lossy().replace('\\', "/"))
            .collect::<BTreeSet<_>>();
        assert_eq!(observed, BTreeSet::from([exact_home]));
    }
}

fn assert_private_authority_fields(source: &str) {
    let syntax = syn::parse_file(source).expect("host-neutral application parses");
    let owner = syntax
        .items
        .iter()
        .find_map(|item| match item {
            syn::Item::Struct(item) if item.ident == "WorthUiHostNeutralApp" => Some(item),
            _ => None,
        })
        .expect("host-neutral application owner struct");
    let syn::Fields::Named(fields) = &owner.fields else {
        panic!("named fields required")
    };
    for expected in [
        "prepared",
        "mounted_frame_retention_budget",
        "host_observation_capacity",
    ] {
        let field = fields
            .named
            .iter()
            .find(|field| field.ident.as_ref().is_some_and(|name| name == expected))
            .unwrap_or_else(|| panic!("host-neutral application omits {expected}"));
        assert!(matches!(field.vis, syn::Visibility::Inherited));
    }
}

fn assert_native_platform_binding(
    inventory: &worth_ui_certification::topology::WorkspaceSourceInventory,
) {
    let native = inventory
        .source("crates/worth-ui-host-native/src/prepared_host.rs")
        .unwrap();
    assert!(!native
        .text()
        .contains("#[derive(Clone)]\npub struct WorthUiPreparedNativeHost"));
    assert!(!native.text().contains("from_platform_binding"));
    let binding = inventory
        .source("crates/worth-ui-runtime/src/native_platform/native_platform_binding.rs")
        .unwrap();
    assert!(binding
        .text()
        .contains("pub(crate) struct UiNativePlatformBindingGrant"));
    assert!(!binding.text().contains("#[derive(Clone"));
    let app = inventory
        .source("crates/worth-ui-runtime/src/facade/entry/host_neutral_app.rs")
        .unwrap();
    assert!(app.text().contains("pub(crate) fn bind_qualified_native"));
    assert!(!app.text().contains("pub fn bind_qualified_native"));
    super::assert_exact_symbol_homes(inventory, &["bind_qualified", "_native("].concat(), &[
        "crates/worth-ui-runtime/src/facade/entry/host_neutral_app.rs",
        "crates/worth-ui-runtime/src/native_platform/application.rs",
        "crates/worth-ui-runtime/src/native_platform/platform.rs",
        "crates/worth-ui/tests/ui/facade/construction/host_binding/product_cannot_bind_native_host.rs",
    ]);
}
