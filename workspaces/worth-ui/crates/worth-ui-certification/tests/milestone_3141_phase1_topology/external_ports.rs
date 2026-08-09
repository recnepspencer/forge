use std::collections::BTreeSet;

#[test]
fn phase_two_external_effect_ports_are_concrete_and_vendor_scoped() {
    let inventory = super::workspace_source_inventory();
    for (path, contract, implementation) in [
        (
            "crates/worth-ui-host-native/src/native/event_loop/window_port.rs",
            "trait UiNativeWindowPort",
            "impl UiNativeWindowPort for UiWinitNativeWindowPort",
        ),
        (
            "crates/worth-ui-host-native/src/native/graphics/port.rs",
            "trait UiNativeGraphicsPort",
            "impl UiNativeGraphicsPort for UiWgpuNativeGraphicsPort",
        ),
        (
            "crates/worth-ui-host-native/src/native/presentation/port.rs",
            "trait UiNativePresentationPort",
            "impl UiNativePresentationPort for UiWgpuNativePresentationPort",
        ),
        (
            "crates/worth-ui-host-native/src/native/presentation/readback_port.rs",
            "trait UiNativeReadbackPort",
            "impl UiNativeReadbackPort for UiWgpuNativeReadbackPort",
        ),
    ] {
        let source = inventory.source(path).expect("external effect port owner");
        assert!(source.text().contains(contract), "{path} omits {contract}");
        assert!(
            source.text().contains(implementation),
            "{path} omits {implementation}"
        );
    }

    let vendor_homes = inventory
        .rust_files_under("crates/worth-ui-host-native/src")
        .filter(|source| source.text().contains("wgpu::") || source.text().contains("winit::"))
        .map(|source| source.relative_path().to_string_lossy().replace('\\', "/"))
        .collect::<BTreeSet<_>>();
    assert!(vendor_homes.iter().all(|path| {
        path.starts_with("crates/worth-ui-host-native/src/native/graphics")
            || path.starts_with("crates/worth-ui-host-native/src/native/presentation")
            || path.starts_with("crates/worth-ui-host-native/src/native/event_loop")
            || path == "crates/worth-ui-host-native/src/qualification_tests.rs"
    }));

    assert_orchestration_crosses_each_port_once(&inventory);
}

fn assert_orchestration_crosses_each_port_once(
    inventory: &worth_ui_certification::topology::WorkspaceSourceInventory,
) {
    for (path, call) in [
        (
            "crates/worth-ui-host-native/src/native/event_loop.rs",
            "UiWinitNativeWindowPort::open(",
        ),
        (
            "crates/worth-ui-host-native/src/native/event_loop.rs",
            "UiWgpuNativeGraphicsPort::prepare(",
        ),
        (
            "crates/worth-ui-host-native/src/native/mechanics_adapter.rs",
            "UiWgpuNativePresentationPort::present(",
        ),
        (
            "crates/worth-ui-host-native/src/native/presentation.rs",
            "UiWgpuNativeReadbackPort::read_two_pixels(",
        ),
    ] {
        let source = inventory.source(path).expect("native orchestration owner");
        assert_eq!(
            source.text().matches(call).count(),
            1,
            "{path} must cross {call} exactly once"
        );
    }

    assert_only_port_owner_calls(
        inventory,
        "UiNativeGraphics::prepare(",
        "crates/worth-ui-host-native/src/native/graphics/port.rs",
    );
    assert_only_port_owner_calls(
        inventory,
        "present_initial(",
        "crates/worth-ui-host-native/src/native/presentation/port.rs",
    );
}

fn assert_only_port_owner_calls(
    inventory: &worth_ui_certification::topology::WorkspaceSourceInventory,
    call: &str,
    owner: &str,
) {
    let callers = inventory
        .rust_files_under("crates/worth-ui-host-native/src/native")
        .filter(|source| source.text().contains(call))
        .filter(|source| !source.text().contains(&format!("fn {call}")))
        .map(|source| source.relative_path().to_string_lossy().replace('\\', "/"))
        .collect::<BTreeSet<_>>();
    assert_eq!(callers, BTreeSet::from([owner.to_owned()]));
}
