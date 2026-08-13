use std::collections::BTreeSet;
use syn::visit::Visit;

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

    assert_port_outputs_are_mechanical(&inventory);

    let vendor_homes = inventory
        .rust_files_under("crates/worth-ui-host-native/src")
        .filter(|source| {
            !source
                .relative_path()
                .file_stem()
                .is_some_and(|stem| stem.to_string_lossy().ends_with("_tests"))
        })
        .filter(|source| uses_native_vendor_path(source.text()))
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

fn assert_port_outputs_are_mechanical(
    inventory: &worth_ui_certification::topology::WorkspaceSourceInventory,
) {
    for path in [
        "crates/worth-ui-host-native/src/native/event_loop/window_port.rs",
        "crates/worth-ui-host-native/src/native/graphics/port.rs",
        "crates/worth-ui-host-native/src/native/presentation/port.rs",
        "crates/worth-ui-host-native/src/native/presentation/readback_port.rs",
    ] {
        let source = inventory.source(path).expect("external port source").text();
        for forbidden in [
            "UiHostSurfacePresentationOutcome",
            "UiNativeEffectPosture",
            "RejectedBeforeEffects",
            "PresentationIndeterminate",
            "InputRejected",
            "CapacityUnavailable",
        ] {
            assert!(!source.contains(forbidden), "{path} mints {forbidden}");
        }
    }
    for (path, denial) in [
        (
            "crates/worth-ui-host-native/src/native/event_loop/window_port.rs",
            "UiNativeWindowPortDenial",
        ),
        (
            "crates/worth-ui-host-native/src/native/graphics/port.rs",
            "UiNativeGraphicsPortDenial",
        ),
        (
            "crates/worth-ui-host-native/src/native/presentation/port.rs",
            "UiNativePresentationPortFailure",
        ),
        (
            "crates/worth-ui-host-native/src/native/presentation/readback_port.rs",
            "UiNativeReadbackFailure",
        ),
    ] {
        assert!(
            inventory
                .source(path)
                .expect("external port source")
                .text()
                .contains(denial),
            "{path} omits named mechanical failure {denial}"
        );
    }
}

fn assert_orchestration_crosses_each_port_once(
    inventory: &worth_ui_certification::topology::WorkspaceSourceInventory,
) {
    for (path, call) in [
        (
            "crates/worth-ui-host-native/src/native/event_loop.rs",
            "UiWinitNativeWindowPort::open",
        ),
        (
            "crates/worth-ui-host-native/src/native/event_loop.rs",
            "UiWgpuNativeGraphicsPort::prepare",
        ),
        (
            "crates/worth-ui-host-native/src/native/presentation.rs",
            "Port::present",
        ),
        (
            "crates/worth-ui-host-native/src/native/presentation/port/transaction.rs",
            "UiWgpuNativeReadbackPort::read_two_pixels",
        ),
    ] {
        let source = inventory.source(path).expect("native orchestration owner");
        assert_eq!(
            rust_call_count(source.text(), call),
            1,
            "{path} must cross {call} exactly once"
        );
    }

    assert_only_port_owner_calls(
        inventory,
        "transaction::present",
        "crates/worth-ui-host-native/src/native/presentation/port.rs",
    );
    for call in [
        "draw_rectangle",
        "retained_transfer",
        "draw_retained_to_surface",
        "copy_evidence_pixels",
    ] {
        assert_only_port_owner_calls(
            inventory,
            call,
            "crates/worth-ui-host-native/src/native/presentation/port/transaction.rs",
        );
    }
}

fn assert_only_port_owner_calls(
    inventory: &worth_ui_certification::topology::WorkspaceSourceInventory,
    call: &str,
    owner: &str,
) {
    let callers = inventory
        .rust_files_under("crates/worth-ui-host-native/src/native")
        .filter(|source| rust_call_count(source.text(), call) > 0)
        .map(|source| source.relative_path().to_string_lossy().replace('\\', "/"))
        .collect::<BTreeSet<_>>();
    assert_eq!(callers, BTreeSet::from([owner.to_owned()]));
}

fn rust_call_count(source: &str, expected: &str) -> usize {
    struct Calls<'a> {
        expected: &'a str,
        count: usize,
    }
    impl<'ast> Visit<'ast> for Calls<'_> {
        fn visit_expr_call(&mut self, call: &'ast syn::ExprCall) {
            if let syn::Expr::Path(path) = call.func.as_ref() {
                let observed = path
                    .path
                    .segments
                    .iter()
                    .map(|segment| segment.ident.to_string())
                    .collect::<Vec<_>>()
                    .join("::");
                self.count += usize::from(observed.ends_with(self.expected));
            }
            syn::visit::visit_expr_call(self, call);
        }
    }
    let syntax = syn::parse_file(source).expect("native source parses");
    let mut calls = Calls { expected, count: 0 };
    calls.visit_file(&syntax);
    calls.count
}

fn uses_native_vendor_path(source: &str) -> bool {
    struct VendorPaths(bool);
    impl<'ast> Visit<'ast> for VendorPaths {
        fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
            let test_only = item.attrs.iter().any(|attribute| {
                attribute.path().is_ident("cfg")
                    && attribute.meta.require_list().is_ok_and(|list| {
                        list.tokens
                            .to_string()
                            .split_whitespace()
                            .any(|token| token == "test")
                    })
            });
            if !test_only {
                syn::visit::visit_item_mod(self, item);
            }
        }

        fn visit_path(&mut self, path: &'ast syn::Path) {
            self.0 |= path
                .segments
                .first()
                .is_some_and(|segment| segment.ident == "wgpu" || segment.ident == "winit");
            syn::visit::visit_path(self, path);
        }
    }
    let syntax = syn::parse_file(source).expect("native source parses");
    let mut paths = VendorPaths(false);
    paths.visit_file(&syntax);
    paths.0
}
